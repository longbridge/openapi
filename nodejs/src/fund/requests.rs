//! napi request-option structs for the fund endpoints.
//!
//! Each struct mirrors a builder-style options type from the Rust core and
//! converts into it. The `symbol` path parameter is passed as a separate method
//! argument, so these structs only carry query-string / request-body fields.

use napi::bindgen_prelude::*;

/// Options for the fund list.
#[napi_derive::napi(object)]
pub struct GetFundsOptions {
    /// Server-defined filter object, as a raw JSON string
    pub filter: Option<String>,
    /// Quick-filter ids
    pub quick_ids: Option<Vec<i64>>,
    /// Earning-rate time intervals
    pub time_interval: Option<Vec<String>>,
}

impl ::std::convert::TryFrom<GetFundsOptions> for longbridge::fund::GetFundsOptions {
    type Error = Error;

    fn try_from(opts: GetFundsOptions) -> Result<Self> {
        let mut opts2 = longbridge::fund::GetFundsOptions::new();
        if let Some(filter) = opts.filter {
            let value: serde_json::Value = serde_json::from_str(&filter)
                .map_err(|err| Error::from_reason(format!("invalid filter JSON: {err}")))?;
            opts2 = opts2.filter(value);
        }
        if let Some(quick_ids) = opts.quick_ids {
            opts2 = opts2.quick_ids(quick_ids);
        }
        if let Some(time_interval) = opts.time_interval {
            opts2 = opts2.time_interval(time_interval);
        }
        Ok(opts2)
    }
}

/// Options for the fund analysis / trend / comparison endpoints.
#[napi_derive::napi(object)]
pub struct GetFundAnalysisOptions {
    /// Analysis period
    pub period: Option<i32>,
}

impl From<GetFundAnalysisOptions> for longbridge::fund::GetFundAnalysisOptions {
    #[inline]
    fn from(opts: GetFundAnalysisOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundAnalysisOptions::new();
        if let Some(period) = opts.period {
            opts2 = opts2.period(period);
        }
        opts2
    }
}

/// Paging options (page / size).
#[napi_derive::napi(object)]
pub struct FundPageOptions {
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
}

impl From<FundPageOptions> for longbridge::fund::FundPageOptions {
    #[inline]
    fn from(opts: FundPageOptions) -> Self {
        let mut opts2 = longbridge::fund::FundPageOptions::new();
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        opts2
    }
}

/// Net-value range options (relative months / years before now).
#[napi_derive::napi(object)]
pub struct FundNavRangeOptions {
    /// Number of months before now
    pub month_before: Option<i32>,
    /// Number of years before now
    pub year_before: Option<i32>,
}

impl From<FundNavRangeOptions> for longbridge::fund::FundNavRangeOptions {
    #[inline]
    fn from(opts: FundNavRangeOptions) -> Self {
        let mut opts2 = longbridge::fund::FundNavRangeOptions::new();
        if let Some(month_before) = opts.month_before {
            opts2 = opts2.month_before(month_before);
        }
        if let Some(year_before) = opts.year_before {
            opts2 = opts2.year_before(year_before);
        }
        opts2
    }
}

/// Options for the fund holdings endpoint.
#[napi_derive::napi(object)]
pub struct GetFundHoldingsOptions {
    /// Scene
    pub scene: Option<i32>,
}

impl From<GetFundHoldingsOptions> for longbridge::fund::GetFundHoldingsOptions {
    #[inline]
    fn from(opts: GetFundHoldingsOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundHoldingsOptions::new();
        if let Some(scene) = opts.scene {
            opts2 = opts2.scene(scene);
        }
        opts2
    }
}

/// Options for the fund stock-holdings (reverse) endpoint.
#[napi_derive::napi(object)]
pub struct GetFundStockHoldingsOptions {
    /// Maximum number of stocks to return
    pub limit: Option<i32>,
}

impl From<GetFundStockHoldingsOptions> for longbridge::fund::GetFundStockHoldingsOptions {
    #[inline]
    fn from(opts: GetFundStockHoldingsOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundStockHoldingsOptions::new();
        if let Some(limit) = opts.limit {
            opts2 = opts2.limit(limit);
        }
        opts2
    }
}

/// Options for the fund positions overview endpoint.
#[napi_derive::napi(object)]
pub struct GetFundPositionsOptions {
    /// Account channel
    pub account_channel: Option<String>,
    /// Account id
    pub aaid: Option<i64>,
}

impl From<GetFundPositionsOptions> for longbridge::fund::GetFundPositionsOptions {
    #[inline]
    fn from(opts: GetFundPositionsOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundPositionsOptions::new();
        if let Some(account_channel) = opts.account_channel {
            opts2 = opts2.account_channel(account_channel);
        }
        if let Some(aaid) = opts.aaid {
            opts2 = opts2.aaid(aaid);
        }
        opts2
    }
}

/// Options for a single fund position detail.
#[napi_derive::napi(object)]
pub struct GetFundPositionOptions {
    /// Account channel
    pub account_channel: Option<String>,
    /// Account id
    pub aaid: Option<i64>,
    /// Range start
    pub start: Option<String>,
    /// Range end
    pub end: Option<String>,
}

impl From<GetFundPositionOptions> for longbridge::fund::GetFundPositionOptions {
    #[inline]
    fn from(opts: GetFundPositionOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundPositionOptions::new();
        if let Some(account_channel) = opts.account_channel {
            opts2 = opts2.account_channel(account_channel);
        }
        if let Some(aaid) = opts.aaid {
            opts2 = opts2.aaid(aaid);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(end);
        }
        opts2
    }
}

/// Options for a single fund position cumulative-profit list.
#[napi_derive::napi(object)]
pub struct GetFundPositionProfitsOptions {
    /// Account channel
    pub account_channel: Option<String>,
    /// Account id
    pub aaid: Option<i64>,
    /// Range start
    pub start: Option<String>,
    /// Range end
    pub end: Option<String>,
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
}

impl From<GetFundPositionProfitsOptions> for longbridge::fund::GetFundPositionProfitsOptions {
    #[inline]
    fn from(opts: GetFundPositionProfitsOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundPositionProfitsOptions::new();
        if let Some(account_channel) = opts.account_channel {
            opts2 = opts2.account_channel(account_channel);
        }
        if let Some(aaid) = opts.aaid {
            opts2 = opts2.aaid(aaid);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(end);
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        opts2
    }
}

/// Options for a single fund position dividend list.
#[napi_derive::napi(object)]
pub struct GetFundPositionDividendsOptions {
    /// Account channel
    pub account_channel: Option<String>,
    /// Account id
    pub aaid: Option<i64>,
    /// Currency
    pub currency: Option<String>,
    /// Range start (unix seconds)
    pub start: Option<i64>,
    /// Range end (unix seconds)
    pub end: Option<i64>,
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
}

impl From<GetFundPositionDividendsOptions> for longbridge::fund::GetFundPositionDividendsOptions {
    #[inline]
    fn from(opts: GetFundPositionDividendsOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundPositionDividendsOptions::new();
        if let Some(account_channel) = opts.account_channel {
            opts2 = opts2.account_channel(account_channel);
        }
        if let Some(aaid) = opts.aaid {
            opts2 = opts2.aaid(aaid);
        }
        if let Some(currency) = opts.currency {
            opts2 = opts2.currency(currency);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(end);
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        opts2
    }
}

/// Options for the fund orders list.
#[napi_derive::napi(object)]
pub struct GetFundOrdersOptions {
    /// Filter by fund symbols
    pub symbols: Option<Vec<String>>,
    /// Filter by actions (comma-separated)
    pub actions: Option<String>,
    /// Filter by states (comma-separated)
    pub states: Option<String>,
    /// Filter by currency
    pub currency: Option<String>,
    /// Range start (unix seconds)
    pub start: Option<i64>,
    /// Range end (unix seconds)
    pub end: Option<i64>,
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
}

impl From<GetFundOrdersOptions> for longbridge::fund::GetFundOrdersOptions {
    #[inline]
    fn from(opts: GetFundOrdersOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundOrdersOptions::new();
        if let Some(symbols) = opts.symbols {
            opts2 = opts2.symbols(symbols);
        }
        if let Some(actions) = opts.actions {
            opts2 = opts2.actions(actions);
        }
        if let Some(states) = opts.states {
            opts2 = opts2.states(states);
        }
        if let Some(currency) = opts.currency {
            opts2 = opts2.currency(currency);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(end);
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        opts2
    }
}

/// Options for the fund transactions (cash-flow) list.
#[napi_derive::napi(object)]
pub struct GetFundTransactionsOptions {
    /// Account channel
    pub account_channel: Option<String>,
    /// Business type
    pub business_type: Option<String>,
    /// Category
    pub category: Option<String>,
    /// Currencies (comma-separated)
    pub currencies: Option<String>,
    /// Range start (unix seconds)
    pub start: Option<i64>,
    /// Range end (unix seconds)
    pub end: Option<i64>,
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
}

impl From<GetFundTransactionsOptions> for longbridge::fund::GetFundTransactionsOptions {
    #[inline]
    fn from(opts: GetFundTransactionsOptions) -> Self {
        let mut opts2 = longbridge::fund::GetFundTransactionsOptions::new();
        if let Some(account_channel) = opts.account_channel {
            opts2 = opts2.account_channel(account_channel);
        }
        if let Some(business_type) = opts.business_type {
            opts2 = opts2.business_type(business_type);
        }
        if let Some(category) = opts.category {
            opts2 = opts2.category(category);
        }
        if let Some(currencies) = opts.currencies {
            opts2 = opts2.currencies(currencies);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(end);
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        opts2
    }
}

/// Options for validating a fund order.
#[napi_derive::napi(object)]
pub struct ValidateFundOrderOptions {
    /// Fund symbol
    pub symbol: String,
    /// Action (buy/sell)
    pub action: String,
    /// Currency
    pub currency: String,
    /// Amount (for amount-based orders)
    pub amount: Option<String>,
    /// Units (for unit-based orders)
    pub units: Option<String>,
    /// Dividend option
    pub dividend_option: Option<i32>,
    /// Fund source
    pub fund_source: Option<i32>,
    /// Account channel
    pub account_channel: Option<String>,
}

impl From<ValidateFundOrderOptions> for longbridge::fund::ValidateFundOrderOptions {
    #[inline]
    fn from(opts: ValidateFundOrderOptions) -> Self {
        let mut opts2 = longbridge::fund::ValidateFundOrderOptions::new(
            opts.symbol,
            opts.action,
            opts.currency,
        );
        if let Some(amount) = opts.amount {
            opts2 = opts2.amount(amount);
        }
        if let Some(units) = opts.units {
            opts2 = opts2.units(units);
        }
        if let Some(dividend_option) = opts.dividend_option {
            opts2 = opts2.dividend_option(dividend_option);
        }
        if let Some(fund_source) = opts.fund_source {
            opts2 = opts2.fund_source(fund_source);
        }
        if let Some(account_channel) = opts.account_channel {
            opts2 = opts2.account_channel(account_channel);
        }
        opts2
    }
}

/// Options for submitting a fund order.
#[napi_derive::napi(object)]
pub struct SubmitFundOrderOptions {
    /// Fund symbol
    pub symbol: String,
    /// Action (buy/sell)
    pub action: String,
    /// Currency
    pub currency: String,
    /// Amount (for amount-based orders)
    pub amount: Option<String>,
    /// Units (for unit-based orders)
    pub units: Option<String>,
    /// Dividend option
    pub dividend_option: Option<i32>,
    /// Fee
    pub fee: Option<String>,
    /// Whether to sell all
    pub is_sell_all: Option<bool>,
    /// Remark
    pub remark: Option<String>,
    /// Trade method
    pub trade_method: Option<i32>,
}

impl From<SubmitFundOrderOptions> for longbridge::fund::SubmitFundOrderOptions {
    #[inline]
    fn from(opts: SubmitFundOrderOptions) -> Self {
        let mut opts2 =
            longbridge::fund::SubmitFundOrderOptions::new(opts.symbol, opts.action, opts.currency);
        if let Some(amount) = opts.amount {
            opts2 = opts2.amount(amount);
        }
        if let Some(units) = opts.units {
            opts2 = opts2.units(units);
        }
        if let Some(dividend_option) = opts.dividend_option {
            opts2 = opts2.dividend_option(dividend_option);
        }
        if let Some(fee) = opts.fee {
            opts2 = opts2.fee(fee);
        }
        if let Some(is_sell_all) = opts.is_sell_all {
            opts2 = opts2.is_sell_all(is_sell_all);
        }
        if let Some(remark) = opts.remark {
            opts2 = opts2.remark(remark);
        }
        if let Some(trade_method) = opts.trade_method {
            opts2 = opts2.trade_method(trade_method);
        }
        opts2
    }
}
