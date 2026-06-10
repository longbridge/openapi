use std::sync::Arc;

use longbridge::blocking::FundamentalContextSync;
use pyo3::prelude::*;

use crate::{config::Config, error::ErrorNewType, fundamental::types::*};

/// Fundamental data context (synchronous).
#[pyclass]
pub(crate) struct FundamentalContext {
    ctx: FundamentalContextSync,
}

#[pymethods]
impl FundamentalContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: FundamentalContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get financial reports.
    ///
    /// `kind`: `FinancialReportKind` (default `All`)
    /// `period`: optional `FinancialReportPeriod`
    #[pyo3(signature = (symbol, kind = FinancialReportKind::All, period = None))]
    fn financial_report(
        &self,
        py: Python<'_>,
        symbol: String,
        kind: FinancialReportKind,
        period: Option<FinancialReportPeriod>,
    ) -> PyResult<FinancialReports> {
        let resp = self
            .ctx
            .financial_report(symbol, kind.into(), period.map(Into::into))
            .map_err(ErrorNewType)?;
        FinancialReports::from_lb(py, resp)
    }

    /// Get analyst ratings (latest snapshot + consensus summary).
    fn institution_rating(&self, symbol: String) -> PyResult<InstitutionRating> {
        Ok(self
            .ctx
            .institution_rating(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical analyst rating details.
    fn institution_rating_detail(&self, symbol: String) -> PyResult<InstitutionRatingDetail> {
        Ok(self
            .ctx
            .institution_rating_detail(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get dividend history.
    fn dividend(&self, symbol: String) -> PyResult<DividendList> {
        Ok(self.ctx.dividend(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get detailed dividend information.
    fn dividend_detail(&self, symbol: String) -> PyResult<DividendList> {
        Ok(self
            .ctx
            .dividend_detail(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get EPS forecasts.
    fn forecast_eps(&self, symbol: String) -> PyResult<ForecastEps> {
        Ok(self.ctx.forecast_eps(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get financial consensus estimates.
    fn consensus(&self, symbol: String) -> PyResult<FinancialConsensus> {
        Ok(self.ctx.consensus(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get valuation metrics (PE / PB / PS / dividend yield).
    fn valuation(&self, symbol: String) -> PyResult<ValuationData> {
        Ok(self.ctx.valuation(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get historical valuation data.
    fn valuation_history(&self, symbol: String) -> PyResult<ValuationHistoryResponse> {
        Ok(self
            .ctx
            .valuation_history(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get industry peer valuation comparison.
    fn industry_valuation(&self, symbol: String) -> PyResult<IndustryValuationList> {
        Ok(self
            .ctx
            .industry_valuation(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get industry valuation distribution.
    fn industry_valuation_dist(&self, symbol: String) -> PyResult<IndustryValuationDist> {
        Ok(self
            .ctx
            .industry_valuation_dist(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get company overview.
    fn company(&self, symbol: String) -> PyResult<CompanyOverview> {
        Ok(self.ctx.company(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get executive and board member information.
    fn executive(&self, symbol: String) -> PyResult<ExecutiveList> {
        Ok(self.ctx.executive(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get major shareholders.
    fn shareholder(&self, symbol: String) -> PyResult<ShareholderList> {
        Ok(self.ctx.shareholder(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get fund and ETF holders.
    fn fund_holder(&self, symbol: String) -> PyResult<FundHolders> {
        Ok(self.ctx.fund_holder(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get corporate actions.
    fn corp_action(&self, symbol: String) -> PyResult<CorpActions> {
        Ok(self.ctx.corp_action(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get investor relations data.
    fn invest_relation(&self, symbol: String) -> PyResult<InvestRelations> {
        Ok(self
            .ctx
            .invest_relation(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get operating metrics and financial report summaries.
    fn operating(&self, symbol: String) -> PyResult<OperatingList> {
        Ok(self.ctx.operating(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get buyback data for a security.
    fn buyback(&self, symbol: String) -> PyResult<BuybackData> {
        Ok(self.ctx.buyback(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get stock ratings for a security.
    fn ratings(&self, symbol: String) -> PyResult<StockRatings> {
        Ok(self.ctx.ratings(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get ranked list of top shareholders.
    fn shareholder_top(&self, symbol: String) -> PyResult<ShareholderTopResponse> {
        Ok(self
            .ctx
            .shareholder_top(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get holding history and detail for one shareholder.
    fn shareholder_detail(
        &self,
        symbol: String,
        object_id: i64,
    ) -> PyResult<ShareholderDetailResponse> {
        Ok(self
            .ctx
            .shareholder_detail(symbol, object_id)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get valuation comparison between a security and optional peers.
    #[pyo3(signature = (symbol, currency, comparison_symbols = None))]
    fn valuation_comparison(
        &self,
        symbol: String,
        currency: String,
        comparison_symbols: Option<Vec<String>>,
    ) -> PyResult<ValuationComparisonResponse> {
        Ok(self
            .ctx
            .valuation_comparison(symbol, currency, comparison_symbols)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get ETF asset allocation (holdings / regional / asset class /
    /// industry).
    fn etf_asset_allocation(&self, symbol: String) -> PyResult<AssetAllocationResponse> {
        Ok(self
            .ctx
            .etf_asset_allocation(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// List macroeconomic indicators.
    fn macrodata_indicators(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<EconomicIndicatorInfo>> {
        Ok(self
            .ctx
            .macrodata_indicators(offset, limit)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Get historical data for a macroeconomic indicator.
    fn macrodata(
        &self,
        indicator_code: String,
        start_date: Option<String>,
        end_date: Option<String>,
        limit: Option<i32>,
    ) -> PyResult<EconomicIndicatorResponse> {
        Ok(self
            .ctx
            .macrodata(indicator_code, start_date, end_date, limit)
            .map_err(ErrorNewType)?
            .into())
    }
}
