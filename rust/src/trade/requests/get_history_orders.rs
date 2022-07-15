use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    serde_utils,
    trade::{OrderSide, OrderStatus},
    Market,
};

/// Options for get history orders request
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetHistoryOrdersOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    status: Vec<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<OrderSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<Market>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::timestamp_opt"
    )]
    start_at: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::timestamp_opt"
    )]
    end_at: Option<OffsetDateTime>,
}

impl GetHistoryOrdersOptions {
    /// Create a new `GetHistoryOrdersOptions`
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

    /// Set the start time
    #[inline]
    #[must_use]
    pub fn start_at(self, start_at: OffsetDateTime) -> Self {
        Self {
            start_at: Some(start_at),
            ..self
        }
    }

    /// Set the end time
    #[inline]
    #[must_use]
    pub fn end_at(self, end_at: OffsetDateTime) -> Self {
        Self {
            end_at: Some(end_at),
            ..self
        }
    }
}
