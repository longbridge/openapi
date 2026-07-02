use std::sync::Arc;

use longbridge::FundamentalContext;
use pyo3::{prelude::*, types::PyType};
// needed for us_financial_overview / us_key_financial_metrics / us_analyst_consensus
#[allow(unused_imports)]
use pythonize;

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

    /// Get ranked list of top shareholders. Returns awaitable.
    fn shareholder_top(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ShareholderTopResponse::from(
                ctx.shareholder_top(symbol).await.map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get holding history and detail for one shareholder. Returns awaitable.
    fn shareholder_detail(
        &self,
        py: Python<'_>,
        symbol: String,
        object_id: i64,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ShareholderDetailResponse::from(
                ctx.shareholder_detail(symbol, object_id)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get valuation comparison between a security and optional peers. Returns
    /// awaitable.
    #[pyo3(signature = (symbol, currency, comparison_symbols = None))]
    fn valuation_comparison(
        &self,
        py: Python<'_>,
        symbol: String,
        currency: String,
        comparison_symbols: Option<Vec<String>>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ValuationComparisonResponse::from(
                ctx.valuation_comparison(symbol, currency, comparison_symbols)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get ETF asset allocation (holdings / regional / asset class /
    /// industry). Returns awaitable.
    fn etf_asset_allocation(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(AssetAllocationResponse::from(
                ctx.etf_asset_allocation(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// List macroeconomic indicators. Returns awaitable.
    fn macroeconomic_indicators(
        &self,
        py: Python<'_>,
        country: Option<MacroeconomicCountry>,
        keyword: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(MacroeconomicIndicatorListResponse::from(
                ctx.macroeconomic_indicators(country.map(Into::into), keyword, offset, limit)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get historical data for a macroeconomic indicator. Returns awaitable.
    fn macroeconomic(
        &self,
        py: Python<'_>,
        indicator_code: String,
        start_date: Option<String>,
        end_date: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(MacroeconomicResponse::from(
                ctx.macroeconomic(indicator_code, start_date, end_date, offset, limit)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    // ── US-market async methods ───────────────────────────────────────────────

    /// Get US company overview. US token required. Returns awaitable.
    fn us_company_overview(&self, py: Python<'_>, counter_id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(USCompanyOverview::from(
                ctx.us_company_overview(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get US valuation overview. US token required. Returns awaitable.
    fn us_valuation_overview(&self, py: Python<'_>, counter_id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(USValuationOverview::from(
                ctx.us_valuation_overview(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get US financial overview (raw dict). Returns awaitable.
    fn us_financial_overview(
        &self,
        py: Python<'_>,
        counter_id: String,
        report: String,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .us_financial_overview(counter_id, report)
                .await
                .map_err(ErrorNewType)?;
            Python::attach(|py| {
                pythonize::pythonize(py, &v)
                    .map(|b| b.unbind())
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
            })
        })
        .map(|b| b.unbind())
    }

    /// Get US financial statement v3. Returns awaitable.
    fn us_financial_statement_v3(
        &self,
        py: Python<'_>,
        counter_id: String,
        kind: String,
        report: String,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(USFinancialStatement::from(
                ctx.us_financial_statement_v3(counter_id, kind, report)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get US key financial metrics (raw dict). Returns awaitable.
    fn us_key_financial_metrics(
        &self,
        py: Python<'_>,
        counter_id: String,
        report: String,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .us_key_financial_metrics(counter_id, report)
                .await
                .map_err(ErrorNewType)?;
            Python::attach(|py| {
                pythonize::pythonize(py, &v)
                    .map(|b| b.unbind())
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
            })
        })
        .map(|b| b.unbind())
    }

    /// Get US analyst consensus (raw dict). Returns awaitable.
    fn us_analyst_consensus(
        &self,
        py: Python<'_>,
        counter_id: String,
        report: String,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .us_analyst_consensus(counter_id, report)
                .await
                .map_err(ErrorNewType)?;
            Python::attach(|py| {
                pythonize::pythonize(py, &v)
                    .map(|b| b.unbind())
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
            })
        })
        .map(|b| b.unbind())
    }

    /// Get US ETF dividend info. Returns awaitable.
    fn us_etf_dividend_info(&self, py: Python<'_>, counter_id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(USETFDividendInfo::from(
                ctx.us_etf_dividend_info(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get US company dividends. Returns awaitable.
    fn us_company_dividends(&self, py: Python<'_>, counter_id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(USCompanyDividends::from(
                ctx.us_company_dividends(symbol)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }

    /// Get US ETF files. Returns awaitable.
    fn us_etf_files(
        &self,
        py: Python<'_>,
        counter_id: String,
        size: Option<u32>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(USETFFilesResponse::from(
                ctx.us_etf_files(counter_id, size)
                    .await
                    .map_err(ErrorNewType)?,
            ))
        })
        .map(|b| b.unbind())
    }
}
