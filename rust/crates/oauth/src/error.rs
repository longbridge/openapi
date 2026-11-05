//! OAuth error types.

use std::path::PathBuf;

/// Error type for OAuth operations
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OAuthError {
    /// Cannot determine home directory for token storage
    #[error("cannot determine home directory")]
    NoHomeDir,

    /// Failed to read token file from disk
    #[error("failed to read token file {path}: {source}")]
    TokenFileRead {
        /// Path to the token file
        path: PathBuf,
        /// I/O error from reading the file
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse token file (invalid JSON or schema)
    #[error("failed to parse token file {path}: {source}")]
    TokenFileParse {
        /// Path to the token file
        path: PathBuf,
        /// JSON parse error
        #[source]
        source: serde_json::Error,
    },

    /// Failed to create token directory
    #[error("failed to create directory {path}: {source}")]
    CreateDirFailed {
        /// Directory path
        path: PathBuf,
        /// I/O error from creating the directory
        #[source]
        source: std::io::Error,
    },

    /// Failed to serialize token to JSON
    #[error("failed to serialize token: {source}")]
    SerializeToken {
        /// JSON serialize error
        #[source]
        source: serde_json::Error,
    },

    /// Failed to write token file
    #[error("failed to write token file {path}: {source}")]
    TokenFileWrite {
        /// Path to the token file
        path: PathBuf,
        /// I/O error from writing the file
        #[source]
        source: std::io::Error,
    },

    /// Failed to delete token file
    #[error("failed to delete token file {path}: {source}")]
    TokenFileDelete {
        /// Path to the token file
        path: PathBuf,
        /// I/O error from deleting the file
        #[source]
        source: std::io::Error,
    },

    /// No token available (not loaded and authorization not yet completed)
    #[error("no token available")]
    NoTokenAvailable,

    /// Failed to bind callback server for OAuth redirect
    #[error("failed to bind callback server on port {port}: {message}")]
    BindCallbackFailed {
        /// Callback port
        port: u16,
        /// Underlying error message
        message: String,
    },

    /// Failed to get local address of callback server
    #[error("failed to get local address")]
    LocalAddressFailed,

    /// CSRF token mismatch (possible CSRF attack)
    ///
    /// No longer returned by the browser authorization flow: a callback whose
    /// `state` does not match is ignored and the flow keeps waiting. Kept for
    /// compatibility (code 8).
    #[error("CSRF token mismatch")]
    CsrfTokenMismatch,

    /// Failed to exchange authorization code for token
    #[error("failed to exchange code for token: {message}")]
    ExchangeCodeFailed {
        /// Server or protocol error message
        message: String,
    },

    /// No refresh token available
    #[error("no refresh token available")]
    NoRefreshToken,

    /// Failed to refresh access token
    #[error("failed to refresh token: {message}")]
    RefreshTokenFailed {
        /// Server or protocol error message
        message: String,
    },

    /// Failed to revoke token on the server
    #[error("failed to revoke token: {message}")]
    RevokeFailed {
        /// Server or protocol error message
        message: String,
    },

    /// OAuth authorization failed (user denied or server error in callback)
    #[error("OAuth authorization failed: {message}")]
    AuthorizationFailed {
        /// Error message from callback or server
        message: String,
    },

    /// Callback channel closed unexpectedly
    #[error("callback channel closed unexpectedly")]
    CallbackChannelClosed,

    /// Authorization timed out waiting for user callback
    #[error("authorization timeout - no response received within 5 minutes")]
    AuthorizationTimeout,

    /// Other internal or low-level error (e.g. runtime setup)
    #[error("{0}")]
    Other(String),
}

impl OAuthError {
    /// Stable numeric code identifying the kind of error, so that callers of
    /// the language bindings (where the error crosses the FFI boundary as a
    /// message plus a code) can tell failures apart without parsing messages.
    ///
    /// | Code | Variant |
    /// |------|---------|
    /// | 1 | [`NoTokenAvailable`](Self::NoTokenAvailable) |
    /// | 2 | [`NoRefreshToken`](Self::NoRefreshToken) |
    /// | 3 | [`RefreshTokenFailed`](Self::RefreshTokenFailed) |
    /// | 4 | [`RevokeFailed`](Self::RevokeFailed) |
    /// | 5 | [`AuthorizationFailed`](Self::AuthorizationFailed) |
    /// | 6 | [`AuthorizationTimeout`](Self::AuthorizationTimeout) |
    /// | 7 | [`ExchangeCodeFailed`](Self::ExchangeCodeFailed) |
    /// | 8 | [`CsrfTokenMismatch`](Self::CsrfTokenMismatch) |
    /// | 9 | [`BindCallbackFailed`](Self::BindCallbackFailed) |
    /// | 10 | [`LocalAddressFailed`](Self::LocalAddressFailed) |
    /// | 11 | [`CallbackChannelClosed`](Self::CallbackChannelClosed) |
    /// | 12 | [`NoHomeDir`](Self::NoHomeDir) |
    /// | 13 | [`TokenFileRead`](Self::TokenFileRead) |
    /// | 14 | [`TokenFileParse`](Self::TokenFileParse) |
    /// | 15 | [`TokenFileWrite`](Self::TokenFileWrite) |
    /// | 16 | [`TokenFileDelete`](Self::TokenFileDelete) |
    /// | 17 | [`CreateDirFailed`](Self::CreateDirFailed) |
    /// | 18 | [`SerializeToken`](Self::SerializeToken) |
    /// | 0 | [`Other`](Self::Other) |
    ///
    /// Codes are never reused; a new variant gets a new code.
    pub fn code(&self) -> i64 {
        match self {
            OAuthError::Other(_) => 0,
            OAuthError::NoTokenAvailable => 1,
            OAuthError::NoRefreshToken => 2,
            OAuthError::RefreshTokenFailed { .. } => 3,
            OAuthError::RevokeFailed { .. } => 4,
            OAuthError::AuthorizationFailed { .. } => 5,
            OAuthError::AuthorizationTimeout => 6,
            OAuthError::ExchangeCodeFailed { .. } => 7,
            OAuthError::CsrfTokenMismatch => 8,
            OAuthError::BindCallbackFailed { .. } => 9,
            OAuthError::LocalAddressFailed => 10,
            OAuthError::CallbackChannelClosed => 11,
            OAuthError::NoHomeDir => 12,
            OAuthError::TokenFileRead { .. } => 13,
            OAuthError::TokenFileParse { .. } => 14,
            OAuthError::TokenFileWrite { .. } => 15,
            OAuthError::TokenFileDelete { .. } => 16,
            OAuthError::CreateDirFailed { .. } => 17,
            OAuthError::SerializeToken { .. } => 18,
        }
    }
}

/// Result type for OAuth operations
pub type OAuthResult<T> = std::result::Result<T, OAuthError>;
