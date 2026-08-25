use rust_decimal::Decimal;
use serde::Serialize;

use crate::trade::{MultiLegStrategy, OrderSide, OrderType};

/// A leg of a multi-leg combination order to submit
#[derive(Debug, Serialize, Clone)]
pub struct SubmitMultiLegOrderLeg {
    /// Option symbol, in `ticker.region` format (e.g. `QQQ260731C764000.US`)
    symbol: String,
    /// Leg ratio quantity — must be a positive number.  The direction of each
    /// leg is implied by `strategy` together with the order `side`, not by the
    /// sign of this value; a negative or zero ratio is rejected by the server
    /// with `602001`.
    ratio_quantity: Decimal,
}

impl SubmitMultiLegOrderLeg {
    /// Create a new `SubmitMultiLegOrderLeg`
    #[inline]
    pub fn new(symbol: impl Into<String>, ratio_quantity: Decimal) -> Self {
        Self {
            symbol: symbol.into(),
            ratio_quantity,
        }
    }
}

/// Options for submit multi-leg order request
#[derive(Debug, Serialize, Clone)]
pub struct SubmitMultiLegOrderOptions {
    side: OrderSide,
    order_type: OrderType,
    submitted_quantity: Decimal,
    strategy: MultiLegStrategy,
    legs: Vec<SubmitMultiLegOrderLeg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submitted_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_request_id: Option<String>,
}

impl SubmitMultiLegOrderOptions {
    /// Create a new `SubmitMultiLegOrderOptions`
    #[inline]
    pub fn new(
        side: OrderSide,
        order_type: OrderType,
        submitted_quantity: Decimal,
        strategy: MultiLegStrategy,
        legs: impl IntoIterator<Item = SubmitMultiLegOrderLeg>,
    ) -> Self {
        Self {
            side,
            order_type,
            submitted_quantity,
            strategy,
            legs: legs.into_iter().collect(),
            submitted_price: None,
            remark: None,
            client_request_id: None,
        }
    }

    /// Set the submitted price
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

    /// Set the remark
    #[inline]
    #[must_use]
    pub fn remark(self, remark: impl Into<String>) -> Self {
        Self {
            remark: Some(remark.into()),
            ..self
        }
    }

    /// Set the client request ID for idempotency control.
    /// If not specified, idempotency control is skipped.
    /// The server caches this ID for 10 minutes; requests with the same ID
    /// within that period return the original response without creating a new
    /// order.
    #[inline]
    #[must_use]
    pub fn client_request_id(self, id: impl Into<String>) -> Self {
        Self {
            client_request_id: Some(id.into()),
            ..self
        }
    }
}
