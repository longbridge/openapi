use chrono::{DateTime, Utc};

use crate::{
    trade::types::{OrderSide, OrderStatus},
    types::Market,
    utils::from_datetime,
};

/// Options for get histroy orders request
#[napi_derive::napi(object)]
pub struct GetHistoryOrdersOptions {
    /// Security symbol
    pub symbol: Option<String>,
    /// Order status
    pub status: Option<Vec<OrderStatus>>,
    /// Order side
    pub side: Option<OrderSide>,
    /// Market
    pub market: Option<Market>,
    /// Start time
    pub start_at: Option<DateTime<Utc>>,
    /// End time
    pub end_at: Option<DateTime<Utc>>,
}

impl From<GetHistoryOrdersOptions> for longport::trade::GetHistoryOrdersOptions {
    #[inline]
    fn from(opts: GetHistoryOrdersOptions) -> Self {
        let mut opts2 = longport::trade::GetHistoryOrdersOptions::new();
        if let Some(symbol) = opts.symbol {
            opts2 = opts2.symbol(symbol);
        }
        if let Some(status) = opts.status {
            opts2 = opts2.status(status.into_iter().map(Into::into));
        }
        if let Some(side) = opts.side {
            opts2 = opts2.side(side.into());
        }
        if let Some(market) = opts.market {
            opts2 = opts2.market(market.into());
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
