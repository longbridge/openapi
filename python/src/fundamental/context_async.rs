use std::sync::Arc;

use longbridge::FundamentalContext;
use pyo3::{prelude::*, types::PyType};

use crate::{config::Config, error::ErrorNewType, fundamental::types::*};

/// Fundamental data context (async).
#[pyclass]
pub(crate) struct AsyncFundamentalContext {
    ctx: Arc<FundamentalContext>,
}

#[pymethods]
impl AsyncFundamentalContext {
    /// Create an async fundamental context.
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        AsyncFundamentalContext {
            ctx: Arc::new(FundamentalContext::new(Arc::new(config.0.clone()))),
        }
    }

    /// Get financial reports. Returns awaitable.
    #[pyo3(signature = (symbol, kind = FinancialReportKind::All, period = None))]
    fn financial_report(
        &self,
        py: Python<'_>,
        symbol: String,
        kind: FinancialReportKind,
        period: Option<FinancialReportPeriod>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = ctx
                .financial_report(symbol, kind.into(), period.map(Into::into))
                .await
                .map_err(ErrorNewType)?;
            Python::attach(|py| FinancialReports::from_lb(py, resp))
        })
        .map(|b| b.unbind())
    }

    /// Get analyst ratings. Returns awaitable.
    fn institution_rating(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(InstitutionRating::from(
                ctx.institution_rating(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get historical analyst rating details. Returns awaitable.
    fn institution_rating_detail(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(InstitutionRatingDetail::from(
                ctx.institution_rating_detail(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get dividend history. Returns awaitable.
    fn dividend(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(DividendList::from(
                ctx.dividend(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get detailed dividend information. Returns awaitable.
    fn dividend_detail(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(DividendList::from(
                ctx.dividend_detail(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get EPS forecasts. Returns awaitable.
    fn forecast_eps(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ForecastEps::from(
                ctx.forecast_eps(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get financial consensus estimates. Returns awaitable.
    fn consensus(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(FinancialConsensus::from(
                ctx.consensus(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get valuation metrics. Returns awaitable.
    fn valuation(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ValuationData::from(
                ctx.valuation(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get historical valuation data. Returns awaitable.
    fn valuation_history(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ValuationHistoryResponse::from(
                ctx.valuation_history(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get industry peer valuation comparison. Returns awaitable.
    fn industry_valuation(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(IndustryValuationList::from(
                ctx.industry_valuation(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get industry valuation distribution. Returns awaitable.
    fn industry_valuation_dist(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(IndustryValuationDist::from(
                ctx.industry_valuation_dist(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get company overview. Returns awaitable.
    fn company(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(CompanyOverview::from(
                ctx.company(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get executive and board member information. Returns awaitable.
    fn executive(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ExecutiveList::from(
                ctx.executive(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get major shareholders. Returns awaitable.
    fn shareholder(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ShareholderList::from(
                ctx.shareholder(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get fund and ETF holders. Returns awaitable.
    fn fund_holder(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(FundHolders::from(
                ctx.fund_holder(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get corporate actions. Returns awaitable.
    fn corp_action(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(CorpActions::from(
                ctx.corp_action(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get investor relations data. Returns awaitable.
    fn invest_relation(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(InvestRelations::from(
                ctx.invest_relation(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get operating metrics. Returns awaitable.
    fn operating(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(OperatingList::from(
                ctx.operating(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get buyback data. Returns awaitable.
    fn buyback(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(BuybackData::from(
                ctx.buyback(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get stock ratings. Returns awaitable.
    fn ratings(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(StockRatings::from(
                ctx.ratings(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get business segment breakdowns. Returns awaitable.
    fn business_segments(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(BusinessSegments::from(
                ctx.business_segments(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get historical business segment breakdowns. Returns awaitable.
    #[pyo3(signature = (symbol, report = None, cate = None))]
    fn business_segments_history(
        &self,
        py: Python<'_>,
        symbol: String,
        report: Option<String>,
        cate: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let report_static: Option<&'static str> = match report.as_deref() {
            Some("qf") => Some("qf"),
            Some("saf") => Some("saf"),
            Some("af") => Some("af"),
            _ => None,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(BusinessSegmentsHistory::from(
                ctx.business_segments_history(symbol, report_static, cate)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get historical institutional rating view time-series. Returns awaitable.
    fn institution_rating_views(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(InstitutionRatingViews::from(
                ctx.institution_rating_views(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get industry rank for a market. Returns awaitable.
    fn industry_rank(
        &self,
        py: Python<'_>,
        market: String,
        indicator: String,
        sort_type: String,
        limit: u32,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(IndustryRankResponse::from(
                ctx.industry_rank(market, indicator, sort_type, limit)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get the industry peer chain for a security or industry. Returns
    /// awaitable.
    #[pyo3(signature = (counter_id, market, industry_id = None))]
    fn industry_peers(
        &self,
        py: Python<'_>,
        counter_id: String,
        market: String,
        industry_id: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(IndustryPeersResponse::from(
                ctx.industry_peers(counter_id, market, industry_id)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get a financial report snapshot. Returns awaitable.
    #[pyo3(signature = (symbol, report = None, fiscal_year = None, fiscal_period = None))]
    fn financial_report_snapshot(
        &self,
        py: Python<'_>,
        symbol: String,
        report: Option<String>,
        fiscal_year: Option<i32>,
        fiscal_period: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let report_static: Option<&'static str> = match report.as_deref() {
            Some("qf") => Some("qf"),
            Some("saf") => Some("saf"),
            Some("af") => Some("af"),
            _ => None,
        };
        let fiscal_period_static: Option<&'static str> = match fiscal_period.as_deref() {
            Some("q1") => Some("q1"),
            Some("q2") => Some("q2"),
            Some("q3") => Some("q3"),
            Some("q4") => Some("q4"),
            Some("fy") => Some("fy"),
            Some("h1") => Some("h1"),
            Some("h2") => Some("h2"),
            _ => None,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(FinancialReportSnapshot::from(
                ctx.financial_report_snapshot(
                    symbol,
                    report_static,
                    fiscal_year,
                    fiscal_period_static,
                )
                .await
                .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
}
