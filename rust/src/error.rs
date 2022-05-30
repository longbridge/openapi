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
}

/// Longbridge OpenAPI SDK result type
pub type Result<T> = ::std::result::Result<T, Error>;
