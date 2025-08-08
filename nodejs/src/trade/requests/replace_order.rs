use napi::bindgen_prelude::ClassInstance;

use crate::decimal::Decimal;

/// Options for replace order request
#[napi_derive::napi(object)]
pub struct ReplaceOrderOptions<'env> {
    /// Order id
    pub order_id: String,
    /// Replaced quantity
    pub quantity: ClassInstance<'env, Decimal>,
    /// Replaced price
    pub price: Option<ClassInstance<'env, Decimal>>,
    /// Trigger price (`LIT` / `MIT` Order Required)
    pub trigger_price: Option<ClassInstance<'env, Decimal>>,
    /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
    pub limit_offset: Option<ClassInstance<'env, Decimal>>,
    /// Trailing amount (`TSLPAMT` / `TSMAMT` Required)
    pub trailing_amount: Option<ClassInstance<'env, Decimal>>,
    /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
    pub trailing_percent: Option<ClassInstance<'env, Decimal>>,
    /// Remark (Maximum 64 characters)
    pub remark: Option<String>,
}

impl<'env> From<ReplaceOrderOptions<'env>> for longport::trade::ReplaceOrderOptions {
    #[inline]
    fn from(opts: ReplaceOrderOptions<'env>) -> Self {
        let mut opts2 = longport::trade::ReplaceOrderOptions::new(opts.order_id, opts.quantity.0);
        if let Some(price) = opts.price {
            opts2 = opts2.price(price.0);
        }
        if let Some(trigger_price) = opts.trigger_price {
            opts2 = opts2.trigger_price(trigger_price.0);
        }
        if let Some(limit_offset) = opts.limit_offset {
            opts2 = opts2.limit_offset(limit_offset.0);
        }
        if let Some(trailing_amount) = opts.trailing_amount {
            opts2 = opts2.trailing_amount(trailing_amount.0);
        }
        if let Some(trailing_percent) = opts.trailing_percent {
            opts2 = opts2.trailing_percent(trailing_percent.0);
        }
        if let Some(remark) = opts.remark {
            opts2 = opts2.remark(remark);
        }
        opts2
    }
}
