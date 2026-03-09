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

mod builder;
mod callback;
mod client;
mod error;
mod token;

pub use builder::OAuthBuilder;
pub use client::OAuth;
pub use error::{OAuthError, OAuthResult};

#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{
        OAuthBuilder,
        client::{DEFAULT_CALLBACK_PORT, OAuth, OAuthInner},
        token::{OAuthToken, token_path_for_client_id},
    };

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
        let path_str = path.to_string_lossy().replace('\\', "/");
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
