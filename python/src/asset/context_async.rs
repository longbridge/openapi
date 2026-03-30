use std::sync::Arc;

use longbridge::asset::AssetContext;
use pyo3::{prelude::*, types::PyType};

use crate::{
    asset::types::{GetStatementListResponse, GetStatementResponse, StatementType},
    config::Config,
    error::ErrorNewType,
};

/// Async asset context.
#[pyclass]
pub(crate) struct AsyncAssetContext {
    ctx: Arc<AssetContext>,
}

#[pymethods]
impl AsyncAssetContext {
    /// Create an async asset context.
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        AsyncAssetContext {
            ctx: Arc::new(AssetContext::new(Arc::new(config.0.clone()))),
        }
    }

    /// Get statement data list. Returns awaitable.
    #[pyo3(signature = (statement_type, start_date = 1, limit = 20))]
    fn statements(
        &self,
        py: Python<'_>,
        statement_type: StatementType,
        start_date: i32,
        limit: i32,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let opts = longbridge::asset::GetStatementListOptions::new(statement_type.into())
                .page(start_date)
                .page_size(limit);
            let resp = ctx.statements(opts).await.map_err(ErrorNewType)?;
            Ok(GetStatementListResponse::from(resp))
        })
        .map(|b| b.unbind())
    }

    /// Get statement data download URL. Returns awaitable.
    fn statement_download_url(&self, py: Python<'_>, file_key: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let opts = longbridge::asset::GetStatementOptions::new(file_key);
            let resp = ctx
                .statement_download_url(opts)
                .await
                .map_err(ErrorNewType)?;
            Ok(GetStatementResponse::from(resp))
        })
        .map(|b| b.unbind())
    }
}
