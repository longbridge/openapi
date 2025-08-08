use napi::bindgen_prelude::ClassInstance;

use crate::{
    decimal::Decimal,
    trade::types::{OrderSide, OrderType},
};

/// Options for get cash flow request
#[napi_derive::napi(object)]
pub struct EstimateMaxPurchaseQuantityOptions<'env> {
    pub symbol: String,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub price: Option<ClassInstance<'env, Decimal>>,
    pub currency: Option<String>,
    pub order_id: Option<String>,
    pub fractional_shares: bool,
}

impl<'env> From<EstimateMaxPurchaseQuantityOptions<'env>>
    for longport::trade::EstimateMaxPurchaseQuantityOptions
{
    #[inline]
    fn from(opts: EstimateMaxPurchaseQuantityOptions) -> Self {
        let mut opts2 = longport::trade::EstimateMaxPurchaseQuantityOptions::new(
            opts.symbol,
            opts.order_type.into(),
            opts.side.into(),
        );
        if let Some(price) = opts.price {
            opts2 = opts2.price(price.0);
        }
        if let Some(currency) = opts.currency {
            opts2 = opts2.currency(currency);
        }
        if let Some(order_id) = opts.order_id {
            opts2 = opts2.order_id(order_id);
        }
        if opts.fractional_shares {
            opts2 = opts2.fractional_shares();
        }
        opts2
    }
}
