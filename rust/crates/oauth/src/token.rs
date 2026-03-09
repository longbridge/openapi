//! OAuth token type and file persistence.

use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

use crate::error::{OAuthError, OAuthResult};

/// Returns the token file path for the given client ID.
///
/// Path: `~/.longbridge-openapi/tokens/<client_id>`
pub(crate) fn token_path_for_client_id(client_id: &str) -> OAuthResult<PathBuf> {
    let home = dirs::home_dir().ok_or(OAuthError::NoHomeDir)?;
    Ok(home
        .join(".longbridge-openapi")
        .join("tokens")
        .join(client_id))
}

/// Persisted OAuth token (access + optional refresh).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OAuthToken {
    /// OAuth client ID
    pub(crate) client_id: String,
    /// Access token
    pub(crate) access_token: String,
    /// Refresh token if provided by server
    pub(crate) refresh_token: Option<String>,
    /// Unix timestamp when access token expires
    pub(crate) expires_at: u64,
}

impl OAuthToken {
    /// Build from oauth2 crate token response.
    pub(crate) fn from_oauth2_response<TT, T>(client_id: String, token_response: &T) -> Self
    where
        TT: oauth2::TokenType,
        T: TokenResponse<TT>,
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expires_in = token_response.expires_in().map_or(3600, |d| d.as_secs());

        Self {
            client_id,
            access_token: token_response.access_token().secret().clone(),
            refresh_token: token_response.refresh_token().map(|t| t.secret().clone()),
            expires_at: now + expires_in,
        }
    }

    /// True if the access token is past its expiry time.
    pub(crate) fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at
    }

    /// True if the token expires within one hour.
    pub(crate) fn expires_soon(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at.saturating_sub(now) < 3600
    }

    /// Load token from a JSON file at the given path.
    pub(crate) fn load_from_path(path: impl AsRef<Path>) -> OAuthResult<Self> {
        let path = path.as_ref();
        tracing::debug!(path = %path.display(), "loading token from disk");
        let data = std::fs::read_to_string(path).map_err(|e| {
            tracing::debug!(path = %path.display(), error = %e, "failed to read token file");
            OAuthError::TokenFileRead {
                path: path.to_path_buf(),
                source: e,
            }
        })?;
        let token = serde_json::from_str(&data).map_err(|e| {
            tracing::warn!(path = %path.display(), error = %e, "failed to parse token file");
            OAuthError::TokenFileParse {
                path: path.to_path_buf(),
                source: e,
            }
        })?;
        tracing::debug!(path = %path.display(), "token loaded successfully");
        Ok(token)
    }

    /// Save token to a JSON file at the given path.
    pub(crate) fn save_to_path(&self, path: impl AsRef<Path>) -> OAuthResult<()> {
        let path = path.as_ref();
        tracing::debug!(path = %path.display(), "saving token to disk");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                tracing::error!(path = %parent.display(), error = %e, "failed to create token directory");
                OAuthError::CreateDirFailed {
                    path: parent.to_path_buf(),
                    source: e,
                }
            })?;
        }
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| OAuthError::SerializeToken { source: e })?;
        std::fs::write(path, data).map_err(|e| {
            tracing::error!(path = %path.display(), error = %e, "failed to write token file");
            OAuthError::TokenFileWrite {
                path: path.to_path_buf(),
                source: e,
            }
        })?;
        tracing::debug!(path = %path.display(), "token saved successfully");
        Ok(())
    }
}
