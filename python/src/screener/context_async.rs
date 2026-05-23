use std::sync::Arc;

use longbridge::ScreenerContext;
use pyo3::{prelude::*, types::PyType};

use crate::{config::Config, error::ErrorNewType, screener::types::*};

/// Screener context (async).
#[pyclass]
pub(crate) struct AsyncScreenerContext {
    ctx: Arc<ScreenerContext>,
}

#[pymethods]
impl AsyncScreenerContext {
    /// Create an async screener context.
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        Self {
            ctx: Arc::new(ScreenerContext::new(Arc::new(config.0.clone()))),
        }
    }

    /// Get recommended built-in screener strategies. Returns awaitable.
    fn screener_recommend_strategies(&self, py: Python<'_>, market: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ScreenerRecommendStrategiesResponse::from(
                ctx.screener_recommend_strategies(market)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get the current user's saved screener strategies. Returns awaitable.
    fn screener_user_strategies(&self, py: Python<'_>, market: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ScreenerUserStrategiesResponse::from(
                ctx.screener_user_strategies(market)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get detail for one screener strategy by ID. Returns awaitable.
    fn screener_strategy(&self, py: Python<'_>, id: i64) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ScreenerStrategyResponse::from(
                ctx.screener_strategy(id).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Search / screen securities using a strategy. Returns awaitable.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (market, strategy_id = None, conditions = vec![], show = vec![], page = 0, size = 20))]
    fn screener_search(
        &self,
        py: Python<'_>,
        market: String,
        strategy_id: Option<i64>,
        conditions: Vec<ScreenerCondition>,
        show: Vec<String>,
        page: u32,
        size: u32,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let lb_conditions: Vec<longbridge::screener::ScreenerCondition> =
            conditions.into_iter().map(Into::into).collect();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ScreenerSearchResponse::from(
                ctx.screener_search(market, strategy_id, lb_conditions, show, page, size)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get all available screener indicator definitions. Returns awaitable.
    fn screener_indicators(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ScreenerIndicatorsResponse::from(
                ctx.screener_indicators().await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
}
