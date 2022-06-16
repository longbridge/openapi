use reqwest::StatusCode;

use crate::qs::QsError;

/// Http client error type
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    /// Invalid api key
    #[error("invalid api key")]
    InvalidApiKey,

    /// Invalid access token
    #[error("invalid access token")]
    InvalidAccessToken,

    /// Missing environment variable
    #[error("missing environment variable: {name}")]
    MissingEnvVar {
        /// Variable name
        name: &'static str,
    },

    /// Unexpected response
    #[error("unexpected response")]
    UnexpectedResponse,

    /// Request timeout
    #[error("request timeout")]
    RequestTimeout,

    /// OpenAPI error
    #[error("openapi error: code={code}: {message}")]
    OpenApi {
        /// Error code
        code: i32,
        /// Error message
        message: String,
    },

    /// Deserialize response body
    #[error("deserialize response body error: {0}")]
    DeserializeResponseBody(serde_json::Error),

    /// Serialize request body
    #[error("serialize request body error: {0}")]
    SerializeRequestBody(serde_json::Error),

    /// Serialize query string error
    #[error("serialize query string error: {0}")]
    SerializeQueryString(#[from] QsError),

    /// Bad status
    #[error("status error: {0}")]
    BadStatus(StatusCode),

    /// Http error
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

/// Http client result type
pub type HttpClientResult<T, E = HttpClientError> = std::result::Result<T, E>;
