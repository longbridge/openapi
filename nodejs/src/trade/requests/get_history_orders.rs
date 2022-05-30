use chrono::{DateTime, Utc};
use napi::Either;

use crate::{
    trade::types::{OrderSide, OrderStatus},
    types::Market,
    utils::from_datetime,
};

/// Options for get histroy orders request
#[napi_derive::napi]
#[derive(Clone, Default)]
pub struct GetHistoryOrdersOptions(longbridge::trade::GetHistoryOrdersOptions);

#[napi_derive::napi(object)]
impl GetHistoryOrdersOptions {
    /// Create a new `GetHistoryOrdersOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new() -> GetHistoryOrdersOptions {
        Default::default()
    }

    /// Set the security symbol
    #[napi]
    #[inline]
    pub fn symbol(&self, symbol: String) -> GetHistoryOrdersOptions {
        Self(self.0.clone().symbol(symbol))
    }

    /// Set the order status
    #[napi]
    #[inline]
    pub fn status(&self, status: Either<OrderStatus, Vec<OrderStatus>>) -> GetHistoryOrdersOptions {
        Self(self.0.clone().status(match status {
            Either::A(status) => vec![status.into()],
            Either::B(status) => status.into_iter().map(Into::into).collect(),
        }))
    }

    /// Set the order side
    #[napi]
    #[inline]
    pub fn side(&self, side: OrderSide) -> GetHistoryOrdersOptions {
        Self(self.0.clone().side(side.into()))
    }

    /// Set the market
    #[napi]
    #[inline]
    pub fn market(&self, market: Market) -> GetHistoryOrdersOptions {
        Self(self.0.clone().market(market.into()))
    }

    /// Set the start time
    #[napi]
    #[inline]
    pub fn start_at(&self, start_at: DateTime<Utc>) -> GetHistoryOrdersOptions {
        Self(self.0.clone().start_at(from_datetime(start_at)))
    }

    /// Set the end time
    #[napi]
    #[inline]
    pub fn end_at(&self, end_at: DateTime<Utc>) -> GetHistoryOrdersOptions {
        Self(self.0.clone().end_at(from_datetime(end_at)))
    }
}

impl From<GetHistoryOrdersOptions> for longbridge::trade::GetHistoryOrdersOptions {
    #[inline]
    fn from(opts: GetHistoryOrdersOptions) -> Self {
        opts.0
    }
}
