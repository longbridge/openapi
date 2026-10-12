use std::os::raw::c_char;

use longbridge::forex::{ForexOrderDetail, ForexQuote};
use longbridge_c_macros::CEnum;

use crate::types::{CDecimal, CString, ToFFI};

/// Forex order state.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::forex::ForexOrderStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CForexOrderStatus {
    /// Processing (includes manual review); keep polling.
    #[c(remote = "Processing")]
    ForexOrderStatusProcessing,
    /// Conversion succeeded.
    #[c(remote = "Success")]
    ForexOrderStatusSuccess,
    /// Conversion failed; frozen funds have been returned.
    #[c(remote = "Failed")]
    ForexOrderStatusFailed,
}

/// A forex quote.
#[repr(C)]
pub struct CForexQuote {
    /// Quote id, used when submitting the order
    pub quote_id: *const c_char,
    /// Customer execution rate (standard currency-pair terms)
    pub rate: *const CDecimal,
    /// Quote expiry, Unix milliseconds
    pub expire_at: i64,
    /// The standard currency pair for `rate`, format `BASE/QUOTE`
    pub ccy_pair: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CForexQuoteOwned {
    quote_id: CString,
    rate: CDecimal,
    expire_at: i64,
    ccy_pair: CString,
}

impl From<ForexQuote> for CForexQuoteOwned {
    fn from(q: ForexQuote) -> Self {
        CForexQuoteOwned {
            quote_id: q.quote_id.into(),
            rate: q.rate.into(),
            expire_at: q.expire_at,
            ccy_pair: q.ccy_pair.into(),
        }
    }
}

impl ToFFI for CForexQuoteOwned {
    type FFIType = CForexQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CForexQuoteOwned {
            quote_id,
            rate,
            expire_at,
            ccy_pair,
        } = self;
        CForexQuote {
            quote_id: quote_id.to_ffi_type(),
            rate: rate.to_ffi_type(),
            expire_at: *expire_at,
            ccy_pair: ccy_pair.to_ffi_type(),
        }
    }
}

/// A forex order's state.
#[repr(C)]
pub struct CForexOrderDetail {
    /// Order state
    pub state: CForexOrderStatus,
    /// Execution rate (standard currency-pair terms); null until filled
    pub rate: *const CDecimal,
    /// Converted-from amount (actual value once filled); null until filled
    pub from_amount: *const CDecimal,
    /// Converted-to amount (actual value once filled); null until filled
    pub to_amount: *const CDecimal,
    /// Failure reason; empty when not failed
    pub fail_reason: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CForexOrderDetailOwned {
    state: CForexOrderStatus,
    rate: Option<CDecimal>,
    from_amount: Option<CDecimal>,
    to_amount: Option<CDecimal>,
    fail_reason: CString,
}

impl From<ForexOrderDetail> for CForexOrderDetailOwned {
    fn from(d: ForexOrderDetail) -> Self {
        CForexOrderDetailOwned {
            state: d.state.into(),
            rate: d.rate.map(Into::into),
            from_amount: d.from_amount.map(Into::into),
            to_amount: d.to_amount.map(Into::into),
            fail_reason: d.fail_reason.into(),
        }
    }
}

impl ToFFI for CForexOrderDetailOwned {
    type FFIType = CForexOrderDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CForexOrderDetailOwned {
            state,
            rate,
            from_amount,
            to_amount,
            fail_reason,
        } = self;
        CForexOrderDetail {
            state: *state,
            rate: rate
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            from_amount: from_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            to_amount: to_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            fail_reason: fail_reason.to_ffi_type(),
        }
    }
}
