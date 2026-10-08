//! Forex (currency exchange) channel types.

use num_enum::{FromPrimitive, IntoPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Forex order state.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ForexOrderStatus {
    /// Processing (includes manual review); keep polling.
    #[num_enum(default)]
    Processing = 0,
    /// Conversion succeeded.
    Success = 1,
    /// Conversion failed; frozen funds have been returned.
    Failed = 2,
}

impl Serialize for ForexOrderStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value: i32 = (*self).into();
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ForexOrderStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        Ok(ForexOrderStatus::from(value))
    }
}

/// A forex quote returned by [`ForexContext::quote`].
///
/// [`ForexContext::quote`]: crate::forex::ForexContext::quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexQuote {
    /// Quote id, used when submitting the order.
    pub quote_id: String,
    /// Customer execution rate in standard currency-pair terms
    /// (`1 BASE = rate QUOTE`), independent of the request from/to direction.
    pub rate: Decimal,
    /// Quote expiry, Unix milliseconds.
    pub expire_at: i64,
    /// The standard currency pair for `rate`, format `BASE/QUOTE`.
    pub ccy_pair: String,
}

/// A forex order's state, returned by [`ForexContext::order`].
///
/// [`ForexContext::order`]: crate::forex::ForexContext::order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexOrderDetail {
    /// Order state.
    pub state: ForexOrderStatus,
    /// Execution rate (standard currency-pair terms); `None` until filled.
    #[serde(with = "crate::serde_utils::decimal_opt_empty_is_none")]
    pub rate: Option<Decimal>,
    /// Converted-from amount (actual value once filled); `None` until filled.
    #[serde(with = "crate::serde_utils::decimal_opt_empty_is_none")]
    pub from_amount: Option<Decimal>,
    /// Converted-to amount (actual value once filled); `None` until filled.
    #[serde(with = "crate::serde_utils::decimal_opt_empty_is_none")]
    pub to_amount: Option<Decimal>,
    /// Failure reason; empty when not failed.
    #[serde(default)]
    pub fail_reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_quote() {
        let q: ForexQuote = serde_json::from_str(
            r#"{"ok":true,"quote_id":"Q1","rate":"7.7812","expire_at":1790000000000,"ccy_pair":"USD/HKD"}"#,
        )
        .unwrap();
        assert_eq!(q.quote_id, "Q1");
        assert_eq!(q.rate, Decimal::new(77812, 4));
        assert_eq!(q.expire_at, 1790000000000);
        assert_eq!(q.ccy_pair, "USD/HKD");
    }

    #[test]
    fn deserialize_order_processing_empty_rate() {
        let o: ForexOrderDetail = serde_json::from_str(
            r#"{"state":0,"rate":"","from_amount":"","to_amount":"","fail_reason":""}"#,
        )
        .unwrap();
        assert_eq!(o.state, ForexOrderStatus::Processing);
        assert_eq!(o.rate, None);
        assert_eq!(o.from_amount, None);
    }

    #[test]
    fn deserialize_order_success() {
        let o: ForexOrderDetail = serde_json::from_str(
            r#"{"state":1,"rate":"7.7812","from_amount":"1000","to_amount":"7781.2","fail_reason":""}"#,
        )
        .unwrap();
        assert_eq!(o.state, ForexOrderStatus::Success);
        assert_eq!(o.to_amount, Some(Decimal::new(77812, 1)));
    }

    #[test]
    fn unknown_state_defaults_to_processing() {
        assert_eq!(ForexOrderStatus::from(99), ForexOrderStatus::Processing);
    }
}
