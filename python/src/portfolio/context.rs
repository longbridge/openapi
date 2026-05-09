use std::sync::Arc;

use longbridge::blocking::PortfolioContextSync;
use pyo3::prelude::*;

use crate::{config::Config, error::ErrorNewType, portfolio::types::*};

/// Portfolio analytics context (synchronous).
#[pyclass]
pub(crate) struct PortfolioContext {
    ctx: PortfolioContextSync,
}

#[pymethods]
impl PortfolioContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: PortfolioContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get exchange rates.
    fn exchange_rate(&self) -> PyResult<ExchangeRates> {
        Ok(self.ctx.exchange_rate().map_err(ErrorNewType)?.into())
    }

    /// Get portfolio P&L analysis.
    #[pyo3(signature = (start = None, end = None))]
    fn profit_analysis(
        &self,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<ProfitAnalysis> {
        Ok(self
            .ctx
            .profit_analysis(start, end)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get P&L detail for a specific security.
    #[pyo3(signature = (symbol, start = None, end = None))]
    fn profit_analysis_detail(
        &self,
        symbol: String,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<ProfitAnalysisDetail> {
        Ok(self
            .ctx
            .profit_analysis_detail(symbol, start, end)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get paginated P&L analysis filtered by market.
    #[pyo3(signature = (page = 1, size = 20, market = None, start = None, end = None, currency = None))]
    fn profit_analysis_by_market(
        &self,
        page: u32,
        size: u32,
        market: Option<String>,
        start: Option<String>,
        end: Option<String>,
        currency: Option<String>,
    ) -> PyResult<ProfitAnalysisByMarket> {
        Ok(self
            .ctx
            .profit_analysis_by_market(market, start, end, currency, page, size)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get paginated P&L flow records for a security.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (symbol, page, size, derivative, start = None, end = None))]
    fn profit_analysis_flows(
        &self,
        symbol: String,
        page: u32,
        size: u32,
        derivative: bool,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<ProfitAnalysisFlows> {
        Ok(self
            .ctx
            .profit_analysis_flows(symbol, page, size, derivative, start, end)
            .map_err(ErrorNewType)?
            .into())
    }
}
