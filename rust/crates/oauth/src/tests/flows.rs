//! Regression tests for the public OAuth token lifecycle and endpoint
//! selection.

use std::{
    collections::{BTreeMap, VecDeque},
    process::Command,
    sync::{Arc, Mutex},
    time::Duration,
};

use oauth2::url::{Url, form_urlencoded};
use poem::{
    EndpointExt, Request, Response, Route, Server, handler,
    http::{Method, StatusCode},
    listener::{Acceptor, Listener, TcpListener},
    web::Data,
};
use tokio::{io::AsyncWriteExt, net::TcpStream, task::JoinHandle, time::timeout};

use super::{MemoryStorage, now_secs};

/// Upper bound for a whole loopback flow. It only guards against hangs, so it
/// is generous for slow CI runners.
const HANG_GUARD: Duration = Duration::from_secs(30);
use crate::{
    OAuth, OAuthBuilder, OAuthError, OAuthResult, StoredToken, TokenStorage,
    client::TEST_OAUTH_BASE_URL,
};

impl TokenStorage for Arc<MemoryStorage> {
    fn load(&self, client_id: &str) -> Option<StoredToken> {
        self.as_ref().load(client_id)
    }

    fn save(&self, token: &StoredToken) -> OAuthResult<()> {
        self.as_ref().save(token)
    }

    fn delete(&self, client_id: &str) -> OAuthResult<()> {
        self.as_ref().delete(client_id)
    }
}

struct FailSecondSaveStorage {
    inner: Arc<MemoryStorage>,
    save_attempts: Mutex<usize>,
}

impl TokenStorage for FailSecondSaveStorage {
    fn load(&self, client_id: &str) -> Option<StoredToken> {
        self.inner.load(client_id)
    }

    fn save(&self, token: &StoredToken) -> OAuthResult<()> {
        let mut attempts = self.save_attempts.lock().unwrap();
        *attempts += 1;
        if *attempts == 2 {
            return Err(OAuthError::Other("test storage save failed".to_string()));
        }
        self.inner.save(token)
    }
}

#[derive(Clone)]
struct TokenRequest {
    method: Method,
    path: String,
    content_type: Option<String>,
    authorization: Option<String>,
    body: String,
}

struct TokenServerState {
    responses: VecDeque<(StatusCode, String)>,
    requests: Vec<TokenRequest>,
    redirect_location: Option<String>,
}

struct TokenServer {
    base_url: String,
    state: Arc<Mutex<TokenServerState>>,
    task: JoinHandle<()>,
}

impl TokenServer {
    async fn start(responses: impl IntoIterator<Item = (StatusCode, String)>) -> Self {
        #[handler]
        async fn token_handler(
            request: &Request,
            body: String,
            state: Data<&Arc<Mutex<TokenServerState>>>,
        ) -> Response {
            let mut state = state.lock().unwrap();
            state.requests.push(TokenRequest {
                method: request.method().clone(),
                path: request.uri().path().to_string(),
                content_type: request
                    .headers()
                    .get("content-type")
                    .map(|value| value.to_str().unwrap().to_string()),
                authorization: request
                    .headers()
                    .get("authorization")
                    .map(|value| value.to_str().unwrap().to_string()),
                body,
            });
            let (status, body) = state
                .responses
                .pop_front()
                .expect("unexpected token request");
            let mut response = Response::builder()
                .status(status)
                .content_type("application/json");
            if status.is_redirection()
                && let Some(location) = state.redirect_location.as_deref()
            {
                response = response.header("location", location);
            }
            response.body(body)
        }

        let acceptor = TcpListener::bind("127.0.0.1:0")
            .into_acceptor()
            .await
            .unwrap();
        let port = acceptor
            .local_addr()
            .into_iter()
            .find_map(|address| address.as_socket_addr().map(|address| address.port()))
            .unwrap();
        let state = Arc::new(Mutex::new(TokenServerState {
            responses: responses.into_iter().collect(),
            requests: Vec::new(),
            redirect_location: None,
        }));
        let app = Route::new()
            .at("/oauth2/token", poem::post(token_handler))
            .at("/oauth2/revoke", poem::post(token_handler))
            .data(Arc::clone(&state));
        let task = tokio::spawn(async move {
            Server::new_with_acceptor(acceptor)
                .run(app)
                .await
                .expect("token server failed");
        });

        Self {
            base_url: format!("http://127.0.0.1:{port}/oauth2"),
            state,
            task,
        }
    }

    fn requests(&self) -> Vec<TokenRequest> {
        self.state.lock().unwrap().requests.clone()
    }

    fn redirect_to(&self, location: String) {
        self.state.lock().unwrap().redirect_location = Some(location);
    }
}

impl Drop for TokenServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn successful_token_response(
    access_token: &str,
    refresh_token: &str,
    expires_in: u64,
) -> (StatusCode, String) {
    (
        StatusCode::OK,
        serde_json::json!({
            "access_token": access_token,
            "refresh_token": refresh_token,
            "token_type": "Bearer",
            "expires_in": expires_in
        })
        .to_string(),
    )
}

fn assert_refresh_requests(server: &TokenServer, expected_refresh_tokens: &[&str]) {
    let requests = server.requests();
    assert_eq!(requests.len(), expected_refresh_tokens.len() + 1);
    for (request, refresh_token) in requests[1..].iter().zip(expected_refresh_tokens) {
        assert_eq!(request.method, Method::POST);
        assert_eq!(request.path, "/oauth2/token");
        assert!(request.authorization.is_none());
        let form: BTreeMap<_, _> = form_urlencoded::parse(request.body.as_bytes())
            .into_owned()
            .collect();
        assert_eq!(
            form,
            BTreeMap::from([
                ("client_id".to_string(), "test-client".to_string()),
                ("grant_type".to_string(), "refresh_token".to_string()),
                ("refresh_token".to_string(), (*refresh_token).to_string()),
            ])
        );
    }
}

fn callback_url(redirect_uri: &str, code: &str, state: &str) -> Url {
    let mut url = Url::parse(redirect_uri).unwrap();
    assert_eq!(url.path(), "/callback");
    url.query_pairs_mut()
        .append_pair("code", code)
        .append_pair("state", state);
    url
}

async fn send_callback(url: Url) {
    assert_eq!(url.scheme(), "http");
    assert_eq!(url.host_str(), Some("localhost"));
    let port = url.port().expect("callback must use an explicit port");
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let request = format!(
        "GET {}?{} HTTP/1.1\r\nHost: localhost:{port}\r\nConnection: close\r\n\r\n",
        url.path(),
        url.query().unwrap()
    );
    stream.write_all(request.as_bytes()).await.unwrap();
    stream.shutdown().await.unwrap();
}

async fn build_with_callback(storage: impl TokenStorage) -> OAuthResult<OAuth> {
    let callback = Mutex::new(None);
    let result = OAuthBuilder::new("test-client")
        .callback_port(0)
        .token_storage(storage)
        .build(|url| {
            let url = Url::parse(url).unwrap();
            let params: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
            let redirect = callback_url(
                params.get("redirect_uri").unwrap(),
                "authorization-code",
                params.get("state").unwrap(),
            );
            *callback.lock().unwrap() = Some(tokio::spawn(send_callback(redirect)));
        })
        .await;
    let callback = callback.lock().unwrap().take();
    if let Some(callback) = callback {
        callback.await.unwrap();
    }
    result
}

#[tokio::test]
async fn build_authorizes_and_persists_token() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([(
            StatusCode::OK,
            serde_json::json!({
                "access_token": "authorized-access-token",
                "refresh_token": "authorized-refresh-token",
                "token_type": "Bearer",
                "expires_in": 3600
            })
            .to_string(),
        )])
        .await;
        let storage = Arc::new(MemoryStorage::default());
        let authorization_url = Mutex::new(None);
        let callback = Mutex::new(None);
        let started_at = now_secs();

        let oauth = TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = OAuthBuilder::new("test-client")
                    .callback_port(0)
                    .token_storage(Arc::clone(&storage))
                    .build(|url| {
                        let url = Url::parse(url).unwrap();
                        let params: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
                        assert_eq!(url.path(), "/oauth2/authorize");
                        assert_eq!(params.get("client_id").unwrap(), "test-client");
                        assert_eq!(params.get("response_type").unwrap(), "code");
                        assert!(!params.get("state").unwrap().is_empty());
                        let redirect = callback_url(
                            params.get("redirect_uri").unwrap(),
                            "authorization-code+ /value",
                            params.get("state").unwrap(),
                        );
                        *authorization_url.lock().unwrap() = Some(url);
                        *callback.lock().unwrap() = Some(tokio::spawn(send_callback(redirect)));
                    })
                    .await
                    .unwrap();
                assert_eq!(
                    oauth.access_token().await.unwrap(),
                    "authorized-access-token"
                );
                oauth
            })
            .await;

        let callback = callback.lock().unwrap().take().unwrap();
        callback.await.unwrap();
        assert_eq!(oauth.client_id(), "test-client");
        let stored = storage.load("test-client").unwrap();
        assert_eq!(stored.client_id, "test-client");
        assert_eq!(stored.access_token, "authorized-access-token");
        assert_eq!(
            stored.refresh_token.as_deref(),
            Some("authorized-refresh-token")
        );
        assert!(stored.expires_at >= started_at + 3600);
        assert!(stored.expires_at <= now_secs() + 3600);
        assert_eq!(storage.save_count(), 1);

        let authorization_url = authorization_url.lock().unwrap().take().unwrap();
        let authorization_params: BTreeMap<_, _> =
            authorization_url.query_pairs().into_owned().collect();
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        assert_eq!(request.method, Method::POST);
        assert_eq!(request.path, "/oauth2/token");
        assert_eq!(
            request.content_type.as_deref(),
            Some("application/x-www-form-urlencoded")
        );
        assert!(request.authorization.is_none());
        let form: BTreeMap<_, _> = form_urlencoded::parse(request.body.as_bytes())
            .into_owned()
            .collect();
        assert_eq!(authorization_params["code_challenge_method"], "S256");
        let verifier = oauth2::PkceCodeVerifier::new(form["code_verifier"].clone());
        assert_eq!(
            authorization_params["code_challenge"],
            oauth2::PkceCodeChallenge::from_code_verifier_sha256(&verifier).as_str()
        );
        let expected = BTreeMap::from([
            ("client_id".to_string(), "test-client".to_string()),
            ("grant_type".to_string(), "authorization_code".to_string()),
            ("code".to_string(), "authorization-code+ /value".to_string()),
            ("code_verifier".to_string(), form["code_verifier"].clone()),
            (
                "redirect_uri".to_string(),
                authorization_params.get("redirect_uri").unwrap().clone(),
            ),
        ]);
        assert_eq!(form, expected);
        assert!(!form.contains_key("client_secret"));
    })
    .await
    .expect("OAuth authorization flow timed out");
}

#[tokio::test]
async fn revoke_rejects_insecure_endpoint_without_changing_local_tokens() {
    timeout(HANG_GUARD, async {
        for refresh_token in [Some("cached-refresh-token"), None] {
            let server = TokenServer::start([]).await;
            let storage = Arc::new(MemoryStorage::with_token(StoredToken {
                client_id: "test-client".to_string(),
                access_token: "cached-access-token".to_string(),
                refresh_token: refresh_token.map(str::to_string),
                expires_at: now_secs() + 7200,
            }));

            TEST_OAUTH_BASE_URL
                .scope(server.base_url.clone(), async {
                    let oauth = OAuthBuilder::new("test-client")
                        .token_storage(Arc::clone(&storage))
                        .build(|_| panic!("cached token must not trigger authorization"))
                        .await
                        .unwrap();
                    assert!(matches!(
                        oauth.revoke().await,
                        Err(OAuthError::RevokeFailed { message })
                            if message == "Scheme for revocation endpoint URL must be HTTPS"
                    ));
                    assert_eq!(oauth.access_token().await.unwrap(), "cached-access-token");
                })
                .await;

            let stored = storage.load("test-client").unwrap();
            assert_eq!(stored.access_token, "cached-access-token");
            assert_eq!(stored.refresh_token.as_deref(), refresh_token);
            assert_eq!(storage.save_count(), 0);
            assert!(server.requests().is_empty());
        }
    })
    .await
    .expect("OAuth insecure revocation rejection timed out");
}

#[tokio::test]
async fn build_rejects_mismatched_csrf_state_without_exchanging_token() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([]).await;
        let storage = Arc::new(MemoryStorage::default());
        let callback = Mutex::new(None);

        let result = TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                OAuthBuilder::new("test-client")
                    .callback_port(0)
                    .token_storage(Arc::clone(&storage))
                    .build(|url| {
                        let url = Url::parse(url).unwrap();
                        let params: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
                        let mismatched_state =
                            format!("{}-mismatched", params.get("state").unwrap());
                        let redirect = callback_url(
                            params.get("redirect_uri").unwrap(),
                            "authorization-code",
                            &mismatched_state,
                        );
                        *callback.lock().unwrap() = Some(tokio::spawn(send_callback(redirect)));
                    })
                    .await
            })
            .await;

        let callback = callback.lock().unwrap().take().unwrap();
        callback.await.unwrap();
        assert!(
            matches!(&result, Err(OAuthError::CsrfTokenMismatch)),
            "unexpected build result: {result:?}"
        );
        assert!(server.requests().is_empty());
        assert_eq!(storage.save_count(), 0);
        assert!(storage.load("test-client").is_none());
    })
    .await
    .expect("OAuth CSRF rejection flow timed out");
}

#[tokio::test]
async fn build_propagates_access_denied_callback_without_exchanging_token() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([]).await;
        let storage = Arc::new(MemoryStorage::default());
        let callback = Mutex::new(None);

        let result = TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                OAuthBuilder::new("test-client")
                    .callback_port(0)
                    .token_storage(Arc::clone(&storage))
                    .build(|url| {
                        let url = Url::parse(url).unwrap();
                        let params: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
                        let mut redirect = Url::parse(params.get("redirect_uri").unwrap()).unwrap();
                        assert_eq!(redirect.path(), "/callback");
                        redirect
                            .query_pairs_mut()
                            .append_pair("error", "access_denied");
                        *callback.lock().unwrap() = Some(tokio::spawn(send_callback(redirect)));
                    })
                    .await
            })
            .await;

        let callback = callback.lock().unwrap().take().unwrap();
        callback.await.unwrap();
        match result {
            Err(OAuthError::AuthorizationFailed { message }) => {
                assert_eq!(message, "access_denied");
            }
            other => panic!("unexpected access_denied callback result: {other:?}"),
        }
        assert!(server.requests().is_empty());
        assert_eq!(storage.save_count(), 0);
        assert!(storage.load("test-client").is_none());
    })
    .await
    .expect("OAuth access_denied callback flow timed out");
}

async fn assert_refresh_persists_and_caches(response: String, expected_refresh_token: &str) {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([
            (
                StatusCode::OK,
                serde_json::json!({
                    "access_token": "expired-access-token",
                    "refresh_token": "old-refresh-token",
                    "token_type": "Bearer",
                    "expires_in": 0
                })
                .to_string(),
            ),
            (StatusCode::OK, response),
        ])
        .await;
        let storage = Arc::new(MemoryStorage::default());
        let started_at = now_secs();

        TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = build_with_callback(Arc::clone(&storage)).await.unwrap();
                assert_eq!(storage.save_count(), 1);
                assert_eq!(
                    storage.load("test-client").unwrap().access_token,
                    "expired-access-token"
                );
                assert_eq!(server.requests().len(), 1);

                assert_eq!(
                    oauth.access_token().await.unwrap(),
                    "refreshed-access-token"
                );
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.client_id, "test-client");
                assert_eq!(stored.access_token, "refreshed-access-token");
                assert_eq!(
                    stored.refresh_token.as_deref(),
                    Some(expected_refresh_token)
                );
                assert!(stored.expires_at >= started_at + 3600);
                assert!(stored.expires_at <= now_secs() + 3600);
                assert_eq!(storage.save_count(), 2);

                let requests = server.requests();
                assert_eq!(requests.len(), 2);
                let request = &requests[1];
                assert_eq!(request.method, Method::POST);
                assert_eq!(request.path, "/oauth2/token");
                assert!(request.authorization.is_none());
                let form: BTreeMap<_, _> = form_urlencoded::parse(request.body.as_bytes())
                    .into_owned()
                    .collect();
                assert_eq!(
                    form,
                    BTreeMap::from([
                        ("client_id".to_string(), "test-client".to_string()),
                        ("grant_type".to_string(), "refresh_token".to_string()),
                        ("refresh_token".to_string(), "old-refresh-token".to_string()),
                    ])
                );

                assert_eq!(
                    oauth.access_token().await.unwrap(),
                    "refreshed-access-token"
                );
                assert_eq!(server.requests().len(), 2);
                assert_eq!(storage.save_count(), 2);
            })
            .await;
    })
    .await
    .expect("OAuth refresh flow timed out");
}

#[tokio::test]
async fn access_token_rotates_refresh_token_and_caches_result() {
    assert_refresh_persists_and_caches(
        serde_json::json!({
            "access_token": "refreshed-access-token",
            "refresh_token": "new-refresh-token",
            "token_type": "Bearer",
            "expires_in": 3600
        })
        .to_string(),
        "new-refresh-token",
    )
    .await;
}

#[tokio::test]
async fn access_token_preserves_refresh_token_when_response_omits_it() {
    assert_refresh_persists_and_caches(
        serde_json::json!({
            "access_token": "refreshed-access-token",
            "token_type": "Bearer",
            "expires_in": 3600
        })
        .to_string(),
        "old-refresh-token",
    )
    .await;
}

#[tokio::test]
async fn access_token_retries_failed_refresh_with_original_token() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([
            successful_token_response("expired-access-token", "old-refresh-token", 0),
            (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid_grant",
                    "error_description": "refresh token rejected"
                })
                .to_string(),
            ),
            successful_token_response("retry-access-token", "retry-refresh-token", 3600),
        ])
        .await;
        let storage = Arc::new(MemoryStorage::default());

        TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = build_with_callback(Arc::clone(&storage)).await.unwrap();
                let error = oauth.access_token().await.unwrap_err();
                assert!(
                    matches!(&error, OAuthError::RefreshTokenFailed { message } if !message.is_empty()),
                    "unexpected refresh error: {error:?}"
                );
                assert_eq!(storage.save_count(), 1);
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.access_token, "expired-access-token");
                assert_eq!(stored.refresh_token.as_deref(), Some("old-refresh-token"));
                assert_refresh_requests(&server, &["old-refresh-token"]);

                assert_eq!(oauth.access_token().await.unwrap(), "retry-access-token");
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.access_token, "retry-access-token");
                assert_eq!(stored.refresh_token.as_deref(), Some("retry-refresh-token"));
                assert_eq!(storage.save_count(), 2);
                assert_refresh_requests(&server, &["old-refresh-token", "old-refresh-token"]);

                assert_eq!(oauth.access_token().await.unwrap(), "retry-access-token");
                assert_eq!(server.requests().len(), 3);
                assert_eq!(storage.save_count(), 2);
            })
            .await;
    })
    .await
    .expect("OAuth failed-refresh retry flow timed out");
}

#[tokio::test]
async fn access_token_preserves_cached_token_until_refreshed_token_is_saved() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([
            successful_token_response("expired-access-token", "old-refresh-token", 0),
            successful_token_response("unsaved-access-token", "unsaved-refresh-token", 3600),
            successful_token_response("retry-access-token", "retry-refresh-token", 3600),
        ])
        .await;
        let storage = Arc::new(MemoryStorage::default());

        TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = build_with_callback(FailSecondSaveStorage {
                    inner: Arc::clone(&storage),
                    save_attempts: Mutex::new(0),
                })
                .await
                .unwrap();
                let error = oauth.access_token().await.unwrap_err();
                assert!(
                    matches!(&error, OAuthError::Other(message) if message == "test storage save failed"),
                    "unexpected storage error: {error:?}"
                );
                assert_eq!(storage.save_count(), 1);
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.access_token, "expired-access-token");
                assert_eq!(stored.refresh_token.as_deref(), Some("old-refresh-token"));
                assert_refresh_requests(&server, &["old-refresh-token"]);

                assert_eq!(oauth.access_token().await.unwrap(), "retry-access-token");
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.access_token, "retry-access-token");
                assert_eq!(stored.refresh_token.as_deref(), Some("retry-refresh-token"));
                assert_eq!(storage.save_count(), 2);
                assert_refresh_requests(&server, &["old-refresh-token", "old-refresh-token"]);

                assert_eq!(oauth.access_token().await.unwrap(), "retry-access-token");
                assert_eq!(server.requests().len(), 3);
                assert_eq!(storage.save_count(), 2);
            })
            .await;
    })
    .await
    .expect("OAuth failed-save retry flow timed out");
}

#[tokio::test]
async fn build_propagates_authorization_exchange_error_without_saving_token() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([(
            StatusCode::BAD_REQUEST,
            serde_json::json!({
                "error": "invalid_grant",
                "error_description": "authorization code rejected"
            })
            .to_string(),
        )])
        .await;
        let storage = Arc::new(MemoryStorage::default());

        let error = TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                build_with_callback(Arc::clone(&storage)).await
            })
            .await
            .unwrap_err();

        assert!(
            matches!(&error, OAuthError::ExchangeCodeFailed { message } if !message.is_empty()),
            "unexpected authorization exchange error: {error:?}"
        );
        assert_eq!(storage.save_count(), 0);
        assert!(storage.load("test-client").is_none());
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, Method::POST);
        assert_eq!(requests[0].path, "/oauth2/token");
        let form: BTreeMap<_, _> = form_urlencoded::parse(requests[0].body.as_bytes())
            .into_owned()
            .collect();
        assert_eq!(form.get("client_id").unwrap(), "test-client");
        assert_eq!(form.get("grant_type").unwrap(), "authorization_code");
        assert_eq!(form.get("code").unwrap(), "authorization-code");
    })
    .await
    .expect("OAuth authorization exchange error flow timed out");
}

#[tokio::test]
async fn build_reauthorizes_when_cached_token_refresh_fails() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([
            (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid_grant",
                    "error_description": "cached refresh token rejected"
                })
                .to_string(),
            ),
            successful_token_response("authorized-access-token", "authorized-refresh-token", 3600),
        ])
        .await;
        let storage = Arc::new(MemoryStorage::with_token(StoredToken {
            client_id: "test-client".to_string(),
            access_token: "expired-access-token".to_string(),
            refresh_token: Some("old-refresh-token".to_string()),
            expires_at: 0,
        }));

        TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = build_with_callback(Arc::clone(&storage)).await.unwrap();
                assert_eq!(
                    oauth.access_token().await.unwrap(),
                    "authorized-access-token"
                );
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.client_id, "test-client");
                assert_eq!(stored.access_token, "authorized-access-token");
                assert_eq!(
                    stored.refresh_token.as_deref(),
                    Some("authorized-refresh-token")
                );
                assert_eq!(storage.save_count(), 1);

                let requests = server.requests();
                assert_eq!(requests.len(), 2);
                let refresh_form: BTreeMap<_, _> =
                    form_urlencoded::parse(requests[0].body.as_bytes())
                        .into_owned()
                        .collect();
                assert_eq!(
                    refresh_form,
                    BTreeMap::from([
                        ("client_id".to_string(), "test-client".to_string()),
                        ("grant_type".to_string(), "refresh_token".to_string()),
                        ("refresh_token".to_string(), "old-refresh-token".to_string()),
                    ])
                );
                let authorization_form: BTreeMap<_, _> =
                    form_urlencoded::parse(requests[1].body.as_bytes())
                        .into_owned()
                        .collect();
                assert_eq!(authorization_form.get("client_id").unwrap(), "test-client");
                assert_eq!(
                    authorization_form.get("grant_type").unwrap(),
                    "authorization_code"
                );
                assert_eq!(
                    authorization_form.get("code").unwrap(),
                    "authorization-code"
                );
            })
            .await;
    })
    .await
    .expect("OAuth refresh fallback authorization flow timed out");
}

#[tokio::test]
async fn build_does_not_follow_authorization_token_redirect() {
    timeout(HANG_GUARD, async {
        let probe = TokenServer::start([successful_token_response(
            "redirected-access-token",
            "redirected-refresh-token",
            3600,
        )])
        .await;
        let server = TokenServer::start([(StatusCode::TEMPORARY_REDIRECT, "{}".to_string())]).await;
        server.redirect_to(format!("{}/token", probe.base_url));
        let storage = Arc::new(MemoryStorage::default());

        let error = TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                build_with_callback(Arc::clone(&storage)).await
            })
            .await
            .unwrap_err();

        assert!(
            matches!(&error, OAuthError::ExchangeCodeFailed { message } if !message.is_empty()),
            "unexpected authorization redirect error: {error:?}"
        );
        assert_eq!(storage.save_count(), 0);
        assert!(storage.load("test-client").is_none());
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        let form: BTreeMap<_, _> = form_urlencoded::parse(requests[0].body.as_bytes())
            .into_owned()
            .collect();
        assert_eq!(form.get("grant_type").unwrap(), "authorization_code");
        assert!(probe.requests().is_empty());
    })
    .await
    .expect("OAuth authorization redirect rejection flow timed out");
}

#[tokio::test]
async fn access_token_does_not_follow_refresh_redirect_and_can_retry() {
    timeout(HANG_GUARD, async {
        let probe = TokenServer::start([successful_token_response(
            "redirected-access-token",
            "redirected-refresh-token",
            3600,
        )])
        .await;
        let server = TokenServer::start([
            successful_token_response("expired-access-token", "old-refresh-token", 0),
            (StatusCode::TEMPORARY_REDIRECT, "{}".to_string()),
            successful_token_response("retry-access-token", "retry-refresh-token", 3600),
        ])
        .await;
        server.redirect_to(format!("{}/token", probe.base_url));
        let storage = Arc::new(MemoryStorage::default());

        TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = build_with_callback(Arc::clone(&storage)).await.unwrap();
                let error = oauth.access_token().await.unwrap_err();
                assert!(
                    matches!(&error, OAuthError::RefreshTokenFailed { message } if !message.is_empty()),
                    "unexpected refresh redirect error: {error:?}"
                );
                assert_eq!(storage.save_count(), 1);
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.access_token, "expired-access-token");
                assert_eq!(stored.refresh_token.as_deref(), Some("old-refresh-token"));
                assert_refresh_requests(&server, &["old-refresh-token"]);
                assert!(probe.requests().is_empty());

                assert_eq!(oauth.access_token().await.unwrap(), "retry-access-token");
                let stored = storage.load("test-client").unwrap();
                assert_eq!(stored.access_token, "retry-access-token");
                assert_eq!(stored.refresh_token.as_deref(), Some("retry-refresh-token"));
                assert_eq!(storage.save_count(), 2);
                assert_refresh_requests(&server, &["old-refresh-token", "old-refresh-token"]);

                assert_eq!(oauth.access_token().await.unwrap(), "retry-access-token");
                assert_eq!(server.requests().len(), 3);
                assert_eq!(storage.save_count(), 2);
                assert!(probe.requests().is_empty());
            })
            .await;
    })
    .await
    .expect("OAuth refresh redirect rejection flow timed out");
}

#[tokio::test]
async fn build_defaults_missing_expiry_to_one_hour() {
    timeout(HANG_GUARD, async {
        let server = TokenServer::start([(
            StatusCode::OK,
            serde_json::json!({
                "access_token": "default-expiry-access-token",
                "refresh_token": "default-expiry-refresh-token",
                "token_type": "Bearer"
            })
            .to_string(),
        )])
        .await;
        let storage = Arc::new(MemoryStorage::default());
        let started_at = now_secs();

        TEST_OAUTH_BASE_URL
            .scope(server.base_url.clone(), async {
                let oauth = build_with_callback(Arc::clone(&storage)).await.unwrap();
                let stored = storage.load("test-client").unwrap();
                assert!(stored.expires_at >= started_at + 3600);
                assert!(stored.expires_at <= now_secs() + 3600);
                assert_eq!(
                    oauth.access_token().await.unwrap(),
                    "default-expiry-access-token"
                );
                assert_eq!(storage.save_count(), 1);
                assert_eq!(server.requests().len(), 1);
            })
            .await;
    })
    .await
    .expect("OAuth default expiry flow timed out");
}

#[cfg(feature = "blocking")]
#[test]
fn build_blocking_loads_valid_cached_token_without_authorizing() {
    let storage = Arc::new(MemoryStorage::with_token(StoredToken {
        client_id: "test-client".to_string(),
        access_token: "cached-access-token".to_string(),
        refresh_token: Some("cached-refresh-token".to_string()),
        expires_at: now_secs() + 7200,
    }));
    let oauth = OAuthBuilder::new("test-client")
        .token_storage(Arc::clone(&storage))
        .build_blocking(|_| panic!("valid cached token must not trigger authorization"))
        .unwrap();
    assert_eq!(oauth.client_id(), "test-client");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let token = runtime.block_on(async {
        timeout(HANG_GUARD, oauth.access_token())
            .await
            .expect("cached access token lookup timed out")
            .unwrap()
    });
    assert_eq!(token, "cached-access-token");
    assert_eq!(storage.save_count(), 0);
}

#[test]
fn authorization_endpoint_selection_uses_isolated_environments() {
    for (environment, region, expected_url) in [
        (
            Some("staging"),
            "CN",
            "https://openapi-global.longbridge.xyz/oauth2/authorize",
        ),
        (None, "CN", "https://openapi.longbridge.cn/oauth2/authorize"),
        (
            None,
            "US",
            "https://openapi.longbridge.com/oauth2/authorize",
        ),
    ] {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--ignored",
                "--exact",
                "tests::flows::authorization_endpoint_selection_child",
                "--nocapture",
            ])
            .env("LONGBRIDGE_REGION", region)
            .env_remove("LONGPORT_REGION")
            .env("TEST_OAUTH_EXPECTED_AUTHORIZATION_URL", expected_url);
        if let Some(environment) = environment {
            command.env("LONGBRIDGE_ENV", environment);
        } else {
            command.env_remove("LONGBRIDGE_ENV");
        }
        let output = command
            .output()
            .expect("failed to launch endpoint selection child");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success()
                && stdout.contains(&format!("verified authorization endpoint: {expected_url}")),
            "endpoint selection failed for environment={environment:?}, region={region}:\n{}\n{}",
            stdout,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[tokio::test]
#[ignore = "launched by authorization_endpoint_selection_uses_isolated_environments"]
async fn authorization_endpoint_selection_child() {
    let expected_url = std::env::var("TEST_OAUTH_EXPECTED_AUTHORIZATION_URL")
        .expect("endpoint selection child must be launched by its parent test");
    timeout(HANG_GUARD, async {
        let storage = Arc::new(MemoryStorage::default());
        let callback = Mutex::new(None);
        let result = OAuthBuilder::new("test-client")
            .callback_port(0)
            .token_storage(Arc::clone(&storage))
            .build(|url| {
                let url = Url::parse(url).unwrap();
                assert_eq!(
                    format!("{}{}", url.origin().ascii_serialization(), url.path()),
                    expected_url
                );
                let params: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
                let mismatched_state = format!("{}-mismatched", params.get("state").unwrap());
                let redirect = callback_url(
                    params.get("redirect_uri").unwrap(),
                    "authorization-code",
                    &mismatched_state,
                );
                *callback.lock().unwrap() = Some(tokio::spawn(send_callback(redirect)));
            })
            .await;
        let callback = callback.lock().unwrap().take().unwrap();
        callback.await.unwrap();
        assert!(
            matches!(&result, Err(OAuthError::CsrfTokenMismatch)),
            "unexpected endpoint selection result: {result:?}"
        );
        assert_eq!(storage.save_count(), 0);
        assert!(storage.load("test-client").is_none());
        println!("verified authorization endpoint: {expected_url}");
    })
    .await
    .expect("OAuth endpoint selection callback flow timed out");
}
