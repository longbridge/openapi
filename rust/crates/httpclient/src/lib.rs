//! LongPort OpenAPI HTTP client

#![forbid(unsafe_code)]
#![deny(unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

mod client;
mod config;
mod error;
mod qs;
mod request;
mod signature;
mod timestamp;

pub use client::HttpClient;
pub use config::HttpClientConfig;
pub use error::{HttpClientError, HttpClientResult};
pub use qs::QsError;
pub use request::{FromPayload, Json, RequestBuilder, ToPayload};
pub use reqwest::Method;
