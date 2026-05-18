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

    /// Get business segment breakdowns (latest snapshot).
    fn business_segments(&self, symbol: String) -> PyResult<BusinessSegments> {
        Ok(self
            .ctx
            .business_segments(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical business segment breakdowns.
    #[pyo3(signature = (symbol, report = None, cate = None))]
    fn business_segments_history(
        &self,
        symbol: String,
        report: Option<String>,
        cate: Option<String>,
    ) -> PyResult<BusinessSegmentsHistory> {
        let report_static: Option<&'static str> = match report.as_deref() {
            Some("qf") => Some("qf"),
            Some("saf") => Some("saf"),
            Some("af") => Some("af"),
            _ => None,
        };
        Ok(self
            .ctx
            .business_segments_history(symbol, report_static, cate)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical institutional rating view time-series.
    fn institution_rating_views(&self, symbol: String) -> PyResult<InstitutionRatingViews> {
        Ok(self
            .ctx
            .institution_rating_views(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get industry rank for a market.
    fn industry_rank(
        &self,
        market: String,
        indicator: String,
        sort_type: String,
        limit: u32,
    ) -> PyResult<IndustryRankResponse> {
        Ok(self
            .ctx
            .industry_rank(market, indicator, sort_type, limit)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the industry peer chain for a security or industry.
    #[pyo3(signature = (counter_id, market, industry_id = None))]
    fn industry_peers(
        &self,
        counter_id: String,
        market: String,
        industry_id: Option<String>,
    ) -> PyResult<IndustryPeersResponse> {
        Ok(self
            .ctx
            .industry_peers(counter_id, market, industry_id)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get a financial report snapshot (earnings snapshot).
    #[pyo3(signature = (symbol, report = None, fiscal_year = None, fiscal_period = None))]
    fn financial_report_snapshot(
        &self,
        symbol: String,
        report: Option<String>,
        fiscal_year: Option<i32>,
        fiscal_period: Option<String>,
    ) -> PyResult<FinancialReportSnapshot> {
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
        Ok(self
            .ctx
            .financial_report_snapshot(symbol, report_static, fiscal_year, fiscal_period_static)
            .map_err(ErrorNewType)?
            .into())
    }
}
