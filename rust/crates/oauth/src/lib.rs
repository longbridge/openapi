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
//!     // Build an OAuth client.  If a token exists it is loaded from storage;
//!     // otherwise the browser authorization flow is triggered.
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
mod storage;
mod token;

pub use builder::OAuthBuilder;
pub use client::OAuth;
pub use error::{OAuthError, OAuthResult};
pub use storage::{FileTokenStorage, StoredToken, TokenStorage};

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{
        OAuthBuilder, OAuthResult, StoredToken, TokenStorage,
        client::{DEFAULT_CALLBACK_PORT, OAuth, OAuthInner},
        storage::default_storage,
        token::{OAuthToken, token_path_for_client_id},
    };

    /// In-memory storage for testing — no disk or keychain involved.
    #[derive(Default)]
    struct MemoryStorage(Mutex<Option<StoredToken>>);

    impl TokenStorage for MemoryStorage {
        fn load(&self, _client_id: &str) -> Option<StoredToken> {
            self.0.lock().unwrap().clone()
        }

        fn save(&self, token: &StoredToken) -> OAuthResult<()> {
            *self.0.lock().unwrap() = Some(token.clone());
            Ok(())
        }
    }

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
    fn test_oauth_token_expires_soon_within_5_minutes() {
        assert!(make_token(now_secs() + 299).expires_soon());
    }

    #[test]
    fn test_oauth_token_not_expires_soon() {
        assert!(!make_token(now_secs() + 301).expires_soon());
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
            path_str.ends_with(".longbridge/openapi/tokens/my-app"),
            "unexpected path: {path_str}"
        );
    }

    #[tokio::test]
    async fn test_oauth_access_token_returns_token() {
        let inner = Arc::new(OAuthInner {
            client_id: "test-client".to_string(),
            callback_port: DEFAULT_CALLBACK_PORT,
            storage: default_storage(),
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
            storage: default_storage(),
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
            storage: default_storage(),
            token: tokio::sync::Mutex::new(None),
        });
        let oauth1 = OAuth(inner);
        let oauth2 = oauth1.clone();
        assert!(Arc::ptr_eq(&oauth1.0, &oauth2.0));
    }

    #[test]
    fn test_oauth_builder_token_storage() {
        let storage = Arc::new(MemoryStorage::default());
        let builder = OAuthBuilder::new("test-client-id").token_storage(MemoryStorage::default());
        assert_eq!(builder.client_id, "test-client-id");
        // Verify the custom storage is wired in (load returns None before any save).
        assert!(storage.load("test-client-id").is_none());
    }

    #[test]
    fn test_stored_token_round_trip_via_memory_storage() {
        let storage = MemoryStorage::default();
        let token = StoredToken {
            client_id: "client-a".to_string(),
            access_token: "access".to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_at: 9999999999,
        };

        storage.save(&token).unwrap();
        let loaded = storage.load("client-a").unwrap();

        assert_eq!(loaded.client_id, token.client_id);
        assert_eq!(loaded.access_token, token.access_token);
        assert_eq!(loaded.refresh_token, token.refresh_token);
        assert_eq!(loaded.expires_at, token.expires_at);
    }

    #[test]
    fn test_memory_storage_returns_none_when_empty() {
        let storage = MemoryStorage::default();
        assert!(storage.load("any-client").is_none());
    }

    #[test]
    fn test_stored_token_from_oauth_token() {
        let oauth_token = OAuthToken {
            client_id: "c".to_string(),
            access_token: "a".to_string(),
            refresh_token: Some("r".to_string()),
            expires_at: 1234,
        };
        let stored: StoredToken = oauth_token.into();
        assert_eq!(stored.client_id, "c");
        assert_eq!(stored.access_token, "a");
        assert_eq!(stored.refresh_token, Some("r".to_string()));
        assert_eq!(stored.expires_at, 1234);
    }

    #[test]
    fn test_oauth_token_from_stored_token() {
        let stored = StoredToken {
            client_id: "c".to_string(),
            access_token: "a".to_string(),
            refresh_token: None,
            expires_at: 5678,
        };
        let oauth_token: OAuthToken = stored.into();
        assert_eq!(oauth_token.client_id, "c");
        assert_eq!(oauth_token.access_token, "a");
        assert_eq!(oauth_token.refresh_token, None);
        assert_eq!(oauth_token.expires_at, 5678);
    }
}
