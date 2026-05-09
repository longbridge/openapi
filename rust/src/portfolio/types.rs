#![allow(missing_docs)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::utils::counter::deserialize_counter_id_as_symbol;

// ── exchange_rate ─────────────────────────────────────────────────

/// Response for [`crate::PortfolioContext::exchange_rate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRates {
    /// List of exchange rates
    pub exchanges: Vec<ExchangeRate>,
}

/// One currency exchange rate
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// ── profit_analysis ───────────────────────────────────────────────

/// Summary response for [`crate::PortfolioContext::profit_analysis`]
///
/// This is a combined response from two API endpoints:
/// `/v1/portfolio/profit-analysis-summary` and
/// `/v1/portfolio/profit-analysis-sublist`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysis {
    /// Summary overview
    pub summary: ProfitAnalysisSummary,
    /// Per-security breakdown
    pub sublist: ProfitAnalysisSublist,
}

/// Account-level P&L summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysisSummary {
    /// Account currency
    pub currency: String,
    /// Current total asset value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub current_total_asset: Option<Decimal>,
    /// Query start date string
    pub start_date: String,
    /// Query end date string
    pub end_date: String,
    /// Start time (unix timestamp string)
    pub start_time: String,
    /// End time (unix timestamp string)
    pub end_time: String,
    /// Ending asset value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub ending_asset_value: Option<Decimal>,
    /// Initial asset value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub initial_asset_value: Option<Decimal>,
    /// Total invested amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub invest_amount: Option<Decimal>,
    /// Whether any trades occurred
    pub is_traded: bool,
    /// Total profit/loss
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub sum_profit: Option<Decimal>,
    /// Total profit/loss rate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub sum_profit_rate: Option<Decimal>,
    /// Per-asset-type breakdown
    pub profits: ProfitSummaryBreakdown,
}

/// P&L breakdown by asset type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSummaryBreakdown {
    /// Stock P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub stock: Option<Decimal>,
    /// Fund P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub fund: Option<Decimal>,
    /// Crypto P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub crypto: Option<Decimal>,
    /// Money market fund P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub mmf: Option<Decimal>,
    /// Other P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub other: Option<Decimal>,
    /// Cumulative transaction amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub cumulative_transaction_amount: Option<Decimal>,
    /// Total number of orders
    pub trade_order_num: String,
    /// Total number of traded securities
    pub trade_stock_num: String,
    /// IPO P&L
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub ipo: Option<Decimal>,
    /// IPO hits
    pub ipo_hit: i32,
    /// IPO subscriptions
    pub ipo_subscription: i32,
    /// Per-category summary info
    pub summary_info: Vec<ProfitSummaryInfo>,
}

/// P&L summary for one asset category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSummaryInfo {
    /// Asset type
    pub asset_type: AssetType,
    /// Security with the maximum profit
    pub profit_max: String,
    /// Name of the max-profit security
    pub profit_max_name: String,
    /// Security with the maximum loss
    pub loss_max: String,
    /// Name of the max-loss security
    pub loss_max_name: String,
}

/// Per-security P&L breakdown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// P&L for one security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysisItem {
    /// Security name
    pub name: String,
    /// Market
    pub market: String,
    /// Whether still holding
    pub is_holding: bool,
    /// Profit/loss amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub profit: Option<Decimal>,
    /// Profit/loss rate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub profit_rate: Option<Decimal>,
    /// Number of completed trades
    pub clearance_times: i64,
    /// Asset type
    #[serde(rename = "type")]
    pub item_type: AssetType,
    /// Currency
    pub currency: String,
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Holding period display string
    #[serde(default)]
    pub holding_period: String,
    /// Ticker code
    pub security_code: String,
    /// ISIN (for funds)
    pub isin: String,
    /// Underlying stock P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub underlying_profit: Option<Decimal>,
    /// Derivatives P&L
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub derivatives_profit: Option<Decimal>,
    /// P&L in order currency
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub order_profit: Option<Decimal>,
}

// ── profit_analysis_detail ────────────────────────────────────────

/// Response for [`crate::PortfolioContext::profit_analysis_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysisDetail {
    /// Total profit/loss
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub profit: Option<Decimal>,
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

/// Detailed P&L breakdown for one asset class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitDetails {
    /// Current holding market value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub holding_value: Option<Decimal>,
    /// Total profit/loss
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub profit: Option<Decimal>,
    /// Cumulative credited amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub cumulative_credited_amount: Option<Decimal>,
    /// Credit detail entries
    pub credited_details: Vec<ProfitDetailEntry>,
    /// Cumulative debited amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub cumulative_debited_amount: Option<Decimal>,
    /// Debit detail entries
    pub debited_details: Vec<ProfitDetailEntry>,
    /// Cumulative fee amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub cumulative_fee_amount: Option<Decimal>,
    /// Fee detail entries
    pub fee_details: Vec<ProfitDetailEntry>,
    /// Short position holding value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub short_holding_value: Option<Decimal>,
    /// Long position holding value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub long_holding_value: Option<Decimal>,
    /// Opening position market value at period start
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub holding_value_at_beginning: Option<Decimal>,
    /// Closing position market value at period end
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub holding_value_at_ending: Option<Decimal>,
}

/// One P&L detail line item (credit, debit, or fee)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitDetailEntry {
    /// Description
    pub describe: String,
    /// Amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub amount: Option<Decimal>,
}

// ── profit_analysis_by_market ─────────────────────────────────────

/// Response for [`crate::PortfolioContext::profit_analysis_by_market`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysisByMarket {
    /// Total P&L across all returned items
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub profit: Option<Decimal>,
    /// Whether more pages are available
    #[serde(default)]
    pub has_more: bool,
    /// Per-security P&L items for the requested market/page
    #[serde(default)]
    pub stock_items: Vec<ProfitAnalysisByMarketItem>,
}

/// One security entry in a by-market P&L response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysisByMarketItem {
    /// Security symbol (ticker code)
    pub code: String,
    /// Security name
    pub name: String,
    /// Market, e.g. `"HK"`, `"US"`
    pub market: String,
    /// Profit/loss amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub profit: Option<Decimal>,
}

// ── profit_analysis_flows ─────────────────────────────────────────

/// Response for [`crate::PortfolioContext::profit_analysis_flows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysisFlows {
    /// Paginated list of flow items
    #[serde(default)]
    pub flows_list: Vec<FlowItem>,
    /// Whether there are more pages
    #[serde(default)]
    pub has_more: bool,
}

/// One profit-analysis flow record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowItem {
    /// Execution date string, e.g. `"2024-01-15"`
    #[serde(default)]
    pub executed_date: String,
    /// Execution timestamp (may be int or string)
    #[serde(default)]
    pub executed_timestamp: serde_json::Value,
    /// Security code / ticker
    #[serde(default)]
    pub code: String,
    /// Direction of the flow
    pub direction: FlowDirection,
    /// Executed quantity
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub executed_quantity: Option<Decimal>,
    /// Executed price
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub executed_price: Option<Decimal>,
    /// Executed cost
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub executed_cost: Option<Decimal>,
    /// Human-readable description
    #[serde(default)]
    pub describe: String,
}

/// Flow direction
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum FlowDirection {
    /// Unknown
    Unknown,
    /// Buy
    #[strum(serialize = "buy")]
    Buy,
    /// Sell
    #[strum(serialize = "sell")]
    Sell,
}

impl_default_for_enum_string!(FlowDirection);
impl_serde_for_enum_string!(FlowDirection);

/// Asset type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum AssetType {
    /// Unknown
    Unknown,
    /// Stock
    #[strum(serialize = "stock")]
    Stock,
    /// Fund
    #[strum(serialize = "fund")]
    Fund,
    /// Crypto
    #[strum(serialize = "crypto")]
    Crypto,
}

impl_default_for_enum_string!(AssetType);
impl_serde_for_enum_string!(AssetType);
