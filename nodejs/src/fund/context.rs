use std::sync::Arc;

use napi::Result;

use crate::{
    config::Config,
    error::ErrorNewType,
    fund::{
        requests::{
            FundNavRangeOptions, FundPageOptions, GetFundAnalysisOptions, GetFundHoldingsOptions,
            GetFundOrdersOptions, GetFundPositionDividendsOptions, GetFundPositionOptions,
            GetFundPositionProfitsOptions, GetFundPositionsOptions, GetFundStockHoldingsOptions,
            GetFundTransactionsOptions, GetFundsOptions, SubmitFundOrderOptions,
            ValidateFundOrderOptions,
        },
        types::{
            FundAnalysis, FundAnalysisDetail, FundAnnualReturn, FundBrief, FundDetail,
            FundDividends, FundFilters, FundHoldings, FundNavValue, FundOrder, FundOrderDetail,
            FundOrderSubmitResponse, FundOrderValidation, FundPerformance,
            FundPerformanceComparison, FundPositionDetail, FundPositionNav,
            FundPositionPerformance, FundPositionProfits, FundPositions, FundQuarterlyReturn,
            FundStockHolding, FundTransaction, FundTrend, HotFund,
        },
    },
};

/// Fund (mutual fund) channel context.
#[napi_derive::napi]
#[derive(Clone)]
pub struct FundContext {
    ctx: longbridge::fund::FundContext,
}

#[napi_derive::napi]
impl FundContext {
    /// Create a new `FundContext`.
    #[napi]
    pub fn new(config: &Config) -> FundContext {
        Self {
            ctx: longbridge::fund::FundContext::new(Arc::new(config.0.clone())),
        }
    }

    // ----- fund catalog / market data (scope: quote) -----

    /// Get the hot-selling fund list.
    #[napi]
    pub async fn hot_funds(&self) -> Result<Vec<HotFund>> {
        self.ctx
            .hot_funds()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the fund list.
    #[napi]
    pub async fn funds(&self, opts: Option<GetFundsOptions>) -> Result<Vec<FundBrief>> {
        let opts = match opts {
            Some(opts) => Some(opts.try_into()?),
            None => None,
        };
        self.ctx
            .funds(opts)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the fund list filter options.
    #[napi]
    pub async fn filters(&self) -> Result<FundFilters> {
        self.ctx.filters().await.map_err(ErrorNewType)?.try_into()
    }

    /// Get fund detail.
    #[napi]
    pub async fn detail(&self, counter_id: String) -> Result<FundDetail> {
        self.ctx
            .detail(counter_id)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund analysis (level 1).
    #[napi]
    pub async fn analysis(
        &self,
        counter_id: String,
        opts: Option<GetFundAnalysisOptions>,
    ) -> Result<FundAnalysis> {
        self.ctx
            .analysis(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund analysis detail (level 2).
    #[napi]
    pub async fn analysis_detail(
        &self,
        counter_id: String,
        opts: Option<GetFundAnalysisOptions>,
    ) -> Result<FundAnalysisDetail> {
        self.ctx
            .analysis_detail(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund trend chart.
    #[napi]
    pub async fn trend(
        &self,
        counter_id: String,
        opts: Option<GetFundAnalysisOptions>,
    ) -> Result<FundTrend> {
        self.ctx
            .trend(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund annual returns.
    #[napi]
    pub async fn annual_returns(
        &self,
        counter_id: String,
        opts: Option<FundPageOptions>,
    ) -> Result<Vec<FundAnnualReturn>> {
        self.ctx
            .annual_returns(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund quarterly returns.
    #[napi]
    pub async fn quarterly_returns(
        &self,
        counter_id: String,
        opts: Option<FundPageOptions>,
    ) -> Result<Vec<FundQuarterlyReturn>> {
        self.ctx
            .quarterly_returns(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund performance figures.
    #[napi]
    pub async fn performance(&self, counter_id: String) -> Result<Vec<FundPerformance>> {
        self.ctx
            .performance(counter_id)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund performance comparison.
    #[napi]
    pub async fn performance_comparison(
        &self,
        counter_id: String,
        opts: Option<GetFundAnalysisOptions>,
    ) -> Result<FundPerformanceComparison> {
        self.ctx
            .performance_comparison(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund latest net value.
    #[napi]
    pub async fn nav(&self, counter_id: String) -> Result<Vec<FundNavValue>> {
        self.ctx
            .nav(counter_id)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund historical net value (paged).
    #[napi]
    pub async fn nav_history(
        &self,
        counter_id: String,
        opts: Option<FundPageOptions>,
    ) -> Result<Vec<FundNavValue>> {
        self.ctx
            .nav_history(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund historical net value by relative time range.
    #[napi]
    pub async fn nav_range(
        &self,
        counter_id: String,
        opts: Option<FundNavRangeOptions>,
    ) -> Result<Vec<FundNavValue>> {
        self.ctx
            .nav_range(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get a fund's top-10 holdings.
    #[napi]
    pub async fn holdings(
        &self,
        counter_id: String,
        opts: Option<GetFundHoldingsOptions>,
    ) -> Result<FundHoldings> {
        self.ctx
            .holdings(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the stocks held by a fund (reverse lookup).
    #[napi]
    pub async fn stock_holdings(
        &self,
        counter_id: String,
        opts: Option<GetFundStockHoldingsOptions>,
    ) -> Result<Vec<FundStockHolding>> {
        self.ctx
            .stock_holdings(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    // ----- user fund positions (scope: portfolio-asset) -----

    /// Get the user's fund positions overview.
    #[napi]
    pub async fn positions(&self, opts: Option<GetFundPositionsOptions>) -> Result<FundPositions> {
        self.ctx
            .positions(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the user's single fund position detail.
    #[napi]
    pub async fn position(
        &self,
        counter_id: String,
        opts: Option<GetFundPositionOptions>,
    ) -> Result<FundPositionDetail> {
        self.ctx
            .position(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the performance figures of a held fund.
    #[napi]
    pub async fn position_performance(
        &self,
        counter_id: String,
    ) -> Result<Vec<FundPositionPerformance>> {
        self.ctx
            .position_performance(counter_id)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the cumulative-profit series of a held fund.
    #[napi]
    pub async fn position_profits(
        &self,
        counter_id: String,
        opts: Option<GetFundPositionProfitsOptions>,
    ) -> Result<FundPositionProfits> {
        self.ctx
            .position_profits(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the net-value history of a held fund.
    #[napi]
    pub async fn position_nav(
        &self,
        counter_id: String,
        opts: Option<FundNavRangeOptions>,
    ) -> Result<Vec<FundPositionNav>> {
        self.ctx
            .position_nav(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the dividend records of a held fund.
    #[napi]
    pub async fn position_dividends(
        &self,
        counter_id: String,
        opts: Option<GetFundPositionDividendsOptions>,
    ) -> Result<FundDividends> {
        self.ctx
            .position_dividends(counter_id, opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    // ----- fund orders & trading (scope: order) -----

    /// Get the user's fund orders (also serves as the trade/execution record).
    #[napi]
    pub async fn orders(&self, opts: Option<GetFundOrdersOptions>) -> Result<Vec<FundOrder>> {
        self.ctx
            .orders(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get a fund order detail.
    #[napi]
    pub async fn order(&self, order_id: i64) -> Result<FundOrderDetail> {
        self.ctx
            .order(order_id)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the user's fund transactions (cash-flow records).
    #[napi]
    pub async fn transactions(
        &self,
        opts: Option<GetFundTransactionsOptions>,
    ) -> Result<Vec<FundTransaction>> {
        self.ctx
            .transactions(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Validate a fund order before submitting.
    #[napi]
    pub async fn validate_order(
        &self,
        opts: ValidateFundOrderOptions,
    ) -> Result<FundOrderValidation> {
        self.ctx
            .validate_order(opts.into())
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Submit a fund order (buy / sell).
    #[napi]
    pub async fn submit_order(
        &self,
        opts: SubmitFundOrderOptions,
    ) -> Result<FundOrderSubmitResponse> {
        self.ctx
            .submit_order(opts.into())
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Cancel (withdraw) a fund order.
    #[napi]
    pub async fn cancel_order(&self, order_id: i64) -> Result<()> {
        self.ctx
            .cancel_order(order_id)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }
}
