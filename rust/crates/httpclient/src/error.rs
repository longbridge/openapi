use std::error::Error;

use longbridge_geo::DcRegion;
use reqwest::{StatusCode, header::HeaderMap};

use crate::qs::QsError;

/// Http client error type
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    /// Invalid request method
    #[error("invalid request method")]
    InvalidRequestMethod,

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
        name: String,
    },

    /// Unexpected response
    #[error("unexpected response")]
    UnexpectedResponse,

    /// Request timeout
    #[error("request timeout")]
    RequestTimeout,

    /// The requested API is restricted to one data center and cannot be reached
    /// from a session in a different region.
    #[error(
        "this API ({path}) is only available in the {required} data center and is not supported for your {current}-region account"
    )]
    DcRegionRestricted {
        /// The restricted API path (or WebSocket command) that was requested.
        path: String,
        /// The data center this API is limited to.
        required: DcRegion,
        /// The session's current data-center region.
        current: DcRegion,
    },

    /// OpenAPI error
    #[error("openapi error: code={code}: {message}")]
    OpenApi {
        /// Error code
        code: i32,
        /// Error message
        message: String,
        /// Trace id
        trace_id: String,
    },

    /// Deserialize response body
    #[error("deserialize response body error: {0}")]
    DeserializeResponseBody(String),

    /// Serialize request body
    #[error("serialize request body error: {0}")]
    SerializeRequestBody(String),

    /// Serialize query string error
    #[error("serialize query string error: {0}")]
    SerializeQueryString(#[from] QsError),

    /// Bad status
    #[error("status error: {0}")]
    BadStatus(StatusCode),

    /// An HTTP response that could not be parsed as an OpenAPI response.
    #[error("unexpected HTTP response: status={status}, trace_id={trace_id}, body={body}")]
    UnexpectedHttpResponse {
        /// HTTP response status.
        status: StatusCode,
        /// Upstream trace ID, when present.
        trace_id: String,
        /// Original HTTP response headers.
        headers: Box<HeaderMap>,
        /// Original HTTP response body.
        body: String,
    },

    /// Http error
    #[error(transparent)]
    Http(#[from] HttpError),

    /// Connection limit exceeded
    #[error("connections limitation is hit, limit = {limit}, online = {online}")]
    ConnectionLimitExceeded {
        /// The limit of connections
        limit: i32,
        /// The number of online connections
        online: i32,
    },

    /// OAuth error
    #[error("oauth error: {0}")]
    OAuth(String),
}

/// Represents an HTTP error
#[derive(Debug)]
pub struct HttpError(pub reqwest::Error);

impl From<reqwest::Error> for HttpError {
    #[inline]
    fn from(err: reqwest::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = self.0.source() {
            write!(f, "{}: {}", self.0, source)
        } else {
            self.0.fmt(f)
        }
    }
}

impl std::error::Error for HttpError {}

/// Http client result type
pub type HttpClientResult<T, E = HttpClientError> = std::result::Result<T, E>;
