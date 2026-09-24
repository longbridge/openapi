use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    fund::{
        FundAnalysis, FundAnalysisDetail, FundAnnualReturn, FundBrief, FundContext, FundDetail,
        FundDividends, FundFilters, FundHoldings, FundNavRangeOptions, FundNavValue, FundOrder,
        FundOrderDetail, FundOrderSubmitResponse, FundOrderValidation, FundPageOptions,
        FundPerformance, FundPerformanceComparison, FundPositionDetail, FundPositionNav,
        FundPositionPerformance, FundPositionProfits, FundPositions, FundQuarterlyReturn,
        FundStockHolding, FundTransaction, FundTrend, GetFundAnalysisOptions,
        GetFundHoldingsOptions, GetFundOrdersOptions, GetFundPositionDividendsOptions,
        GetFundPositionOptions, GetFundPositionProfitsOptions, GetFundPositionsOptions,
        GetFundStockHoldingsOptions, GetFundTransactionsOptions, GetFundsOptions, HotFund,
        SubmitFundOrderOptions, ValidateFundOrderOptions,
    },
};

/// Blocking fund (mutual fund) channel context.
pub struct FundContextSync {
    rt: BlockingRuntime<FundContext>,
}

impl FundContextSync {
    /// Create a [`FundContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = FundContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get the hot-selling fund list (blocking)
    pub fn hot_funds(&self) -> Result<Vec<HotFund>> {
        self.rt
            .call(move |ctx| async move { ctx.hot_funds().await })
    }

    /// Get the fund list (blocking)
    pub fn funds(
        &self,
        options: impl Into<Option<GetFundsOptions>> + Send + 'static,
    ) -> Result<Vec<FundBrief>> {
        self.rt
            .call(move |ctx| async move { ctx.funds(options).await })
    }

    /// Get the fund list filter options (blocking)
    pub fn filters(&self) -> Result<FundFilters> {
        self.rt.call(move |ctx| async move { ctx.filters().await })
    }

    /// Get fund detail (blocking)
    pub fn detail(&self, symbol: impl Into<String> + Send + 'static) -> Result<FundDetail> {
        self.rt
            .call(move |ctx| async move { ctx.detail(symbol).await })
    }

    /// Get fund analysis (level 1) (blocking)
    pub fn analysis(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundAnalysisOptions>> + Send + 'static,
    ) -> Result<FundAnalysis> {
        self.rt
            .call(move |ctx| async move { ctx.analysis(symbol, options).await })
    }

    /// Get fund analysis detail (level 2) (blocking)
    pub fn analysis_detail(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundAnalysisOptions>> + Send + 'static,
    ) -> Result<FundAnalysisDetail> {
        self.rt
            .call(move |ctx| async move { ctx.analysis_detail(symbol, options).await })
    }

    /// Get fund trend chart (blocking)
    pub fn trend(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundAnalysisOptions>> + Send + 'static,
    ) -> Result<FundTrend> {
        self.rt
            .call(move |ctx| async move { ctx.trend(symbol, options).await })
    }

    /// Get fund annual returns (blocking)
    pub fn annual_returns(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<FundPageOptions>> + Send + 'static,
    ) -> Result<Vec<FundAnnualReturn>> {
        self.rt
            .call(move |ctx| async move { ctx.annual_returns(symbol, options).await })
    }

    /// Get fund quarterly returns (blocking)
    pub fn quarterly_returns(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<FundPageOptions>> + Send + 'static,
    ) -> Result<Vec<FundQuarterlyReturn>> {
        self.rt
            .call(move |ctx| async move { ctx.quarterly_returns(symbol, options).await })
    }

    /// Get fund performance figures (blocking)
    pub fn performance(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<FundPerformance>> {
        self.rt
            .call(move |ctx| async move { ctx.performance(symbol).await })
    }

    /// Get fund performance comparison (blocking)
    pub fn performance_comparison(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundAnalysisOptions>> + Send + 'static,
    ) -> Result<FundPerformanceComparison> {
        self.rt
            .call(move |ctx| async move { ctx.performance_comparison(symbol, options).await })
    }

    /// Get fund latest net value (blocking)
    pub fn nav(&self, symbol: impl Into<String> + Send + 'static) -> Result<Vec<FundNavValue>> {
        self.rt
            .call(move |ctx| async move { ctx.nav(symbol).await })
    }

    /// Get fund historical net value (paged) (blocking)
    pub fn nav_history(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<FundPageOptions>> + Send + 'static,
    ) -> Result<Vec<FundNavValue>> {
        self.rt
            .call(move |ctx| async move { ctx.nav_history(symbol, options).await })
    }

    /// Get fund historical net value by relative time range (blocking)
    pub fn nav_range(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<FundNavRangeOptions>> + Send + 'static,
    ) -> Result<Vec<FundNavValue>> {
        self.rt
            .call(move |ctx| async move { ctx.nav_range(symbol, options).await })
    }

    /// Get a fund's top-10 holdings (blocking)
    pub fn holdings(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundHoldingsOptions>> + Send + 'static,
    ) -> Result<FundHoldings> {
        self.rt
            .call(move |ctx| async move { ctx.holdings(symbol, options).await })
    }

    /// Get the stocks held by a fund (reverse lookup) (blocking)
    pub fn stock_holdings(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundStockHoldingsOptions>> + Send + 'static,
    ) -> Result<Vec<FundStockHolding>> {
        self.rt
            .call(move |ctx| async move { ctx.stock_holdings(symbol, options).await })
    }

    /// Get the user's fund positions overview (blocking)
    pub fn positions(
        &self,
        options: impl Into<Option<GetFundPositionsOptions>> + Send + 'static,
    ) -> Result<FundPositions> {
        self.rt
            .call(move |ctx| async move { ctx.positions(options).await })
    }

    /// Get the user's single fund position detail (blocking)
    pub fn position(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundPositionOptions>> + Send + 'static,
    ) -> Result<FundPositionDetail> {
        self.rt
            .call(move |ctx| async move { ctx.position(symbol, options).await })
    }

    /// Get the performance figures of a held fund (blocking)
    pub fn position_performance(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<FundPositionPerformance>> {
        self.rt
            .call(move |ctx| async move { ctx.position_performance(symbol).await })
    }

    /// Get the cumulative-profit series of a held fund (blocking)
    pub fn position_profits(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundPositionProfitsOptions>> + Send + 'static,
    ) -> Result<FundPositionProfits> {
        self.rt
            .call(move |ctx| async move { ctx.position_profits(symbol, options).await })
    }

    /// Get the net-value history of a held fund (blocking)
    pub fn position_nav(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<FundNavRangeOptions>> + Send + 'static,
    ) -> Result<Vec<FundPositionNav>> {
        self.rt
            .call(move |ctx| async move { ctx.position_nav(symbol, options).await })
    }

    /// Get the dividend records of a held fund (blocking)
    pub fn position_dividends(
        &self,
        symbol: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetFundPositionDividendsOptions>> + Send + 'static,
    ) -> Result<FundDividends> {
        self.rt
            .call(move |ctx| async move { ctx.position_dividends(symbol, options).await })
    }

    /// Get the user's fund orders (blocking)
    pub fn orders(
        &self,
        options: impl Into<Option<GetFundOrdersOptions>> + Send + 'static,
    ) -> Result<Vec<FundOrder>> {
        self.rt
            .call(move |ctx| async move { ctx.orders(options).await })
    }

    /// Get a fund order detail (blocking)
    pub fn order(&self, order_id: i64) -> Result<FundOrderDetail> {
        self.rt
            .call(move |ctx| async move { ctx.order(order_id).await })
    }

    /// Get the user's fund transactions (cash-flow records) (blocking)
    pub fn transactions(
        &self,
        options: impl Into<Option<GetFundTransactionsOptions>> + Send + 'static,
    ) -> Result<Vec<FundTransaction>> {
        self.rt
            .call(move |ctx| async move { ctx.transactions(options).await })
    }

    /// Validate a fund order before submitting (blocking)
    pub fn validate_order(&self, options: ValidateFundOrderOptions) -> Result<FundOrderValidation> {
        self.rt
            .call(move |ctx| async move { ctx.validate_order(options).await })
    }

    /// Submit a fund order (buy / sell) (blocking)
    pub fn submit_order(&self, options: SubmitFundOrderOptions) -> Result<FundOrderSubmitResponse> {
        self.rt
            .call(move |ctx| async move { ctx.submit_order(options).await })
    }

    /// Cancel (withdraw) a fund order (blocking)
    pub fn cancel_order(&self, order_id: i64) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.cancel_order(order_id).await })
    }
}
