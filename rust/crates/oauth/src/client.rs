use std::{fmt, sync::Arc};

use longbridge_geo::is_cn;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, RequestTokenError, RevocationErrorResponseType,
    RevocationUrl, Scope, StandardErrorResponse, StandardRevocableToken, TokenUrl,
    basic::BasicClient, reqwest::async_http_client,
};
use poem::listener::{Acceptor, Listener, TcpListener};

use crate::{
    callback::wait_for_callback,
    error::{OAuthError, OAuthResult},
    storage::TokenStorage,
    token::OAuthToken,
};

const OAUTH_BASE_URL: &str = "https://openapi.longbridge.com/oauth2";
const OAUTH_BASE_URL_CN: &str = "https://openapi.longbridge.cn/oauth2";
const OAUTH_BASE_URL_TEST: &str = "https://openapi-global.longbridge.xyz/oauth2";

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
    pub(crate) storage: Arc<dyn TokenStorage>,
    pub(crate) token: tokio::sync::Mutex<Option<OAuthToken>>,
}

/// OAuth 2.0 client for Longbridge OpenAPI.
///
/// Obtain an instance via [`crate::OAuthBuilder`].  Cloning is cheap — all
/// clones share the same internal state through an [`Arc`].
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
    /// Create an OAuth client from a pre-existing access token.
    ///
    /// Useful for server-side scenarios where the token is obtained externally
    /// (e.g. via an MCP OAuth proxy).  The returned instance does **not**
    /// support token refresh — [`access_token`](OAuth::access_token) simply
    /// returns the provided token until it expires.
    pub fn from_token(access_token: impl Into<String>) -> Self {
        let access_token = access_token.into();
        Self(Arc::new(OAuthInner {
            client_id: String::new(),
            callback_port: DEFAULT_CALLBACK_PORT,
            storage: crate::storage::default_storage(),
            token: tokio::sync::Mutex::new(Some(OAuthToken {
                client_id: String::new(),
                access_token,
                refresh_token: None,
                expires_at: u64::MAX,
            })),
        }))
    }

    /// Return the OAuth client ID.
    pub fn client_id(&self) -> &str {
        &self.0.client_id
    }

    /// Return a valid access token, refreshing it first if it has expired or
    /// will expire within five minutes.
    ///
    /// The refreshed token is persisted via the configured storage backend so
    /// that subsequent runs can avoid a full re-authorization.
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
            let refreshed = self.refresh_token(current).await?;
            self.0.storage.save(&refreshed.clone().into())?;
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

    /// Revoke the current token on the server (RFC 7009).
    ///
    /// The refresh token is revoked when present, otherwise the access token.
    /// This only calls the server: the in-memory and persisted tokens are left
    /// untouched. Call [`clear`](OAuth::clear) afterwards to drop them locally.
    pub async fn revoke(&self) -> OAuthResult<()> {
        // `from_token` clients have no client ID: the token was issued
        // elsewhere, so the server cannot revoke it on our behalf.
        if self.0.client_id.is_empty() {
            return Err(OAuthError::RevokeFailed {
                message: "token was not issued through an OAuth client (created with \
                          `from_token`), nothing to revoke"
                    .to_string(),
            });
        }

        let token = self
            .0
            .token
            .lock()
            .await
            .clone()
            .ok_or(OAuthError::NoTokenAvailable)?;

        let revocable = match token.refresh_token {
            Some(refresh_token) => {
                StandardRevocableToken::RefreshToken(RefreshToken::new(refresh_token))
            }
            None => StandardRevocableToken::AccessToken(AccessToken::new(token.access_token)),
        };

        tracing::debug!(client_id = %self.0.client_id, "revoking OAuth token");

        let client = create_oauth_client(
            &self.0.client_id,
            &format!("http://localhost:{}/callback", self.0.callback_port),
        )
        .await;
        client
            .revoke_token(revocable)
            .map_err(|e| OAuthError::RevokeFailed {
                message: e.to_string(),
            })?
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                let message = revoke_error_message(&e);
                tracing::error!(client_id = %self.0.client_id, error = %message, "failed to revoke token");
                OAuthError::RevokeFailed { message }
            })?;

        tracing::info!(client_id = %self.0.client_id, "token revoked");
        Ok(())
    }

    /// Drop the token locally: clear the in-memory copy and delete the
    /// persisted one from the configured storage backend (by default
    /// `~/.longbridge/openapi/tokens/<client_id>`).
    ///
    /// This does not contact the server; use [`revoke`](OAuth::revoke) for
    /// that. After this call [`access_token`](OAuth::access_token) fails with
    /// [`OAuthError::NoTokenAvailable`] until a new client is built.
    ///
    /// The persisted token is deleted first: if that fails the error is
    /// returned and the in-memory token is kept, so memory and storage never
    /// disagree about whether the client is logged in.
    pub async fn clear(&self) -> OAuthResult<()> {
        // Hold the lock across the delete so a concurrent refresh cannot
        // re-save the token between the delete and the in-memory clear.
        let mut guard = self.0.token.lock().await;
        // `from_token` clients have no client ID and nothing persisted.
        if !self.0.client_id.is_empty() {
            self.0.storage.delete(&self.0.client_id)?;
        }
        *guard = None;
        Ok(())
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

        let (auth_url, csrf_token, pkce_verifier) = build_authorize_url(&client);

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
            .set_pkce_verifier(pkce_verifier)
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

/// Describe a revocation failure, including the server's `error` and
/// `error_description` (the `Display` of `ServerResponse` omits them).
fn revoke_error_message<RE>(
    e: &RequestTokenError<RE, StandardErrorResponse<RevocationErrorResponseType>>,
) -> String
where
    RE: std::error::Error + 'static,
{
    match e {
        RequestTokenError::ServerResponse(resp) => match resp.error_description() {
            Some(desc) => format!("{}: {desc}", resp.error()),
            None => resp.error().to_string(),
        },
        other => other.to_string(),
    }
}

/// Build the authorization URL with a fresh CSRF token and PKCE challenge.
///
/// PKCE (RFC 7636, `S256`) is required by the server for public clients and
/// ignored for clients that don't need it. The returned verifier must be sent
/// with the token exchange.
fn build_authorize_url(client: &BasicClient) -> (oauth2::url::Url, CsrfToken, PkceCodeVerifier) {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(String::new()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    (auth_url, csrf_token, pkce_verifier)
}

/// Build the oauth2 BasicClient for Longbridge endpoints.
async fn create_oauth_client(client_id: &str, redirect_uri: &str) -> BasicClient {
    build_oauth_client(oauth_base_url().await, client_id, redirect_uri)
}

fn build_oauth_client(base: &str, client_id: &str, redirect_uri: &str) -> BasicClient {
    BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new(format!("{base}/authorize")).unwrap(),
        Some(TokenUrl::new(format!("{base}/token")).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap())
    .set_revocation_uri(RevocationUrl::new(format!("{base}/revoke")).unwrap())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::storage::StoredToken;

    #[test]
    fn authorize_url_carries_pkce_challenge_matching_verifier() {
        let client = build_oauth_client(
            OAUTH_BASE_URL,
            "my-client",
            "http://localhost:60355/callback",
        );
        let (url, csrf, verifier) = build_authorize_url(&client);

        let query: HashMap<String, String> = url.query_pairs().into_owned().collect();
        assert_eq!(query["code_challenge_method"], "S256");
        assert_eq!(
            query["code_challenge"],
            PkceCodeChallenge::from_code_verifier_sha256(&verifier).as_str()
        );
        assert_eq!(query["state"], *csrf.secret());
        assert_eq!(query["client_id"], "my-client");
    }

    #[test]
    fn authorize_url_uses_a_fresh_verifier_each_time() {
        let client = build_oauth_client(OAUTH_BASE_URL, "c", "http://localhost:60355/callback");
        let (_, _, v1) = build_authorize_url(&client);
        let (_, _, v2) = build_authorize_url(&client);
        assert_ne!(v1.secret(), v2.secret());
    }

    #[test]
    fn revoke_error_message_includes_server_error_and_description() {
        let with_desc: RequestTokenError<std::io::Error, _> =
            RequestTokenError::ServerResponse(StandardErrorResponse::new(
                RevocationErrorResponseType::UnsupportedTokenType,
                Some("token type not supported".to_string()),
                None,
            ));
        assert_eq!(
            revoke_error_message(&with_desc),
            "unsupported_token_type: token type not supported"
        );

        let without_desc: RequestTokenError<std::io::Error, _> =
            RequestTokenError::ServerResponse(StandardErrorResponse::new(
                RevocationErrorResponseType::UnsupportedTokenType,
                None,
                None,
            ));
        assert_eq!(
            revoke_error_message(&without_desc),
            "unsupported_token_type"
        );
    }

    #[test]
    fn revoke_error_message_falls_back_to_display_for_other_errors() {
        let e: RequestTokenError<
            std::io::Error,
            StandardErrorResponse<RevocationErrorResponseType>,
        > = RequestTokenError::Other("boom".to_string());
        assert_eq!(revoke_error_message(&e), e.to_string());
    }

    #[tokio::test]
    async fn revoke_rejects_from_token_clients() {
        let oauth = OAuth::from_token("external-token");
        assert!(matches!(
            oauth.revoke().await,
            Err(OAuthError::RevokeFailed { .. })
        ));
    }

    /// Storage whose `delete` always fails.
    struct FailingDelete;

    impl TokenStorage for FailingDelete {
        fn load(&self, _client_id: &str) -> Option<StoredToken> {
            None
        }

        fn save(&self, _token: &StoredToken) -> OAuthResult<()> {
            Ok(())
        }

        fn delete(&self, _client_id: &str) -> OAuthResult<()> {
            Err(OAuthError::Other("delete failed".to_string()))
        }
    }

    /// Storage that only implements the required methods.
    struct LegacyStorage;

    impl TokenStorage for LegacyStorage {
        fn load(&self, _client_id: &str) -> Option<StoredToken> {
            None
        }

        fn save(&self, _token: &StoredToken) -> OAuthResult<()> {
            Ok(())
        }
    }

    fn oauth_with_storage(storage: Arc<dyn TokenStorage>) -> OAuth {
        OAuth(Arc::new(OAuthInner {
            client_id: "my-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            storage,
            token: tokio::sync::Mutex::new(Some(OAuthToken {
                client_id: "my-client".to_string(),
                access_token: "access".to_string(),
                refresh_token: None,
                expires_at: u64::MAX,
            })),
        }))
    }

    #[tokio::test]
    async fn clear_keeps_memory_token_when_storage_delete_fails() {
        let oauth = oauth_with_storage(Arc::new(FailingDelete));

        assert!(oauth.clear().await.is_err());
        assert_eq!(oauth.access_token().await.unwrap(), "access");
    }

    #[tokio::test]
    async fn clear_fails_for_storage_without_delete_support() {
        let oauth = oauth_with_storage(Arc::new(LegacyStorage));

        assert!(matches!(oauth.clear().await, Err(OAuthError::Other(_))));
        assert_eq!(oauth.access_token().await.unwrap(), "access");
    }
}
