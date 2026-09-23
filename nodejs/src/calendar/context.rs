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
    ///
    /// The endpoint paginates: the server caps each response (historically 10
    /// events per page unless a larger `count` is requested) and returns a
    /// `nextDate` cursor. To page through the full window, request a larger
    /// `count`, or re-call with the returned `nextDate` as `start`.
    ///
    /// - `count` — maximum number of events per page (server default when
    ///   omitted).
    /// - `offset` — number of events to skip from the start of the window.
    /// - `next` — direction to page from the cursor.
    #[napi]
    pub async fn finance_calendar(
        &self,
        category: CalendarCategory,
        start: String,
        end: String,
        market: Option<String>,
        count: Option<i32>,
        offset: Option<i32>,
        next: Option<CalendarPageDirection>,
    ) -> Result<CalendarEventsResponse> {
        Ok(self
            .ctx
            .finance_calendar(
                category.into(),
                start,
                end,
                market,
                count,
                offset,
                next.map(Into::into),
            )
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
