use std::sync::Arc;

use longbridge::{
    blocking::FundContextSync,
    fund::{
        FundNavRangeOptions, FundPageOptions, GetFundAnalysisOptions, GetFundHoldingsOptions,
        GetFundOrdersOptions, GetFundPositionDividendsOptions, GetFundPositionOptions,
        GetFundPositionProfitsOptions, GetFundPositionsOptions, GetFundStockHoldingsOptions,
        GetFundTransactionsOptions, GetFundsOptions, SubmitFundOrderOptions,
        ValidateFundOrderOptions,
    },
};
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

use crate::{
    config::Config,
    error::ErrorNewType,
    fund::types::{
        FundAnalysis, FundAnalysisDetail, FundAnnualReturn, FundBrief, FundDetail, FundDividends,
        FundFilters, FundHoldings, FundNavValue, FundOrder, FundOrderDetail,
        FundOrderSubmitResponse, FundOrderValidation, FundPerformance, FundPerformanceComparison,
        FundPositionDetail, FundPositionNav, FundPositionPerformance, FundPositionProfits,
        FundPositions, FundQuarterlyReturn, FundStockHolding, FundTransaction, FundTrend, HotFund,
    },
};

/// Fund (mutual fund) channel context (REST-only).
#[pyclass]
pub(crate) struct FundContext {
    ctx: FundContextSync,
}

#[pymethods]
impl FundContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: FundContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    // ----- fund catalog / market data (scope: quote) -----

    /// Get the hot-selling fund list
    fn hot_funds(&self) -> PyResult<Vec<HotFund>> {
        self.ctx
            .hot_funds()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the fund list
    #[pyo3(signature = (filter = None, quick_ids = None, time_interval = None))]
    fn funds(
        &self,
        filter: Option<String>,
        quick_ids: Option<Vec<i64>>,
        time_interval: Option<Vec<String>>,
    ) -> PyResult<Vec<FundBrief>> {
        let mut opts = GetFundsOptions::new();
        if let Some(filter) = filter {
            let value: serde_json::Value = serde_json::from_str(&filter)
                .map_err(|err| PyValueError::new_err(format!("invalid filter JSON: {err}")))?;
            opts = opts.filter(value);
        }
        if let Some(quick_ids) = quick_ids {
            opts = opts.quick_ids(quick_ids);
        }
        if let Some(time_interval) = time_interval {
            opts = opts.time_interval(time_interval);
        }
        self.ctx
            .funds(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the fund list filter options
    fn filters(&self) -> PyResult<FundFilters> {
        self.ctx.filters().map_err(ErrorNewType)?.try_into()
    }

    /// Get fund detail
    fn detail(&self, symbol: String) -> PyResult<FundDetail> {
        self.ctx.detail(symbol).map_err(ErrorNewType)?.try_into()
    }

    /// Get fund analysis (level 1)
    #[pyo3(signature = (symbol, period = None))]
    fn analysis(&self, symbol: String, period: Option<i32>) -> PyResult<FundAnalysis> {
        let mut opts = GetFundAnalysisOptions::new();
        if let Some(period) = period {
            opts = opts.period(period);
        }
        self.ctx
            .analysis(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund analysis detail (level 2)
    #[pyo3(signature = (symbol, period = None))]
    fn analysis_detail(&self, symbol: String, period: Option<i32>) -> PyResult<FundAnalysisDetail> {
        let mut opts = GetFundAnalysisOptions::new();
        if let Some(period) = period {
            opts = opts.period(period);
        }
        self.ctx
            .analysis_detail(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund trend chart
    #[pyo3(signature = (symbol, period = None))]
    fn trend(&self, symbol: String, period: Option<i32>) -> PyResult<FundTrend> {
        let mut opts = GetFundAnalysisOptions::new();
        if let Some(period) = period {
            opts = opts.period(period);
        }
        self.ctx
            .trend(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund annual returns
    #[pyo3(signature = (symbol, page = None, size = None))]
    fn annual_returns(
        &self,
        symbol: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<FundAnnualReturn>> {
        let opts = build_page_options(page, size);
        self.ctx
            .annual_returns(symbol, opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund quarterly returns
    #[pyo3(signature = (symbol, page = None, size = None))]
    fn quarterly_returns(
        &self,
        symbol: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<FundQuarterlyReturn>> {
        let opts = build_page_options(page, size);
        self.ctx
            .quarterly_returns(symbol, opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund performance figures
    fn performance(&self, symbol: String) -> PyResult<Vec<FundPerformance>> {
        self.ctx
            .performance(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund performance comparison
    #[pyo3(signature = (symbol, period = None))]
    fn performance_comparison(
        &self,
        symbol: String,
        period: Option<i32>,
    ) -> PyResult<FundPerformanceComparison> {
        let mut opts = GetFundAnalysisOptions::new();
        if let Some(period) = period {
            opts = opts.period(period);
        }
        self.ctx
            .performance_comparison(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get fund latest net value
    fn nav(&self, symbol: String) -> PyResult<Vec<FundNavValue>> {
        self.ctx
            .nav(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund historical net value (paged)
    #[pyo3(signature = (symbol, page = None, size = None))]
    fn nav_history(
        &self,
        symbol: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<FundNavValue>> {
        let opts = build_page_options(page, size);
        self.ctx
            .nav_history(symbol, opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get fund historical net value by relative time range
    #[pyo3(signature = (symbol, month_before = None, year_before = None))]
    fn nav_range(
        &self,
        symbol: String,
        month_before: Option<i32>,
        year_before: Option<i32>,
    ) -> PyResult<Vec<FundNavValue>> {
        let opts = build_nav_range_options(month_before, year_before);
        self.ctx
            .nav_range(symbol, opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get a fund's top-10 holdings
    #[pyo3(signature = (symbol, scene = None))]
    fn holdings(&self, symbol: String, scene: Option<i32>) -> PyResult<FundHoldings> {
        let mut opts = GetFundHoldingsOptions::new();
        if let Some(scene) = scene {
            opts = opts.scene(scene);
        }
        self.ctx
            .holdings(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the stocks held by a fund (reverse lookup)
    #[pyo3(signature = (symbol, limit = None))]
    fn stock_holdings(
        &self,
        symbol: String,
        limit: Option<i32>,
    ) -> PyResult<Vec<FundStockHolding>> {
        let mut opts = GetFundStockHoldingsOptions::new();
        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }
        self.ctx
            .stock_holdings(symbol, opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    // ----- user fund positions (scope: portfolio-asset) -----

    /// Get the user's fund positions overview
    #[pyo3(signature = (account_channel = None, aaid = None))]
    fn positions(
        &self,
        account_channel: Option<String>,
        aaid: Option<i64>,
    ) -> PyResult<FundPositions> {
        let mut opts = GetFundPositionsOptions::new();
        if let Some(account_channel) = account_channel {
            opts = opts.account_channel(account_channel);
        }
        if let Some(aaid) = aaid {
            opts = opts.aaid(aaid);
        }
        self.ctx.positions(opts).map_err(ErrorNewType)?.try_into()
    }

    /// Get the user's single fund position detail
    #[pyo3(signature = (symbol, account_channel = None, aaid = None, start = None, end = None))]
    fn position(
        &self,
        symbol: String,
        account_channel: Option<String>,
        aaid: Option<i64>,
        start: Option<String>,
        end: Option<String>,
    ) -> PyResult<FundPositionDetail> {
        let mut opts = GetFundPositionOptions::new();
        if let Some(account_channel) = account_channel {
            opts = opts.account_channel(account_channel);
        }
        if let Some(aaid) = aaid {
            opts = opts.aaid(aaid);
        }
        if let Some(start) = start {
            opts = opts.start(start);
        }
        if let Some(end) = end {
            opts = opts.end(end);
        }
        self.ctx
            .position(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the performance figures of a held fund
    fn position_performance(&self, symbol: String) -> PyResult<Vec<FundPositionPerformance>> {
        self.ctx
            .position_performance(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the cumulative-profit series of a held fund
    #[pyo3(signature = (symbol, account_channel = None, aaid = None, start = None, end = None, page = None, size = None))]
    #[allow(clippy::too_many_arguments)]
    fn position_profits(
        &self,
        symbol: String,
        account_channel: Option<String>,
        aaid: Option<i64>,
        start: Option<String>,
        end: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<FundPositionProfits> {
        let mut opts = GetFundPositionProfitsOptions::new();
        if let Some(account_channel) = account_channel {
            opts = opts.account_channel(account_channel);
        }
        if let Some(aaid) = aaid {
            opts = opts.aaid(aaid);
        }
        if let Some(start) = start {
            opts = opts.start(start);
        }
        if let Some(end) = end {
            opts = opts.end(end);
        }
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(size) = size {
            opts = opts.size(size);
        }
        self.ctx
            .position_profits(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the net-value history of a held fund
    #[pyo3(signature = (symbol, month_before = None, year_before = None))]
    fn position_nav(
        &self,
        symbol: String,
        month_before: Option<i32>,
        year_before: Option<i32>,
    ) -> PyResult<Vec<FundPositionNav>> {
        let opts = build_nav_range_options(month_before, year_before);
        self.ctx
            .position_nav(symbol, opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the dividend records of a held fund
    #[pyo3(signature = (symbol, account_channel = None, aaid = None, currency = None, start = None, end = None, page = None, size = None))]
    #[allow(clippy::too_many_arguments)]
    fn position_dividends(
        &self,
        symbol: String,
        account_channel: Option<String>,
        aaid: Option<i64>,
        currency: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<FundDividends> {
        let mut opts = GetFundPositionDividendsOptions::new();
        if let Some(account_channel) = account_channel {
            opts = opts.account_channel(account_channel);
        }
        if let Some(aaid) = aaid {
            opts = opts.aaid(aaid);
        }
        if let Some(currency) = currency {
            opts = opts.currency(currency);
        }
        if let Some(start) = start {
            opts = opts.start(start);
        }
        if let Some(end) = end {
            opts = opts.end(end);
        }
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(size) = size {
            opts = opts.size(size);
        }
        self.ctx
            .position_dividends(symbol, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    // ----- fund orders & trading (scope: order) -----

    /// Get the user's fund orders (also serves as the trade/execution record)
    #[pyo3(signature = (symbols = None, actions = None, states = None, currency = None, start = None, end = None, page = None, size = None))]
    #[allow(clippy::too_many_arguments)]
    fn orders(
        &self,
        symbols: Option<Vec<String>>,
        actions: Option<String>,
        states: Option<String>,
        currency: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<FundOrder>> {
        let mut opts = GetFundOrdersOptions::new();
        if let Some(symbols) = symbols {
            opts = opts.symbols(symbols);
        }
        if let Some(actions) = actions {
            opts = opts.actions(actions);
        }
        if let Some(states) = states {
            opts = opts.states(states);
        }
        if let Some(currency) = currency {
            opts = opts.currency(currency);
        }
        if let Some(start) = start {
            opts = opts.start(start);
        }
        if let Some(end) = end {
            opts = opts.end(end);
        }
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(size) = size {
            opts = opts.size(size);
        }
        self.ctx
            .orders(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get a fund order detail
    fn order(&self, order_id: i64) -> PyResult<FundOrderDetail> {
        self.ctx.order(order_id).map_err(ErrorNewType)?.try_into()
    }

    /// Get the user's fund transactions (cash-flow records)
    #[pyo3(signature = (account_channel = None, business_type = None, category = None, currencies = None, start = None, end = None, page = None, size = None))]
    #[allow(clippy::too_many_arguments)]
    fn transactions(
        &self,
        account_channel: Option<String>,
        business_type: Option<String>,
        category: Option<String>,
        currencies: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<FundTransaction>> {
        let mut opts = GetFundTransactionsOptions::new();
        if let Some(account_channel) = account_channel {
            opts = opts.account_channel(account_channel);
        }
        if let Some(business_type) = business_type {
            opts = opts.business_type(business_type);
        }
        if let Some(category) = category {
            opts = opts.category(category);
        }
        if let Some(currencies) = currencies {
            opts = opts.currencies(currencies);
        }
        if let Some(start) = start {
            opts = opts.start(start);
        }
        if let Some(end) = end {
            opts = opts.end(end);
        }
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(size) = size {
            opts = opts.size(size);
        }
        self.ctx
            .transactions(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Validate a fund order before submitting
    #[pyo3(signature = (symbol, action, currency, amount = None, units = None, dividend_option = None, fund_source = None, account_channel = None))]
    #[allow(clippy::too_many_arguments)]
    fn validate_order(
        &self,
        symbol: String,
        action: String,
        currency: String,
        amount: Option<String>,
        units: Option<String>,
        dividend_option: Option<i32>,
        fund_source: Option<i32>,
        account_channel: Option<String>,
    ) -> PyResult<FundOrderValidation> {
        let mut opts = ValidateFundOrderOptions::new(symbol, action, currency);
        if let Some(amount) = amount {
            opts = opts.amount(amount);
        }
        if let Some(units) = units {
            opts = opts.units(units);
        }
        if let Some(dividend_option) = dividend_option {
            opts = opts.dividend_option(dividend_option);
        }
        if let Some(fund_source) = fund_source {
            opts = opts.fund_source(fund_source);
        }
        if let Some(account_channel) = account_channel {
            opts = opts.account_channel(account_channel);
        }
        self.ctx
            .validate_order(opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Submit a fund order (buy / sell)
    #[pyo3(signature = (symbol, action, currency, amount = None, units = None, dividend_option = None, fee = None, is_sell_all = None, remark = None, trade_method = None))]
    #[allow(clippy::too_many_arguments)]
    fn submit_order(
        &self,
        symbol: String,
        action: String,
        currency: String,
        amount: Option<String>,
        units: Option<String>,
        dividend_option: Option<i32>,
        fee: Option<String>,
        is_sell_all: Option<bool>,
        remark: Option<String>,
        trade_method: Option<i32>,
    ) -> PyResult<FundOrderSubmitResponse> {
        let mut opts = SubmitFundOrderOptions::new(symbol, action, currency);
        if let Some(amount) = amount {
            opts = opts.amount(amount);
        }
        if let Some(units) = units {
            opts = opts.units(units);
        }
        if let Some(dividend_option) = dividend_option {
            opts = opts.dividend_option(dividend_option);
        }
        if let Some(fee) = fee {
            opts = opts.fee(fee);
        }
        if let Some(is_sell_all) = is_sell_all {
            opts = opts.is_sell_all(is_sell_all);
        }
        if let Some(remark) = remark {
            opts = opts.remark(remark);
        }
        if let Some(trade_method) = trade_method {
            opts = opts.trade_method(trade_method);
        }
        self.ctx
            .submit_order(opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Cancel (withdraw) a fund order
    fn cancel_order(&self, order_id: i64) -> PyResult<()> {
        self.ctx.cancel_order(order_id).map_err(ErrorNewType)?;
        Ok(())
    }
}

fn build_page_options(page: Option<i32>, size: Option<i32>) -> FundPageOptions {
    let mut opts = FundPageOptions::new();
    if let Some(page) = page {
        opts = opts.page(page);
    }
    if let Some(size) = size {
        opts = opts.size(size);
    }
    opts
}

fn build_nav_range_options(
    month_before: Option<i32>,
    year_before: Option<i32>,
) -> FundNavRangeOptions {
    let mut opts = FundNavRangeOptions::new();
    if let Some(month_before) = month_before {
        opts = opts.month_before(month_before);
    }
    if let Some(year_before) = year_before {
        opts = opts.year_before(year_before);
    }
    opts
}
