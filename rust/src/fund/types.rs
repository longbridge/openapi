//! Fund (mutual fund) types.
//!
//! Response models for the fund channel OpenAPI endpoints. Numeric fields that
//! arrive as strings on the wire are kept as [`String`] to preserve the exact
//! server formatting; unix-second/millisecond timestamps are kept as `i64`.
//! Every struct uses `#[serde(default)]` so missing fields decode to their
//! default instead of failing.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A fund net-asset-value data point (latest / historical).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundNavValue {
    /// Net value change
    pub change: String,
    /// Net value change percent
    pub change_percent: String,
    /// Formatted change percent
    pub change_percent_format: String,
    /// Fund counter id
    pub counter_id: String,
    /// Fund name
    pub counter_name: String,
    /// Currency
    pub currency: String,
    /// Formatted date
    pub date_format: String,
    /// ISIN
    pub isin: String,
    /// Last update time (unix seconds)
    pub last_update_time: i64,
    /// Net value
    pub value: String,
    /// Formatted net value
    pub value_format: String,
}

/// A recent performance point used by the hot-fund list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPerformancePoint {
    /// Date
    pub date: String,
    /// Last done value
    pub last_done: String,
}

/// A hot-selling fund entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HotFund {
    /// Asset class
    pub asset_class: i32,
    /// Asset class name
    pub asset_class_name: String,
    /// Fund counter id
    pub counter_id: String,
    /// Currency
    pub currency: String,
    /// Earning rate
    pub earning_rate: String,
    /// Recent performance points
    pub fund_performances: Vec<FundPerformancePoint>,
    /// Fund name
    pub name: String,
    /// Minimum purchase amount
    pub purchase_amount: String,
    /// Recommendation text
    pub recommendation_text: String,
    /// Risk level
    pub risk_level: i32,
    /// Risk level name
    pub risk_level_name: String,
    /// Time interval of the earning rate
    pub time_interval: String,
}

/// A fund entry in the fund list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundBrief {
    /// Asset class
    pub asset_class: i32,
    /// Asset class name
    pub asset_class_name: String,
    /// Fund code
    pub code: String,
    /// Fund counter id
    pub counter_id: String,
    /// Currency
    pub currency: String,
    /// Description
    pub description: String,
    /// Earning rate
    pub earning_rate: String,
    /// Whether the user is holding this fund
    pub holding: bool,
    /// ISIN
    pub isin: String,
    /// Fund name
    pub name: String,
    /// Product
    pub product: String,
    /// Minimum purchase amount
    pub purchase_amount: String,
    /// Recommendation text
    pub recommendation_text: String,
    /// Risk level
    pub risk_level: i32,
    /// Risk level name
    pub risk_level_name: String,
    /// Time interval of the earning rate
    pub time_interval: String,
    /// Unit value
    pub unit_value: String,
}

/// Fund list filter options. Each list holds server-defined option objects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundFilters {
    /// Asset class options
    pub asset_class: Vec<Value>,
    /// Company options
    pub company: Vec<Value>,
    /// Currency options
    pub currency: Vec<Value>,
    /// Industry category options
    pub industry_category_name: Vec<Value>,
    /// Risk level options
    pub risk_level: Vec<Value>,
}

/// A single holding entry inside a fund's asset allocation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundAssetAllocationItem {
    /// Security code
    pub code: String,
    /// Security counter id
    pub counter_id: String,
    /// Name
    pub name: String,
    /// Position ratio
    pub position_ratio: String,
}

/// A fund's asset allocation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundAssetAllocation {
    /// Asset type
    pub asset_type: i32,
    /// Allocation entries
    pub lists: Vec<FundAssetAllocationItem>,
    /// Report date
    pub report_date: String,
}

/// Fund detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundDetail {
    /// Additional purchase amount
    pub additional_purchase_amount: String,
    /// Affirm day
    pub affirm_day: i32,
    /// Amount affirm day
    pub amount_affirm_day: String,
    /// Asset allocation
    pub asset_allocation: FundAssetAllocation,
    /// Asset class
    pub asset_class: i32,
    /// Asset class name
    pub asset_class_name: String,
    /// Bill purchase rate
    pub bill_purchase_rate: String,
    /// Channel
    pub channel: String,
    /// Close period
    pub close_period: String,
    /// Fund code
    pub code: String,
    /// Currency
    pub currency: String,
    /// Cut off time
    pub cut_off_time: String,
    /// Whether it is a derivative
    pub derivatives: bool,
    /// Done day
    pub done_day: i32,
    /// Excess return fee
    pub excess_return_fee: String,
    /// GST rate
    pub gst_rate: String,
    /// Introduction
    pub introduce: String,
    /// Whether it is a cash-plus fund
    pub is_cash_plus: bool,
    /// Whether it is a complex product
    pub is_complex: bool,
    /// Whether it is a new cash-plus fund
    pub is_new_cash_plus: bool,
    /// Whether it is a Yinghebao fund
    pub is_yinghebao: bool,
    /// ISIN
    pub isin: String,
    /// Management rate
    pub manage_rate: String,
    /// Manager
    pub manager: String,
    /// Minimum holding cash
    pub min_hold_cash: String,
    /// Minimum holding share
    pub min_hold_share: String,
    /// Minimum sell share
    pub min_sell_share: String,
    /// Month raise day
    pub month_raise_day: String,
    /// Fund name
    pub name: String,
    /// Net value deadline
    pub nav_deadline: String,
    /// Whether it is no-load
    pub no_load: bool,
    /// Open date
    pub open_date: String,
    /// Open period
    pub open_period: String,
    /// Product
    pub product: String,
    /// Product information locals
    pub product_information_locals: String,
    /// Profile
    pub profile: String,
    /// Whether purchasable
    pub purchasable: i32,
    /// Purchase affirm day
    pub purchase_affirm_day: String,
    /// Minimum purchase amount
    pub purchase_amount: String,
    /// Purchase rate
    pub purchase_rate: String,
    /// Rating
    pub rating: i32,
    /// Whether redeemable
    pub redeemable: i32,
    /// Redemption advance day
    pub redemption_advance_day: String,
    /// Redemption amount
    pub redemption_amount: String,
    /// Redemption close period text
    pub redemption_close_period_shows: String,
    /// Redemption done day
    pub redemption_done_day: String,
    /// Redemption open day text
    pub redemption_open_day_shows: String,
    /// Risk level
    pub risk_level: i32,
    /// Risk level name
    pub risk_level_name: String,
    /// Verify status
    pub verify_status: i32,
    /// Whether it is a virtual currency fund
    pub virtual_currency: bool,
    /// Year to date yield
    pub year_to_date_yield: String,
    /// Year to date yield type
    pub ytd_yield_type: i32,
}

/// Fund analysis (level 1).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundAnalysis {
    /// Actual period
    pub actual_period: i32,
    /// Cost level (server-defined structure)
    pub cost_level: Value,
    /// Return ability (server-defined structure)
    pub return_ability: Value,
    /// Risk ability (server-defined structure)
    pub risk_ability: Value,
    /// Updated at
    pub updated_at: String,
    /// Value for money (server-defined structure)
    pub value_for_money: Value,
    /// Whether visible
    pub visible: bool,
}

/// Fund analysis detail (level 2).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundAnalysisDetail {
    /// Actual period
    pub actual_period: i32,
    /// Available periods
    pub available_periods: Vec<i32>,
    /// Cost level (server-defined structure)
    pub cost_level: Value,
    /// Return ability (server-defined structure)
    pub return_ability: Value,
    /// Risk ability (server-defined structure)
    pub risk_ability: Value,
    /// Updated at
    pub updated_at: String,
    /// Value for money (server-defined structure)
    pub value_for_money: Value,
    /// Whether visible
    pub visible: bool,
}

/// A benchmark contrast series in a fund trend chart.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundTrendContrast {
    /// Benchmark name
    pub benchmark_name: String,
    /// Performance points (server-defined structure)
    pub performances: Vec<Value>,
}

/// Fund trend chart.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundTrend {
    /// Actual period
    pub actual_period: i32,
    /// Available periods
    pub available_periods: Vec<i32>,
    /// Category average performances (server-defined structure)
    pub category_average_performances: Vec<Value>,
    /// Benchmark contrast performances
    pub contrast_performances: FundTrendContrast,
    /// Fund performances (server-defined structure)
    pub fund_performances: Vec<Value>,
}

/// A named contrast performance series.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundNamedContrast {
    /// Series name
    pub name: String,
    /// Performance points (server-defined structure)
    pub performances: Vec<Value>,
}

/// Fund performance comparison.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPerformanceComparison {
    /// Contrast performance series
    pub contrast_performances: Vec<FundNamedContrast>,
    /// Fund performances (server-defined structure)
    pub fund_performances: Vec<Value>,
}

/// A fund annual return entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundAnnualReturn {
    /// Change percent
    pub change_percent: String,
    /// Year
    pub year: i32,
}

/// A fund quarterly return entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundQuarterlyReturn {
    /// Change percent
    pub change_percent: String,
    /// Quarter
    pub quarter: i32,
    /// Year
    pub year: i32,
}

/// A fund's detailed performance figures.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPerformance {
    /// Annualized return (5y)
    pub annualized_return_five: String,
    /// Annualized return (1y)
    pub annualized_return_one: String,
    /// Annualized return (10y)
    pub annualized_return_ten: String,
    /// Annualized return (3y)
    pub annualized_return_three: String,
    /// Annualized return (2y)
    pub annualized_return_two: String,
    /// Fund counter id
    pub counter_id: String,
    /// Fund name
    pub fund_name: String,
    /// Rank (5y)
    pub performance_rank_five_years: i32,
    /// Rank (1d)
    pub performance_rank_one_day: i32,
    /// Rank (1m)
    pub performance_rank_one_month: i32,
    /// Rank (1w)
    pub performance_rank_one_week: i32,
    /// Rank (1y)
    pub performance_rank_one_year: i32,
    /// Rank (6m)
    pub performance_rank_six_months: i32,
    /// Rank (10y)
    pub performance_rank_ten_years: i32,
    /// Rank (3m)
    pub performance_rank_three_months: i32,
    /// Rank (3y)
    pub performance_rank_three_years: i32,
    /// Rank (2y)
    pub performance_rank_two_years: i32,
    /// Rank (ytd)
    pub performance_rank_ytd: i32,
    /// Return (5y)
    pub performance_return_five_years: String,
    /// Return (1d)
    pub performance_return_one_day: String,
    /// Return (1m)
    pub performance_return_one_month: String,
    /// Return (1w)
    pub performance_return_one_week: String,
    /// Return (1y)
    pub performance_return_one_year: String,
    /// Return (6m)
    pub performance_return_six_months: String,
    /// Return (10y)
    pub performance_return_ten_years: String,
    /// Return (3m)
    pub performance_return_three_months: String,
    /// Return (3y)
    pub performance_return_three_years: String,
    /// Return (2y)
    pub performance_return_two_years: String,
    /// Return (ytd)
    pub performance_return_ytd: String,
    /// Total peers (5y)
    pub performance_total_five_years: i32,
    /// Total peers (1d)
    pub performance_total_one_day: i32,
    /// Total peers (1m)
    pub performance_total_one_month: i32,
    /// Total peers (1w)
    pub performance_total_one_week: i32,
    /// Total peers (1y)
    pub performance_total_one_year: i32,
    /// Total peers (6m)
    pub performance_total_six_months: i32,
    /// Total peers (10y)
    pub performance_total_ten_years: i32,
    /// Total peers (3m)
    pub performance_total_three_months: i32,
    /// Total peers (3y)
    pub performance_total_three_years: i32,
    /// Total peers (2y)
    pub performance_total_two_years: i32,
    /// Total peers (ytd)
    pub performance_total_ytd: i32,
    /// Seven days annualized
    pub seven_days_annualized: String,
    /// Ten thousand price
    pub ten_thousand_price: String,
    /// Update time (unix seconds)
    pub update_time: i64,
}

/// A single fund holding (top-10 holdings).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundHolding {
    /// Bond type
    pub bond_type: String,
    /// Bond type name
    pub bond_type_name: String,
    /// Country name
    pub country_name: String,
    /// Holding type
    pub holding_type: String,
    /// Industry name
    pub industry_name: String,
    /// Market value
    pub market_value: String,
    /// Maturity date
    pub maturity_date: String,
    /// Name
    pub name: String,
    /// Share change
    pub share_change: String,
    /// Share change percent
    pub share_change_percent: String,
    /// Shares
    pub shares: String,
    /// Weighting
    pub weighting: String,
}

/// A fund's top-10 holdings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundHoldings {
    /// Holding entries
    pub holdings: Vec<FundHolding>,
    /// Report date
    pub report_date: String,
    /// Total weighting
    pub weighting: String,
}

/// A stock held by the fund (reverse lookup).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundStockHolding {
    /// Stock code
    pub code: String,
    /// Stock counter id
    pub counter_id: String,
    /// Currency
    pub currency: String,
    /// Stock name
    pub name: String,
    /// Position ratio
    pub position_ratio: String,
    /// Report date
    pub report_date: String,
}

/// A single fund position held by the user.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPosition {
    /// Holding amount
    pub amount: String,
    /// Fund counter id
    pub counter_id: String,
    /// Currency
    pub currency: String,
    /// Frozen units
    pub freeze_units: String,
    /// Holding profit
    pub holding_profit: String,
    /// Holding units
    pub holding_units: String,
    /// Fund name
    pub name: String,
    /// Recent profit
    pub recent_profit: String,
    /// Recent trading day (unix seconds)
    pub recent_trading_day: i64,
    /// Accumulated recent profit
    pub sum_recent_profit: String,
}

/// The user's fund positions overview.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPositions {
    /// Account channel
    pub account_channel: String,
    /// Position entries
    pub list: Vec<FundPosition>,
    /// Pending buy orders amount
    pub pending_buy_orders: String,
    /// Recent trading day (unix seconds)
    pub recent_trading_day: i64,
    /// Sold pending credit orders amount
    pub sold_pending_credit_orders: String,
}

/// A dated value point.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundDatedValue {
    /// Date (unix seconds)
    pub date: i64,
    /// Value
    pub value: String,
}

/// A fund unit-value point (position view).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundUnitValue {
    /// Date (unix seconds)
    pub date: i64,
    /// Day increase rate
    pub day_increase_rate: String,
    /// Total value
    pub total_value: String,
    /// Unit value
    pub unit_value: String,
}

/// Detail values of a single fund position.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPositionDetailValues {
    /// Amount
    pub amount: String,
    /// Currency
    pub currency: String,
    /// Holding cost
    pub holding_cost: String,
    /// Holding profit
    pub holding_profit: String,
    /// Holding profit rate
    pub holding_profit_rate: String,
    /// Holding units
    pub holding_units: String,
    /// Holding value
    pub holding_value: String,
    /// Pending buy value
    pub pending_buy_value: String,
    /// Pending sell value
    pub pending_sell_value: String,
    /// Accumulated profit (to date)
    pub profit_amount_accum_td: String,
    /// Accumulated profit rate (to date)
    pub profit_amount_accum_td_rate: String,
    /// Recent profit
    pub recent_profit: String,
    /// Recent trading day (unix seconds)
    pub recent_tradingday: i64,
    /// Recent unit value
    pub recent_unit_value: String,
    /// Sold pending-confirm units
    pub sold_pending_confirm_units: String,
}

/// Detail of a single fund position.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPositionDetail {
    /// Detail values
    pub detail_values: FundPositionDetailValues,
    /// Accumulated profit series
    pub sum_profit: Vec<FundDatedValue>,
    /// Unit value series
    pub ut_value: Vec<FundUnitValue>,
}

/// Performance figures for a held fund.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPositionPerformance {
    /// Annualized return (5y)
    pub annualized_return_five: String,
    /// Annualized return (1y)
    pub annualized_return_one: String,
    /// Annualized return (10y)
    pub annualized_return_ten: String,
    /// Annualized return (3y)
    pub annualized_return_three: String,
    /// Annualized return (2y)
    pub annualized_return_two: String,
    /// Fund counter id
    pub counter_id: String,
    /// Fund name
    pub fund_name: String,
    /// Return (5y)
    pub performance_return_five_years: String,
    /// Return (1d)
    pub performance_return_one_day: String,
    /// Return (1m)
    pub performance_return_one_month: String,
    /// Return (1w)
    pub performance_return_one_week: String,
    /// Return (1y)
    pub performance_return_one_year: String,
    /// Return (6m)
    pub performance_return_six_months: String,
    /// Return (10y)
    pub performance_return_ten_years: String,
    /// Return (3m)
    pub performance_return_three_months: String,
    /// Return (3y)
    pub performance_return_three_years: String,
    /// Return (2y)
    pub performance_return_two_years: String,
    /// Return (ytd)
    pub performance_return_ytd: String,
    /// Update time (unix seconds)
    pub update_time: i64,
}

/// The user's cumulative profit for a held fund.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPositionProfits {
    /// Currency
    pub currency: String,
    /// Profit series
    pub history_value: Vec<FundDatedValue>,
    /// Last update time (unix seconds)
    pub last_update_time: i64,
    /// Total profit
    pub sum_profit: String,
}

/// A held-fund net-value point (position view).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundPositionNav {
    /// Net value change
    pub change: String,
    /// Net value change percent
    pub change_percent: String,
    /// Fund counter id
    pub counter_id: String,
    /// Fund name
    pub counter_name: String,
    /// Last update time (unix seconds)
    pub last_update_time: i64,
    /// Net value
    pub value: String,
}

/// A cash dividend record for a held fund.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundDividend {
    /// Amount
    pub amount: String,
    /// Fund counter id
    pub counter_id: String,
    /// Currency
    pub currency: String,
    /// Date (unix seconds)
    pub date: i64,
    /// Dividend method
    pub div_method: String,
    /// Fund name
    pub name: String,
}

/// The user's dividend records for a held fund.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundDividends {
    /// Currency
    pub currency: String,
    /// Dividend records
    pub div_cash_infos: Vec<FundDividend>,
    /// Latest dividend date (unix seconds)
    pub lastest_date: i64,
    /// Total cash dividend
    pub total_div_cash: String,
}

/// A fund order (list view).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrder {
    /// Action (buy/sell)
    pub action: String,
    /// Amount
    pub amount: String,
    /// Fund counter id
    pub counter_id: String,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Currency
    pub currency: String,
    /// Fund name
    pub fund_name: String,
    /// Order id
    pub id: i64,
    /// Whether it is an auto (DCA) order
    pub is_auto: bool,
    /// Net worth
    pub net_worth: String,
    /// Product type
    pub product_type: String,
    /// State
    pub state: String,
    /// State description
    pub state_desc: String,
    /// Units
    pub units: String,
}

/// A keyword block in a fund order detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrderKeyword {
    /// Content
    pub content: String,
    /// Group
    pub group: String,
    /// Key
    pub key: String,
    /// Line strategy
    pub line_strategy: String,
    /// Title
    pub title: String,
}

/// A processing stage in a fund order detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrderStage {
    /// Description
    pub desc: String,
    /// Key
    pub key: String,
    /// Link
    pub link: String,
    /// Link text
    pub link_text: String,
    /// Progress
    pub progress: String,
    /// Stage
    pub stage: String,
}

/// The full information of a fund order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrderInfo {
    /// Account id
    pub aaid: i64,
    /// Account channel
    pub account_channel: String,
    /// Action (buy/sell)
    pub action: String,
    /// Amount
    pub amount: String,
    /// Channel
    pub channel: String,
    /// Fund counter id
    pub counter_id: String,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Currency
    pub currency: String,
    /// Dividend option
    pub dividend_option: String,
    /// Equity time (unix seconds)
    pub eq_at: i64,
    /// Fee
    pub fee: String,
    /// Fund name
    pub fund_name: String,
    /// Fund source
    pub fund_source: String,
    /// Histories
    pub histories: String,
    /// Order id
    pub id: i64,
    /// Message
    pub message: String,
    /// Net worth
    pub net_worth: String,
    /// Price time (unix seconds)
    pub price_at: i64,
    /// Processed at (unix seconds)
    pub processed_at: i64,
    /// Product type
    pub product_type: String,
    /// Whether repurchaseable
    pub repurchaseable: bool,
    /// Sale proceeds
    pub sale_proceeds: String,
    /// Sales charge
    pub sales_charge: String,
    /// Sales price
    pub sales_price: String,
    /// Sales unit
    pub sales_unit: String,
    /// State
    pub state: String,
    /// State description
    pub state_desc: String,
    /// Status
    pub status: i32,
    /// Extended status
    pub status_ex: i32,
    /// T+ description
    pub t_description: String,
    /// Time partition
    pub time_partition: String,
    /// Total amount
    pub total_amount: String,
    /// Transaction at (unix seconds)
    pub transaction_at: i64,
    /// Units
    pub units: String,
    /// Withdraw at (unix seconds)
    pub withdraw_at: i64,
    /// Whether withdrawable
    pub withdrawable: bool,
}

/// Fund order detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrderDetail {
    /// Keyword blocks
    pub keywords: Vec<FundOrderKeyword>,
    /// The order
    pub order: FundOrderInfo,
    /// Processing stages
    pub stages: Vec<FundOrderStage>,
}

/// A fund transaction / cash-flow record.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundTransaction {
    /// Amount
    pub amount: String,
    /// Category
    pub category: String,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Currency
    pub currency: String,
    /// Description
    pub description: String,
    /// Detail created at (unix seconds)
    pub detail_created_at: i64,
    /// Detail type
    pub detail_type: String,
    /// Done at (unix seconds)
    pub done_at: i64,
    /// Quantity description
    pub quantity_description: String,
    /// Redirect page
    pub redirect_page: String,
    /// Redirect page (v2)
    pub redirect_page_v2: String,
    /// Reference number
    pub ref_no: String,
    /// Stock quantity
    pub stock_quantity: String,
    /// Transaction type
    pub tx_type: String,
    /// Type name
    pub type_name: String,
}

/// The result of validating a fund order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrderValidation {
    /// Auth token to carry into submit
    pub auth_token: String,
    /// Risk-assessment eval address
    pub eval_address: String,
    /// Fund risk level
    pub fund_risk_level: i32,
    /// Message
    pub msg: String,
    /// User PI status
    pub user_pi: i32,
    /// User risk level
    pub user_risk_level: i32,
}

/// The result of submitting a fund order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FundOrderSubmitResponse {
    /// Action (buy/sell)
    pub action: String,
    /// Amount
    pub amount: String,
    /// Fund counter id
    pub counter_id: String,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Fund name
    pub fund_name: String,
    /// Order id
    pub id: i64,
    /// Message
    pub msg: String,
    /// Status
    pub status: i32,
    /// Units
    pub units: String,
}
