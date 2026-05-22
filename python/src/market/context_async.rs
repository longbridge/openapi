use std::sync::Arc;

use longbridge::MarketContext;
use pyo3::{prelude::*, types::PyType};

use crate::{config::Config, error::ErrorNewType, market::types::*};

/// Market data context (async).
#[pyclass]
pub(crate) struct AsyncMarketContext {
    ctx: Arc<MarketContext>,
}

#[pymethods]
impl AsyncMarketContext {
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        Self {
            ctx: Arc::new(MarketContext::new(Arc::new(config.0.clone()))),
        }
    }

    fn market_status(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(MarketStatusResponse::from(
                ctx.market_status().await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    #[pyo3(signature = (symbol, period = BrokerHoldingPeriod::Rct1))]
    fn broker_holding(
        &self,
        py: Python<'_>,
        symbol: String,
        period: BrokerHoldingPeriod,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(BrokerHoldingTop::from(
                ctx.broker_holding(symbol, period.into())
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    fn broker_holding_detail(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(BrokerHoldingDetail::from(
                ctx.broker_holding_detail(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    fn broker_holding_daily(
        &self,
        py: Python<'_>,
        symbol: String,
        broker_id: String,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(BrokerHoldingDailyHistory::from(
                ctx.broker_holding_daily(symbol, broker_id)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    #[pyo3(signature = (symbol, period = AhPremiumPeriod::Day, count = 100))]
    fn ah_premium(
        &self,
        py: Python<'_>,
        symbol: String,
        period: AhPremiumPeriod,
        count: u32,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(AhPremiumKlines::from(
                ctx.ah_premium(symbol, period.into(), count)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    fn ah_premium_intraday(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(AhPremiumIntraday::from(
                ctx.ah_premium_intraday(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    fn trade_stats(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(TradeStatsResponse::from(
                ctx.trade_stats(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    fn anomaly(&self, py: Python<'_>, market: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(AnomalyResponse::from(
                ctx.anomaly(market).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    fn constituent(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(IndexConstituents::from(
                ctx.constituent(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get top movers (stocks with unusual price movements) across one or more
    /// markets. Returns awaitable.
    #[pyo3(signature = (markets, sort = 0, date = None, limit = 20))]
    fn top_movers(
        &self,
        py: Python<'_>,
        markets: Vec<String>,
        sort: u32,
        date: Option<String>,
        limit: u32,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(TopMoversResponse::from(
                ctx.top_movers(markets, sort, date, limit)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get all available rank category keys and labels. Returns awaitable.
    fn rank_categories(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(RankCategoriesResponse::from(
                ctx.rank_categories().await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get a ranked list of securities for the given category key. Returns
    /// awaitable.
    #[pyo3(signature = (key, need_article = false))]
    fn rank_list(&self, py: Python<'_>, key: String, need_article: bool) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(RankListResponse::from(
                ctx.rank_list(key, need_article)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
}
