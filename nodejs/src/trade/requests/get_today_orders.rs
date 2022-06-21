use napi::Either;

use crate::{
    trade::types::{OrderSide, OrderStatus},
    types::Market,
};

/// Options for get today orders request
#[napi_derive::napi]
#[derive(Clone, Default)]
pub struct GetTodayOrdersOptions(longbridge::trade::GetTodayOrdersOptions);

#[napi_derive::napi(object)]
impl GetTodayOrdersOptions {
    /// Create a new `GetTodayOrdersOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new() -> GetTodayOrdersOptions {
        Default::default()
    }

    /// Set the security symbol
    #[napi]
    #[inline]
    pub fn symbol(&self, symbol: String) -> GetTodayOrdersOptions {
        Self(self.0.clone().symbol(symbol))
    }

    /// Set the order status
    #[napi]
    #[inline]
    pub fn status(&self, status: Either<OrderStatus, Vec<OrderStatus>>) -> GetTodayOrdersOptions {
        Self(self.0.clone().status(match status {
            Either::A(status) => vec![status.into()],
            Either::B(status) => status.into_iter().map(Into::into).collect(),
        }))
    }

    /// Set the order side
    #[napi]
    #[inline]
    pub fn side(&self, side: OrderSide) -> GetTodayOrdersOptions {
        Self(self.0.clone().side(side.into()))
    }

    /// Set the market
    #[napi]
    #[inline]
    pub fn market(&self, market: Market) -> GetTodayOrdersOptions {
        Self(self.0.clone().market(market.into()))
    }

    /// Set the order id
    #[napi]
    #[inline]
    pub fn order_id(&self, order_id: String) -> GetTodayOrdersOptions {
        Self(self.0.clone().order_id(order_id))
    }
}

impl From<GetTodayOrdersOptions> for longbridge::trade::GetTodayOrdersOptions {
    #[inline]
    fn from(opts: GetTodayOrdersOptions) -> Self {
        opts.0
    }
}
