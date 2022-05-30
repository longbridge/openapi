use crate::{
    decimal::Decimal,
    time::NaiveDate,
    trade::types::{OrderSide, OrderType, OutsideRTH, TimeInForceType},
};

/// Options for submit order request
#[napi_derive::napi]
#[derive(Clone)]
pub struct SubmitOrderOptions(longbridge::trade::SubmitOrderOptions);

#[napi_derive::napi]
impl SubmitOrderOptions {
    /// Create a new `SubmitOrderOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new(
        symbol: String,
        order_type: OrderType,
        side: OrderSide,
        submitted_quantity: &Decimal,
        time_in_force: TimeInForceType,
    ) -> SubmitOrderOptions {
        Self(longbridge::trade::SubmitOrderOptions::new(
            symbol,
            order_type.into(),
            side.into(),
            submitted_quantity.0,
            time_in_force.into(),
        ))
    }

    /// Set the submitted price
    #[napi]
    #[inline]
    pub fn submitted_price(&self, submitted_price: &Decimal) -> SubmitOrderOptions {
        Self(self.0.clone().submitted_price(submitted_price.0))
    }

    /// Set the trigger price
    #[napi]
    #[inline]
    pub fn trigger_price(&self, trigger_price: &Decimal) -> SubmitOrderOptions {
        Self(self.0.clone().trigger_price(trigger_price.0))
    }

    /// Set the limit offset
    #[napi]
    #[inline]
    pub fn limit_offset(&self, limit_offset: &Decimal) -> SubmitOrderOptions {
        Self(self.0.clone().limit_offset(limit_offset.0))
    }

    /// Set the trailing amount
    #[napi]
    #[inline]
    pub fn trailing_amount(&self, trailing_amount: &Decimal) -> SubmitOrderOptions {
        Self(self.0.clone().trailing_amount(trailing_amount.0))
    }

    /// Set the trailing percent
    #[napi]
    #[inline]
    pub fn trailing_percent(&self, trailing_percent: &Decimal) -> SubmitOrderOptions {
        Self(self.0.clone().trailing_percent(trailing_percent.0))
    }

    /// Set the expire date
    #[napi]
    #[inline]
    pub fn expire_date(&self, expire_date: &NaiveDate) -> SubmitOrderOptions {
        Self(self.0.clone().expire_date(expire_date.0))
    }

    /// Enable or disable outside regular trading hours
    #[napi]
    #[inline]
    pub fn outside_rth(&self, outside_rth: OutsideRTH) -> SubmitOrderOptions {
        Self(self.0.clone().outside_rth(outside_rth.into()))
    }

    /// Set the remark
    #[napi]
    #[inline]
    pub fn remark(&self, remark: String) -> SubmitOrderOptions {
        Self(self.0.clone().remark(remark))
    }
}

impl From<SubmitOrderOptions> for longbridge::trade::SubmitOrderOptions {
    #[inline]
    fn from(opts: SubmitOrderOptions) -> Self {
        opts.0
    }
}
