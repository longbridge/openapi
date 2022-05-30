use chrono::{DateTime, Utc};

use crate::{trade::types::BalanceType, utils::from_datetime};

/// Options for submit order request
#[napi_derive::napi]
#[derive(Clone)]
pub struct GetCashFlowOptions(longbridge::trade::GetCashFlowOptions);

#[napi_derive::napi(object)]
impl GetCashFlowOptions {
    /// Create a new `GetCashFlowOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new(start_at: DateTime<Utc>, end_at: DateTime<Utc>) -> GetCashFlowOptions {
        Self(longbridge::trade::GetCashFlowOptions::new(
            from_datetime(start_at),
            from_datetime(end_at),
        ))
    }

    /// Set the business type
    #[napi]
    #[inline]
    pub fn business_type(&self, business_type: BalanceType) -> GetCashFlowOptions {
        Self(self.0.clone().business_type(business_type.into()))
    }

    /// Set the security symbol
    #[napi]
    #[inline]
    pub fn symbol(&self, symbol: String) -> GetCashFlowOptions {
        Self(self.0.clone().symbol(symbol))
    }

    /// Set the page number
    #[napi]
    #[inline]
    pub fn page(&self, page: i32) -> GetCashFlowOptions {
        Self(self.0.clone().page(page.max(0) as usize))
    }

    /// Set the page size
    #[napi]
    #[inline]
    pub fn size(&self, size: i32) -> GetCashFlowOptions {
        Self(self.0.clone().size(size.max(0) as usize))
    }
}

impl From<GetCashFlowOptions> for longbridge::trade::GetCashFlowOptions {
    #[inline]
    fn from(opts: GetCashFlowOptions) -> Self {
        opts.0
    }
}
