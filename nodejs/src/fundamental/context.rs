use std::sync::Arc;

use napi::Result;

use crate::{config::Config, error::ErrorNewType, fundamental::types::*};

/// Fundamental data context
#[napi_derive::napi]
#[derive(Clone)]
pub struct FundamentalContext {
    ctx: longbridge::FundamentalContext,
}

#[napi_derive::napi]
impl FundamentalContext {
    /// Create a new `FundamentalContext`
    #[napi]
    pub fn new(config: &Config) -> FundamentalContext {
        Self {
            ctx: longbridge::FundamentalContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get financial reports
    #[napi]
    pub async fn financial_report(
        &self,
        symbol: String,
        kind: FinancialReportKind,
        period: Option<FinancialReportPeriod>,
    ) -> Result<FinancialReports> {
        Ok(self
            .ctx
            .financial_report(symbol, kind.into(), period.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get analyst ratings (latest + consensus summary)
    #[napi]
    pub async fn institution_rating(&self, symbol: String) -> Result<InstitutionRating> {
        Ok(self
            .ctx
            .institution_rating(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical analyst rating details
    #[napi]
    pub async fn institution_rating_detail(
        &self,
        symbol: String,
    ) -> Result<InstitutionRatingDetail> {
        Ok(self
            .ctx
            .institution_rating_detail(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get dividend history
    #[napi]
    pub async fn dividend(&self, symbol: String) -> Result<DividendList> {
        Ok(self
            .ctx
            .dividend(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get detailed dividend information
    #[napi]
    pub async fn dividend_detail(&self, symbol: String) -> Result<DividendList> {
        Ok(self
            .ctx
            .dividend_detail(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get EPS forecasts
    #[napi]
    pub async fn forecast_eps(&self, symbol: String) -> Result<ForecastEps> {
        Ok(self
            .ctx
            .forecast_eps(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get financial consensus estimates
    #[napi]
    pub async fn consensus(&self, symbol: String) -> Result<FinancialConsensus> {
        Ok(self
            .ctx
            .consensus(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get valuation metrics (PE / PB / PS / dividend yield)
    #[napi]
    pub async fn valuation(&self, symbol: String) -> Result<ValuationData> {
        Ok(self
            .ctx
            .valuation(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical valuation data
    #[napi]
    pub async fn valuation_history(&self, symbol: String) -> Result<ValuationHistoryResponse> {
        Ok(self
            .ctx
            .valuation_history(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get industry peer valuation comparison
    #[napi]
    pub async fn industry_valuation(&self, symbol: String) -> Result<IndustryValuationList> {
        Ok(self
            .ctx
            .industry_valuation(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get industry valuation distribution
    #[napi]
    pub async fn industry_valuation_dist(&self, symbol: String) -> Result<IndustryValuationDist> {
        Ok(self
            .ctx
            .industry_valuation_dist(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get company overview
    #[napi]
    pub async fn company(&self, symbol: String) -> Result<CompanyOverview> {
        Ok(self.ctx.company(symbol).await.map_err(ErrorNewType)?.into())
    }

    /// Get executive and board member information
    #[napi]
    pub async fn executive(&self, symbol: String) -> Result<ExecutiveList> {
        Ok(self
            .ctx
            .executive(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get major shareholders
    #[napi]
    pub async fn shareholder(&self, symbol: String) -> Result<ShareholderList> {
        Ok(self
            .ctx
            .shareholder(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get fund and ETF holders
    #[napi]
    pub async fn fund_holder(&self, symbol: String) -> Result<FundHolders> {
        Ok(self
            .ctx
            .fund_holder(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get corporate actions
    #[napi]
    pub async fn corp_action(&self, symbol: String) -> Result<CorpActions> {
        Ok(self
            .ctx
            .corp_action(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get investor relations data
    #[napi]
    pub async fn invest_relation(&self, symbol: String) -> Result<InvestRelations> {
        Ok(self
            .ctx
            .invest_relation(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get operating metrics and financial report summaries
    #[napi]
    pub async fn operating(&self, symbol: String) -> Result<OperatingList> {
        Ok(self
            .ctx
            .operating(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get buyback data for a security
    #[napi]
    pub async fn buyback(&self, symbol: String) -> Result<BuybackData> {
        Ok(self.ctx.buyback(symbol).await.map_err(ErrorNewType)?.into())
    }

    /// Get stock ratings for a security
    #[napi]
    pub async fn ratings(&self, symbol: String) -> Result<StockRatings> {
        Ok(self.ctx.ratings(symbol).await.map_err(ErrorNewType)?.into())
    }

    /// Get business segment breakdowns (latest snapshot)
    #[napi]
    pub async fn business_segments(&self, symbol: String) -> Result<BusinessSegments> {
        Ok(self
            .ctx
            .business_segments(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical business segment breakdowns
    #[napi]
    pub async fn business_segments_history(
        &self,
        symbol: String,
        report: Option<String>,
        cate: Option<String>,
    ) -> Result<BusinessSegmentsHistory> {
        let report_static: Option<&'static str> = match report.as_deref() {
            Some("qf") => Some("qf"),
            Some("saf") => Some("saf"),
            Some("af") => Some("af"),
            _ => None,
        };
        Ok(self
            .ctx
            .business_segments_history(symbol, report_static, cate)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical institutional rating view time-series
    #[napi]
    pub async fn institution_rating_views(&self, symbol: String) -> Result<InstitutionRatingViews> {
        Ok(self
            .ctx
            .institution_rating_views(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get industry rank for a market
    #[napi]
    pub async fn industry_rank(
        &self,
        market: String,
        indicator: String,
        sort_type: String,
        limit: u32,
    ) -> Result<IndustryRankResponse> {
        Ok(self
            .ctx
            .industry_rank(market, indicator, sort_type, limit)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the industry peer chain for a security or industry
    #[napi]
    pub async fn industry_peers(
        &self,
        counter_id: String,
        market: String,
        industry_id: Option<String>,
    ) -> Result<IndustryPeersResponse> {
        Ok(self
            .ctx
            .industry_peers(counter_id, market, industry_id)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get a financial report snapshot (earnings snapshot)
    #[napi]
    pub async fn financial_report_snapshot(
        &self,
        symbol: String,
        report: Option<String>,
        fiscal_year: Option<i32>,
        fiscal_period: Option<String>,
    ) -> Result<FinancialReportSnapshot> {
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
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
