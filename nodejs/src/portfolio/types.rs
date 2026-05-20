use longbridge::portfolio::types as lb;

/// One currency exchange rate
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ExchangeRate {
    /// Average rate (base_currency / other_currency)
    pub average_rate: f64,
    /// Base currency, e.g. `"USD"`
    pub base_currency: String,
    /// Bid rate
    pub bid_rate: f64,
    /// Offer rate
    pub offer_rate: f64,
    /// Other currency, e.g. `"HKD"`
    pub other_currency: String,
}
impl From<lb::ExchangeRate> for ExchangeRate {
    fn from(v: lb::ExchangeRate) -> Self {
        Self {
            average_rate: v.average_rate,
            base_currency: v.base_currency,
            bid_rate: v.bid_rate,
            offer_rate: v.offer_rate,
            other_currency: v.other_currency,
        }
    }
}

/// Response for exchange rate query
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ExchangeRates {
    /// List of exchange rates
    pub exchanges: Vec<ExchangeRate>,
}
impl From<lb::ExchangeRates> for ExchangeRates {
    fn from(v: lb::ExchangeRates) -> Self {
        Self {
            exchanges: v.exchanges.into_iter().map(Into::into).collect(),
        }
    }
}

/// P&L summary for one asset category
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitSummaryInfo {
    /// Asset type
    pub asset_type: crate::types::AssetType,
    /// Security with the maximum profit
    pub profit_max: String,
    /// Name of the max-profit security
    pub profit_max_name: String,
    /// Security with the maximum loss
    pub loss_max: String,
    /// Name of the max-loss security
    pub loss_max_name: String,
}
impl From<lb::ProfitSummaryInfo> for ProfitSummaryInfo {
    fn from(v: lb::ProfitSummaryInfo) -> Self {
        Self {
            asset_type: v.asset_type.into(),
            profit_max: v.profit_max,
            profit_max_name: v.profit_max_name,
            loss_max: v.loss_max,
            loss_max_name: v.loss_max_name,
        }
    }
}

/// P&L breakdown by asset type
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitSummaryBreakdown {
    /// Stock P&L
    pub stock: Option<String>,
    /// Fund P&L
    pub fund: Option<String>,
    /// Crypto P&L
    pub crypto: Option<String>,
    /// Money market fund P&L
    pub mmf: Option<String>,
    /// Other P&L
    pub other: Option<String>,
    /// Cumulative transaction amount
    pub cumulative_transaction_amount: Option<String>,
    /// Total number of orders
    pub trade_order_num: String,
    /// Total number of traded securities
    pub trade_stock_num: String,
    /// IPO P&L
    pub ipo: Option<String>,
    /// IPO hits
    pub ipo_hit: i32,
    /// IPO subscriptions
    pub ipo_subscription: i32,
    /// Per-category summary info
    pub summary_info: Vec<ProfitSummaryInfo>,
}
impl From<lb::ProfitSummaryBreakdown> for ProfitSummaryBreakdown {
    fn from(v: lb::ProfitSummaryBreakdown) -> Self {
        Self {
            stock: v.stock.map(|d| d.to_string()),
            fund: v.fund.map(|d| d.to_string()),
            crypto: v.crypto.map(|d| d.to_string()),
            mmf: v.mmf.map(|d| d.to_string()),
            other: v.other.map(|d| d.to_string()),
            cumulative_transaction_amount: v.cumulative_transaction_amount.map(|d| d.to_string()),
            trade_order_num: v.trade_order_num,
            trade_stock_num: v.trade_stock_num,
            ipo: v.ipo.map(|d| d.to_string()),
            ipo_hit: v.ipo_hit,
            ipo_subscription: v.ipo_subscription,
            summary_info: v.summary_info.into_iter().map(Into::into).collect(),
        }
    }
}

/// Account-level P&L summary
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisSummary {
    /// Account currency
    pub currency: String,
    /// Current total asset value
    pub current_total_asset: Option<String>,
    /// Query start date string
    pub start_date: String,
    /// Query end date string
    pub end_date: String,
    /// Start time (unix timestamp string)
    pub start_time: String,
    /// End time (unix timestamp string)
    pub end_time: String,
    /// Ending asset value
    pub ending_asset_value: Option<String>,
    /// Initial asset value
    pub initial_asset_value: Option<String>,
    /// Total invested amount
    pub invest_amount: Option<String>,
    /// Whether any trades occurred
    pub is_traded: bool,
    /// Total profit/loss
    pub sum_profit: Option<String>,
    /// Total profit/loss rate
    pub sum_profit_rate: Option<String>,
    /// Per-asset-type breakdown
    pub profits: ProfitSummaryBreakdown,
}
impl From<lb::ProfitAnalysisSummary> for ProfitAnalysisSummary {
    fn from(v: lb::ProfitAnalysisSummary) -> Self {
        Self {
            currency: v.currency,
            current_total_asset: v.current_total_asset.map(|d| d.to_string()),
            start_date: v.start_date,
            end_date: v.end_date,
            start_time: v.start_time,
            end_time: v.end_time,
            ending_asset_value: v.ending_asset_value.map(|d| d.to_string()),
            initial_asset_value: v.initial_asset_value.map(|d| d.to_string()),
            invest_amount: v.invest_amount.map(|d| d.to_string()),
            is_traded: v.is_traded,
            sum_profit: v.sum_profit.map(|d| d.to_string()),
            sum_profit_rate: v.sum_profit_rate.map(|d| d.to_string()),
            profits: v.profits.into(),
        }
    }
}

/// P&L for one security
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisItem {
    /// Security name
    pub name: String,
    /// Market
    pub market: String,
    /// Whether still holding
    pub is_holding: bool,
    /// Profit/loss amount
    pub profit: Option<String>,
    /// Profit/loss rate
    pub profit_rate: Option<String>,
    /// Number of completed trades
    pub clearance_times: i64,
    /// Asset type
    pub item_type: crate::types::AssetType,
    /// Currency
    pub currency: String,
    /// Security symbol
    pub symbol: String,
    /// Holding period display string
    pub holding_period: String,
    /// Ticker code
    pub security_code: String,
    /// ISIN (for funds)
    pub isin: String,
    /// Underlying stock P&L
    pub underlying_profit: Option<String>,
    /// Derivatives P&L
    pub derivatives_profit: Option<String>,
    /// P&L in order currency
    pub order_profit: Option<String>,
}
impl From<lb::ProfitAnalysisItem> for ProfitAnalysisItem {
    fn from(v: lb::ProfitAnalysisItem) -> Self {
        Self {
            name: v.name,
            market: v.market,
            is_holding: v.is_holding,
            profit: v.profit.map(|d| d.to_string()),
            profit_rate: v.profit_rate.map(|d| d.to_string()),
            clearance_times: v.clearance_times,
            item_type: v.item_type.into(),
            currency: v.currency,
            symbol: v.symbol,
            holding_period: v.holding_period,
            security_code: v.security_code,
            isin: v.isin,
            underlying_profit: v.underlying_profit.map(|d| d.to_string()),
            derivatives_profit: v.derivatives_profit.map(|d| d.to_string()),
            order_profit: v.order_profit.map(|d| d.to_string()),
        }
    }
}

/// Per-security P&L breakdown
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisSublist {
    /// Start time (unix timestamp string)
    pub start: String,
    /// End time (unix timestamp string)
    pub end: String,
    /// Start date string
    pub start_date: String,
    /// End date string
    pub end_date: String,
    /// Last updated time (unix timestamp string)
    pub updated_at: String,
    /// Last updated date string
    pub updated_date: String,
    /// Per-security items
    pub items: Vec<ProfitAnalysisItem>,
}
impl From<lb::ProfitAnalysisSublist> for ProfitAnalysisSublist {
    fn from(v: lb::ProfitAnalysisSublist) -> Self {
        Self {
            start: v.start,
            end: v.end,
            start_date: v.start_date,
            end_date: v.end_date,
            updated_at: v.updated_at,
            updated_date: v.updated_date,
            items: v.items.into_iter().map(Into::into).collect(),
        }
    }
}

/// Combined profit analysis response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysis {
    /// Summary overview
    pub summary: ProfitAnalysisSummary,
    /// Per-security breakdown
    pub sublist: ProfitAnalysisSublist,
}
impl From<lb::ProfitAnalysis> for ProfitAnalysis {
    fn from(v: lb::ProfitAnalysis) -> Self {
        Self {
            summary: v.summary.into(),
            sublist: v.sublist.into(),
        }
    }
}

/// One P&L detail line item (credit, debit, or fee)
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitDetailEntry {
    /// Description
    pub describe: String,
    /// Amount
    pub amount: Option<String>,
}
impl From<lb::ProfitDetailEntry> for ProfitDetailEntry {
    fn from(v: lb::ProfitDetailEntry) -> Self {
        Self {
            describe: v.describe,
            amount: v.amount.map(|d| d.to_string()),
        }
    }
}

/// Detailed P&L breakdown for one asset class
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitDetails {
    /// Current holding market value
    pub holding_value: Option<String>,
    /// Total profit/loss
    pub profit: Option<String>,
    /// Cumulative credited amount
    pub cumulative_credited_amount: Option<String>,
    /// Credit detail entries
    pub credited_details: Vec<ProfitDetailEntry>,
    /// Cumulative debited amount
    pub cumulative_debited_amount: Option<String>,
    /// Debit detail entries
    pub debited_details: Vec<ProfitDetailEntry>,
    /// Cumulative fee amount
    pub cumulative_fee_amount: Option<String>,
    /// Fee detail entries
    pub fee_details: Vec<ProfitDetailEntry>,
    /// Short position holding value
    pub short_holding_value: Option<String>,
    /// Long position holding value
    pub long_holding_value: Option<String>,
    /// Opening position market value at period start
    pub holding_value_at_beginning: Option<String>,
    /// Closing position market value at period end
    pub holding_value_at_ending: Option<String>,
}
impl From<lb::ProfitDetails> for ProfitDetails {
    fn from(v: lb::ProfitDetails) -> Self {
        Self {
            holding_value: v.holding_value.map(|d| d.to_string()),
            profit: v.profit.map(|d| d.to_string()),
            cumulative_credited_amount: v.cumulative_credited_amount.map(|d| d.to_string()),
            credited_details: v.credited_details.into_iter().map(Into::into).collect(),
            cumulative_debited_amount: v.cumulative_debited_amount.map(|d| d.to_string()),
            debited_details: v.debited_details.into_iter().map(Into::into).collect(),
            cumulative_fee_amount: v.cumulative_fee_amount.map(|d| d.to_string()),
            fee_details: v.fee_details.into_iter().map(Into::into).collect(),
            short_holding_value: v.short_holding_value.map(|d| d.to_string()),
            long_holding_value: v.long_holding_value.map(|d| d.to_string()),
            holding_value_at_beginning: v.holding_value_at_beginning.map(|d| d.to_string()),
            holding_value_at_ending: v.holding_value_at_ending.map(|d| d.to_string()),
        }
    }
}

/// Detailed profit analysis for one security
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisDetail {
    /// Total profit/loss
    pub profit: Option<String>,
    /// Underlying stock P&L details
    pub underlying_details: ProfitDetails,
    /// Derivative P&L details
    pub derivative_pnl_details: ProfitDetails,
    /// Security name
    pub name: String,
    /// Last updated time (unix timestamp string)
    pub updated_at: String,
    /// Last updated date string
    pub updated_date: String,
    /// Currency
    pub currency: String,
    /// Default detail tab: 0 = underlying, 1 = derivative
    pub default_tag: i32,
    /// Query start time (unix timestamp string)
    pub start: String,
    /// Query end time (unix timestamp string)
    pub end: String,
    /// Query start date string
    pub start_date: String,
    /// Query end date string
    pub end_date: String,
}
impl From<lb::ProfitAnalysisDetail> for ProfitAnalysisDetail {
    fn from(v: lb::ProfitAnalysisDetail) -> Self {
        Self {
            profit: v.profit.map(|d| d.to_string()),
            underlying_details: v.underlying_details.into(),
            derivative_pnl_details: v.derivative_pnl_details.into(),
            name: v.name,
            updated_at: v.updated_at,
            updated_date: v.updated_date,
            currency: v.currency,
            default_tag: v.default_tag,
            start: v.start,
            end: v.end,
            start_date: v.start_date,
            end_date: v.end_date,
        }
    }
}

/// One security entry in a by-market P&L response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisByMarketItem {
    /// Security symbol (ticker code)
    pub code: String,
    /// Security name
    pub name: String,
    /// Market, e.g. `"HK"`, `"US"`
    pub market: String,
    /// Profit/loss amount
    pub profit: Option<String>,
}
impl From<lb::ProfitAnalysisByMarketItem> for ProfitAnalysisByMarketItem {
    fn from(v: lb::ProfitAnalysisByMarketItem) -> Self {
        Self {
            code: v.code,
            name: v.name,
            market: v.market,
            profit: v.profit.map(|d| d.to_string()),
        }
    }
}

/// P&L analysis grouped by market
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisByMarket {
    /// Total P&L across all returned items
    pub profit: Option<String>,
    /// Whether more pages are available
    pub has_more: bool,
    /// Per-security P&L items for the requested market/page
    pub stock_items: Vec<ProfitAnalysisByMarketItem>,
}
impl From<lb::ProfitAnalysisByMarket> for ProfitAnalysisByMarket {
    fn from(v: lb::ProfitAnalysisByMarket) -> Self {
        Self {
            profit: v.profit.map(|d| d.to_string()),
            has_more: v.has_more,
            stock_items: v.stock_items.into_iter().map(Into::into).collect(),
        }
    }
}

// ── ProfitAnalysisFlows ───────────────────────────────────────────

/// One profit-analysis flow record
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct FlowItem {
    pub executed_date: String,
    /// Execution timestamp as a JSON value string
    pub executed_timestamp: String,
    pub code: String,
    pub direction: crate::types::FlowDirection,
    pub executed_quantity: Option<String>,
    pub executed_price: Option<String>,
    pub executed_cost: Option<String>,
    pub describe: String,
}

impl From<lb::FlowItem> for FlowItem {
    fn from(v: lb::FlowItem) -> Self {
        Self {
            executed_date: v.executed_date,
            executed_timestamp: v.executed_timestamp.to_string(),
            code: v.code,
            direction: v.direction.into(),
            executed_quantity: v.executed_quantity.map(|d| d.to_string()),
            executed_price: v.executed_price.map(|d| d.to_string()),
            executed_cost: v.executed_cost.map(|d| d.to_string()),
            describe: v.describe,
        }
    }
}

/// Profit-analysis flows response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ProfitAnalysisFlows {
    pub flows_list: Vec<FlowItem>,
    pub has_more: bool,
}

impl From<lb::ProfitAnalysisFlows> for ProfitAnalysisFlows {
    fn from(v: lb::ProfitAnalysisFlows) -> Self {
        Self {
            flows_list: v.flows_list.into_iter().map(Into::into).collect(),
            has_more: v.has_more,
        }
    }
}
