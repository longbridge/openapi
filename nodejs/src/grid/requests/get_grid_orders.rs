use crate::types::Market;

/// Options for get grid trading orders (list) request
#[napi_derive::napi(object)]
pub struct GetGridOrdersOptions {
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub limit: Option<i32>,
    /// Market
    pub market: Option<Market>,
    /// Comma-joined status filter (e.g. `Performing,Suspended`)
    pub status: Option<String>,
    /// Security symbol filter (e.g. `700.HK`)
    pub symbol: Option<String>,
    /// Sort field
    pub sort_by: Option<String>,
    /// Sort order
    pub sort_order: Option<String>,
}

impl From<GetGridOrdersOptions> for longbridge::grid::GetGridOrdersOptions {
    #[inline]
    fn from(opts: GetGridOrdersOptions) -> Self {
        let mut opts2 = longbridge::grid::GetGridOrdersOptions::new();
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(limit) = opts.limit {
            opts2 = opts2.limit(limit);
        }
        if let Some(market) = opts.market {
            opts2 = opts2.market(market.into());
        }
        if let Some(status) = opts.status {
            opts2 = opts2.status(status);
        }
        if let Some(symbol) = opts.symbol {
            opts2 = opts2.symbol(symbol);
        }
        if let Some(sort_by) = opts.sort_by {
            opts2 = opts2.sort_by(sort_by);
        }
        if let Some(sort_order) = opts.sort_order {
            opts2 = opts2.sort_order(sort_order);
        }
        opts2
    }
}
