use std::sync::Arc;

use longbridge::content::ContentContext;
use pyo3::{prelude::*, types::PyType};

use crate::{
    config::Config,
    content::types::{NewsItem, TopicItem},
    error::ErrorNewType,
};

/// Async content context.
#[pyclass]
pub(crate) struct AsyncContentContext {
    ctx: Arc<ContentContext>,
}

#[pymethods]
impl AsyncContentContext {
    /// Create an async content context.
    #[classmethod]
    fn create(cls: &Bound<PyType>, config: &Config) -> PyResult<Py<PyAny>> {
        let py = cls.py();
        let config = Arc::new(config.0.clone());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(AsyncContentContext {
                ctx: Arc::new(ContentContext::try_new(config).map_err(ErrorNewType)?),
            })
        })
        .map(|b| b.unbind())
    }

    /// Get discussion topics list. Returns awaitable.
    fn topics(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.topics(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<TopicItem> { x.try_into() })
                .collect::<PyResult<Vec<TopicItem>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get news list. Returns awaitable.
    fn news(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.news(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<NewsItem> { x.try_into() })
                .collect::<PyResult<Vec<NewsItem>>>()
        })
        .map(|b| b.unbind())
    }
}
