use serde::Serialize;

use crate::{
    Market,
    trade::{OrderSide, OrderStatus},
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
    #[serde(skip_serializing_if = "Option::is_none")]
    is_attached: Option<bool>,
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

    /// When set together with [`order_id`], indicates that `order_id` is an
    /// attached sub-order ID. The server returns the attached sub-order itself
    /// as an [`Order`] entry (not the parent order). Has no effect without
    /// [`order_id`].
    pub fn is_attached(self) -> Self {
        Self {
            is_attached: Some(true),
            ..self
        }
    }
}
