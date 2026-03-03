use reqwest::Request;
use sha1::{Digest, Sha1};

use crate::timestamp::Timestamp;

pub(crate) struct SignatureParams<'a> {
    pub(crate) request: &'a Request,
    pub(crate) app_key: &'a str,
    pub(crate) access_token: Option<&'a str>,
    pub(crate) app_secret: &'a str,
    pub(crate) timestamp: Timestamp,
}

/// Compute the `X-Api-Signature` header value for a request, or return `None`
/// when the request uses OAuth 2.0 Bearer token authentication.
///
/// OAuth 2.0 mode is detected by the `"Bearer "` prefix on `access_token`.
/// This matches the token format written by
/// [`crate::HttpClientConfig::from_oauth`] and must be the **only** heuristic
/// used — an empty `app_secret` is not a reliable indicator.
///
/// In legacy mode the signature is computed as:
///
/// ```text
/// HMAC-SHA256 SignedHeaders=<headers>, Signature=<hex>
/// ```
///
/// where the payload is:
///
/// ```text
/// HMAC-SHA256|SHA1("<METHOD>|<path>|<query>|<signed-header-values>|<signed-header-names>|[SHA1(body)]")
/// ```
pub(crate) fn signature(params: SignatureParams<'_>) -> Option<String> {
    // OAuth 2.0 Bearer token mode: no HMAC signature is required because the
    // `Authorization: Bearer <token>` header is already set by the caller.
    let is_oauth2 = params
        .access_token
        .map(|token| token.starts_with("Bearer "))
        .unwrap_or(false);

    if is_oauth2 {
        return None;
    }

    // Legacy HMAC-SHA256 mode.
    let method = params.request.method().as_str();

    let (signed_headers, signed_values) = match params.access_token {
        Some(access_token) => (
            "authorization;x-api-key;x-timestamp",
            format!(
                "authorization:{}\nx-api-key:{}\nx-timestamp:{}\n",
                access_token, params.app_key, params.timestamp
            ),
        ),
        None => (
            "x-api-key;x-timestamp",
            format!(
                "x-api-key:{}\nx-timestamp:{}\n",
                params.app_key, params.timestamp
            ),
        ),
    };

    let url = params.request.url();
    let path = url.path();
    let query = url.query().unwrap_or_default();

    let mut str_to_sign = format!("{method}|{path}|{query}|{signed_values}|{signed_headers}|",);

    if let Some(body) = params.request.body().and_then(|b| b.as_bytes()) {
        str_to_sign.push_str(&sha1(body));
    }

    let str_to_sign = format!("HMAC-SHA256|{}", sha1(str_to_sign.as_bytes()));
    let signature = hmac_sha256(&str_to_sign, params.app_secret);

    Some(format!(
        "HMAC-SHA256 SignedHeaders={signed_headers}, Signature={signature}"
    ))
}

fn sha1(data: &[u8]) -> String {
    format!("{:x}", Sha1::digest(data))
}

fn hmac_sha256(str_to_sign: &str, key: &str) -> String {
    use hmac::Mac;
    let result = hmac::Hmac::<sha2::Sha256>::new_from_slice(key.as_bytes())
        .expect("invalid app secret length")
        .chain_update(str_to_sign)
        .finalize();
    format!("{:x}", result.into_bytes())
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Method};

    use super::*;

    // Build a minimal GET request for tests that do not need a body.
    fn make_get_request(url: &str) -> Request {
        Client::new()
            .request(Method::GET, url)
            .build()
            .expect("valid test request")
    }

    fn make_timestamp() -> Timestamp {
        Timestamp::now()
    }

    // --- OAuth 2.0 mode: signature must be None ------------------------------

    #[test]
    fn test_signature_returns_none_for_bearer_token() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "client-id",
            access_token: Some("Bearer my-oauth-token"),
            app_secret: "",
            timestamp: make_timestamp(),
        });
        assert!(
            result.is_none(),
            "OAuth 2.0 Bearer token mode must not produce a signature"
        );
    }

    #[test]
    fn test_signature_returns_none_for_bearer_token_with_nonempty_secret() {
        // Even when a non-empty app_secret is present, the Bearer prefix
        // takes precedence and suppresses signature generation.
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("Bearer token"),
            app_secret: "should-be-ignored",
            timestamp: make_timestamp(),
        });
        assert!(result.is_none());
    }

    // --- Legacy mode: signature must be Some ---------------------------------

    #[test]
    fn test_signature_returns_some_for_legacy_token() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("plain-access-token"),
            app_secret: "app-secret",
            timestamp: make_timestamp(),
        });
        assert!(
            result.is_some(),
            "Legacy mode with a plain access token must produce a signature"
        );
    }

    #[test]
    fn test_signature_returns_some_when_no_access_token() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: None,
            app_secret: "app-secret",
            timestamp: make_timestamp(),
        });
        assert!(result.is_some());
    }

    // --- Output format checks (legacy mode) ----------------------------------

    #[test]
    fn test_signature_output_starts_with_hmac_sha256() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("access-token"),
            app_secret: "app-secret",
            timestamp: make_timestamp(),
        })
        .unwrap();
        assert!(
            result.starts_with("HMAC-SHA256 SignedHeaders="),
            "unexpected signature format: {result}"
        );
    }

    #[test]
    fn test_signature_output_contains_signature_field() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("access-token"),
            app_secret: "app-secret",
            timestamp: make_timestamp(),
        })
        .unwrap();
        assert!(
            result.contains("Signature="),
            "signature output must contain 'Signature=': {result}"
        );
    }

    #[test]
    fn test_signature_includes_authorization_in_signed_headers_when_token_present() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("access-token"),
            app_secret: "app-secret",
            timestamp: make_timestamp(),
        })
        .unwrap();
        assert!(
            result.contains("authorization"),
            "signed headers must include 'authorization' when access_token is Some: {result}"
        );
    }

    #[test]
    fn test_signature_excludes_authorization_in_signed_headers_when_no_token() {
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: None,
            app_secret: "app-secret",
            timestamp: make_timestamp(),
        })
        .unwrap();
        // The signed-headers list must be "x-api-key;x-timestamp" only.
        assert!(
            !result.contains("authorization"),
            "signed headers must NOT include 'authorization' when access_token is None: {result}"
        );
    }

    // --- Determinism ---------------------------------------------------------

    #[test]
    fn test_signature_is_deterministic_for_same_inputs() {
        use crate::timestamp::Timestamp;

        // Use a fixed timestamp so the output is reproducible.
        let ts = Timestamp::from_secs(1_700_000_000);
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");

        let params = || SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("access-token"),
            app_secret: "app-secret",
            timestamp: ts,
        };

        assert_eq!(signature(params()), signature(params()));
    }

    // --- No-token OAuth boundary ---------------------------------------------

    #[test]
    fn test_signature_with_empty_secret_but_no_bearer_prefix_produces_signature() {
        // Empty app_secret alone must NOT suppress signature generation.
        // This is the boundary case that was previously buggy.
        let req = make_get_request("https://openapi.longportapp.com/v1/quote");
        let result = signature(SignatureParams {
            request: &req,
            app_key: "app-key",
            access_token: Some("plain-token"),
            app_secret: "",
            timestamp: make_timestamp(),
        });
        // With an empty key, hmac::Hmac still produces a value.
        assert!(
            result.is_some(),
            "empty app_secret without Bearer prefix must still attempt signature"
        );
    }
}
