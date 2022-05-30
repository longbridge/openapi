use crate::decimal::Decimal;

/// Options for get today orders request
#[napi_derive::napi]
#[derive(Clone)]
pub struct ReplaceOrderOptions(longbridge::trade::ReplaceOrderOptions);

#[napi_derive::napi(object)]
impl ReplaceOrderOptions {
    /// Create a new `ReplaceOrderOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new(order_id: String, quantity: &Decimal) -> ReplaceOrderOptions {
        Self(longbridge::trade::ReplaceOrderOptions::new(
            order_id, quantity.0,
        ))
    }

    /// Set the price
    #[napi]
    #[inline]
    pub fn price(&self, price: &Decimal) -> ReplaceOrderOptions {
        Self(self.0.clone().price(price.0))
    }

    /// Set the trigger price
    #[napi]
    #[inline]
    pub fn trigger_price(&self, trigger_price: &Decimal) -> Self {
        Self(self.0.clone().trigger_price(trigger_price.0))
    }

    /// Set the limit offset
    #[napi]
    #[inline]
    pub fn limit_offset(&self, limit_offset: &Decimal) -> Self {
        Self(self.0.clone().limit_offset(limit_offset.0))
    }

    /// Set the trailing amount
    #[napi]
    #[inline]
    pub fn trailing_amount(&self, trailing_amount: &Decimal) -> Self {
        Self(self.0.clone().trailing_amount(trailing_amount.0))
    }

    /// Set the trailing percent
    #[napi]
    #[inline]
    pub fn trailing_percent(&self, trailing_percent: &Decimal) -> Self {
        Self(self.0.clone().trailing_percent(trailing_percent.0))
    }

    /// Set the remark
    #[napi]
    #[inline]
    pub fn remark(&self, remark: String) -> Self {
        Self(self.0.clone().remark(remark))
    }
}

impl From<ReplaceOrderOptions> for longbridge::trade::ReplaceOrderOptions {
    #[inline]
    fn from(opts: ReplaceOrderOptions) -> Self {
        opts.0
    }
}
