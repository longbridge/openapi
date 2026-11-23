use std::sync::Arc;

use longbridge::blocking::CalendarContextSync;
use pyo3::prelude::*;

use crate::{calendar::types::*, config::Config, error::ErrorNewType};

/// Financial calendar context (synchronous).
#[pyclass]
pub(crate) struct CalendarContext {
    ctx: CalendarContextSync,
}

#[pymethods]
impl CalendarContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: CalendarContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get financial calendar events.
    ///
    /// Events come back grouped by date. ``count`` caps the events returned for
    /// *each date* (server default when omitted), and a group's ``count`` field
    /// is that date's total, so a group holding fewer events than its ``count``
    /// was truncated: fetch the rest by calling again with ``start`` = ``end``
    /// = that date and ``offset`` set to the number of events already received.
    /// The ``next_date`` cursor points at the next date group and moves past
    /// truncated dates. ``end`` is not a hard bound: the server may return
    /// later dates, so filter by date on the client.
    #[pyo3(signature = (category, start, end, market = None, count = None, offset = None, next = None))]
    fn finance_calendar(
        &self,
        category: CalendarCategory,
        start: String,
        end: String,
        market: Option<String>,
        count: Option<i32>,
        offset: Option<i32>,
        next: Option<CalendarPageDirection>,
    ) -> PyResult<CalendarEventsResponse> {
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
            .map_err(ErrorNewType)?
            .into())
    }
}
