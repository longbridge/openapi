use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    fundamental::{FundamentalContext, types::*},
};

/// Blocking fundamental data context
pub struct FundamentalContextSync {
    rt: BlockingRuntime<FundamentalContext>,
}

impl FundamentalContextSync {
    /// Create a [`FundamentalContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = FundamentalContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get financial reports
    pub fn financial_report(
        &self,
        symbol: impl Into<String> + Send + 'static,
        kind: FinancialReportKind,
        period: Option<FinancialReportPeriod>,
    ) -> Result<FinancialReports> {
        self.rt
            .call(move |ctx| async move { ctx.financial_report(symbol, kind, period).await })
    }

    /// Get analyst ratings (latest + summary)
    pub fn institution_rating(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<InstitutionRating> {
        self.rt
            .call(move |ctx| async move { ctx.institution_rating(symbol).await })
    }

    /// Get historical analyst rating details
    pub fn institution_rating_detail(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<InstitutionRatingDetail> {
        self.rt
            .call(move |ctx| async move { ctx.institution_rating_detail(symbol).await })
    }

    /// Get dividend history
    pub fn dividend(&self, symbol: impl Into<String> + Send + 'static) -> Result<DividendList> {
        self.rt
            .call(move |ctx| async move { ctx.dividend(symbol).await })
    }

    /// Get detailed dividend information
    pub fn dividend_detail(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<DividendList> {
        self.rt
            .call(move |ctx| async move { ctx.dividend_detail(symbol).await })
    }

    /// Get EPS forecasts
    pub fn forecast_eps(&self, symbol: impl Into<String> + Send + 'static) -> Result<ForecastEps> {
        self.rt
            .call(move |ctx| async move { ctx.forecast_eps(symbol).await })
    }

    /// Get financial consensus estimates
    pub fn consensus(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<FinancialConsensus> {
        self.rt
            .call(move |ctx| async move { ctx.consensus(symbol).await })
    }

    /// Get valuation metrics
    pub fn valuation(&self, symbol: impl Into<String> + Send + 'static) -> Result<ValuationData> {
        self.rt
            .call(move |ctx| async move { ctx.valuation(symbol).await })
    }

    /// Get historical valuation data
    pub fn valuation_history(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<ValuationHistoryResponse> {
        self.rt
            .call(move |ctx| async move { ctx.valuation_history(symbol).await })
    }

    /// Get industry peer valuation comparison
    pub fn industry_valuation(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<IndustryValuationList> {
        self.rt
            .call(move |ctx| async move { ctx.industry_valuation(symbol).await })
    }

    /// Get industry valuation distribution
    pub fn industry_valuation_dist(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<IndustryValuationDist> {
        self.rt
            .call(move |ctx| async move { ctx.industry_valuation_dist(symbol).await })
    }

    /// Get company overview
    pub fn company(&self, symbol: impl Into<String> + Send + 'static) -> Result<CompanyOverview> {
        self.rt
            .call(move |ctx| async move { ctx.company(symbol).await })
    }

    /// Get executive and board member information
    pub fn executive(&self, symbol: impl Into<String> + Send + 'static) -> Result<ExecutiveList> {
        self.rt
            .call(move |ctx| async move { ctx.executive(symbol).await })
    }

    /// Get major shareholders
    pub fn shareholder(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<ShareholderList> {
        self.rt
            .call(move |ctx| async move { ctx.shareholder(symbol).await })
    }

    /// Get fund and ETF holders
    pub fn fund_holder(&self, symbol: impl Into<String> + Send + 'static) -> Result<FundHolders> {
        self.rt
            .call(move |ctx| async move { ctx.fund_holder(symbol).await })
    }

    /// Get corporate actions
    pub fn corp_action(&self, symbol: impl Into<String> + Send + 'static) -> Result<CorpActions> {
        self.rt
            .call(move |ctx| async move { ctx.corp_action(symbol).await })
    }

    /// Get investor relations data
    pub fn invest_relation(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<InvestRelations> {
        self.rt
            .call(move |ctx| async move { ctx.invest_relation(symbol).await })
    }

    /// Get operating metrics and financial summaries
    pub fn operating(&self, symbol: impl Into<String> + Send + 'static) -> Result<OperatingList> {
        self.rt
            .call(move |ctx| async move { ctx.operating(symbol).await })
    }

    /// Get buyback data
    pub fn buyback(&self, symbol: impl Into<String> + Send + 'static) -> Result<BuybackData> {
        self.rt
            .call(move |ctx| async move { ctx.buyback(symbol).await })
    }

    /// Get stock ratings
    pub fn ratings(&self, symbol: impl Into<String> + Send + 'static) -> Result<StockRatings> {
        self.rt
            .call(move |ctx| async move { ctx.ratings(symbol).await })
    }

    /// Get latest business segment breakdown
    pub fn business_segments(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<BusinessSegments> {
        self.rt
            .call(move |ctx| async move { ctx.business_segments(symbol).await })
    }

    /// Get historical business segment breakdowns
    pub fn business_segments_history(
        &self,
        symbol: impl Into<String> + Send + 'static,
        report: Option<&'static str>,
        cate: Option<String>,
    ) -> Result<BusinessSegmentsHistory> {
        self.rt.call(
            move |ctx| async move { ctx.business_segments_history(symbol, report, cate).await },
        )
    }

    /// Get historical institutional rating views
    pub fn institution_rating_views(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<InstitutionRatingViews> {
        self.rt
            .call(move |ctx| async move { ctx.institution_rating_views(symbol).await })
    }

    /// Get industry rank for a market
    pub fn industry_rank(
        &self,
        market: impl Into<String> + Send + 'static,
        indicator: impl Into<String> + Send + 'static,
        sort_type: impl Into<String> + Send + 'static,
        limit: u32,
    ) -> Result<IndustryRankResponse> {
        self.rt.call(move |ctx| async move {
            ctx.industry_rank(market, indicator, sort_type, limit).await
        })
    }

    /// Get industry peer chain
    pub fn industry_peers(
        &self,
        counter_id: impl Into<String> + Send + 'static,
        market: impl Into<String> + Send + 'static,
        industry_id: Option<String>,
    ) -> Result<IndustryPeersResponse> {
        self.rt.call(
            move |ctx| async move { ctx.industry_peers(counter_id, market, industry_id).await },
        )
    }

    /// Get financial report snapshot (earnings snapshot)
    pub fn financial_report_snapshot(
        &self,
        symbol: impl Into<String> + Send + 'static,
        report: Option<&'static str>,
        fiscal_year: Option<i32>,
        fiscal_period: Option<&'static str>,
    ) -> Result<FinancialReportSnapshot> {
        self.rt.call(move |ctx| async move {
            ctx.financial_report_snapshot(symbol, report, fiscal_year, fiscal_period)
                .await
        })
    }

    /// Get ranked list of top shareholders
    pub fn shareholder_top(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<ShareholderTopResponse> {
        self.rt
            .call(move |ctx| async move { ctx.shareholder_top(symbol).await })
    }

    /// Get holding history and detail for one shareholder object
    pub fn shareholder_detail(
        &self,
        symbol: impl Into<String> + Send + 'static,
        object_id: i64,
    ) -> Result<ShareholderDetailResponse> {
        self.rt
            .call(move |ctx| async move { ctx.shareholder_detail(symbol, object_id).await })
    }

    /// Get valuation comparison between a security and optional peers
    pub fn valuation_comparison(
        &self,
        symbol: impl Into<String> + Send + 'static,
        currency: impl Into<String> + Send + 'static,
        comparison_symbols: Option<Vec<String>>,
    ) -> Result<ValuationComparisonResponse> {
        self.rt.call(move |ctx| async move {
            ctx.valuation_comparison(symbol, currency, comparison_symbols)
                .await
        })
    }

    /// Get ETF asset allocation (holdings / regional / asset class /
    /// industry)
    pub fn etf_asset_allocation(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<AssetAllocationResponse> {
        self.rt
            .call(move |ctx| async move { ctx.etf_asset_allocation(symbol).await })
    }

    /// List macroeconomic indicators
    pub fn macroeconomic_indicators(
        &self,
        country: Option<MacroeconomicCountry>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MacroeconomicIndicatorListResponse> {
        self.rt.call(move |ctx| async move {
            ctx.macroeconomic_indicators(country, offset, limit).await
        })
    }

    /// Get historical data for a macroeconomic indicator
    pub fn macroeconomic(
        &self,
        indicator_code: impl Into<String> + Send + 'static,
        start_date: Option<impl Into<String> + Send + 'static>,
        end_date: Option<impl Into<String> + Send + 'static>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MacroeconomicResponse> {
        self.rt.call(move |ctx| async move {
            ctx.macroeconomic(indicator_code, start_date, end_date, offset, limit)
                .await
        })
    }
}
