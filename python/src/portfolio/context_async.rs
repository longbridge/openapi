use std::sync::Arc;

use longbridge::PortfolioContext;
use pyo3::{prelude::*, types::PyType};

use crate::{config::Config, error::ErrorNewType, portfolio::types::*};

/// Portfolio analytics context (async).
#[pyclass]
pub(crate) struct AsyncPortfolioContext {
    ctx: Arc<PortfolioContext>,
}

#[pymethods]
impl AsyncPortfolioContext {
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        Self {
            ctx: Arc::new(PortfolioContext::new(Arc::new(config.0.clone()))),
        }
    }
    fn exchange_rate(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ExchangeRates::from(
                ctx.exchange_rate().await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
    #[pyo3(signature = (start = None, end = None))]
    fn profit_analysis(
        &self,
        py: Python<'_>,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ProfitAnalysis::from(
                ctx.profit_analysis(start, end)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
    #[pyo3(signature = (symbol, start = None, end = None))]
    fn profit_analysis_detail(
        &self,
        py: Python<'_>,
        symbol: String,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ProfitAnalysisDetail::from(
                ctx.profit_analysis_detail(symbol, start, end)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (symbol, page, size, derivative, start = None, end = None))]
    fn profit_analysis_flows(
        &self,
        py: Python<'_>,
        symbol: String,
        page: u32,
        size: u32,
        derivative: bool,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ProfitAnalysisFlows::from(
                ctx.profit_analysis_flows(symbol, page, size, derivative, start, end)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
}
