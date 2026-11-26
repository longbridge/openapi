//! Reading the account channel from the access token.
//!
//! The account (ATM / reward-center) routes take the account channel of the
//! signed-in account (`account_channel`, e.g. `lb`) as a query parameter, as
//! the Longbridge app and MCP server send it. The SDK reads it from the access
//! token, which is a JWT for both API-key and OAuth sessions. The payload is
//! only decoded, never verified: the gateway remains the authority on the
//! token.

use serde_json::Value;

/// The account channel used when the token decodes but names none, as the
/// Longbridge MCP server does.
const DEFAULT_ACCOUNT_CHANNEL: &str = "lb";

/// The account channel of an access token, or `None` when the token is not a
/// decodable JWT (the parameter is then not sent).
///
/// Read from the top-level `ac` claim, else from `account_channel` inside the
/// `sub` claim (a JSON string); a decodable token naming neither yields
/// [`DEFAULT_ACCOUNT_CHANNEL`].
pub(crate) fn account_channel(token: &str) -> Option<String> {
    let token = token.strip_prefix("Bearer ").unwrap_or(token);
    let payload = token.split('.').nth(1)?;
    let claims: Value = serde_json::from_slice(&base64url_decode(payload)?).ok()?;
    if !claims.is_object() {
        return None;
    }
    let non_empty = |v: &Value| v.as_str().filter(|s| !s.is_empty()).map(str::to_owned);
    let from_sub = || {
        let sub: Value = serde_json::from_str(claims.get("sub")?.as_str()?).ok()?;
        non_empty(sub.get("account_channel")?)
    };
    Some(
        claims
            .get("ac")
            .and_then(non_empty)
            .or_else(from_sub)
            .unwrap_or_else(|| DEFAULT_ACCOUNT_CHANNEL.to_string()),
    )
}

/// Decodes unpadded (or padded) base64url; `None` on an invalid character.
fn base64url_decode(input: &str) -> Option<Vec<u8>> {
    fn sextet(c: u8) -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'-' | b'+' => 62,
            b'_' | b'/' => 63,
            _ => return None,
        } as u32)
    }
    let bytes: Vec<u8> = input.bytes().filter(|b| *b != b'=').collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        if chunk.len() == 1 {
            return None;
        }
        let mut acc = 0u32;
        for (i, c) in chunk.iter().enumerate() {
            acc |= sextet(*c)? << (18 - 6 * i);
        }
        let [_, b0, b1, b2] = acc.to_be_bytes();
        out.push(b0);
        if chunk.len() > 2 {
            out.push(b1);
        }
        if chunk.len() > 3 {
            out.push(b2);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(bytes: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let mut acc = 0u32;
            for (i, b) in chunk.iter().enumerate() {
                acc |= u32::from(*b) << (16 - 8 * i);
            }
            for i in 0..=chunk.len() {
                out.push(ALPHABET[((acc >> (18 - 6 * i)) & 63) as usize] as char);
            }
        }
        out
    }

    fn token(claims: &str) -> String {
        format!(
            "hk_m_{}.{}.sig",
            encode(b"{\"alg\":\"none\"}"),
            encode(claims.as_bytes())
        )
    }

    #[test]
    fn reads_the_channel_claims() {
        assert_eq!(
            account_channel(&token(r#"{"ac":"lb_papertrading"}"#)).as_deref(),
            Some("lb_papertrading"),
            "top-level ac claim"
        );
        assert_eq!(
            account_channel(&token(r#"{"sub":"{\"account_channel\":\"us\"}"}"#)).as_deref(),
            Some("us"),
            "account_channel inside sub"
        );
        assert_eq!(
            account_channel(&format!("Bearer {}", token(r#"{"ac":"","sub":"x"}"#))).as_deref(),
            Some("lb"),
            "a JWT naming no channel gets the default"
        );
        assert_eq!(account_channel("opaque-token"), None, "not a JWT");
        assert_eq!(account_channel("a.!!!.c"), None, "invalid base64");
        assert_eq!(
            account_channel(&token("[1]")),
            None,
            "payload not an object"
        );
    }

    #[test]
    fn base64url_round_trips() {
        for text in ["", "a", "ab", "abc", "abcd", "{\"ac\":\"lb\"}"] {
            assert_eq!(
                base64url_decode(&encode(text.as_bytes())).as_deref(),
                Some(text.as_bytes()),
                "{text}"
            );
        }
    }
}
