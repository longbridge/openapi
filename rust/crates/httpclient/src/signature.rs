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

pub(crate) fn signature(params: SignatureParams<'_>) -> Option<String> {
    // OAuth 2.0 mode: skip HMAC signature generation
    // Detected by Bearer token prefix or empty app_secret
    let is_oauth2 = params
        .access_token
        .map(|token| token.starts_with("Bearer "))
        .unwrap_or(false)
        || params.app_secret.is_empty();

    if is_oauth2 {
        // OAuth 2.0 uses Bearer token authentication, no signature needed
        return None;
    }

    // Legacy mode: generate HMAC-SHA256 signature
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
