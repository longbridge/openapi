use chrono::{DateTime, Utc};

use crate::utils::from_datetime;

/// Options for get histroy executions request
#[napi_derive::napi]
#[derive(Clone, Default)]
pub struct GetHistoryExecutionsOptions(longbridge::trade::GetHistoryExecutionsOptions);

#[napi_derive::napi(object)]
impl GetHistoryExecutionsOptions {
    /// Create a new `GetHistoryExecutionsOptions`
    #[napi(constructor)]
    #[inline]
    pub fn new() -> GetHistoryExecutionsOptions {
        Default::default()
    }

    /// Set the security symbol
    #[napi]
    #[inline]
    pub fn symbol(&self, symbol: String) -> GetHistoryExecutionsOptions {
        Self(self.0.clone().symbol(symbol))
    }

    /// Set the start time
    #[napi]
    #[inline]
    pub fn start_at(&self, start_at: DateTime<Utc>) -> GetHistoryExecutionsOptions {
        Self(self.0.clone().start_at(from_datetime(start_at)))
    }

    /// Set the end time
    #[napi]
    #[inline]
    pub fn end_at(&self, end_at: DateTime<Utc>) -> GetHistoryExecutionsOptions {
        Self(self.0.clone().end_at(from_datetime(end_at)))
    }
}

impl From<GetHistoryExecutionsOptions> for longbridge::trade::GetHistoryExecutionsOptions {
    #[inline]
    fn from(opts: GetHistoryExecutionsOptions) -> Self {
        opts.0
    }
}
