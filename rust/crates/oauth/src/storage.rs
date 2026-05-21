use std::sync::Arc;

use crate::{
    error::OAuthResult,
    token::{OAuthToken, token_path_for_client_id},
};

/// Token data passed to and from [`TokenStorage`] implementations.
#[derive(Debug, Clone)]
pub struct StoredToken {
    /// OAuth client ID
    pub client_id: String,
    /// Access token string
    pub access_token: String,
    /// Refresh token, if the server provided one
    pub refresh_token: Option<String>,
    /// Unix timestamp (seconds) at which the access token expires
    pub expires_at: u64,
}

impl From<OAuthToken> for StoredToken {
    fn from(t: OAuthToken) -> Self {
        Self {
            client_id: t.client_id,
            access_token: t.access_token,
            refresh_token: t.refresh_token,
            expires_at: t.expires_at,
        }
    }
}

impl From<StoredToken> for OAuthToken {
    fn from(s: StoredToken) -> Self {
        Self {
            client_id: s.client_id,
            access_token: s.access_token,
            refresh_token: s.refresh_token,
            expires_at: s.expires_at,
        }
    }
}

/// Custom token persistence backend for [`OAuthBuilder`](crate::OAuthBuilder).
///
/// Implement this trait to store tokens somewhere other than the default
/// `~/.longbridge/openapi/tokens/<client_id>` file (e.g. OS keychain,
/// encrypted store, or an in-memory cache for testing).
pub trait TokenStorage: Send + Sync + 'static {
    /// Load the persisted token for `client_id`. Returns `None` if not found.
    fn load(&self, client_id: &str) -> Option<StoredToken>;

    /// Persist `token`. Called after every successful authorization or refresh.
    fn save(&self, token: &StoredToken) -> OAuthResult<()>;
}

/// Default file-based token storage.
///
/// Tokens are written as JSON to `~/.longbridge/openapi/tokens/<client_id>`.
pub struct FileTokenStorage;

impl TokenStorage for FileTokenStorage {
    fn load(&self, client_id: &str) -> Option<StoredToken> {
        let path = token_path_for_client_id(client_id).ok()?;
        OAuthToken::load_from_path(path).ok().map(Into::into)
    }

    fn save(&self, token: &StoredToken) -> OAuthResult<()> {
        let path = token_path_for_client_id(&token.client_id)?;
        OAuthToken::from(token.clone()).save_to_path(path)
    }
}

pub(crate) fn default_storage() -> Arc<dyn TokenStorage> {
    Arc::new(FileTokenStorage)
}
