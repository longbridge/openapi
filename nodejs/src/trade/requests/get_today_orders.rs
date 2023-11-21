use crate::{
    trade::types::{OrderSide, OrderStatus},
    types::Market,
};

/// Options for get today orders request
#[napi_derive::napi(object)]
pub struct GetTodayOrdersOptions {
    /// Security symbol
    pub symbol: Option<String>,
    /// Order status
    pub status: Option<Vec<OrderStatus>>,
    /// Order side
    pub side: Option<OrderSide>,
    /// Market
    pub market: Option<Market>,
    /// Order id
    pub order_id: Option<String>,
}

impl From<GetTodayOrdersOptions> for longport::trade::GetTodayOrdersOptions {
    #[inline]
    fn from(opts: GetTodayOrdersOptions) -> Self {
        let mut opts2 = longport::trade::GetTodayOrdersOptions::new();
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
        if let Some(order_id) = opts.order_id {
            opts2 = opts2.order_id(order_id);
        }
        opts2
    }
}
