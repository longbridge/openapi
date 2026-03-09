//! HTTP callback server for OAuth redirect.

use std::{sync::Arc, time::Duration};

use poem::{EndpointExt, Route, Server, handler, web::Query};
use serde::Deserialize;
use tokio::{sync::oneshot, time::timeout};

use crate::error::{OAuthError, OAuthResult};

/// Time to wait for the user to complete the browser authorization flow.
const AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

type CallbackTx =
    Arc<tokio::sync::Mutex<Option<oneshot::Sender<std::result::Result<(String, String), String>>>>>;

/// Runs the local HTTP server until the OAuth redirect is received or timeout.
pub(crate) async fn wait_for_callback(
    acceptor: poem::listener::TcpAcceptor,
) -> OAuthResult<(String, String)> {
    #[derive(Deserialize)]
    struct CallbackParams {
        code: Option<String>,
        state: Option<String>,
        error: Option<String>,
    }

    const STYLE: &str = "<style>html { \
            font-family: system-ui, -apple-system, BlinkMacSystemFont, \
            sans-serif; font-size: 16px; color: #e0e0e0; background: #202020; \
            padding: 2rem; text-align: center; } </style>";

    let (tx, rx) = oneshot::channel::<std::result::Result<(String, String), String>>();
    let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

    #[handler]
    async fn callback_handler(
        Query(params): Query<CallbackParams>,
        tx: poem::web::Data<&CallbackTx>,
    ) -> poem::Response {
        let result = if let Some(err) = params.error {
            Err(err)
        } else if let (Some(code), Some(state)) = (params.code, params.state) {
            Ok((code, state))
        } else {
            Err("Missing authorization code or state".to_string())
        };

        let (status, body) = match &result {
            Ok(_) => (
                poem::http::StatusCode::OK,
                format!(
                    "<html><body>{STYLE}<h1>✓ Authorization Successful!</h1>\
                     <p>You can close this window and return to the terminal.</p></body></html>"
                ),
            ),
            Err(err) => (
                poem::http::StatusCode::BAD_REQUEST,
                format!(
                    "<html><body>{STYLE}<h1>Authorization Failed</h1>\
                     <p>Error: {err}</p></body></html>"
                ),
            ),
        };

        if let Some(sender) = tx.lock().await.take() {
            let _ = sender.send(result);
        }

        poem::Response::builder()
            .status(status)
            .content_type("text/html; charset=utf-8")
            .body(body)
    }

    let app = Route::new()
        .at("/callback", poem::get(callback_handler))
        .data(tx);

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
        Ok(Ok(r)) => r.map_err(|e| {
            tracing::warn!(error = %e, "OAuth authorization failed at callback");
            OAuthError::AuthorizationFailed {
                message: e.to_string(),
            }
        }),
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
