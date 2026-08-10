/// Options for query grid trading orders by IDs request
#[napi_derive::napi(object)]
pub struct GetGridOrdersByIdsOptions {
    /// Grid master order IDs
    pub order_ids: Vec<String>,
}

impl From<GetGridOrdersByIdsOptions> for longbridge::trade::GetGridOrdersByIdsOptions {
    #[inline]
    fn from(opts: GetGridOrdersByIdsOptions) -> Self {
        longbridge::trade::GetGridOrdersByIdsOptions::new(opts.order_ids)
    }
}
