use prost::DecodeError;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

/// Connection close reason
#[derive(Debug)]
pub struct WsCloseReason {
    /// Close code
    pub code: CloseCode,

    /// Reason string
    pub message: String,
}

/// Detail message for response error
#[derive(Debug)]
pub struct WsResponseErrorDetail {
    /// Error code
    pub code: u64,
    /// Error message
    pub msg: String,
}

/// Websocket client error type
#[derive(Debug, thiserror::Error)]
pub enum WsClientError {
    /// Unexpected response
    #[error("unexpected response")]
    UnexpectedResponse,

    /// Decode message error
    #[error("decode message error")]
    Decode(#[from] DecodeError),

    /// Connect timeout
    #[error("connect timeout")]
    ConnectTimeout,

    /// Request timeout
    #[error("request timeout")]
    RequestTimeout,

    /// Connection closed
    #[error("connection closed")]
    ConnectionClosed {
        /// The reason the connection was closed
        reason: Option<WsCloseReason>,
    },

    /// Client is closed
    #[error("Client is closed")]
    ClientClosed,

    /// The server responded a status code not equal to `0`
    #[error("response error: {status}: detail:{detail:?}")]
    ResponseError {
        /// Status code
        status: u8,
        /// Error detail
        detail: Option<WsResponseErrorDetail>,
    },

    /// The request has been cancelled
    #[error("cancelled")]
    Cancelled,

    /// Invalid url
    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),

    /// Websocket error
    #[error(transparent)]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
}

/// Websocket client result type
pub type WsClientResult<T, E = WsClientError> = std::result::Result<T, E>;
