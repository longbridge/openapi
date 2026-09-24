//! Request option builders for the fund endpoints.
//!
//! The fund identifier is exposed as `counter_id` (e.g. `UT/FD/HK0000384492`).
//! Because it contains `/` it cannot live in the URL path, so endpoints that
//! target a single fund take `counter_id` as a separate method argument and
//! send it as the `counter_id` query parameter; these option structs only carry
//! the remaining query-string / request-body fields.

use serde::Serialize;
use serde_json::Value;

/// Options for the fund list ([`FundContext::list_funds`]).
///
/// [`FundContext::list_funds`]: crate::fund::FundContext::list_funds
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundsOptions {
    /// Server-defined filter object
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Value>,
    /// Quick-filter ids
    #[serde(skip_serializing_if = "Vec::is_empty")]
    quick_ids: Vec<i64>,
    /// Earning-rate time intervals
    #[serde(skip_serializing_if = "Vec::is_empty")]
    time_interval: Vec<String>,
}

impl GetFundsOptions {
    /// Create a new [`GetFundsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the filter object
    #[inline]
    #[must_use]
    pub fn filter(self, filter: Value) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
    }

    /// Set the quick-filter ids
    #[inline]
    #[must_use]
    pub fn quick_ids<I>(self, quick_ids: I) -> Self
    where
        I: IntoIterator<Item = i64>,
    {
        Self {
            quick_ids: quick_ids.into_iter().collect(),
            ..self
        }
    }

    /// Set the earning-rate time intervals
    #[inline]
    #[must_use]
    pub fn time_interval<I, T>(self, time_interval: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            time_interval: time_interval.into_iter().map(Into::into).collect(),
            ..self
        }
    }
}

/// Options for the fund analysis / trend / comparison endpoints.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundAnalysisOptions {
    /// Analysis period
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<i32>,
}

impl GetFundAnalysisOptions {
    /// Create a new [`GetFundAnalysisOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the analysis period
    #[inline]
    #[must_use]
    pub fn period(mut self, period: i32) -> Self {
        self.period = Some(period);
        self
    }
}

/// Paging options (page / size).
#[derive(Debug, Default, Clone, Serialize)]
pub struct FundPageOptions {
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
}

impl FundPageOptions {
    /// Create a new [`FundPageOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}

/// Net-value range options (relative months / years before now).
#[derive(Debug, Default, Clone, Serialize)]
pub struct FundNavRangeOptions {
    /// Number of months before now
    #[serde(skip_serializing_if = "Option::is_none")]
    month_before: Option<i32>,
    /// Number of years before now
    #[serde(skip_serializing_if = "Option::is_none")]
    year_before: Option<i32>,
}

impl FundNavRangeOptions {
    /// Create a new [`FundNavRangeOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of months before now
    #[inline]
    #[must_use]
    pub fn month_before(self, month_before: i32) -> Self {
        Self {
            month_before: Some(month_before),
            ..self
        }
    }

    /// Set the number of years before now
    #[inline]
    #[must_use]
    pub fn year_before(self, year_before: i32) -> Self {
        Self {
            year_before: Some(year_before),
            ..self
        }
    }
}

/// Options for the fund holdings endpoint.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundHoldingsOptions {
    /// Scene
    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<i32>,
}

impl GetFundHoldingsOptions {
    /// Create a new [`GetFundHoldingsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the scene
    #[inline]
    #[must_use]
    pub fn scene(mut self, scene: i32) -> Self {
        self.scene = Some(scene);
        self
    }
}

/// Options for the fund stock-holdings (reverse) endpoint.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundStockHoldingsOptions {
    /// Maximum number of stocks to return
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl GetFundStockHoldingsOptions {
    /// Create a new [`GetFundStockHoldingsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of stocks to return
    #[inline]
    #[must_use]
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Options for the fund positions overview endpoint.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundPositionsOptions {
    /// Account channel
    #[serde(skip_serializing_if = "Option::is_none")]
    account_channel: Option<String>,
    /// Account id
    #[serde(skip_serializing_if = "Option::is_none")]
    aaid: Option<i64>,
}

impl GetFundPositionsOptions {
    /// Create a new [`GetFundPositionsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account channel
    #[inline]
    #[must_use]
    pub fn account_channel(self, account_channel: impl Into<String>) -> Self {
        Self {
            account_channel: Some(account_channel.into()),
            ..self
        }
    }

    /// Set the account id
    #[inline]
    #[must_use]
    pub fn aaid(self, aaid: i64) -> Self {
        Self {
            aaid: Some(aaid),
            ..self
        }
    }
}

/// Options for a single fund position detail.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundPositionOptions {
    /// Account channel
    #[serde(skip_serializing_if = "Option::is_none")]
    account_channel: Option<String>,
    /// Account id
    #[serde(skip_serializing_if = "Option::is_none")]
    aaid: Option<i64>,
    /// Range start
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<String>,
    /// Range end
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
}

impl GetFundPositionOptions {
    /// Create a new [`GetFundPositionOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account channel
    #[inline]
    #[must_use]
    pub fn account_channel(self, account_channel: impl Into<String>) -> Self {
        Self {
            account_channel: Some(account_channel.into()),
            ..self
        }
    }

    /// Set the account id
    #[inline]
    #[must_use]
    pub fn aaid(self, aaid: i64) -> Self {
        Self {
            aaid: Some(aaid),
            ..self
        }
    }

    /// Set the range start
    #[inline]
    #[must_use]
    pub fn start(self, start: impl Into<String>) -> Self {
        Self {
            start: Some(start.into()),
            ..self
        }
    }

    /// Set the range end
    #[inline]
    #[must_use]
    pub fn end(self, end: impl Into<String>) -> Self {
        Self {
            end: Some(end.into()),
            ..self
        }
    }
}

/// Options for a single fund position cumulative-profit list.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundPositionProfitsOptions {
    /// Account channel
    #[serde(skip_serializing_if = "Option::is_none")]
    account_channel: Option<String>,
    /// Account id
    #[serde(skip_serializing_if = "Option::is_none")]
    aaid: Option<i64>,
    /// Range start
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<String>,
    /// Range end
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
}

impl GetFundPositionProfitsOptions {
    /// Create a new [`GetFundPositionProfitsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account channel
    #[inline]
    #[must_use]
    pub fn account_channel(self, account_channel: impl Into<String>) -> Self {
        Self {
            account_channel: Some(account_channel.into()),
            ..self
        }
    }

    /// Set the account id
    #[inline]
    #[must_use]
    pub fn aaid(self, aaid: i64) -> Self {
        Self {
            aaid: Some(aaid),
            ..self
        }
    }

    /// Set the range start
    #[inline]
    #[must_use]
    pub fn start(self, start: impl Into<String>) -> Self {
        Self {
            start: Some(start.into()),
            ..self
        }
    }

    /// Set the range end
    #[inline]
    #[must_use]
    pub fn end(self, end: impl Into<String>) -> Self {
        Self {
            end: Some(end.into()),
            ..self
        }
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}

/// Options for a single fund position dividend list.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundPositionDividendsOptions {
    /// Account channel
    #[serde(skip_serializing_if = "Option::is_none")]
    account_channel: Option<String>,
    /// Account id
    #[serde(skip_serializing_if = "Option::is_none")]
    aaid: Option<i64>,
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    /// Range start (unix seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,
    /// Range end (unix seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<i64>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
}

impl GetFundPositionDividendsOptions {
    /// Create a new [`GetFundPositionDividendsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account channel
    #[inline]
    #[must_use]
    pub fn account_channel(self, account_channel: impl Into<String>) -> Self {
        Self {
            account_channel: Some(account_channel.into()),
            ..self
        }
    }

    /// Set the account id
    #[inline]
    #[must_use]
    pub fn aaid(self, aaid: i64) -> Self {
        Self {
            aaid: Some(aaid),
            ..self
        }
    }

    /// Set the currency
    #[inline]
    #[must_use]
    pub fn currency(self, currency: impl Into<String>) -> Self {
        Self {
            currency: Some(currency.into()),
            ..self
        }
    }

    /// Set the range start (unix seconds)
    #[inline]
    #[must_use]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    /// Set the range end (unix seconds)
    #[inline]
    #[must_use]
    pub fn end(self, end: i64) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}

/// Options for the fund orders list.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundOrdersOptions {
    /// Filter by fund symbols
    #[serde(rename = "symbol", skip_serializing_if = "Vec::is_empty")]
    symbols: Vec<String>,
    /// Filter by actions (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<String>,
    /// Filter by states (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    states: Option<String>,
    /// Filter by currency
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    /// Range start (unix seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,
    /// Range end (unix seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<i64>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
}

impl GetFundOrdersOptions {
    /// Create a new [`GetFundOrdersOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by fund symbols
    #[inline]
    #[must_use]
    pub fn symbols<I, T>(self, symbols: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            symbols: symbols.into_iter().map(Into::into).collect(),
            ..self
        }
    }

    /// Filter by actions
    #[inline]
    #[must_use]
    pub fn actions(self, actions: impl Into<String>) -> Self {
        Self {
            actions: Some(actions.into()),
            ..self
        }
    }

    /// Filter by states
    #[inline]
    #[must_use]
    pub fn states(self, states: impl Into<String>) -> Self {
        Self {
            states: Some(states.into()),
            ..self
        }
    }

    /// Filter by currency
    #[inline]
    #[must_use]
    pub fn currency(self, currency: impl Into<String>) -> Self {
        Self {
            currency: Some(currency.into()),
            ..self
        }
    }

    /// Set the range start (unix seconds)
    #[inline]
    #[must_use]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    /// Set the range end (unix seconds)
    #[inline]
    #[must_use]
    pub fn end(self, end: i64) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}

/// Options for the fund transactions (cash-flow) list.
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetFundTransactionsOptions {
    /// Account channel
    #[serde(skip_serializing_if = "Option::is_none")]
    account_channel: Option<String>,
    /// Business type
    #[serde(skip_serializing_if = "Option::is_none")]
    business_type: Option<String>,
    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    /// Currencies (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    currencies: Option<String>,
    /// Range start (unix seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,
    /// Range end (unix seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<i64>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
}

impl GetFundTransactionsOptions {
    /// Create a new [`GetFundTransactionsOptions`]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account channel
    #[inline]
    #[must_use]
    pub fn account_channel(self, account_channel: impl Into<String>) -> Self {
        Self {
            account_channel: Some(account_channel.into()),
            ..self
        }
    }

    /// Set the business type
    #[inline]
    #[must_use]
    pub fn business_type(self, business_type: impl Into<String>) -> Self {
        Self {
            business_type: Some(business_type.into()),
            ..self
        }
    }

    /// Set the category
    #[inline]
    #[must_use]
    pub fn category(self, category: impl Into<String>) -> Self {
        Self {
            category: Some(category.into()),
            ..self
        }
    }

    /// Set the currencies
    #[inline]
    #[must_use]
    pub fn currencies(self, currencies: impl Into<String>) -> Self {
        Self {
            currencies: Some(currencies.into()),
            ..self
        }
    }

    /// Set the range start (unix seconds)
    #[inline]
    #[must_use]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    /// Set the range end (unix seconds)
    #[inline]
    #[must_use]
    pub fn end(self, end: i64) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}

/// Options for validating a fund order.
#[derive(Debug, Clone, Serialize)]
pub struct ValidateFundOrderOptions {
    /// Fund symbol
    symbol: String,
    /// Action (buy/sell)
    action: String,
    /// Currency
    currency: String,
    /// Amount (for amount-based orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<String>,
    /// Units (for unit-based orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<String>,
    /// Dividend option
    #[serde(skip_serializing_if = "Option::is_none")]
    dividend_option: Option<i32>,
    /// Fund source
    #[serde(skip_serializing_if = "Option::is_none")]
    fund_source: Option<i32>,
    /// Account channel
    #[serde(skip_serializing_if = "Option::is_none")]
    account_channel: Option<String>,
}

impl ValidateFundOrderOptions {
    /// Create a new [`ValidateFundOrderOptions`]
    #[inline]
    #[must_use]
    pub fn new(
        symbol: impl Into<String>,
        action: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            action: action.into(),
            currency: currency.into(),
            amount: None,
            units: None,
            dividend_option: None,
            fund_source: None,
            account_channel: None,
        }
    }

    /// Set the amount
    #[inline]
    #[must_use]
    pub fn amount(self, amount: impl Into<String>) -> Self {
        Self {
            amount: Some(amount.into()),
            ..self
        }
    }

    /// Set the units
    #[inline]
    #[must_use]
    pub fn units(self, units: impl Into<String>) -> Self {
        Self {
            units: Some(units.into()),
            ..self
        }
    }

    /// Set the dividend option
    #[inline]
    #[must_use]
    pub fn dividend_option(self, dividend_option: i32) -> Self {
        Self {
            dividend_option: Some(dividend_option),
            ..self
        }
    }

    /// Set the fund source
    #[inline]
    #[must_use]
    pub fn fund_source(self, fund_source: i32) -> Self {
        Self {
            fund_source: Some(fund_source),
            ..self
        }
    }

    /// Set the account channel
    #[inline]
    #[must_use]
    pub fn account_channel(self, account_channel: impl Into<String>) -> Self {
        Self {
            account_channel: Some(account_channel.into()),
            ..self
        }
    }
}

/// Options for submitting a fund order.
#[derive(Debug, Clone, Serialize)]
pub struct SubmitFundOrderOptions {
    /// Fund symbol
    symbol: String,
    /// Action (buy/sell)
    action: String,
    /// Currency
    currency: String,
    /// Amount (for amount-based orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<String>,
    /// Units (for unit-based orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<String>,
    /// Dividend option
    #[serde(skip_serializing_if = "Option::is_none")]
    dividend_option: Option<i32>,
    /// Fee
    #[serde(skip_serializing_if = "Option::is_none")]
    fee: Option<String>,
    /// Whether to sell all
    #[serde(skip_serializing_if = "Option::is_none")]
    is_sell_all: Option<bool>,
    /// Remark
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>,
    /// Trade method
    #[serde(skip_serializing_if = "Option::is_none")]
    trade_method: Option<i32>,
}

impl SubmitFundOrderOptions {
    /// Create a new [`SubmitFundOrderOptions`]
    #[inline]
    #[must_use]
    pub fn new(
        symbol: impl Into<String>,
        action: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            action: action.into(),
            currency: currency.into(),
            amount: None,
            units: None,
            dividend_option: None,
            fee: None,
            is_sell_all: None,
            remark: None,
            trade_method: None,
        }
    }

    /// Set the amount
    #[inline]
    #[must_use]
    pub fn amount(self, amount: impl Into<String>) -> Self {
        Self {
            amount: Some(amount.into()),
            ..self
        }
    }

    /// Set the units
    #[inline]
    #[must_use]
    pub fn units(self, units: impl Into<String>) -> Self {
        Self {
            units: Some(units.into()),
            ..self
        }
    }

    /// Set the dividend option
    #[inline]
    #[must_use]
    pub fn dividend_option(self, dividend_option: i32) -> Self {
        Self {
            dividend_option: Some(dividend_option),
            ..self
        }
    }

    /// Set the fee
    #[inline]
    #[must_use]
    pub fn fee(self, fee: impl Into<String>) -> Self {
        Self {
            fee: Some(fee.into()),
            ..self
        }
    }

    /// Set whether to sell all
    #[inline]
    #[must_use]
    pub fn is_sell_all(self, is_sell_all: bool) -> Self {
        Self {
            is_sell_all: Some(is_sell_all),
            ..self
        }
    }

    /// Set the remark
    #[inline]
    #[must_use]
    pub fn remark(self, remark: impl Into<String>) -> Self {
        Self {
            remark: Some(remark.into()),
            ..self
        }
    }

    /// Set the trade method
    #[inline]
    #[must_use]
    pub fn trade_method(self, trade_method: i32) -> Self {
        Self {
            trade_method: Some(trade_method),
            ..self
        }
    }
}
