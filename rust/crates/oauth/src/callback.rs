//! HTTP callback server for OAuth redirect.

use std::{sync::Arc, time::Duration};

use poem::{EndpointExt, Route, Server, handler, web::Query};
use serde::Deserialize;
use tokio::{sync::oneshot, time::timeout};

use crate::error::{OAuthError, OAuthResult};

/// Time to wait for the user to complete the browser authorization flow.
const AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

/// Longest error text taken from the redirect (in characters).
const MAX_ERROR_LEN: usize = 512;

/// The authorization code, or the error a redirect reported.
type CallbackResult = std::result::Result<String, String>;
type CallbackTx = Arc<tokio::sync::Mutex<Option<oneshot::Sender<CallbackResult>>>>;

/// The CSRF state of the authorization request being waited for.
#[derive(Clone)]
struct ExpectedState(Arc<String>);

/// The error text comes from the redirect query string, so it must not be
/// echoed into the page as raw HTML.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Make redirect-supplied text safe to put in an error, a log line or a
/// terminal: control characters (including ANSI escape sequences), line and
/// paragraph separators, zero-width characters and bidirectional overrides
/// become spaces, and the length is capped.
fn sanitize(s: &str) -> String {
    let mut out: String = s
        .chars()
        .take(MAX_ERROR_LEN)
        .map(|c| if is_unsafe_char(c) { ' ' } else { c })
        .collect();
    if s.chars().count() > MAX_ERROR_LEN {
        out.push('…');
    }
    out
}

/// Characters that can hide or reorder text in a terminal or log.
fn is_unsafe_char(c: char) -> bool {
    c.is_control()
        || matches!(
            c,
            '\u{200b}'..='\u{200f}'
                | '\u{2028}'..='\u{202e}'
                | '\u{2060}'..='\u{2069}'
                | '\u{feff}'
        )
}

/// Build the error message for an error redirect (RFC 6749 §4.1.2.1):
/// `error`, followed by the human-readable `error_description` when present.
fn error_message(error: &str, description: Option<&str>) -> String {
    match description.filter(|d| !d.is_empty()) {
        Some(desc) => format!("{}: {}", sanitize(error), sanitize(desc)),
        None => sanitize(error),
    }
}

/// Runs the local HTTP server until the OAuth redirect is received or timeout,
/// and returns the authorization code.
///
/// The redirect's `state` is checked against `expected_state` before anything
/// else, including error redirects. A redirect with a wrong or missing state
/// is answered with an error page and otherwise ignored: the server keeps
/// waiting for the real redirect, so another page can neither end the flow nor
/// inject an error text of its choosing.
pub(crate) async fn wait_for_callback(
    acceptor: poem::listener::TcpAcceptor,
    expected_state: &str,
) -> OAuthResult<String> {
    #[derive(Deserialize)]
    struct CallbackParams {
        code: Option<String>,
        state: Option<String>,
        error: Option<String>,
        error_description: Option<String>,
    }

    const STYLE: &str = "<style>html { \
            font-family: system-ui, -apple-system, BlinkMacSystemFont, \
            sans-serif; font-size: 16px; color: #e0e0e0; background: #202020; \
            padding: 2rem; text-align: center; } </style>";

    let (tx, rx) = oneshot::channel::<CallbackResult>();
    let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

    #[handler]
    async fn callback_handler(
        Query(params): Query<CallbackParams>,
        tx: poem::web::Data<&CallbackTx>,
        poem::web::Data(expected): poem::web::Data<&ExpectedState>,
    ) -> poem::Response {
        if params.state.as_deref() != Some(expected.0.as_str()) {
            tracing::warn!("ignoring OAuth callback with a mismatched state, possible CSRF attack");
            return html_response(
                poem::http::StatusCode::BAD_REQUEST,
                "<h1>Not the pending authorization</h1>\
                 <p>This page belongs to an earlier or different authorization \
                 request. Finish the authorization in the most recently opened \
                 window.</p>",
            );
        }

        let result = if let Some(error) = params.error {
            Err(error_message(&error, params.error_description.as_deref()))
        } else if let Some(code) = params.code {
            Ok(code)
        } else {
            Err("Missing authorization code".to_string())
        };

        let response = match &result {
            Ok(_) => html_response(
                poem::http::StatusCode::OK,
                "<h1>✓ Authorization Successful!</h1>\
                 <p>You can close this window and return to the terminal.</p>",
            ),
            Err(err) => html_response(
                poem::http::StatusCode::BAD_REQUEST,
                &format!(
                    "<h1>Authorization Failed</h1><p>Error: {}</p>",
                    escape_html(err)
                ),
            ),
        };

        if let Some(sender) = tx.lock().await.take() {
            let _ = sender.send(result);
        }
        response
    }

    fn html_response(status: poem::http::StatusCode, content: &str) -> poem::Response {
        poem::Response::builder()
            .status(status)
            .content_type("text/html; charset=utf-8")
            .body(format!("<html><body>{STYLE}{content}</body></html>"))
    }

    let app = Route::new()
        .at("/callback", poem::get(callback_handler))
        .data(tx)
        .data(ExpectedState(Arc::new(expected_state.to_string())));

    let server_task = tokio::spawn(
        Server::new_with_acceptor(acceptor).run_with_graceful_shutdown(
            app,
            async move {
                futures_util::future::pending::<()>().await;
            },
            None,
        ),
    );

    tracing::debug!(
        "waiting for OAuth callback (timeout: {}s)",
        AUTH_TIMEOUT.as_secs()
    );
    let result = match timeout(AUTH_TIMEOUT, rx).await {
        Ok(Ok(Ok(code))) => Ok(code),
        Ok(Ok(Err(message))) => {
            tracing::warn!(error = %message, "OAuth authorization failed at callback");
            Err(OAuthError::AuthorizationFailed { message })
        }
        Ok(Err(_)) => {
            tracing::error!("OAuth callback channel closed unexpectedly");
            Err(OAuthError::CallbackChannelClosed)
        }
        Err(_) => {
            tracing::warn!(
                timeout_secs = AUTH_TIMEOUT.as_secs(),
                "OAuth authorization timed out waiting for callback"
            );
            Err(OAuthError::AuthorizationTimeout)
        }
    };

    server_task.abort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_escapes_markup() {
        assert_eq!(
            escape_html(r#"<script>alert("x")</script> & 'y'"#),
            "&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt; &amp; &#39;y&#39;"
        );
    }

    #[test]
    fn error_message_appends_description() {
        assert_eq!(
            error_message(
                "internal_server_error",
                Some("validate client err:oauth client not found")
            ),
            "internal_server_error: validate client err:oauth client not found"
        );
        assert_eq!(error_message("access_denied", Some("")), "access_denied");
        assert_eq!(error_message("access_denied", None), "access_denied");
    }

    #[test]
    fn error_message_strips_control_characters_and_caps_length() {
        assert_eq!(
            error_message("bad\u{1b}[31m", Some("line1\nline2")),
            "bad [31m: line1 line2"
        );
        assert_eq!(
            error_message("a\u{202e}b\u{2028}c\u{200b}d", None),
            "a b c d"
        );
        let long = "x".repeat(MAX_ERROR_LEN + 10);
        let message = error_message(&long, None);
        assert_eq!(message.chars().count(), MAX_ERROR_LEN + 1);
        assert!(message.ends_with('…'));
    }
}
