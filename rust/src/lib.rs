#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(private_in_public, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[macro_use]
mod macros;

mod config;
mod error;
mod types;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;
pub mod quote;
pub mod trade;

pub use config::Config;
pub use error::{Error, Result};
pub use quote::QuoteContext;
pub use rust_decimal::Decimal;
pub use trade::TradeContext;
pub use types::Market;
