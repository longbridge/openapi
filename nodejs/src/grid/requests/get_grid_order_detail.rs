/// Options for get grid trading order detail request
#[napi_derive::napi(object)]
pub struct GetGridOrderDetailOptions {
    /// Grid master order id
    pub order_id: String,
    /// History cursor for paging through the trigger history
    pub history_id: Option<String>,
    /// Page size
    pub limit: Option<i32>,
}

impl From<GetGridOrderDetailOptions> for longbridge::grid::GetGridOrderDetailOptions {
    #[inline]
    fn from(opts: GetGridOrderDetailOptions) -> Self {
        let mut opts2 = longbridge::grid::GetGridOrderDetailOptions::new(opts.order_id);
        if let Some(history_id) = opts.history_id {
            opts2 = opts2.history_id(history_id);
        }
        if let Some(limit) = opts.limit {
            opts2 = opts2.limit(limit);
        }
        opts2
    }
}
