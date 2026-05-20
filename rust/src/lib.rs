#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![allow(clippy::result_large_err)]

#[macro_use]
mod macros;

mod config;
mod error;
pub mod runtime;
pub use runtime::runtime_handle;
mod serde_utils;
mod types;
mod utils;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;

pub use longbridge_oauth as oauth;
pub mod alert;
pub mod asset;
pub mod calendar;
pub mod content;
pub mod dca;
pub mod fundamental;
pub mod market;
pub mod portfolio;
pub mod quote;
pub mod screener;
pub mod sharelist;
pub mod trade;

pub use alert::AlertContext;
pub use asset::AssetContext;
pub use calendar::CalendarContext;
pub use config::{Config, Language, PushCandlestickMode};
pub use content::ContentContext;
pub use dca::DCAContext;
pub use error::{Error, Result, SimpleError, SimpleErrorKind};
pub use fundamental::FundamentalContext;
pub use longbridge_httpcli as httpclient;
pub use longbridge_wscli as wsclient;
pub use market::MarketContext;
pub use portfolio::PortfolioContext;
pub use quote::QuoteContext;
pub use rust_decimal::Decimal;
pub use screener::ScreenerContext;
pub use sharelist::SharelistContext;
pub use trade::TradeContext;
pub use types::Market;
