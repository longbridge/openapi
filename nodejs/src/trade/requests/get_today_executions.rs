/// Options for get today executions request
#[napi_derive::napi(object)]
pub struct GetTodayExecutionsOptions {
    /// Security symbol
    pub symbol: Option<String>,
    /// Order id
    pub order_id: Option<String>,
}

impl From<GetTodayExecutionsOptions> for longport::trade::GetTodayExecutionsOptions {
    #[inline]
    fn from(opts: GetTodayExecutionsOptions) -> Self {
        let mut opts2 = longport::trade::GetTodayExecutionsOptions::new();
        if let Some(symbol) = opts.symbol {
            opts2 = opts2.symbol(symbol);
        }
        if let Some(order_id) = opts.order_id {
            opts2 = opts2.order_id(order_id);
        }
        opts2
    }
}
