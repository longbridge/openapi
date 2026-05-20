use std::sync::Arc;

use napi::Result;

use crate::{calendar::types::*, config::Config, error::ErrorNewType};

/// Financial calendar context — earnings, dividends, splits, IPOs, macro data.
#[napi_derive::napi]
#[derive(Clone)]
pub struct CalendarContext {
    ctx: longbridge::CalendarContext,
}

#[napi_derive::napi]
impl CalendarContext {
    /// Create a new CalendarContext.
    #[napi]
    pub fn new(config: &Config) -> CalendarContext {
        Self {
            ctx: longbridge::CalendarContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get financial calendar events.
    ///
    /// `start` and `end` are date strings in `YYYY-MM-DD` format.
    /// `market` is an optional market filter (e.g. `"HK"` or `"US"`).
    #[napi]
    pub async fn finance_calendar(
        &self,
        category: CalendarCategory,
        start: String,
        end: String,
        market: Option<String>,
    ) -> Result<CalendarEventsResponse> {
        Ok(self
            .ctx
            .finance_calendar(category.into(), start, end, market)
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
