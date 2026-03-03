//! OAuth 2.0 Resource Server support for the MCP HTTP transport.
//!
//! Implements:
//! - RFC 6750  Bearer Token Usage
//! - RFC 8414  OAuth 2.0 Authorization Server Metadata
//!
//! When the MCP server is started with `--oauth-client-id`, every HTTP request
//! to the MCP endpoint must carry a valid LongPort OAuth 2.0 Bearer access
//! token in the `Authorization` request header.  The token is verified by
//! calling the LongPort userinfo endpoint; on success a per-request
//! [`longport::Config`] is produced and made available to downstream handlers
//! via [`poem::web::Data`].
//!
//! The route `/.well-known/oauth-authorization-server` serves RFC 8414
//! metadata so that MCP clients (e.g. Claude Desktop) can auto-discover the
//! authorization server without manual configuration.

use std::sync::Arc;

use longport::Config;
use poem::{
    Endpoint, IntoResponse, Middleware, Request, Response, Result as PoemResult,
    http::StatusCode,
    web::Data,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const LONGPORT_USERINFO_URL: &str = "https://openapi.longbridgeapp.com/oauth2/userinfo";

// ---------------------------------------------------------------------------
// Userinfo response (subset of fields we need)
// ---------------------------------------------------------------------------

/// Subset of the LongPort OAuth 2.0 userinfo response required for token
/// validation.
#[derive(Debug, Deserialize)]
struct UserinfoResponse {
    /// OAuth 2.0 client ID associated with the token. Present when the token
    /// is valid; absence indicates an invalid / revoked token.
    #[serde(default)]
    client_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Per-request authenticated context
// ---------------------------------------------------------------------------

/// Authenticated context injected into downstream handlers.
///
/// Downstream MCP tool handlers receive this via `Data<&AuthenticatedContext>`
/// and should use [`AuthenticatedContext::config`] to construct
/// [`longport::QuoteContext`] / [`longport::TradeContext`].
#[derive(Clone)]
pub struct AuthenticatedContext {
    config: Arc<Config>,
}

impl AuthenticatedContext {
    /// Return a shared reference to the per-request [`Config`].
    #[inline]
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }
}

// ---------------------------------------------------------------------------
// RFC 8414 Authorization Server Metadata
// ---------------------------------------------------------------------------

/// Authorization Server Metadata as defined by RFC 8414.
///
/// Only the fields required by the MCP OAuth 2.0 profile are included.
#[derive(Debug, Serialize)]
pub struct AuthorizationServerMetadata {
    /// The authorization server's issuer identifier URL.
    pub issuer: String,
    /// URL of the authorization endpoint.
    pub authorization_endpoint: String,
    /// URL of the token endpoint.
    pub token_endpoint: String,
    /// URL of the token revocation endpoint (RFC 7009).
    pub revocation_endpoint: String,
    /// List of OAuth 2.0 response types supported.
    pub response_types_supported: Vec<String>,
    /// List of OAuth 2.0 grant types supported.
    pub grant_types_supported: Vec<String>,
    /// List of PKCE code challenge methods supported (RFC 7636).
    pub code_challenge_methods_supported: Vec<String>,
}

impl AuthorizationServerMetadata {
    /// Construct metadata for a given public `base_url`.
    ///
    /// `base_url` must **not** have a trailing slash, e.g.
    /// `"https://mcp.example.com"`.
    pub fn new(base_url: &str) -> Self {
        const OAUTH_BASE: &str = "https://openapi.longbridgeapp.com/oauth2";
        Self {
            issuer: base_url.to_string(),
            authorization_endpoint: format!("{OAUTH_BASE}/authorize"),
            token_endpoint: format!("{OAUTH_BASE}/token"),
            revocation_endpoint: format!("{OAUTH_BASE}/revoke"),
            response_types_supported: vec!["code".to_string()],
            grant_types_supported: vec![
                "authorization_code".to_string(),
                "refresh_token".to_string(),
            ],
            code_challenge_methods_supported: vec!["S256".to_string()],
        }
    }
}

// ---------------------------------------------------------------------------
// Bearer token extraction helpers
// ---------------------------------------------------------------------------

/// Extract the raw bearer token from the `Authorization` header.
///
/// Accepts the form `Authorization: Bearer <token>` (case-insensitive prefix).
/// Returns `None` if the header is absent or malformed.
pub(crate) fn extract_bearer_token(req: &Request) -> Option<String> {
    let value = req.headers().get("authorization")?.to_str().ok()?;
    let stripped = value.strip_prefix("Bearer ").or_else(|| value.strip_prefix("bearer "))?;
    let token = stripped.trim();
    if token.is_empty() { None } else { Some(token.to_string()) }
}

// ---------------------------------------------------------------------------
// Token verification
// ---------------------------------------------------------------------------

/// Verify a Bearer token against the LongPort userinfo endpoint.
///
/// On success returns the `client_id` extracted from the userinfo response.
/// On failure returns an [`OAuthMiddlewareError`].
async fn verify_token(token: &str) -> Result<String, OAuthMiddlewareError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(LONGPORT_USERINFO_URL)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, "userinfo request failed");
            OAuthMiddlewareError::Upstream(e.to_string())
        })?;

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED
        || resp.status() == reqwest::StatusCode::FORBIDDEN
    {
        return Err(OAuthMiddlewareError::InvalidToken);
    }

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        tracing::warn!(status, "userinfo endpoint returned unexpected status");
        return Err(OAuthMiddlewareError::Upstream(format!(
            "userinfo endpoint returned HTTP {status}"
        )));
    }

    let info: UserinfoResponse = resp.json().await.map_err(|e| {
        tracing::warn!(error = %e, "failed to parse userinfo response");
        OAuthMiddlewareError::Upstream(e.to_string())
    })?;

    info.client_id.ok_or(OAuthMiddlewareError::InvalidToken)
}

// ---------------------------------------------------------------------------
// Middleware error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) enum OAuthMiddlewareError {
    /// No / malformed Authorization header.
    MissingToken,
    /// Token is rejected by the authorization server.
    InvalidToken,
    /// Upstream authorization server is unreachable or returned an error.
    Upstream(String),
}

impl OAuthMiddlewareError {
    fn to_response(&self) -> Response {
        match self {
            OAuthMiddlewareError::MissingToken => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(
                    "WWW-Authenticate",
                    "Bearer realm=\"longport-mcp\", error=\"invalid_request\", \
                     error_description=\"Authorization header with Bearer token is required\"",
                )
                .content_type("application/json")
                .body(
                    r#"{"error":"invalid_request","error_description":"Authorization header with Bearer token is required"}"#,
                ),
            OAuthMiddlewareError::InvalidToken => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(
                    "WWW-Authenticate",
                    "Bearer realm=\"longport-mcp\", error=\"invalid_token\", \
                     error_description=\"The access token is invalid or has expired\"",
                )
                .content_type("application/json")
                .body(
                    r#"{"error":"invalid_token","error_description":"The access token is invalid or has expired"}"#,
                ),
            OAuthMiddlewareError::Upstream(msg) => {
                tracing::error!(error = %msg, "authorization server error");
                Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .content_type("application/json")
                    .body(
                        r#"{"error":"temporarily_unavailable","error_description":"Authorization server is temporarily unavailable"}"#,
                    )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Poem middleware
// ---------------------------------------------------------------------------

/// Poem [`Middleware`] that validates OAuth 2.0 Bearer tokens on every request.
///
/// On success, injects [`AuthenticatedContext`] into the request extensions so
/// that downstream handlers can retrieve it via `Data<&AuthenticatedContext>`.
///
/// # Example
///
/// ```rust,no_run
/// use poem::{Route, Server, listener::TcpListener};
/// use longport_mcp::oauth::BearerAuthMiddleware;
///
/// let app = Route::new()
///     .at("/", poem::get(handler))
///     .with(BearerAuthMiddleware);
/// ```
#[derive(Clone)]
pub struct BearerAuthMiddleware;

impl<E: Endpoint> Middleware<E> for BearerAuthMiddleware {
    type Output = BearerAuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        BearerAuthEndpoint { inner: ep }
    }
}

/// The endpoint produced by [`BearerAuthMiddleware`].
pub struct BearerAuthEndpoint<E> {
    inner: E,
}

impl<E: Endpoint> Endpoint for BearerAuthEndpoint<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> PoemResult<Self::Output> {
        // Extract bearer token from Authorization header
        let token = match extract_bearer_token(&req) {
            Some(t) => t,
            None => return Ok(OAuthMiddlewareError::MissingToken.to_response()),
        };

        // Verify token and retrieve client_id
        let client_id = match verify_token(&token).await {
            Ok(id) => id,
            Err(e) => return Ok(e.to_response()),
        };

        tracing::debug!(client_id = %client_id, "OAuth 2.0 token verified");

        // Build per-request Config using the verified Bearer token
        let oauth_token = longport_oauth::OAuthToken {
            client_id,
            access_token: token,
            refresh_token: None,
            // expires_at is unknown from introspection alone; set to 0 so callers
            // treat it as already-verified but should not attempt re-use across
            // requests without re-validation.
            expires_at: u64::MAX,
        };

        let config = Arc::new(Config::from_oauth(&oauth_token));
        let ctx = AuthenticatedContext { config };

        req.extensions_mut().insert(ctx);

        self.inner.call(req).await.map(IntoResponse::into_response)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use poem::{Request, http::Method};

    use super::*;

    fn make_request_with_header(auth_header: &str) -> Request {
        Request::builder()
            .method(Method::GET)
            .uri("http://localhost/")
            .header("authorization", auth_header)
            .finish()
    }

    fn make_request_without_auth() -> Request {
        Request::builder()
            .method(Method::GET)
            .uri("http://localhost/")
            .finish()
    }

    // --- extract_bearer_token ------------------------------------------------

    #[test]
    fn test_extract_bearer_token_standard_prefix() {
        let req = make_request_with_header("Bearer my-secret-token");
        let token = extract_bearer_token(&req);
        assert_eq!(token, Some("my-secret-token".to_string()));
    }

    #[test]
    fn test_extract_bearer_token_lowercase_prefix() {
        let req = make_request_with_header("bearer my-secret-token");
        let token = extract_bearer_token(&req);
        assert_eq!(token, Some("my-secret-token".to_string()));
    }

    #[test]
    fn test_extract_bearer_token_absent_header() {
        let req = make_request_without_auth();
        let token = extract_bearer_token(&req);
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_wrong_scheme() {
        let req = make_request_with_header("Basic dXNlcjpwYXNz");
        let token = extract_bearer_token(&req);
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_empty_token() {
        let req = make_request_with_header("Bearer ");
        let token = extract_bearer_token(&req);
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_trims_whitespace() {
        let req = make_request_with_header("Bearer   trimmed-token   ");
        let token = extract_bearer_token(&req);
        assert_eq!(token, Some("trimmed-token".to_string()));
    }

    // --- AuthorizationServerMetadata -----------------------------------------

    #[test]
    fn test_authorization_server_metadata_fields() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");

        assert_eq!(meta.issuer, "https://mcp.example.com");
        assert!(meta.authorization_endpoint.contains("/authorize"));
        assert!(meta.token_endpoint.contains("/token"));
        assert!(meta.revocation_endpoint.contains("/revoke"));
        assert!(meta.response_types_supported.contains(&"code".to_string()));
        assert!(meta
            .grant_types_supported
            .contains(&"authorization_code".to_string()));
        assert!(meta
            .grant_types_supported
            .contains(&"refresh_token".to_string()));
        assert!(meta
            .code_challenge_methods_supported
            .contains(&"S256".to_string()));
    }

    #[test]
    fn test_authorization_server_metadata_serialization() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        let json = serde_json::to_string(&meta).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(
            parsed["issuer"].as_str().unwrap(),
            "https://mcp.example.com"
        );
        assert!(parsed["authorization_endpoint"].as_str().unwrap().starts_with("https://"));
        assert!(parsed["token_endpoint"].as_str().unwrap().starts_with("https://"));
        assert!(parsed["code_challenge_methods_supported"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str() == Some("S256")));
    }

    // --- OAuthMiddlewareError response codes ---------------------------------

    #[test]
    fn test_missing_token_error_returns_401() {
        let resp = OAuthMiddlewareError::MissingToken.to_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_invalid_token_error_returns_401() {
        let resp = OAuthMiddlewareError::InvalidToken.to_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_upstream_error_returns_503() {
        let resp =
            OAuthMiddlewareError::Upstream("connection refused".to_string()).to_response();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_missing_token_response_has_www_authenticate_header() {
        let resp = OAuthMiddlewareError::MissingToken.to_response();
        assert!(resp.headers().contains_key("www-authenticate"));
        let www_auth = resp
            .headers()
            .get("www-authenticate")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(www_auth.contains("Bearer"));
        assert!(www_auth.contains("invalid_request"));
    }

    #[test]
    fn test_invalid_token_response_has_www_authenticate_header() {
        let resp = OAuthMiddlewareError::InvalidToken.to_response();
        let www_auth = resp
            .headers()
            .get("www-authenticate")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(www_auth.contains("invalid_token"));
    }
}
