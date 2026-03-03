//! OAuth 2.0 Resource Server support for the MCP HTTP transport.
//!
//! Implements:
//! - RFC 6750  Bearer Token Usage
//! - RFC 8414  OAuth 2.0 Authorization Server Metadata
//!
//! When the MCP server is started with `--oauth`, every HTTP request to the
//! MCP endpoint must carry a valid LongPort OAuth 2.0 Bearer access token in
//! the `Authorization` request header.  The token is verified by calling the
//! LongPort userinfo endpoint; on success a per-request [`longport::Config`]
//! is produced and made available to downstream handlers via
//! [`poem::web::Data`] / request extensions.
//!
//! The route `/.well-known/oauth-authorization-server` serves RFC 8414
//! metadata so that MCP clients (e.g. Claude Desktop) can auto-discover the
//! authorization server without manual configuration.

use std::sync::Arc;

use longport::Config;
use poem::{
    Endpoint, IntoResponse, Middleware, Request, Response, Result as PoemResult,
    http::StatusCode,
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
    /// OAuth 2.0 client ID associated with the token.  Present when the token
    /// is valid; absence indicates an invalid or revoked token.
    #[serde(default)]
    client_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Per-request authenticated context
// ---------------------------------------------------------------------------

/// Authenticated context injected into request extensions by
/// [`BearerAuthMiddleware`].
///
/// Downstream handlers (e.g. the MCP session factory) retrieve this via
/// `req.extensions().get::<AuthenticatedContext>()` and call
/// [`AuthenticatedContext::build_config`] to obtain a fresh [`Config`].
#[derive(Clone)]
pub struct AuthenticatedContext {
    /// The raw Bearer token extracted from the `Authorization` header.
    access_token: String,
    /// The `client_id` returned by the LongPort userinfo endpoint.
    client_id: String,
}

impl AuthenticatedContext {
    /// Build a [`Config`] from this context.
    ///
    /// A new [`Config`] is constructed every time this method is called; it
    /// does **not** cache the result so callers can apply further builder
    /// methods (e.g. `dont_print_quote_packages()`).
    pub fn build_config(&self) -> Config {
        let oauth_token = longport_oauth::OAuthToken {
            client_id: self.client_id.clone(),
            access_token: self.access_token.clone(),
            refresh_token: None,
            // The token was already validated by the middleware; set
            // expires_at to u64::MAX so the per-request Config does not
            // attempt to re-validate or refresh it.
            expires_at: u64::MAX,
        };
        Config::from_oauth(&oauth_token)
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
    /// Supported OAuth 2.0 response types.
    pub response_types_supported: Vec<String>,
    /// Supported OAuth 2.0 grant types.
    pub grant_types_supported: Vec<String>,
    /// Supported PKCE code challenge methods (RFC 7636).
    pub code_challenge_methods_supported: Vec<String>,
}

impl AuthorizationServerMetadata {
    /// Construct metadata for a given public `base_url`.
    ///
    /// `base_url` must **not** include a trailing slash, e.g.
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

/// Extract the raw Bearer token from the `Authorization` header.
///
/// Accepts `Authorization: Bearer <token>` with a case-insensitive prefix.
/// Returns `None` when the header is absent, malformed, or the token is empty.
pub(crate) fn extract_bearer_token(req: &Request) -> Option<String> {
    let value = req.headers().get("authorization")?.to_str().ok()?;
    let stripped = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))?;
    let token = stripped.trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

// ---------------------------------------------------------------------------
// Token verification
// ---------------------------------------------------------------------------

/// Verify a Bearer token against the LongPort userinfo endpoint.
///
/// Returns the `client_id` on success, or an [`OAuthMiddlewareError`] on
/// failure.
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

    match resp.status() {
        s if s == reqwest::StatusCode::UNAUTHORIZED || s == reqwest::StatusCode::FORBIDDEN => {
            return Err(OAuthMiddlewareError::InvalidToken);
        }
        s if !s.is_success() => {
            let code = s.as_u16();
            tracing::warn!(status = code, "userinfo endpoint returned unexpected status");
            return Err(OAuthMiddlewareError::Upstream(format!(
                "userinfo endpoint returned HTTP {code}"
            )));
        }
        _ => {}
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
    /// No / malformed `Authorization` header.
    MissingToken,
    /// Token rejected by the authorization server.
    InvalidToken,
    /// Authorization server unreachable or returned an unexpected error.
    Upstream(String),
}

impl OAuthMiddlewareError {
    pub(crate) fn to_response(&self) -> Response {
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
/// that downstream handlers can retrieve it via
/// `req.extensions().get::<AuthenticatedContext>()`.
///
/// Requests without a valid token are rejected immediately with an appropriate
/// RFC 6750 error response; the inner endpoint is never invoked.
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
        // RFC 8414 discovery endpoint must be exempt from authentication.
        if req.uri().path() == "/.well-known/oauth-authorization-server" {
            return self.inner.call(req).await.map(IntoResponse::into_response);
        }

        let token = match extract_bearer_token(&req) {
            Some(t) => t,
            None => return Ok(OAuthMiddlewareError::MissingToken.to_response()),
        };

        let client_id = match verify_token(&token).await {
            Ok(id) => id,
            Err(e) => return Ok(e.to_response()),
        };

        tracing::debug!(client_id = %client_id, "OAuth 2.0 Bearer token verified");

        let ctx = AuthenticatedContext {
            access_token: token,
            client_id,
        };
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

    // --- helpers -------------------------------------------------------------

    fn make_request(auth_header: Option<&str>) -> Request {
        let mut builder = Request::builder().method(Method::GET).uri("http://localhost/");
        if let Some(h) = auth_header {
            builder = builder.header("authorization", h);
        }
        builder.finish()
    }

    // --- extract_bearer_token ------------------------------------------------

    #[test]
    fn test_extract_bearer_token_standard_prefix() {
        let req = make_request(Some("Bearer my-secret-token"));
        assert_eq!(
            extract_bearer_token(&req),
            Some("my-secret-token".to_string())
        );
    }

    #[test]
    fn test_extract_bearer_token_lowercase_prefix() {
        let req = make_request(Some("bearer my-secret-token"));
        assert_eq!(
            extract_bearer_token(&req),
            Some("my-secret-token".to_string())
        );
    }

    #[test]
    fn test_extract_bearer_token_absent_header() {
        let req = make_request(None);
        assert_eq!(extract_bearer_token(&req), None);
    }

    #[test]
    fn test_extract_bearer_token_wrong_scheme() {
        let req = make_request(Some("Basic dXNlcjpwYXNz"));
        assert_eq!(extract_bearer_token(&req), None);
    }

    #[test]
    fn test_extract_bearer_token_empty_after_prefix() {
        let req = make_request(Some("Bearer "));
        assert_eq!(extract_bearer_token(&req), None);
    }

    #[test]
    fn test_extract_bearer_token_trims_whitespace() {
        let req = make_request(Some("Bearer   trimmed-token   "));
        assert_eq!(
            extract_bearer_token(&req),
            Some("trimmed-token".to_string())
        );
    }

    // --- AuthorizationServerMetadata -----------------------------------------

    #[test]
    fn test_metadata_issuer_matches_base_url() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        assert_eq!(meta.issuer, "https://mcp.example.com");
    }

    #[test]
    fn test_metadata_endpoints_use_longport_base() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        assert!(meta.authorization_endpoint.contains("longbridgeapp.com"));
        assert!(meta.token_endpoint.contains("longbridgeapp.com"));
        assert!(meta.revocation_endpoint.contains("longbridgeapp.com"));
    }

    #[test]
    fn test_metadata_response_types_include_code() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        assert!(meta.response_types_supported.contains(&"code".to_string()));
    }

    #[test]
    fn test_metadata_grant_types() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        assert!(meta
            .grant_types_supported
            .contains(&"authorization_code".to_string()));
        assert!(meta
            .grant_types_supported
            .contains(&"refresh_token".to_string()));
    }

    #[test]
    fn test_metadata_pkce_s256() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        assert!(meta
            .code_challenge_methods_supported
            .contains(&"S256".to_string()));
    }

    #[test]
    fn test_metadata_serialization_roundtrip() {
        let meta = AuthorizationServerMetadata::new("https://mcp.example.com");
        let json = serde_json::to_string(&meta).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(
            parsed["issuer"].as_str().unwrap(),
            "https://mcp.example.com"
        );
        assert!(parsed["authorization_endpoint"].as_str().is_some());
        assert!(parsed["token_endpoint"].as_str().is_some());
        assert!(parsed["code_challenge_methods_supported"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str() == Some("S256")));
    }

    // --- OAuthMiddlewareError response codes ---------------------------------

    #[test]
    fn test_missing_token_returns_401() {
        let resp = OAuthMiddlewareError::MissingToken.to_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_invalid_token_returns_401() {
        let resp = OAuthMiddlewareError::InvalidToken.to_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_upstream_error_returns_503() {
        let resp = OAuthMiddlewareError::Upstream("timeout".to_string()).to_response();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_missing_token_www_authenticate_header_contains_realm() {
        let resp = OAuthMiddlewareError::MissingToken.to_response();
        let www_auth = resp
            .headers()
            .get("www-authenticate")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(www_auth.contains("Bearer"));
        assert!(www_auth.contains("longport-mcp"));
        assert!(www_auth.contains("invalid_request"));
    }

    #[test]
    fn test_invalid_token_www_authenticate_header() {
        let resp = OAuthMiddlewareError::InvalidToken.to_response();
        let www_auth = resp
            .headers()
            .get("www-authenticate")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(www_auth.contains("invalid_token"));
    }

    // --- AuthenticatedContext ------------------------------------------------

    #[test]
    fn test_authenticated_context_build_config_uses_bearer_prefix() {
        let ctx = AuthenticatedContext {
            access_token: "raw-token-value".to_string(),
            client_id: "client-123".to_string(),
        };
        let config = ctx.build_config();
        // Config::from_oauth sets access_token as "Bearer <token>"
        assert!(config
            .http_cli_config
            .access_token
            .starts_with("Bearer "));
        assert!(config
            .http_cli_config
            .access_token
            .contains("raw-token-value"));
    }

    #[test]
    fn test_authenticated_context_build_config_sets_client_id_as_app_key() {
        let ctx = AuthenticatedContext {
            access_token: "token".to_string(),
            client_id: "my-client".to_string(),
        };
        let config = ctx.build_config();
        assert_eq!(config.http_cli_config.app_key, "my-client");
    }

    #[test]
    fn test_authenticated_context_build_config_is_oauth2_mode() {
        let ctx = AuthenticatedContext {
            access_token: "token".to_string(),
            client_id: "client".to_string(),
        };
        let config = ctx.build_config();
        assert!(config.http_cli_config.is_oauth2());
    }
}
