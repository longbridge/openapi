use chrono::{DateTime, Utc};

use crate::{trade::types::BalanceType, utils::from_datetime};

/// Options for get cash flow request
#[napi_derive::napi(object)]
pub struct GetCashFlowOptions {
    /// Start time
    pub start_at: DateTime<Utc>,
    /// End time
    pub end_at: DateTime<Utc>,
    /// Business type
    pub business_type: Option<BalanceType>,
    /// Security symbol
    pub symbol: Option<String>,
    /// Page number
    pub page: Option<i64>,
    /// Page size
    pub size: Option<i64>,
}

impl From<GetCashFlowOptions> for longbridge::trade::GetCashFlowOptions {
    #[inline]
    fn from(opts: GetCashFlowOptions) -> Self {
        let mut opts2 = longbridge::trade::GetCashFlowOptions::new(
            from_datetime(opts.start_at),
            from_datetime(opts.end_at),
        );
        if let Some(business_type) = opts.business_type {
            opts2 = opts2.business_type(business_type.into());
        }
        if let Some(symbol) = opts.symbol {
            opts2 = opts2.symbol(symbol);
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page.max(0) as usize);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size.max(0) as usize);
        }
        opts2
    }
}
