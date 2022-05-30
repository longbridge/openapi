/// Options for get histroy executions request
#[napi_derive::napi]
#[derive(Clone, Default)]
pub struct GetTodayExecutionsOptions(longbridge::trade::GetTodayExecutionsOptions);

#[napi_derive::napi(object)]
impl GetTodayExecutionsOptions {
    /// Create a new `GetTodayExecutionsOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new() -> GetTodayExecutionsOptions {
        Default::default()
    }

    /// Set the security symbol
    #[napi]
    #[inline]
    pub fn symbol(&self, symbol: String) -> GetTodayExecutionsOptions {
        Self(self.0.clone().symbol(symbol))
    }

    /// Set the order id
    #[napi]
    #[inline]
    pub fn order_id(&self, order_id: String) -> GetTodayExecutionsOptions {
        Self(self.0.clone().order_id(order_id))
    }
}

impl From<GetTodayExecutionsOptions> for longbridge::trade::GetTodayExecutionsOptions {
    #[inline]
    fn from(opts: GetTodayExecutionsOptions) -> Self {
        opts.0
    }
}
