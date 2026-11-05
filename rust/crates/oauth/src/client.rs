use std::{fmt, sync::Arc, time::Duration};

use longbridge_geo::is_cn;
use oauth2::{
    AccessToken, AsyncHttpClient, AuthUrl, AuthorizationCode, ClientId, CsrfToken, EndpointNotSet,
    EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, RevocationUrl,
    Scope, StandardRevocableToken, TokenUrl, basic::BasicClient, reqwest,
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

#[cfg(test)]
tokio::task_local! {
    pub(crate) static TEST_OAUTH_BASE_URL: String;
}

async fn oauth_base_url() -> String {
    // Keep protocol tests on their own loopback server without process-wide
    // state.
    #[cfg(test)]
    {
        if let Ok(base) = TEST_OAUTH_BASE_URL.try_with(Clone::clone) {
            return base;
        }
        // Fail closed: a test outside the scope (e.g. a request moved into a
        // spawned task, which does not inherit task-locals) must not silently
        // reach the real endpoints. Only the endpoint-selection child process
        // resolves them on purpose, with the region fixed by environment.
        assert!(
            std::env::var("TEST_OAUTH_EXPECTED_AUTHORIZATION_URL").is_ok(),
            "test reached the real OAuth endpoint; run it inside TEST_OAUTH_BASE_URL.scope"
        );
    }

    if std::env::var("LONGBRIDGE_ENV")
        .map(|v| v == "staging")
        .unwrap_or(false)
    {
        OAUTH_BASE_URL_TEST.to_string()
    } else if is_cn().await {
        OAUTH_BASE_URL_CN.to_string()
    } else {
        OAUTH_BASE_URL.to_string()
    }
}

/// The oauth2 client with the authorization, token and revocation endpoints
/// set.
type LongbridgeClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet>;

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
    ///
    /// The token lock is held for the whole request, so a concurrent
    /// [`access_token`](OAuth::access_token) refresh cannot rotate the refresh
    /// token between reading and revoking it (which would revoke a stale token
    /// while the new one stays valid). While it runs, `access_token` (and every
    /// request that needs a token) waits; the request is therefore bounded by a
    /// 30-second timeout, as is a token refresh it may have to wait for.
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

        // Built before taking the lock: the first call may probe the region.
        let client = create_oauth_client(
            &self.0.client_id,
            &format!("http://localhost:{}/callback", self.0.callback_port),
        )
        .await;
        let http_client = create_http_client().map_err(|e| OAuthError::RevokeFailed {
            message: e.to_string(),
        })?;
        self.revoke_with(&client, &http_client).await
    }

    /// Revoke through `http_client` (injectable for tests), holding the token
    /// lock for the whole request.
    async fn revoke_with<C>(&self, client: &LongbridgeClient, http_client: &C) -> OAuthResult<()>
    where
        C: for<'c> AsyncHttpClient<'c>,
    {
        let guard = self.0.token.lock().await;
        let token = guard.as_ref().ok_or(OAuthError::NoTokenAvailable)?;

        let revocable = match &token.refresh_token {
            Some(refresh_token) => {
                StandardRevocableToken::RefreshToken(RefreshToken::new(refresh_token.clone()))
            }
            None => {
                StandardRevocableToken::AccessToken(AccessToken::new(token.access_token.clone()))
            }
        };

        tracing::debug!(client_id = %self.0.client_id, "revoking OAuth token");

        let request = client
            .revoke_token(revocable)
            .map_err(|e| OAuthError::RevokeFailed {
                message: e.to_string(),
            })?
            .request_async(http_client);
        tokio::time::timeout(TOKEN_REQUEST_TIMEOUT, request)
            .await
            .map_err(|_| {
                tracing::error!(client_id = %self.0.client_id, "token revocation timed out");
                OAuthError::RevokeFailed {
                    message: format!("timed out after {}s", TOKEN_REQUEST_TIMEOUT.as_secs()),
                }
            })?
            .map_err(|e| {
                let message = error_chain(&e);
                tracing::error!(client_id = %self.0.client_id, error = %message, "failed to revoke token");
                OAuthError::RevokeFailed { message }
            })?;

        drop(guard);
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
        let http_client = create_http_client().map_err(|e| OAuthError::ExchangeCodeFailed {
            message: e.to_string(),
        })?;
        let token_response = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| {
                tracing::error!(client_id = %self.0.client_id, error = %e, "failed to exchange authorization code for token");
                OAuthError::ExchangeCodeFailed {
                    message: error_chain(&e),
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
        // Refreshes run under the token lock (see `access_token`), so a hung
        // request must not hold it forever.
        let http_client = create_http_client().map_err(|e| OAuthError::RefreshTokenFailed {
            message: e.to_string(),
        })?;
        let refresh_token = RefreshToken::new(refresh_token_str.to_string());
        let request = client
            .exchange_refresh_token(&refresh_token)
            .request_async(&http_client);
        let token_response = tokio::time::timeout(TOKEN_REQUEST_TIMEOUT, request)
            .await
            .map_err(|_| {
                tracing::error!(client_id = %self.0.client_id, "token refresh timed out");
                OAuthError::RefreshTokenFailed {
                    message: format!("timed out after {}s", TOKEN_REQUEST_TIMEOUT.as_secs()),
                }
            })?
            .map_err(|e| {
                tracing::error!(client_id = %self.0.client_id, error = %e, "failed to refresh token");
                OAuthError::RefreshTokenFailed {
                    message: error_chain(&e),
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

/// Upper bound for a refresh or revocation request; both hold the token lock.
const TOKEN_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Total timeout of the HTTP client, which also bounds the authorization code
/// exchange. It must exceed `TOKEN_REQUEST_TIMEOUT`, so that for refresh and
/// revoke the tokio timeout fires first and reports "timed out after 30s"
/// instead of the client's generic transport error.
const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(35);
const _: () = assert!(HTTP_REQUEST_TIMEOUT.as_secs() > TOKEN_REQUEST_TIMEOUT.as_secs());

/// `e` followed by its `source()` chain. oauth2 5 reports every transport
/// failure as a bare "Request failed"; the cause (connection refused, timed
/// out, ...) is only in the sources.
fn error_chain(e: &dyn std::error::Error) -> String {
    let mut message = e.to_string();
    let mut source = e.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

/// Build the authorization URL with a fresh CSRF token and PKCE challenge.
///
/// PKCE (RFC 7636, `S256`) is required by the server for public clients and
/// ignored for clients that don't need it. The returned verifier must be sent
/// with the token exchange.
fn build_authorize_url(
    client: &LongbridgeClient,
) -> (oauth2::url::Url, CsrfToken, PkceCodeVerifier) {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(String::new()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    (auth_url, csrf_token, pkce_verifier)
}

/// Build the oauth2 BasicClient for Longbridge endpoints.
async fn create_oauth_client(client_id: &str, redirect_uri: &str) -> LongbridgeClient {
    build_oauth_client(&oauth_base_url().await, client_id, redirect_uri)
}

fn build_oauth_client(base: &str, client_id: &str, redirect_uri: &str) -> LongbridgeClient {
    BasicClient::new(ClientId::new(client_id.to_string()))
        .set_auth_uri(AuthUrl::new(format!("{base}/authorize")).unwrap())
        .set_token_uri(TokenUrl::new(format!("{base}/token")).unwrap())
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap())
        .set_revocation_url(RevocationUrl::new(format!("{base}/revoke")).unwrap())
}

/// HTTP client for token requests.
fn create_http_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        // Preserve the previous OAuth transport's protection against redirects.
        .redirect(reqwest::redirect::Policy::none())
        // Bounds every token request, including the authorization code
        // exchange. Refresh and revoke are wrapped in TOKEN_REQUEST_TIMEOUT,
        // which must fire first so they report "timed out after 30s".
        .connect_timeout(Duration::from_secs(10))
        .timeout(HTTP_REQUEST_TIMEOUT)
        .build()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use oauth2::{
        HttpRequest, HttpResponse, RequestTokenError, RevocationErrorResponseType,
        StandardErrorResponse,
    };

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
    fn server_error_messages_include_error_and_description() {
        let with_desc: RequestTokenError<std::io::Error, _> =
            RequestTokenError::ServerResponse(StandardErrorResponse::new(
                RevocationErrorResponseType::UnsupportedTokenType,
                Some("token type not supported".to_string()),
                None,
            ));
        assert_eq!(
            error_chain(&with_desc),
            "Server returned error response: unsupported_token_type: token type not supported"
        );

        let without_desc: RequestTokenError<std::io::Error, _> =
            RequestTokenError::ServerResponse(StandardErrorResponse::new(
                RevocationErrorResponseType::UnsupportedTokenType,
                None,
                None,
            ));
        assert_eq!(
            error_chain(&without_desc),
            "Server returned error response: unsupported_token_type"
        );
    }

    #[tokio::test]
    async fn refresh_transport_failure_reports_the_cause() {
        // A port with nothing listening: the connection is refused.
        let port = {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            listener.local_addr().unwrap().port()
        };
        let oauth = OAuth(Arc::new(OAuthInner {
            client_id: "my-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            storage: Arc::new(LegacyStorage),
            token: tokio::sync::Mutex::new(Some(OAuthToken {
                client_id: "my-client".to_string(),
                access_token: "access".to_string(),
                refresh_token: Some("refresh".to_string()),
                expires_at: 0,
            })),
        }));
        let result = TEST_OAUTH_BASE_URL
            .scope(
                format!("http://127.0.0.1:{port}/oauth2"),
                oauth.access_token(),
            )
            .await;
        match result {
            Err(OAuthError::RefreshTokenFailed { message }) => {
                // oauth2 5 alone would only say "Request failed".
                assert!(message.starts_with("Request failed: "), "{message}");
                assert!(message.contains("tcp connect error"), "{message}");
            }
            other => panic!("unexpected result: {other:?}"),
        }
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
    async fn revoke_holds_token_lock_for_the_whole_request() {
        let oauth = OAuth(Arc::new(OAuthInner {
            client_id: "my-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            storage: Arc::new(LegacyStorage),
            token: tokio::sync::Mutex::new(Some(OAuthToken {
                client_id: "my-client".to_string(),
                access_token: "access".to_string(),
                refresh_token: Some("refresh".to_string()),
                expires_at: u64::MAX,
            })),
        }));
        let client = build_oauth_client(
            OAUTH_BASE_URL,
            "my-client",
            "http://localhost:60355/callback",
        );

        // oauth2 5 requires an `Fn` client; it is called exactly once here.
        let (started_tx, started_rx) = tokio::sync::oneshot::channel::<HttpRequest>();
        let (release_tx, release_rx) = tokio::sync::oneshot::channel::<()>();
        let channels = std::sync::Mutex::new(Some((started_tx, release_rx)));
        let http_client = move |req: HttpRequest| {
            let (started_tx, release_rx) = channels.lock().unwrap().take().unwrap();
            async move {
                started_tx.send(req).unwrap();
                release_rx.await.unwrap();
                Ok::<_, std::io::Error>(HttpResponse::new(Vec::new()))
            }
        };

        let task = {
            let oauth = oauth.clone();
            tokio::spawn(async move { oauth.revoke_with(&client, &http_client).await })
        };

        let req = started_rx.await.unwrap();
        let body = String::from_utf8(req.into_body()).unwrap();
        assert!(body.contains("token=refresh"), "{body}");
        assert!(body.contains("token_type_hint=refresh_token"), "{body}");
        // A refresh (access_token) cannot run while the request is in flight.
        assert!(oauth.0.token.try_lock().is_err());

        release_tx.send(()).unwrap();
        task.await.unwrap().unwrap();
        assert!(oauth.0.token.try_lock().is_ok());
    }

    #[test]
    fn error_chain_includes_the_transport_cause() {
        let e: RequestTokenError<
            std::io::Error,
            StandardErrorResponse<RevocationErrorResponseType>,
        > = RequestTokenError::Request(std::io::Error::other("connection refused"));
        assert_eq!(error_chain(&e), "Request failed: connection refused");
    }

    #[tokio::test(start_paused = true)]
    async fn revoke_times_out_and_releases_the_token_lock() {
        let oauth = oauth_with_storage(Arc::new(LegacyStorage));
        let client = build_oauth_client(
            OAUTH_BASE_URL,
            "my-client",
            "http://localhost:60355/callback",
        );
        // The request never completes; the paused clock auto-advances to the
        // timeout.
        let http_client =
            |_req: HttpRequest| std::future::pending::<Result<HttpResponse, std::io::Error>>();

        match oauth.revoke_with(&client, &http_client).await {
            Err(OAuthError::RevokeFailed { message }) => {
                assert_eq!(message, "timed out after 30s")
            }
            other => panic!("unexpected result: {other:?}"),
        }
        assert_eq!(oauth.access_token().await.unwrap(), "access");
    }

    #[test]
    fn oauth_error_codes_are_stable() {
        let io = || std::io::Error::other("x");
        let json = || serde_json::from_str::<()>("x").unwrap_err();
        let path = std::path::PathBuf::new;
        let msg = String::new;
        let cases = [
            (OAuthError::Other(msg()), 0),
            (OAuthError::NoTokenAvailable, 1),
            (OAuthError::NoRefreshToken, 2),
            (OAuthError::RefreshTokenFailed { message: msg() }, 3),
            (OAuthError::RevokeFailed { message: msg() }, 4),
            (OAuthError::AuthorizationFailed { message: msg() }, 5),
            (OAuthError::AuthorizationTimeout, 6),
            (OAuthError::ExchangeCodeFailed { message: msg() }, 7),
            (OAuthError::CsrfTokenMismatch, 8),
            (
                OAuthError::BindCallbackFailed {
                    port: 0,
                    message: msg(),
                },
                9,
            ),
            (OAuthError::LocalAddressFailed, 10),
            (OAuthError::CallbackChannelClosed, 11),
            (OAuthError::NoHomeDir, 12),
            (
                OAuthError::TokenFileRead {
                    path: path(),
                    source: io(),
                },
                13,
            ),
            (
                OAuthError::TokenFileParse {
                    path: path(),
                    source: json(),
                },
                14,
            ),
            (
                OAuthError::TokenFileWrite {
                    path: path(),
                    source: io(),
                },
                15,
            ),
            (
                OAuthError::TokenFileDelete {
                    path: path(),
                    source: io(),
                },
                16,
            ),
            (
                OAuthError::CreateDirFailed {
                    path: path(),
                    source: io(),
                },
                17,
            ),
            (OAuthError::SerializeToken { source: json() }, 18),
        ];
        for (err, code) in cases {
            assert_eq!(err.code(), code, "{err:?}");
        }
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
