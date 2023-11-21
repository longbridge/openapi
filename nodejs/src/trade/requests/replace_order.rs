use napi::bindgen_prelude::ClassInstance;

use crate::decimal::Decimal;

/// Options for replace order request
#[napi_derive::napi(object)]
pub struct ReplaceOrderOptions {
    /// Order id
    pub order_id: String,
    /// Replaced quantity
    pub quantity: i64,
    /// Replaced price
    pub price: Option<ClassInstance<Decimal>>,
    /// Trigger price (`LIT` / `MIT` Order Required)
    pub trigger_price: Option<ClassInstance<Decimal>>,
    /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
    pub limit_offset: Option<ClassInstance<Decimal>>,
    /// Trailing amount (`TSLPAMT` / `TSMAMT` Required)
    pub trailing_amount: Option<ClassInstance<Decimal>>,
    /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
    pub trailing_percent: Option<ClassInstance<Decimal>>,
    /// Remark (Maximum 64 characters)
    pub remark: Option<String>,
}

impl From<ReplaceOrderOptions> for longport::trade::ReplaceOrderOptions {
    #[inline]
    fn from(opts: ReplaceOrderOptions) -> Self {
        let mut opts2 = longport::trade::ReplaceOrderOptions::new(opts.order_id, opts.quantity);
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
