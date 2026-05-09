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
    #[pyo3(signature = (category, start, end, market = None))]
    fn finance_calendar(
        &self,
        category: CalendarCategory,
        start: String,
        end: String,
        market: Option<String>,
    ) -> PyResult<CalendarEventsResponse> {
        Ok(self
            .ctx
            .finance_calendar(category.into(), start, end, market)
            .map_err(ErrorNewType)?
            .into())
    }
}
