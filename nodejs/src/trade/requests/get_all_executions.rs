use chrono::{DateTime, Utc};

use crate::utils::from_datetime;

/// Options for get all executions request
#[napi_derive::napi(object)]
pub struct GetAllExecutionsOptions {
    /// Security symbol
    pub symbol: Option<String>,
    /// Order id
    pub order_id: Option<String>,
    /// Start time
    pub start_at: Option<DateTime<Utc>>,
    /// End time
    pub end_at: Option<DateTime<Utc>>,
    /// Page number
    pub page: Option<i64>,
}

impl From<GetAllExecutionsOptions> for longbridge::trade::GetAllExecutionsOptions {
    fn from(opts: GetAllExecutionsOptions) -> Self {
        let mut opts2 = longbridge::trade::GetAllExecutionsOptions::new();
        if let Some(symbol) = opts.symbol {
            opts2 = opts2.symbol(symbol);
        }
        if let Some(order_id) = opts.order_id {
            opts2 = opts2.order_id(order_id);
        }
        if let Some(start_at) = opts.start_at {
            opts2 = opts2.start_at(from_datetime(start_at));
        }
        if let Some(end_at) = opts.end_at {
            opts2 = opts2.end_at(from_datetime(end_at));
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page as u64);
        }
        opts2
    }
}
