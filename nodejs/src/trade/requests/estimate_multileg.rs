use napi::bindgen_prelude::ClassInstance;

use crate::{
    decimal::Decimal,
    trade::types::{MultiLegStrategy, OrderSide, OrderType},
};

/// A leg of a multi-leg combination for the pre-trade estimate
#[napi_derive::napi(object)]
pub struct EstimateMultiLegOrderLeg {
    /// Option or underlying-stock symbol, in `ticker.region` format
    /// (e.g. `QQQ260731C764000.US`)
    pub symbol: String,
    /// Leg order side. Reserved field, not in use for now.
    pub side: Option<OrderSide>,
}

/// Options for estimating a multi-leg option combination's maximum tradable
/// quantity and margin impact
#[napi_derive::napi(object)]
pub struct EstimateMultiLegAvailableQuantityOptions<'env> {
    /// Order side of the combination
    pub side: OrderSide,
    /// Order type
    pub order_type: OrderType,
    /// Submitted quantity (number of combinations)
    pub submitted_quantity: ClassInstance<'env, Decimal>,
    /// Multi-leg strategy
    pub strategy: MultiLegStrategy,
    /// Legs of the combination
    pub legs: Vec<EstimateMultiLegOrderLeg>,
    /// Submitted price (required for limit order types such as `LO`)
    pub submitted_price: Option<ClassInstance<'env, Decimal>>,
}

impl<'env> From<EstimateMultiLegAvailableQuantityOptions<'env>>
    for longbridge::trade::EstimateMultiLegAvailableQuantityOptions
{
    #[inline]
    fn from(opts: EstimateMultiLegAvailableQuantityOptions<'env>) -> Self {
        let legs = opts.legs.into_iter().map(|leg| {
            let mut l = longbridge::trade::EstimateMultiLegOrderLeg::new(leg.symbol);
            if let Some(side) = leg.side {
                l = l.side(side.into());
            }
            l
        });
        let mut opts2 = longbridge::trade::EstimateMultiLegAvailableQuantityOptions::new(
            opts.side.into(),
            opts.order_type.into(),
            opts.submitted_quantity.0,
            opts.strategy.into(),
            legs,
        );
        if let Some(submitted_price) = opts.submitted_price {
            opts2 = opts2.submitted_price(submitted_price.0);
        }
        opts2
    }
}
