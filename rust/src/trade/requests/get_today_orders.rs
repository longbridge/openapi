use serde::Serialize;

use crate::{
    trade::{OrderSide, OrderStatus},
    Market,
};

/// Options for get today orders request
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetTodayOrdersOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    status: Vec<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<OrderSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<Market>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
}

impl GetTodayOrdersOptions {
    /// Create a new `GetTodayOrdersOptions`
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the security symbol
    #[inline]
    #[must_use]
    pub fn symbol(self, symbol: impl Into<String>) -> Self {
        Self {
            symbol: Some(symbol.into()),
            ..self
        }
    }

    /// Set the order status
    #[inline]
    #[must_use]
    pub fn status(self, status: impl IntoIterator<Item = OrderStatus>) -> Self {
        Self {
            status: status.into_iter().collect(),
            ..self
        }
    }

    /// Set the order side
    #[inline]
    #[must_use]
    pub fn side(self, side: OrderSide) -> Self {
        Self {
            side: Some(side),
            ..self
        }
    }

    /// Set the market
    #[inline]
    #[must_use]
    pub fn market(self, market: Market) -> Self {
        Self {
            market: Some(market),
            ..self
        }
    }

    /// Set the order id
    #[inline]
    #[must_use]
    pub fn order_id(self, order_id: String) -> Self {
        Self {
            order_id: Some(order_id),
            ..self
        }
    }
}
