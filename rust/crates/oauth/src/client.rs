//! OAuth client and HTTP server bindings.

use std::{fmt, sync::Arc};

use longbridge_geo::is_cn;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, RedirectUrl, RefreshToken, RevocationUrl,
    Scope, TokenUrl, basic::BasicClient, reqwest::async_http_client,
};
use poem::listener::{Acceptor, Listener, TcpListener};

use crate::{
    callback::wait_for_callback,
    error::{OAuthError, OAuthResult},
    token::{OAuthToken, token_path_for_client_id},
};

const OAUTH_BASE_URL: &str = "https://openapi.longbridge.com/oauth2";
const OAUTH_BASE_URL_CN: &str = "https://openapi.longbridge.cn/oauth2";
const OAUTH_BASE_URL_TEST: &str = "https://openapi.longbridge.xyz/oauth2";

async fn oauth_base_url() -> &'static str {
    if std::env::var("LONGBRIDGE_ENV")
        .map(|v| v == "staging")
        .unwrap_or(false)
    {
        OAUTH_BASE_URL_TEST
    } else if is_cn().await {
        OAUTH_BASE_URL_CN
    } else {
        OAUTH_BASE_URL
    }
}

/// Default port for the local OAuth callback server.
pub(crate) const DEFAULT_CALLBACK_PORT: u16 = 60355;

/// Inner state shared across [`OAuth`] clones.
pub(crate) struct OAuthInner {
    pub(crate) client_id: String,
    pub(crate) callback_port: u16,
    pub(crate) token: tokio::sync::Mutex<Option<OAuthToken>>,
}

/// OAuth 2.0 client for Longbridge OpenAPI
///
/// Obtain an instance via [`crate::OAuthBuilder`].  Cloning is cheap – all
/// clones share the same internal state through an [`Arc`].
///
/// The token file is stored at `~/.longbridge/openapi/tokens/<client_id>`.
#[derive(Clone)]
pub struct OAuth(pub(crate) Arc<OAuthInner>);

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
    /// `~/.longbridge/openapi/tokens/<client_id>` so that subsequent runs can
    /// avoid a full re-authorization.
    pub async fn access_token(&self) -> OAuthResult<String> {
        let mut guard = self.0.token.lock().await;

        let needs_refresh = match guard.as_ref() {
            None => {
                tracing::debug!(client_id = %self.0.client_id, "no in-memory token, refresh needed");
                true
            }
            Some(t) if t.expires_soon() => {
                tracing::debug!(client_id = %self.0.client_id, expires_at = t.expires_at, "token expired or expiring soon, refresh needed");
                true
            }
            Some(_) => false,
        };

        if needs_refresh && let Some(current) = guard.as_ref() {
            let token_path = token_path_for_client_id(&self.0.client_id)?;
            let refreshed = self.refresh_token(current).await?;
            refreshed.save_to_path(&token_path)?;
            *guard = Some(refreshed);
        }

        guard
            .as_ref()
            .map(|t| t.access_token.clone())
            .ok_or_else(|| {
                tracing::error!(client_id = %self.0.client_id, "no token available");
                OAuthError::NoTokenAvailable
            })
    }

    /// Run the browser authorization flow and return a new token.
    pub(crate) async fn authorize_inner(&self, open_url: impl Fn(&str)) -> OAuthResult<OAuthToken> {
        let acceptor = TcpListener::bind(format!("127.0.0.1:{}", self.0.callback_port))
            .into_acceptor()
            .await
            .map_err(|e| OAuthError::BindCallbackFailed {
                port: self.0.callback_port,
                message: e.to_string(),
            })?;
        let port = acceptor
            .local_addr()
            .into_iter()
            .next()
            .and_then(|a| a.as_socket_addr().map(|s| s.port()))
            .ok_or(OAuthError::LocalAddressFailed)?;

        tracing::debug!("Callback server listening on port {port}");

        let client = create_oauth_client(
            &self.0.client_id,
            &format!("http://localhost:{port}/callback"),
        )
        .await;

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
            return Err(OAuthError::CsrfTokenMismatch);
        }

        tracing::debug!(client_id = %self.0.client_id, "exchanging authorization code for token");
        let token_response = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                tracing::error!(client_id = %self.0.client_id, error = %e, "failed to exchange authorization code for token");
                OAuthError::ExchangeCodeFailed {
                    message: e.to_string(),
                }
            })?;

        let token = OAuthToken::from_oauth2_response(self.0.client_id.clone(), &token_response);
        tracing::info!(client_id = %self.0.client_id, expires_at = token.expires_at, "authorization flow completed, token obtained");
        Ok(token)
    }

    /// Refresh the access token using the stored refresh token.
    pub(crate) async fn refresh_token(&self, token: &OAuthToken) -> OAuthResult<OAuthToken> {
        let refresh_token_str = token.refresh_token.as_deref().ok_or_else(|| {
            tracing::warn!(client_id = %self.0.client_id, "no refresh token available");
            OAuthError::NoRefreshToken
        })?;

        tracing::debug!(client_id = %self.0.client_id, "refreshing OAuth token");

        let client = create_oauth_client(
            &self.0.client_id,
            &format!("http://localhost:{}/callback", self.0.callback_port),
        )
        .await;
        let token_response = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token_str.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                tracing::error!(client_id = %self.0.client_id, error = %e, "failed to refresh token");
                OAuthError::RefreshTokenFailed {
                    message: e.to_string(),
                }
            })?;

        let mut new_token =
            OAuthToken::from_oauth2_response(self.0.client_id.clone(), &token_response);

        if new_token.refresh_token.is_none() {
            tracing::debug!(client_id = %self.0.client_id, "server did not return new refresh token, preserving existing one");
            new_token.refresh_token = Some(refresh_token_str.to_string());
        }

        tracing::debug!(client_id = %self.0.client_id, expires_at = new_token.expires_at, "token refreshed successfully");
        Ok(new_token)
    }
}

/// Build the oauth2 BasicClient for Longbridge endpoints.
async fn create_oauth_client(client_id: &str, redirect_uri: &str) -> BasicClient {
    let base = oauth_base_url().await;
    BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new(format!("{base}/authorize")).unwrap(),
        Some(TokenUrl::new(format!("{base}/token")).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap())
    .set_revocation_uri(RevocationUrl::new(format!("{base}/revoke")).unwrap())
}
