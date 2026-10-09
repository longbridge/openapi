use rust_decimal::Decimal;
use serde::Serialize;

use crate::trade::{MultiLegStrategy, OrderSide, OrderType};

/// A leg of a multi-leg combination for the pre-trade estimate.
#[derive(Debug, Serialize, Clone)]
pub struct EstimateMultiLegOrderLeg {
    /// Option or underlying-stock symbol, in `ticker.region` format
    /// (e.g. `QQQ260731C764000.US`).
    symbol: String,
    /// Leg order side. Reserved field, not in use for now; the direction of
    /// each leg is implied by `strategy` together with the order `side`.
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<OrderSide>,
}

impl EstimateMultiLegOrderLeg {
    /// Create a new `EstimateMultiLegOrderLeg`.
    #[inline]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            side: None,
        }
    }

    /// Set the leg order side (reserved; not in use for now).
    #[inline]
    #[must_use]
    pub fn side(self, side: OrderSide) -> Self {
        Self {
            side: Some(side),
            ..self
        }
    }
}

/// Options for estimating a multi-leg option combination's maximum tradable
/// quantity and margin impact
/// ([`TradeContext::estimate_multileg_available_quantity`]).
///
/// Only the US market is supported; all legs must share the same underlying and
/// settlement currency, each leg must be an option or the underlying stock, and
/// duplicate legs (same symbol and side) are rejected.
///
/// [`TradeContext::estimate_multileg_available_quantity`]: crate::trade::TradeContext::estimate_multileg_available_quantity
#[derive(Debug, Serialize, Clone)]
pub struct EstimateMultiLegAvailableQuantityOptions {
    side: OrderSide,
    order_type: OrderType,
    submitted_quantity: Decimal,
    strategy: MultiLegStrategy,
    legs: Vec<EstimateMultiLegOrderLeg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submitted_price: Option<Decimal>,
}

impl EstimateMultiLegAvailableQuantityOptions {
    /// Create a new `EstimateMultiLegAvailableQuantityOptions`.
    #[inline]
    pub fn new(
        side: OrderSide,
        order_type: OrderType,
        submitted_quantity: Decimal,
        strategy: MultiLegStrategy,
        legs: impl IntoIterator<Item = EstimateMultiLegOrderLeg>,
    ) -> Self {
        Self {
            side,
            order_type,
            submitted_quantity,
            strategy,
            legs: legs.into_iter().collect(),
            submitted_price: None,
        }
    }

    /// Set the submitted price.
    ///
    /// Required for limit order types such as `LO`.
    #[inline]
    #[must_use]
    pub fn submitted_price(self, submitted_price: Decimal) -> Self {
        Self {
            submitted_price: Some(submitted_price),
            ..self
        }
    }
}
