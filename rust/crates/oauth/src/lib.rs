//! OAuth 2.0 authentication support for LongPort OpenAPI
//!
//! This crate provides utilities for performing OAuth 2.0 authorization code
//! flow to obtain access tokens for API authentication.
//!
//! # Example
//!
//! ```no_run
//! use longport_oauth::{OAuth, OAuthToken};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Start OAuth authorization flow (default callback port 60355)
//!     let oauth = OAuth::new("your-client-id");
//!     let token = oauth
//!         .authorize(|url| {
//!             // Open the URL however you like, e.g. print it or launch a browser
//!             println!("Please visit: {url}");
//!         })
//!         .await?;
//!
//!     println!("Access token: {}", token.access_token);
//!
//!     // Or specify a custom callback port via set_callback_port
//!     let mut oauth2 = OAuth::new("your-client-id");
//!     oauth2.set_callback_port(8080);
//!     let _token2 = oauth2
//!         .authorize(|url| println!("Please visit: {url}"))
//!         .await?;
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(unreachable_pub)]
#![warn(missing_docs)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, RedirectUrl, RefreshToken, RevocationUrl,
    Scope, TokenResponse, TokenUrl, basic::BasicClient, reqwest::async_http_client,
};
use poem::{
    EndpointExt, Route, Server, handler,
    listener::{Acceptor, Listener, TcpListener},
    web::Query,
};
use serde::{Deserialize, Serialize};
use tokio::{sync::oneshot, time::timeout};

const AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
const OAUTH_BASE_URL: &str = "https://openapi.longbridgeapp.com/oauth2";
const DEFAULT_CALLBACK_PORT: u16 = 60355;

/// Error type for OAuth operations
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    /// OAuth flow error
    #[error("oauth error: {0}")]
    OAuth(String),
}

/// Result type for OAuth operations
pub type OAuthResult<T> = std::result::Result<T, OAuthError>;

/// OAuth 2.0 access token with expiration and refresh information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    /// The OAuth 2.0 client ID associated with this token
    pub client_id: String,
    /// The access token for API authentication
    pub access_token: String,
    /// Optional refresh token for obtaining new access tokens
    pub refresh_token: Option<String>,
    /// Unix timestamp when the token expires
    pub expires_at: u64,
}

impl OAuthToken {
    fn from_oauth2_response<TT, T>(client_id: String, token_response: &T) -> Self
    where
        TT: oauth2::TokenType,
        T: TokenResponse<TT>,
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expires_in = token_response.expires_in().map_or(3600, |d| d.as_secs());

        Self {
            client_id,
            access_token: token_response.access_token().secret().clone(),
            refresh_token: token_response.refresh_token().map(|t| t.secret().clone()),
            expires_at: now + expires_in,
        }
    }

    /// Check if the token has expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at
    }

    /// Check if the token will expire soon (within 1 hour)
    pub fn expires_soon(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at.saturating_sub(now) < 3600
    }
}

type CallbackTx = std::sync::Arc<
    tokio::sync::Mutex<Option<oneshot::Sender<std::result::Result<(String, String), String>>>>,
>;

/// OAuth 2.0 client for LongPort OpenAPI
#[derive(Clone)]
pub struct OAuth {
    client_id: String,
    callback_port: u16,
}

impl OAuth {
    /// Create a new OAuth client with the default callback port (60355)
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth 2.0 client ID obtained from LongPort developer
    ///   portal
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            callback_port: DEFAULT_CALLBACK_PORT,
        }
    }

    /// Set the callback port
    ///
    /// # Arguments
    ///
    /// * `callback_port` - TCP port for the local callback server (e.g. 8080).
    ///   This port must match one of the redirect URIs registered for the
    ///   client. Defaults to `60355` when using [`OAuth::new`].
    pub fn set_callback_port(&mut self, callback_port: u16) {
        self.callback_port = callback_port;
    }

    /// Get the client ID
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the callback port
    pub fn callback_port(&self) -> u16 {
        self.callback_port
    }

    /// Start the OAuth 2.0 authorization flow
    ///
    /// This will:
    /// 1. Start a local HTTP server to receive the callback
    /// 2. Invoke `open_url` with the authorization URL, so the caller can open
    ///    it in a browser or handle it in any other way
    /// 3. Wait for the user to authorize and receive the authorization code
    /// 4. Exchange the code for an access token
    ///
    /// # Returns
    ///
    /// An [`OAuthToken`] containing the access token and expiration information
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The callback server cannot be started
    /// - The user denies authorization
    /// - The authorization times out (5 minutes)
    /// - Token exchange fails
    pub async fn authorize(&self, open_url: impl Fn(&str)) -> OAuthResult<OAuthToken> {
        // Bind callback server on the configured port via poem's TcpListener
        let acceptor = TcpListener::bind(format!("127.0.0.1:{}", self.callback_port))
            .into_acceptor()
            .await
            .map_err(|e| {
                OAuthError::OAuth(format!(
                    "Failed to bind callback server on port {}: {}",
                    self.callback_port, e
                ))
            })?;
        let port = acceptor
            .local_addr()
            .into_iter()
            .next()
            .and_then(|a| a.as_socket_addr().map(|s| s.port()))
            .ok_or_else(|| OAuthError::OAuth("Failed to get local address".to_string()))?;

        tracing::debug!("Callback server listening on port {port}");
        tracing::debug!("Redirect URI: http://localhost:{port}/callback");

        let client = self.create_oauth_client(&format!("http://localhost:{port}/callback"));

        // Generate authorization URL
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(String::new()))
            .url();

        tracing::info!("Starting OAuth authorization flow");

        open_url(auth_url.as_str());

        // Start local callback server and wait for authorization code
        let (code, state) = Self::wait_for_callback(acceptor).await?;

        // Verify CSRF token
        if state != *csrf_token.secret() {
            return Err(OAuthError::OAuth("CSRF token mismatch".to_string()));
        }

        // Exchange code for token
        tracing::debug!("Exchanging authorization code for token");
        let token_response = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuthError::OAuth(format!("Failed to exchange code for token: {}", e)))?;

        Ok(OAuthToken::from_oauth2_response(
            self.client_id.clone(),
            &token_response,
        ))
    }

    /// Refresh an access token using a refresh token
    ///
    /// # Arguments
    ///
    /// * `token` - The [`OAuthToken`] from a previous authorization (must
    ///   contain a refresh token)
    ///
    /// # Returns
    ///
    /// A new [`OAuthToken`] with a fresh access token
    pub async fn refresh(&self, token: &OAuthToken) -> OAuthResult<OAuthToken> {
        let refresh_token = token
            .refresh_token
            .as_deref()
            .ok_or_else(|| OAuthError::OAuth("No refresh token available".to_string()))?;

        tracing::debug!("Refreshing OAuth token");

        let client =
            self.create_oauth_client(&format!("http://localhost:{}/callback", self.callback_port));
        let token_response = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuthError::OAuth(format!("Failed to refresh token: {}", e)))?;

        let mut new_token =
            OAuthToken::from_oauth2_response(self.client_id.clone(), &token_response);

        // Preserve refresh token if not returned
        if new_token.refresh_token.is_none() {
            new_token.refresh_token = Some(refresh_token.to_string());
        }

        Ok(new_token)
    }

    fn create_oauth_client(&self, redirect_uri: &str) -> BasicClient {
        BasicClient::new(
            ClientId::new(self.client_id.clone()),
            None, // No client secret for public clients
            AuthUrl::new(format!("{OAUTH_BASE_URL}/authorize")).unwrap(),
            Some(TokenUrl::new(format!("{OAUTH_BASE_URL}/token")).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap())
        .set_revocation_uri(RevocationUrl::new(format!("{OAUTH_BASE_URL}/revoke")).unwrap())
    }

    async fn wait_for_callback(
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
        let tx = std::sync::Arc::new(tokio::sync::Mutex::new(Some(tx)));

        #[handler]
        async fn callback(
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

        let app = Route::new().at("/callback", poem::get(callback)).data(tx);

        let server_task = tokio::spawn(
            Server::new_with_acceptor(acceptor).run_with_graceful_shutdown(
                app,
                async move {
                    futures_util::future::pending::<()>().await;
                },
                None,
            ),
        );

        let result = match timeout(AUTH_TIMEOUT, rx).await {
            Ok(Ok(r)) => {
                r.map_err(|e| OAuthError::OAuth(format!("OAuth authorization failed: {e}")))
            }
            Ok(Err(_)) => Err(OAuthError::OAuth(
                "Callback channel closed unexpectedly".to_string(),
            )),
            Err(_) => Err(OAuthError::OAuth(
                "Authorization timeout - no response received within 5 minutes".to_string(),
            )),
        };

        server_task.abort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_token_not_expired() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            expires_at: now + 7200, // expires in 2 hours
        };

        assert!(!token.is_expired());
    }

    #[test]
    fn test_oauth_token_expired() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            expires_at: now - 1, // expired 1 second ago
        };

        assert!(token.is_expired());
    }

    #[test]
    fn test_oauth_token_expires_soon() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Token expires in 30 minutes
        let token = OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            expires_at: now + 1800,
        };

        assert!(token.expires_soon());
    }

    #[test]
    fn test_oauth_token_not_expires_soon() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Token expires in 2 hours
        let token = OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            expires_at: now + 7200,
        };

        assert!(!token.expires_soon());
    }

    #[test]
    fn test_oauth_token_serialization() {
        let token = OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_access_token".to_string(),
            refresh_token: Some("test_refresh_token".to_string()),
            expires_at: 1234567890,
        };

        let json = serde_json::to_string(&token).unwrap();
        let deserialized: OAuthToken = serde_json::from_str(&json).unwrap();

        assert_eq!(token.client_id, deserialized.client_id);
        assert_eq!(token.access_token, deserialized.access_token);
        assert_eq!(token.refresh_token, deserialized.refresh_token);
        assert_eq!(token.expires_at, deserialized.expires_at);
    }

    #[test]
    fn test_oauth_token_serialization_without_refresh() {
        let token = OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_access_token".to_string(),
            refresh_token: None,
            expires_at: 1234567890,
        };

        let json = serde_json::to_string(&token).unwrap();
        let deserialized: OAuthToken = serde_json::from_str(&json).unwrap();

        assert_eq!(token.client_id, deserialized.client_id);
        assert_eq!(token.access_token, deserialized.access_token);
        assert_eq!(token.refresh_token, deserialized.refresh_token);
        assert_eq!(token.expires_at, deserialized.expires_at);
    }

    #[test]
    fn test_oauth_new() {
        let oauth = OAuth::new("test-client-id");
        assert_eq!(oauth.client_id(), "test-client-id");
        assert_eq!(oauth.callback_port(), DEFAULT_CALLBACK_PORT);
    }

    #[test]
    fn test_oauth_set_callback_port() {
        let mut oauth = OAuth::new("test-client-id");
        assert_eq!(oauth.callback_port(), DEFAULT_CALLBACK_PORT);
        oauth.set_callback_port(9090);
        assert_eq!(oauth.callback_port(), 9090);
    }

    #[test]
    fn test_oauth_create_client() {
        let oauth = OAuth::new("test-client-id");
        let client = oauth.create_oauth_client("http://localhost:60355/callback");

        // Client should be created successfully
        // We can't easily test the internal state, but we can verify it doesn't panic
        drop(client);
    }
}
