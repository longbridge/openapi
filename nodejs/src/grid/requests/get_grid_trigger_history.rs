/// Options for get grid trading trigger history request
#[napi_derive::napi(object)]
pub struct GetGridTriggerHistoryOptions {
    /// Grid master order id
    pub grid_order_id: String,
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub limit: Option<i32>,
}

impl From<GetGridTriggerHistoryOptions> for longbridge::grid::GetGridTriggerHistoryOptions {
    #[inline]
    fn from(opts: GetGridTriggerHistoryOptions) -> Self {
        let mut opts2 = longbridge::grid::GetGridTriggerHistoryOptions::new(opts.grid_order_id);
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(limit) = opts.limit {
            opts2 = opts2.limit(limit);
        }
        opts2
    }
}
