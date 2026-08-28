use napi::bindgen_prelude::ClassInstance;

use crate::{
    decimal::Decimal,
    trade::types::{MultiLegStrategy, OrderSide, OrderType},
};

/// A leg of a multi-leg combination order to submit
#[napi_derive::napi(object)]
pub struct SubmitMultiLegOrderLeg<'env> {
    /// Option symbol, in `ticker.region` format (e.g. `QQQ260731C764000.US`)
    pub symbol: String,
    /// Leg ratio quantity — must be a positive number.  The direction of each
    /// leg is implied by `strategy` together with the order `side`, not by the
    /// sign of this value; a negative or zero ratio is rejected by the server
    /// with `602001`.
    pub ratio_quantity: ClassInstance<'env, Decimal>,
}

/// Options for submit multi-leg order request
#[napi_derive::napi(object)]
pub struct SubmitMultiLegOrderOptions<'env> {
    /// Order side
    pub side: OrderSide,
    /// Order type
    pub order_type: OrderType,
    /// Submitted quantity (number of combinations)
    pub submitted_quantity: ClassInstance<'env, Decimal>,
    /// Multi-leg strategy
    pub strategy: MultiLegStrategy,
    /// Legs of the combination order
    pub legs: Vec<SubmitMultiLegOrderLeg<'env>>,
    /// Submitted price (required for limit order types such as `LO`)
    pub submitted_price: Option<ClassInstance<'env, Decimal>>,
    /// Remark (Maximum 255 characters)
    pub remark: Option<String>,
    /// Client request ID for idempotency control.
    /// If not specified, idempotency control is skipped.
    /// The server caches this ID for 10 minutes.
    pub client_request_id: Option<String>,
}

impl<'env> From<SubmitMultiLegOrderOptions<'env>>
    for longbridge::trade::SubmitMultiLegOrderOptions
{
    #[inline]
    fn from(opts: SubmitMultiLegOrderOptions<'env>) -> Self {
        let legs = opts.legs.into_iter().map(|leg| {
            longbridge::trade::SubmitMultiLegOrderLeg::new(leg.symbol, leg.ratio_quantity.0)
        });
        let mut opts2 = longbridge::trade::SubmitMultiLegOrderOptions::new(
            opts.side.into(),
            opts.order_type.into(),
            opts.submitted_quantity.0,
            opts.strategy.into(),
            legs,
        );
        if let Some(submitted_price) = opts.submitted_price {
            opts2 = opts2.submitted_price(submitted_price.0);
        }
        if let Some(remark) = opts.remark {
            opts2 = opts2.remark(remark);
        }
        if let Some(id) = opts.client_request_id {
            opts2 = opts2.client_request_id(id);
        }
        opts2
    }
}
