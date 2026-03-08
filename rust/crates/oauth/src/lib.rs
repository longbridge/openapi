//! OAuth 2.0 authentication support for Longbridge OpenAPI
//!
//! This crate provides utilities for performing OAuth 2.0 authorization code
//! flow to obtain access tokens for API authentication.
//!
//! # Example
//!
//! ```no_run
//! use longbridge_oauth::OAuthBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Build an OAuth client.  If a token exists on disk it is loaded;
//!     // otherwise the browser authorization flow is triggered.
//!     // Token is persisted at ~/.longbridge-openapi/tokens/<client_id>
//!     let oauth = OAuthBuilder::new("your-client-id")
//!         // .callback_port(8080)  // optional, default 60355
//!         .build(|url| println!("Please visit: {url}"))
//!         .await?;
//!
//!     // access_token() automatically refreshes when expired.
//!     let token = oauth.access_token().await?;
//!     println!("Access token: {token}");
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(unreachable_pub)]
#![warn(missing_docs)]

use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

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

/// Returns the token file path for the given client ID.
///
/// Path: `~/.longbridge-openapi/tokens/<client_id>`
fn token_path_for_client_id(client_id: &str) -> OAuthResult<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| OAuthError::OAuth("Cannot determine home directory".to_string()))?;
    Ok(home
        .join(".longbridge-openapi")
        .join("tokens")
        .join(client_id))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OAuthToken {
    pub(crate) client_id: String,
    pub(crate) access_token: String,
    pub(crate) refresh_token: Option<String>,
    pub(crate) expires_at: u64,
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

    fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at
    }

    fn expires_soon(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at.saturating_sub(now) < 3600
    }

    fn load_from_path(path: impl AsRef<Path>) -> OAuthResult<Self> {
        let path = path.as_ref();
        tracing::debug!(path = %path.display(), "loading token from disk");
        let data = std::fs::read_to_string(path).map_err(|e| {
            tracing::debug!(path = %path.display(), error = %e, "failed to read token file");
            OAuthError::OAuth(format!("Failed to read token file {}: {e}", path.display()))
        })?;
        let token = serde_json::from_str(&data).map_err(|e| {
            tracing::warn!(path = %path.display(), error = %e, "failed to parse token file");
            OAuthError::OAuth(format!(
                "Failed to parse token file {}: {e}",
                path.display()
            ))
        })?;
        tracing::debug!(path = %path.display(), "token loaded successfully");
        Ok(token)
    }

    fn save_to_path(&self, path: impl AsRef<Path>) -> OAuthResult<()> {
        let path = path.as_ref();
        tracing::debug!(path = %path.display(), "saving token to disk");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                tracing::error!(path = %parent.display(), error = %e, "failed to create token directory");
                OAuthError::OAuth(format!(
                    "Failed to create directory {}: {e}",
                    parent.display()
                ))
            })?;
        }
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| OAuthError::OAuth(format!("Failed to serialize token: {e}")))?;
        std::fs::write(path, data).map_err(|e| {
            tracing::error!(path = %path.display(), error = %e, "failed to write token file");
            OAuthError::OAuth(format!(
                "Failed to write token file {}: {e}",
                path.display()
            ))
        })?;
        tracing::debug!(path = %path.display(), "token saved successfully");
        Ok(())
    }
}

type CallbackTx = std::sync::Arc<
    tokio::sync::Mutex<Option<oneshot::Sender<std::result::Result<(String, String), String>>>>,
>;

// ---------------------------------------------------------------------------
// Inner state shared across clones
// ---------------------------------------------------------------------------

struct OAuthInner {
    client_id: String,
    callback_port: u16,
    // token_path is derived on demand via token_path_for_client_id(&client_id)
    token: tokio::sync::Mutex<Option<OAuthToken>>,
}

// ---------------------------------------------------------------------------
// Public OAuth handle (Clone = Arc reference bump)
// ---------------------------------------------------------------------------

/// OAuth 2.0 client for Longbridge OpenAPI
///
/// Obtain an instance via [`OAuthBuilder`].  Cloning is cheap – all clones
/// share the same internal state through an [`Arc`].
///
/// The token file is stored at `~/.longbridge-openapi/tokens/<client_id>`.
#[derive(Clone)]
pub struct OAuth(Arc<OAuthInner>);

impl fmt::Debug for OAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuth")
            .field("client_id", &self.0.client_id)
            .field("callback_port", &self.0.callback_port)
            .finish()
    }
}

impl OAuth {
    /// Return the OAuth client ID
    pub fn client_id(&self) -> &str {
        &self.0.client_id
    }

    /// Return a valid access token, refreshing it first if it has expired or
    /// will expire within one hour.
    ///
    /// The refreshed token is persisted to
    /// `~/.longbridge-openapi/tokens/<client_id>` so that subsequent runs can
    /// avoid a full re-authorization.
    pub async fn access_token(&self) -> OAuthResult<String> {
        let mut guard = self.0.token.lock().await;

        let needs_refresh = match guard.as_ref() {
            None => {
                tracing::debug!(client_id = %self.0.client_id, "no in-memory token, refresh needed");
                true
            }
            Some(t) if t.is_expired() => {
                tracing::debug!(client_id = %self.0.client_id, "token expired, refresh needed");
                true
            }
            Some(t) if t.expires_soon() => {
                tracing::debug!(client_id = %self.0.client_id, expires_at = t.expires_at, "token expiring soon, proactive refresh");
                true
            }
            Some(_) => false,
        };

        if needs_refresh && let Some(current) = guard.as_ref() {
            let token_path = token_path_for_client_id(&self.0.client_id)?;
            let refreshed = self.refresh_token(current).await?;
            refreshed.save_to_path(&token_path)?;
            *guard = Some(refreshed);
            // If guard is None, fall through to the error below.
        }

        guard
            .as_ref()
            .map(|t| t.access_token.clone())
            .ok_or_else(|| {
                tracing::error!(client_id = %self.0.client_id, "no token available");
                OAuthError::OAuth("No token available".to_string())
            })
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    async fn authorize_inner(&self, open_url: impl Fn(&str)) -> OAuthResult<OAuthToken> {
        let acceptor = TcpListener::bind(format!("127.0.0.1:{}", self.0.callback_port))
            .into_acceptor()
            .await
            .map_err(|e| {
                OAuthError::OAuth(format!(
                    "Failed to bind callback server on port {}: {}",
                    self.0.callback_port, e
                ))
            })?;
        let port = acceptor
            .local_addr()
            .into_iter()
            .next()
            .and_then(|a| a.as_socket_addr().map(|s| s.port()))
            .ok_or_else(|| OAuthError::OAuth("Failed to get local address".to_string()))?;

        tracing::debug!("Callback server listening on port {port}");

        let client = create_oauth_client(
            &self.0.client_id,
            &format!("http://localhost:{port}/callback"),
        );

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(String::new()))
            .url();

        tracing::info!("starting OAuth authorization flow");
        open_url(auth_url.as_str());

        let (code, state) = wait_for_callback(acceptor).await?;

        tracing::debug!(client_id = %self.0.client_id, "received OAuth callback, verifying CSRF token");
        if state != *csrf_token.secret() {
            tracing::warn!(client_id = %self.0.client_id, "CSRF token mismatch, possible CSRF attack");
            return Err(OAuthError::OAuth("CSRF token mismatch".to_string()));
        }

        tracing::debug!(client_id = %self.0.client_id, "exchanging authorization code for token");
        let token_response = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                tracing::error!(client_id = %self.0.client_id, error = %e, "failed to exchange authorization code for token");
                OAuthError::OAuth(format!("Failed to exchange code for token: {}", e))
            })?;

        let token = OAuthToken::from_oauth2_response(self.0.client_id.clone(), &token_response);
        tracing::info!(client_id = %self.0.client_id, expires_at = token.expires_at, "authorization flow completed, token obtained");
        Ok(token)
    }

    async fn refresh_token(&self, token: &OAuthToken) -> OAuthResult<OAuthToken> {
        let refresh_token_str = token.refresh_token.as_deref().ok_or_else(|| {
            tracing::warn!(client_id = %self.0.client_id, "no refresh token available");
            OAuthError::OAuth("No refresh token available".to_string())
        })?;

        tracing::debug!(client_id = %self.0.client_id, "refreshing OAuth token");

        let client = create_oauth_client(
            &self.0.client_id,
            &format!("http://localhost:{}/callback", self.0.callback_port),
        );
        let token_response = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token_str.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                tracing::error!(client_id = %self.0.client_id, error = %e, "failed to refresh token");
                OAuthError::OAuth(format!("Failed to refresh token: {}", e))
            })?;

        let mut new_token =
            OAuthToken::from_oauth2_response(self.0.client_id.clone(), &token_response);

        // Preserve refresh token if not returned
        if new_token.refresh_token.is_none() {
            tracing::debug!(client_id = %self.0.client_id, "server did not return new refresh token, preserving existing one");
            new_token.refresh_token = Some(refresh_token_str.to_string());
        }

        tracing::debug!(client_id = %self.0.client_id, expires_at = new_token.expires_at, "token refreshed successfully");
        Ok(new_token)
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for constructing an [`OAuth`] client
///
/// `client_id` is the only required field.
///
/// The token is persisted at `~/.longbridge-openapi/tokens/<client_id>`.
pub struct OAuthBuilder {
    client_id: String,
    callback_port: u16,
}

impl OAuthBuilder {
    /// Create a new builder with the given client ID
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            callback_port: DEFAULT_CALLBACK_PORT,
        }
    }

    /// Set the local callback server port (default: `60355`)
    #[must_use]
    pub fn callback_port(mut self, port: u16) -> Self {
        self.callback_port = port;
        self
    }

    /// Synchronously build the [`OAuth`] client.
    ///
    /// This is the blocking equivalent of [`build`](OAuthBuilder::build).  It
    /// spins up a temporary single-threaded Tokio runtime internally so it can
    /// be called from a non-async context such as a blocking application or a
    /// doc-test `fn main()`.
    ///
    /// First tries to load an existing token from
    /// `~/.longbridge-openapi/tokens/<client_id>`.  If no valid token is found
    /// the full browser-based authorization flow is started and `open_url` is
    /// called with the authorization URL.  The resulting token is persisted for
    /// future use.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use longbridge_oauth::OAuthBuilder;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let oauth =
    ///         OAuthBuilder::new("your-client-id").build_blocking(|url| println!("Visit: {url}"))?;
    ///     println!("client_id: {}", oauth.client_id());
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "blocking")]
    pub fn build_blocking(self, open_url: impl Fn(&str)) -> OAuthResult<OAuth> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| OAuthError::OAuth(format!("Failed to create Tokio runtime: {e}")))?
            .block_on(self.build(open_url))
    }

    /// Asynchronously build the [`OAuth`] client.
    ///
    /// First tries to load an existing token from
    /// `~/.longbridge-openapi/tokens/<client_id>`.  If no valid token is found
    /// the full browser-based authorization flow is started and `open_url` is
    /// called with the authorization URL.  The resulting token is persisted for
    /// future use.
    pub async fn build(self, open_url: impl Fn(&str)) -> OAuthResult<OAuth> {
        let token_path = token_path_for_client_id(&self.client_id)?;

        let inner = Arc::new(OAuthInner {
            client_id: self.client_id,
            callback_port: self.callback_port,
            token: tokio::sync::Mutex::new(None),
        });
        let oauth = OAuth(inner);

        let loaded = OAuthToken::load_from_path(&token_path).ok();

        let token = match loaded {
            Some(t) if !t.is_expired() => {
                tracing::debug!(path = %token_path.display(), expires_at = t.expires_at, "loaded valid token from disk");
                t
            }
            Some(t) => {
                tracing::debug!(
                    path = %token_path.display(),
                    "loaded expired token from disk, attempting refresh"
                );
                match oauth.refresh_token(&t).await {
                    Ok(refreshed) => {
                        refreshed.save_to_path(&token_path)?;
                        refreshed
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "token refresh failed, falling back to authorization flow");
                        let new_token = oauth.authorize_inner(open_url).await?;
                        new_token.save_to_path(&token_path)?;
                        new_token
                    }
                }
            }
            None => {
                tracing::debug!("no cached token found, starting authorization flow");
                let new_token = oauth.authorize_inner(open_url).await?;
                new_token.save_to_path(&token_path)?;
                new_token
            }
        };

        *oauth.0.token.lock().await = Some(token);
        Ok(oauth)
    }
}

// ---------------------------------------------------------------------------
// Free helpers
// ---------------------------------------------------------------------------

fn create_oauth_client(client_id: &str, redirect_uri: &str) -> BasicClient {
    BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new(format!("{OAUTH_BASE_URL}/authorize")).unwrap(),
        Some(TokenUrl::new(format!("{OAUTH_BASE_URL}/token")).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap())
    .set_revocation_uri(RevocationUrl::new(format!("{OAUTH_BASE_URL}/revoke")).unwrap())
}

async fn wait_for_callback(acceptor: poem::listener::TcpAcceptor) -> OAuthResult<(String, String)> {
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

    tracing::debug!(
        "waiting for OAuth callback (timeout: {}s)",
        AUTH_TIMEOUT.as_secs()
    );
    let result = match timeout(AUTH_TIMEOUT, rx).await {
        Ok(Ok(r)) => r.map_err(|e| {
            tracing::warn!(error = %e, "OAuth authorization failed at callback");
            OAuthError::OAuth(format!("OAuth authorization failed: {e}"))
        }),
        Ok(Err(_)) => {
            tracing::error!("OAuth callback channel closed unexpectedly");
            Err(OAuthError::OAuth(
                "Callback channel closed unexpectedly".to_string(),
            ))
        }
        Err(_) => {
            tracing::warn!(
                timeout_secs = AUTH_TIMEOUT.as_secs(),
                "OAuth authorization timed out waiting for callback"
            );
            Err(OAuthError::OAuth(
                "Authorization timeout - no response received within 5 minutes".to_string(),
            ))
        }
    };

    server_task.abort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(expires_at: u64) -> OAuthToken {
        OAuthToken {
            client_id: "test-client".to_string(),
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            expires_at,
        }
    }

    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn test_oauth_token_not_expired() {
        assert!(!make_token(now_secs() + 7200).is_expired());
    }

    #[test]
    fn test_oauth_token_expired() {
        assert!(make_token(now_secs() - 1).is_expired());
    }

    #[test]
    fn test_oauth_token_expires_soon() {
        assert!(make_token(now_secs() + 1800).expires_soon());
    }

    #[test]
    fn test_oauth_token_not_expires_soon() {
        assert!(!make_token(now_secs() + 7200).expires_soon());
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
    fn test_oauth_builder_new() {
        let builder = OAuthBuilder::new("test-client-id");
        assert_eq!(builder.client_id, "test-client-id");
        assert_eq!(builder.callback_port, DEFAULT_CALLBACK_PORT);
    }

    #[test]
    fn test_oauth_builder_callback_port() {
        let builder = OAuthBuilder::new("test-client-id").callback_port(9090);
        assert_eq!(builder.callback_port, 9090);
    }

    #[test]
    fn test_token_path_for_client_id() {
        let path = token_path_for_client_id("my-app").unwrap();
        let path_str = path.to_string_lossy();
        assert!(
            path_str.ends_with(".longbridge-openapi/tokens/my-app"),
            "unexpected path: {path_str}"
        );
    }

    #[tokio::test]
    async fn test_oauth_access_token_returns_token() {
        let inner = Arc::new(OAuthInner {
            client_id: "test-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            token: tokio::sync::Mutex::new(Some(make_token(now_secs() + 7200))),
        });
        let oauth = OAuth(inner);
        let token = oauth.access_token().await.unwrap();
        assert_eq!(token, "test_token");
    }

    #[test]
    fn test_oauth_client_id() {
        let inner = Arc::new(OAuthInner {
            client_id: "my-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            token: tokio::sync::Mutex::new(None),
        });
        let oauth = OAuth(inner);
        assert_eq!(oauth.client_id(), "my-client");
    }

    #[test]
    fn test_oauth_clone_shares_state() {
        let inner = Arc::new(OAuthInner {
            client_id: "shared-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            token: tokio::sync::Mutex::new(None),
        });
        let oauth1 = OAuth(inner);
        let oauth2 = oauth1.clone();
        assert!(Arc::ptr_eq(&oauth1.0, &oauth2.0));
    }
}
