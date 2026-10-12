use longbridge_nodejs_macros::{JsEnum, JsObject};

use crate::decimal::Decimal;

/// Forex order state.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::forex::ForexOrderStatus")]
pub enum ForexOrderStatus {
    /// Processing (includes manual review); keep polling.
    Processing,
    /// Conversion succeeded.
    Success,
    /// Conversion failed; frozen funds have been returned.
    Failed,
}

/// A forex quote.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::forex::ForexQuote")]
pub struct ForexQuote {
    /// Quote id, used when submitting the order
    quote_id: String,
    /// Customer execution rate (standard currency-pair terms)
    rate: Decimal,
    /// Quote expiry, Unix milliseconds
    expire_at: i64,
    /// The standard currency pair for `rate`, format `BASE/QUOTE`
    ccy_pair: String,
}

/// A forex order's state.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::forex::ForexOrderDetail")]
pub struct ForexOrderDetail {
    /// Order state
    state: ForexOrderStatus,
    /// Execution rate (standard currency-pair terms); empty until filled
    #[js(opt)]
    rate: Option<Decimal>,
    /// Converted-from amount (actual value once filled); empty until filled
    #[js(opt)]
    from_amount: Option<Decimal>,
    /// Converted-to amount (actual value once filled); empty until filled
    #[js(opt)]
    to_amount: Option<Decimal>,
    /// Failure reason; empty when not failed
    fail_reason: String,
}
