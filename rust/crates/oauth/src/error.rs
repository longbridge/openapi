//! OAuth error types.

use std::path::PathBuf;

/// Error type for OAuth operations
#[derive(Debug, thiserror::Error)]
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

/// Result type for OAuth operations
pub type OAuthResult<T> = std::result::Result<T, OAuthError>;
