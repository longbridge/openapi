use rust_decimal::Decimal;
use serde::Serialize;

/// Options for replace order request
#[derive(Debug, Serialize, Clone)]
pub struct ReplaceOrderOptions {
    order_id: String,
    quantity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_offset: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_percent: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>,
}

impl ReplaceOrderOptions {
    /// Create a new `ReplaceOrderOptions`
    #[inline]
    pub fn new(order_id: impl Into<String>, quantity: Decimal) -> Self {
        Self {
            order_id: order_id.into(),
            quantity,
            price: None,
            trigger_price: None,
            limit_offset: None,
            trailing_amount: None,
            trailing_percent: None,
            remark: None,
        }
    }

    /// Set the price
    #[inline]
    #[must_use]
    pub fn price(self, price: Decimal) -> Self {
        Self {
            price: Some(price),
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
