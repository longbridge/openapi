use std::fmt::Display;

use longbridge_httpcli::HttpClientError;
use longbridge_wscli::WsClientError;

/// Longbridge OpenAPI SDK error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Decode Protobuf error
    #[error(transparent)]
    DecodeProtobuf(#[from] prost::DecodeError),

    /// Decode JSON error
    #[error(transparent)]
    DecodeJSON(#[from] serde_json::Error),

    /// Parse field
    #[error("parse field: {name}: {error}")]
    ParseField {
        /// Field name
        name: &'static str,

        /// Error detail
        error: String,
    },

    /// Unknown command
    #[error("unknown command: {0}")]
    UnknownCommand(
        /// Command code
        u8,
    ),

    /// HTTP client error
    #[error(transparent)]
    HttpClient(#[from] HttpClientError),

    /// Websocket client error
    #[error(transparent)]
    WsClient(#[from] WsClientError),

    /// Blocking error
    #[cfg(feature = "blocking")]
    #[error(transparent)]
    Blocking(#[from] crate::blocking::BlockingError),
}

impl Error {
    #[inline]
    pub(crate) fn parse_field_error(name: &'static str, error: impl Display) -> Self {
        Self::ParseField {
            name,
            error: error.to_string(),
        }
    }

    /// Consumes this error and returns a simple error
    pub fn into_simple_error(self) -> SimpleError {
        match self {
            Error::HttpClient(HttpClientError::OpenApi { code, message }) => {
                SimpleError::Response {
                    code: code as i64,
                    message,
                }
            }
            Error::WsClient(WsClientError::ResponseError {
                detail: Some(detail),
                ..
            }) => SimpleError::Response {
                code: detail.code as i64,
                message: detail.msg,
            },
            Error::DecodeProtobuf(_)
            | Error::DecodeJSON(_)
            | Error::ParseField { .. }
            | Error::UnknownCommand(_)
            | Error::HttpClient(_)
            | Error::WsClient(_) => SimpleError::Other(self.to_string()),
            #[cfg(feature = "blocking")]
            Error::Blocking(_) => SimpleError::Other(self.to_string()),
        }
    }
}

/// Longbridge OpenAPI SDK result type
pub type Result<T> = ::std::result::Result<T, Error>;

/// Simple error type
#[derive(Debug, thiserror::Error)]
pub enum SimpleError {
    /// Response error
    #[error("response error: code={code} message={message}")]
    Response {
        /// Error code
        code: i64,
        /// Error message
        message: String,
    },
    /// Other error
    #[error("other error: {0}")]
    Other(String),
}

impl From<Error> for SimpleError {
    #[inline]
    fn from(err: Error) -> Self {
        err.into_simple_error()
    }
}

impl SimpleError {
    /// Returns the error code
    pub fn code(&self) -> Option<i64> {
        match self {
            SimpleError::Response { code, .. } => Some(*code),
            SimpleError::Other(_) => None,
        }
    }

    /// Returns the error message
    pub fn message(&self) -> &str {
        match self {
            SimpleError::Response { message, .. } => message.as_str(),
            SimpleError::Other(message) => message.as_str(),
        }
    }
}
