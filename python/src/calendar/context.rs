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
    /// The endpoint paginates: the server caps each response (historically 10
    /// events per page unless a larger ``count`` is requested) and returns a
    /// ``next_date`` cursor. To page through the full window, request a larger
    /// ``count``, or re-call with the returned ``next_date`` as ``start``.
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
