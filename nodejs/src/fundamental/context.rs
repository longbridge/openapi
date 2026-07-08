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

    /// Get ranked list of top shareholders
    #[napi]
    pub async fn shareholder_top(&self, symbol: String) -> Result<ShareholderTopResponse> {
        Ok(self
            .ctx
            .shareholder_top(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get holding history and detail for one shareholder
    #[napi]
    pub async fn shareholder_detail(
        &self,
        symbol: String,
        object_id: i64,
    ) -> Result<ShareholderDetailResponse> {
        Ok(self
            .ctx
            .shareholder_detail(symbol, object_id)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get valuation comparison between a security and optional peers
    #[napi]
    pub async fn valuation_comparison(
        &self,
        symbol: String,
        currency: String,
        comparison_symbols: Option<Vec<String>>,
    ) -> Result<ValuationComparisonResponse> {
        Ok(self
            .ctx
            .valuation_comparison(symbol, currency, comparison_symbols)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get ETF asset allocation (holdings / regional / asset class /
    /// industry)
    #[napi]
    pub async fn etf_asset_allocation(&self, symbol: String) -> Result<AssetAllocationResponse> {
        Ok(self
            .ctx
            .etf_asset_allocation(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// List macroeconomic indicators
    #[napi]
    pub async fn macroeconomic_indicators(
        &self,
        country: Option<MacroeconomicCountry>,
        keyword: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MacroeconomicIndicatorListResponse> {
        Ok(self
            .ctx
            .macroeconomic_indicators(country.map(Into::into), keyword, offset, limit)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get historical data for a macroeconomic indicator
    #[napi]
    pub async fn macroeconomic(
        &self,
        indicator_code: String,
        start_date: Option<String>,
        end_date: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MacroeconomicResponse> {
        Ok(self
            .ctx
            .macroeconomic(indicator_code, start_date, end_date, offset, limit)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    // ── US-market methods ─────────────────────────────────────────────────────

    /// Get US company overview. US token required.
    #[napi]
    pub async fn us_company_overview(&self, symbol: String) -> Result<USCompanyOverview> {
        Ok(self
            .ctx
            .us_company_overview(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US valuation overview. US token required.
    #[napi]
    pub async fn us_valuation_overview(&self, symbol: String) -> Result<USValuationOverview> {
        Ok(self
            .ctx
            .us_valuation_overview(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US financial overview. US token required.
    #[napi]
    pub async fn us_financial_overview(
        &self,
        symbol: String,
        report: String,
    ) -> Result<USFinancialOverview> {
        Ok(self
            .ctx
            .us_financial_overview(symbol, report)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US financial statement v3. kind: "IS"/"BS"/"CF". US token required.
    #[napi]
    pub async fn us_financial_statement_v3(
        &self,
        symbol: String,
        kind: String,
        report: String,
    ) -> Result<USFinancialStatement> {
        Ok(self
            .ctx
            .us_financial_statement_v3(symbol, kind, report)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US key financial metrics. US token required.
    #[napi]
    pub async fn us_key_financial_metrics(
        &self,
        symbol: String,
        report: String,
    ) -> Result<USKeyFinancialMetrics> {
        Ok(self
            .ctx
            .us_key_financial_metrics(symbol, report)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US analyst consensus estimates. US token required.
    #[napi]
    pub async fn us_analyst_consensus(
        &self,
        symbol: String,
        report: String,
    ) -> Result<USAnalystConsensus> {
        Ok(self
            .ctx
            .us_analyst_consensus(symbol, report)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US ETF dividend history. US token required.
    #[napi]
    pub async fn us_etf_dividend_info(&self, symbol: String) -> Result<USETFDividendInfo> {
        Ok(self
            .ctx
            .us_etf_dividend_info(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US company dividends. US token required.
    #[napi]
    pub async fn us_company_dividends(&self, symbol: String) -> Result<USCompanyDividends> {
        Ok(self
            .ctx
            .us_company_dividends(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get US ETF document list. size=None returns all. US token required.
    #[napi]
    pub async fn us_etf_files(
        &self,
        symbol: String,
        size: Option<u32>,
    ) -> Result<USETFFilesResponse> {
        Ok(self
            .ctx
            .us_etf_files(symbol, size)
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
