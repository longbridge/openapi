use rust_decimal::Decimal;
use serde::Serialize;

use crate::trade::{OrderSide, OrderType};

/// Options for estimate maximum purchase quantity
#[derive(Debug, Serialize, Clone)]
pub struct EstimateMaxPurchaseQuantityOptions {
    symbol: String,
    order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    side: OrderSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
}

impl EstimateMaxPurchaseQuantityOptions {
    /// Create a new `EstimateMaxPurchaseQuantityOptions`
    #[inline]
    pub fn new(symbol: impl Into<String>, order_type: OrderType, side: OrderSide) -> Self {
        Self {
            symbol: symbol.into(),
            order_type,
            price: None,
            side,
            currency: None,
            order_id: None,
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

    /// Set the currency
    #[inline]
    #[must_use]
    pub fn currency(self, currency: impl Into<String>) -> Self {
        Self {
            currency: Some(currency.into()),
            ..self
        }
    }

    /// Set the order id
    #[inline]
    #[must_use]
    pub fn order_id(self, order_id: impl Into<String>) -> Self {
        Self {
            order_id: Some(order_id.into()),
            ..self
        }
    }
}
