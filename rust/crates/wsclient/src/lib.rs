//! Longbridge OpenAPI Websocket client

#![forbid(unsafe_code)]
#![deny(private_in_public, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

mod client;
mod codec;
mod error;
mod event;

pub use client::{CodecType, Platform, ProtocolVersion, RateLimit, WsClient, WsSession};
pub use error::{WsClientError, WsClientResult, WsCloseReason, WsResponseErrorDetail};
pub use event::WsEvent;
