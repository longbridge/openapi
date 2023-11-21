use chrono::{DateTime, Utc};

use crate::utils::from_datetime;

/// Options for get histroy executions request
#[napi_derive::napi(object)]
pub struct GetHistoryExecutionsOptions {
    /// Security symbol
    pub symbol: Option<String>,
    /// Start time
    pub start_at: Option<DateTime<Utc>>,
    /// End time
    pub end_at: Option<DateTime<Utc>>,
}

impl From<GetHistoryExecutionsOptions> for longport::trade::GetHistoryExecutionsOptions {
    fn from(opts: GetHistoryExecutionsOptions) -> Self {
        let mut opts2 = longport::trade::GetHistoryExecutionsOptions::new();
        if let Some(symbol) = opts.symbol {
            opts2 = opts2.symbol(symbol);
        }
        if let Some(start_at) = opts.start_at {
            opts2 = opts2.start_at(from_datetime(start_at));
        }
        if let Some(end_at) = opts.end_at {
            opts2 = opts2.end_at(from_datetime(end_at));
        }
        opts2
    }
}
