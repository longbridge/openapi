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
    /// When set together with order_id, indicates that order_id is an attached
    /// sub-order ID. The server returns the attached sub-order itself as an
    /// Order entry (not the parent order). Has no effect without order_id.
    pub is_attached: Option<bool>,
}

impl From<GetTodayOrdersOptions> for longbridge::trade::GetTodayOrdersOptions {
    #[inline]
    fn from(opts: GetTodayOrdersOptions) -> Self {
        let mut opts2 = longbridge::trade::GetTodayOrdersOptions::new();
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
        if opts.is_attached == Some(true) {
            opts2 = opts2.is_attached();
        }
        opts2
    }
}
