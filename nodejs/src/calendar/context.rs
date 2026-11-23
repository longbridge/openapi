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
    /// Events come back grouped by date. `count` caps the events returned for
    /// *each date* (server default when omitted), and a group's `count` field
    /// is that date's total, so a group holding fewer events than its `count`
    /// was truncated: fetch the rest by calling again with `start` = `end` =
    /// that date and `offset` set to the number of events already received. The
    /// `nextDate` cursor points at the next date group and moves past truncated
    /// dates. `end` is not a hard bound: the server may return later dates, so
    /// filter by date on the client.
    ///
    /// - `count` — maximum number of events per date (server default when
    ///   omitted).
    /// - `offset` — number of events to skip within a date.
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
