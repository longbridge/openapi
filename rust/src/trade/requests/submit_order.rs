use rust_decimal::Decimal;
use serde::Serialize;
use time::Date;

use crate::{
    serde_utils,
    trade::{OrderSide, OrderType, OutsideRTH, TimeInForceType},
};

/// Options for submit order request
#[derive(Debug, Serialize, Clone)]
pub struct SubmitOrderOptions {
    symbol: String,
    order_type: OrderType,
    side: OrderSide,
    #[serde(with = "serde_utils::int64_str")]
    submitted_quantity: i64,
    time_in_force: TimeInForceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    submitted_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_offset: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_percent: Option<Decimal>,
    #[serde(with = "serde_utils::date_opt")]
    expire_date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outside_rth: Option<OutsideRTH>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>,
}

impl SubmitOrderOptions {
    /// Create a new `SubmitOrderOptions`
    #[inline]
    pub fn new(
        symbol: impl Into<String>,
        order_type: OrderType,
        side: OrderSide,
        submitted_quantity: i64,
        time_in_force: TimeInForceType,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_type,
            side,
            submitted_quantity,
            time_in_force,
            submitted_price: None,
            trigger_price: None,
            limit_offset: None,
            trailing_amount: None,
            trailing_percent: None,
            expire_date: None,
            outside_rth: None,
            remark: None,
        }
    }

    /// Set the submitted price
    #[inline]
    #[must_use]
    pub fn submitted_price(self, submitted_price: Decimal) -> Self {
        Self {
            submitted_price: Some(submitted_price),
            ..self
        }
    }

    /// Set the trigger price
    #[inline]
    #[must_use]
    pub fn trigger_price(self, trigger_price: Decimal) -> Self {
        Self {
            trigger_price: Some(trigger_price),
            ..self
        }
    }

    /// Set the limit offset
    #[inline]
    #[must_use]
    pub fn limit_offset(self, limit_offset: Decimal) -> Self {
        Self {
            limit_offset: Some(limit_offset),
            ..self
        }
    }

    /// Set the trailing amount
    #[inline]
    #[must_use]
    pub fn trailing_amount(self, trailing_amount: Decimal) -> Self {
        Self {
            trailing_amount: Some(trailing_amount),
            ..self
        }
    }

    /// Set the trailing percent
    #[inline]
    #[must_use]
    pub fn trailing_percent(self, trailing_percent: Decimal) -> Self {
        Self {
            trailing_percent: Some(trailing_percent),
            ..self
        }
    }

    /// Set the expire date
    #[inline]
    #[must_use]
    pub fn expire_date(self, expire_date: Date) -> Self {
        Self {
            expire_date: Some(expire_date),
            ..self
        }
    }

    /// Enable or disable outside regular trading hours
    #[inline]
    #[must_use]
    pub fn outside_rth(self, outside_rth: OutsideRTH) -> Self {
        Self {
            outside_rth: Some(outside_rth),
            ..self
        }
    }

    /// Set the remark
    #[inline]
    #[must_use]
    pub fn remark(self, remark: impl Into<String>) -> Self {
        Self {
            remark: Some(remark.into()),
            ..self
        }
    }
}
