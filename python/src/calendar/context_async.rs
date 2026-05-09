use std::sync::Arc;

use longbridge::CalendarContext;
use pyo3::{prelude::*, types::PyType};

use crate::{calendar::types::*, config::Config, error::ErrorNewType};

/// Financial calendar context (async).
#[pyclass]
pub(crate) struct AsyncCalendarContext {
    ctx: Arc<CalendarContext>,
}

#[pymethods]
impl AsyncCalendarContext {
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        Self {
            ctx: Arc::new(CalendarContext::new(Arc::new(config.0.clone()))),
        }
    }

    #[pyo3(signature = (category, start, end, market = None))]
    fn finance_calendar(
        &self,
        py: Python<'_>,
        category: CalendarCategory,
        start: String,
        end: String,
        market: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(CalendarEventsResponse::from(
                ctx.finance_calendar(category.into(), start, end, market)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
}
