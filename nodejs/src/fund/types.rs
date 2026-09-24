// `to_string` here backs the JS `toString()` method for the raw-JSON "any"
// fields; it is an intentional inherent method, not a `Display` impl.
#![allow(clippy::inherent_to_string)]

use longbridge_nodejs_macros::JsObject;

use crate::utils::ToJSON;

/// Convert a `serde_json::Value` into its raw JSON string.
#[inline]
fn value_to_json_string(value: serde_json::Value) -> String {
    value.to_string()
}

/// Convert a list of `serde_json::Value` into a list of raw JSON strings.
#[inline]
fn values_to_json_strings(values: Vec<serde_json::Value>) -> Vec<String> {
    values.into_iter().map(value_to_json_string).collect()
}

/// A fund net-asset-value data point (latest / historical).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundNavValue")]
pub struct FundNavValue {
    /// Net value change
    change: String,
    /// Net value change percent
    change_percent: String,
    /// Formatted change percent
    change_percent_format: String,
    /// Fund counter id
    counter_id: String,
    /// Fund name
    counter_name: String,
    /// Currency
    currency: String,
    /// Formatted date
    date_format: String,
    /// ISIN
    isin: String,
    /// Last update time (unix seconds)
    last_update_time: i64,
    /// Net value
    value: String,
    /// Formatted net value
    value_format: String,
}

/// A recent performance point used by the hot-fund list.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPerformancePoint")]
pub struct FundPerformancePoint {
    /// Date
    date: String,
    /// Last done value
    last_done: String,
}

/// A hot-selling fund entry.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::HotFund")]
pub struct HotFund {
    /// Asset class
    asset_class: i32,
    /// Asset class name
    asset_class_name: String,
    /// Fund counter id
    counter_id: String,
    /// Currency
    currency: String,
    /// Earning rate
    earning_rate: String,
    /// Recent performance points
    #[js(array)]
    fund_performances: Vec<FundPerformancePoint>,
    /// Fund name
    name: String,
    /// Minimum purchase amount
    purchase_amount: String,
    /// Recommendation text
    recommendation_text: String,
    /// Risk level
    risk_level: i32,
    /// Risk level name
    risk_level_name: String,
    /// Time interval of the earning rate
    time_interval: String,
}

/// A fund entry in the fund list.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundBrief")]
pub struct FundBrief {
    /// Asset class
    asset_class: i32,
    /// Asset class name
    asset_class_name: String,
    /// Fund code
    code: String,
    /// Fund counter id
    counter_id: String,
    /// Currency
    currency: String,
    /// Description
    description: String,
    /// Earning rate
    earning_rate: String,
    /// Whether the user is holding this fund
    holding: bool,
    /// ISIN
    isin: String,
    /// Fund name
    name: String,
    /// Product
    product: String,
    /// Minimum purchase amount
    purchase_amount: String,
    /// Recommendation text
    recommendation_text: String,
    /// Risk level
    risk_level: i32,
    /// Risk level name
    risk_level_name: String,
    /// Time interval of the earning rate
    time_interval: String,
    /// Unit value
    unit_value: String,
}

/// A single holding entry inside a fund's asset allocation.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundAssetAllocationItem")]
pub struct FundAssetAllocationItem {
    /// Security code
    code: String,
    /// Security counter id
    counter_id: String,
    /// Name
    name: String,
    /// Position ratio
    position_ratio: String,
}

/// A fund's asset allocation.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundAssetAllocation")]
pub struct FundAssetAllocation {
    /// Asset type
    asset_type: i32,
    /// Allocation entries
    #[js(array)]
    lists: Vec<FundAssetAllocationItem>,
    /// Report date
    report_date: String,
}

/// Fund detail.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundDetail")]
pub struct FundDetail {
    /// Additional purchase amount
    additional_purchase_amount: String,
    /// Affirm day
    affirm_day: i32,
    /// Amount affirm day
    amount_affirm_day: String,
    /// Asset allocation
    asset_allocation: FundAssetAllocation,
    /// Asset class
    asset_class: i32,
    /// Asset class name
    asset_class_name: String,
    /// Bill purchase rate
    bill_purchase_rate: String,
    /// Channel
    channel: String,
    /// Close period
    close_period: String,
    /// Fund code
    code: String,
    /// Currency
    currency: String,
    /// Cut off time
    cut_off_time: String,
    /// Whether it is a derivative
    derivatives: bool,
    /// Done day
    done_day: i32,
    /// Excess return fee
    excess_return_fee: String,
    /// GST rate
    gst_rate: String,
    /// Introduction
    introduce: String,
    /// Whether it is a cash-plus fund
    is_cash_plus: bool,
    /// Whether it is a complex product
    is_complex: bool,
    /// Whether it is a new cash-plus fund
    is_new_cash_plus: bool,
    /// Whether it is a Yinghebao fund
    is_yinghebao: bool,
    /// ISIN
    isin: String,
    /// Management rate
    manage_rate: String,
    /// Manager
    manager: String,
    /// Minimum holding cash
    min_hold_cash: String,
    /// Minimum holding share
    min_hold_share: String,
    /// Minimum sell share
    min_sell_share: String,
    /// Month raise day
    month_raise_day: String,
    /// Fund name
    name: String,
    /// Net value deadline
    nav_deadline: String,
    /// Whether it is no-load
    no_load: bool,
    /// Open date
    open_date: String,
    /// Open period
    open_period: String,
    /// Product
    product: String,
    /// Product information locals
    product_information_locals: String,
    /// Profile
    profile: String,
    /// Whether purchasable
    purchasable: i32,
    /// Purchase affirm day
    purchase_affirm_day: String,
    /// Minimum purchase amount
    purchase_amount: String,
    /// Purchase rate
    purchase_rate: String,
    /// Rating
    rating: i32,
    /// Whether redeemable
    redeemable: i32,
    /// Redemption advance day
    redemption_advance_day: String,
    /// Redemption amount
    redemption_amount: String,
    /// Redemption close period text
    redemption_close_period_shows: String,
    /// Redemption done day
    redemption_done_day: String,
    /// Redemption open day text
    redemption_open_day_shows: String,
    /// Risk level
    risk_level: i32,
    /// Risk level name
    risk_level_name: String,
    /// Verify status
    verify_status: i32,
    /// Whether it is a virtual currency fund
    virtual_currency: bool,
    /// Year to date yield
    year_to_date_yield: String,
    /// Year to date yield type
    ytd_yield_type: i32,
}

/// A fund annual return entry.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundAnnualReturn")]
pub struct FundAnnualReturn {
    /// Change percent
    change_percent: String,
    /// Year
    year: i32,
}

/// A fund quarterly return entry.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundQuarterlyReturn")]
pub struct FundQuarterlyReturn {
    /// Change percent
    change_percent: String,
    /// Quarter
    quarter: i32,
    /// Year
    year: i32,
}

/// A fund's detailed performance figures.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPerformance")]
pub struct FundPerformance {
    /// Annualized return (5y)
    annualized_return_five: String,
    /// Annualized return (1y)
    annualized_return_one: String,
    /// Annualized return (10y)
    annualized_return_ten: String,
    /// Annualized return (3y)
    annualized_return_three: String,
    /// Annualized return (2y)
    annualized_return_two: String,
    /// Fund counter id
    counter_id: String,
    /// Fund name
    fund_name: String,
    /// Rank (5y)
    performance_rank_five_years: i32,
    /// Rank (1d)
    performance_rank_one_day: i32,
    /// Rank (1m)
    performance_rank_one_month: i32,
    /// Rank (1w)
    performance_rank_one_week: i32,
    /// Rank (1y)
    performance_rank_one_year: i32,
    /// Rank (6m)
    performance_rank_six_months: i32,
    /// Rank (10y)
    performance_rank_ten_years: i32,
    /// Rank (3m)
    performance_rank_three_months: i32,
    /// Rank (3y)
    performance_rank_three_years: i32,
    /// Rank (2y)
    performance_rank_two_years: i32,
    /// Rank (ytd)
    performance_rank_ytd: i32,
    /// Return (5y)
    performance_return_five_years: String,
    /// Return (1d)
    performance_return_one_day: String,
    /// Return (1m)
    performance_return_one_month: String,
    /// Return (1w)
    performance_return_one_week: String,
    /// Return (1y)
    performance_return_one_year: String,
    /// Return (6m)
    performance_return_six_months: String,
    /// Return (10y)
    performance_return_ten_years: String,
    /// Return (3m)
    performance_return_three_months: String,
    /// Return (3y)
    performance_return_three_years: String,
    /// Return (2y)
    performance_return_two_years: String,
    /// Return (ytd)
    performance_return_ytd: String,
    /// Total peers (5y)
    performance_total_five_years: i32,
    /// Total peers (1d)
    performance_total_one_day: i32,
    /// Total peers (1m)
    performance_total_one_month: i32,
    /// Total peers (1w)
    performance_total_one_week: i32,
    /// Total peers (1y)
    performance_total_one_year: i32,
    /// Total peers (6m)
    performance_total_six_months: i32,
    /// Total peers (10y)
    performance_total_ten_years: i32,
    /// Total peers (3m)
    performance_total_three_months: i32,
    /// Total peers (3y)
    performance_total_three_years: i32,
    /// Total peers (2y)
    performance_total_two_years: i32,
    /// Total peers (ytd)
    performance_total_ytd: i32,
    /// Seven days annualized
    seven_days_annualized: String,
    /// Ten thousand price
    ten_thousand_price: String,
    /// Update time (unix seconds)
    update_time: i64,
}

/// A single fund holding (top-10 holdings).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundHolding")]
pub struct FundHolding {
    /// Bond type
    bond_type: String,
    /// Bond type name
    bond_type_name: String,
    /// Country name
    country_name: String,
    /// Holding type
    holding_type: String,
    /// Industry name
    industry_name: String,
    /// Market value
    market_value: String,
    /// Maturity date
    maturity_date: String,
    /// Name
    name: String,
    /// Share change
    share_change: String,
    /// Share change percent
    share_change_percent: String,
    /// Shares
    shares: String,
    /// Weighting
    weighting: String,
}

/// A fund's top-10 holdings.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundHoldings")]
pub struct FundHoldings {
    /// Holding entries
    #[js(array)]
    holdings: Vec<FundHolding>,
    /// Report date
    report_date: String,
    /// Total weighting
    weighting: String,
}

/// A stock held by the fund (reverse lookup).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundStockHolding")]
pub struct FundStockHolding {
    /// Stock code
    code: String,
    /// Stock counter id
    counter_id: String,
    /// Currency
    currency: String,
    /// Stock name
    name: String,
    /// Position ratio
    position_ratio: String,
    /// Report date
    report_date: String,
}

/// A single fund position held by the user.
///
/// Exposed to JavaScript as `FundHoldingPosition` to avoid a name clash with
/// the trade channel's existing `FundPosition` class.
#[napi_derive::napi(js_name = "FundHoldingPosition")]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPosition")]
pub struct FundPosition {
    /// Holding amount
    amount: String,
    /// Fund counter id
    counter_id: String,
    /// Currency
    currency: String,
    /// Frozen units
    freeze_units: String,
    /// Holding profit
    holding_profit: String,
    /// Holding units
    holding_units: String,
    /// Fund name
    name: String,
    /// Recent profit
    recent_profit: String,
    /// Recent trading day (unix seconds)
    recent_trading_day: i64,
    /// Accumulated recent profit
    sum_recent_profit: String,
}

/// The user's fund positions overview.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPositions")]
pub struct FundPositions {
    /// Account channel
    account_channel: String,
    /// Position entries
    #[js(array)]
    list: Vec<FundPosition>,
    /// Pending buy orders amount
    pending_buy_orders: String,
    /// Recent trading day (unix seconds)
    recent_trading_day: i64,
    /// Sold pending credit orders amount
    sold_pending_credit_orders: String,
}

/// A dated value point.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundDatedValue")]
pub struct FundDatedValue {
    /// Date (unix seconds)
    date: i64,
    /// Value
    value: String,
}

/// A fund unit-value point (position view).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundUnitValue")]
pub struct FundUnitValue {
    /// Date (unix seconds)
    date: i64,
    /// Day increase rate
    day_increase_rate: String,
    /// Total value
    total_value: String,
    /// Unit value
    unit_value: String,
}

/// Detail values of a single fund position.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPositionDetailValues")]
pub struct FundPositionDetailValues {
    /// Amount
    amount: String,
    /// Currency
    currency: String,
    /// Holding cost
    holding_cost: String,
    /// Holding profit
    holding_profit: String,
    /// Holding profit rate
    holding_profit_rate: String,
    /// Holding units
    holding_units: String,
    /// Holding value
    holding_value: String,
    /// Pending buy value
    pending_buy_value: String,
    /// Pending sell value
    pending_sell_value: String,
    /// Accumulated profit (to date)
    profit_amount_accum_td: String,
    /// Accumulated profit rate (to date)
    profit_amount_accum_td_rate: String,
    /// Recent profit
    recent_profit: String,
    /// Recent trading day (unix seconds)
    recent_tradingday: i64,
    /// Recent unit value
    recent_unit_value: String,
    /// Sold pending-confirm units
    sold_pending_confirm_units: String,
}

/// Detail of a single fund position.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPositionDetail")]
pub struct FundPositionDetail {
    /// Detail values
    detail_values: FundPositionDetailValues,
    /// Accumulated profit series
    #[js(array)]
    sum_profit: Vec<FundDatedValue>,
    /// Unit value series
    #[js(array)]
    ut_value: Vec<FundUnitValue>,
}

/// Performance figures for a held fund.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPositionPerformance")]
pub struct FundPositionPerformance {
    /// Annualized return (5y)
    annualized_return_five: String,
    /// Annualized return (1y)
    annualized_return_one: String,
    /// Annualized return (10y)
    annualized_return_ten: String,
    /// Annualized return (3y)
    annualized_return_three: String,
    /// Annualized return (2y)
    annualized_return_two: String,
    /// Fund counter id
    counter_id: String,
    /// Fund name
    fund_name: String,
    /// Return (5y)
    performance_return_five_years: String,
    /// Return (1d)
    performance_return_one_day: String,
    /// Return (1m)
    performance_return_one_month: String,
    /// Return (1w)
    performance_return_one_week: String,
    /// Return (1y)
    performance_return_one_year: String,
    /// Return (6m)
    performance_return_six_months: String,
    /// Return (10y)
    performance_return_ten_years: String,
    /// Return (3m)
    performance_return_three_months: String,
    /// Return (3y)
    performance_return_three_years: String,
    /// Return (2y)
    performance_return_two_years: String,
    /// Return (ytd)
    performance_return_ytd: String,
    /// Update time (unix seconds)
    update_time: i64,
}

/// The user's cumulative profit for a held fund.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPositionProfits")]
pub struct FundPositionProfits {
    /// Currency
    currency: String,
    /// Profit series
    #[js(array)]
    history_value: Vec<FundDatedValue>,
    /// Last update time (unix seconds)
    last_update_time: i64,
    /// Total profit
    sum_profit: String,
}

/// A held-fund net-value point (position view).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundPositionNav")]
pub struct FundPositionNav {
    /// Net value change
    change: String,
    /// Net value change percent
    change_percent: String,
    /// Fund counter id
    counter_id: String,
    /// Fund name
    counter_name: String,
    /// Last update time (unix seconds)
    last_update_time: i64,
    /// Net value
    value: String,
}

/// A cash dividend record for a held fund.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundDividend")]
pub struct FundDividend {
    /// Amount
    amount: String,
    /// Fund counter id
    counter_id: String,
    /// Currency
    currency: String,
    /// Date (unix seconds)
    date: i64,
    /// Dividend method
    div_method: String,
    /// Fund name
    name: String,
}

/// The user's dividend records for a held fund.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundDividends")]
pub struct FundDividends {
    /// Currency
    currency: String,
    /// Dividend records
    #[js(array)]
    div_cash_infos: Vec<FundDividend>,
    /// Latest dividend date (unix seconds)
    lastest_date: i64,
    /// Total cash dividend
    total_div_cash: String,
}

/// A fund order (list view).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrder")]
pub struct FundOrder {
    /// Action (buy/sell)
    action: String,
    /// Amount
    amount: String,
    /// Fund counter id
    counter_id: String,
    /// Created at (unix seconds)
    created_at: i64,
    /// Currency
    currency: String,
    /// Fund name
    fund_name: String,
    /// Order id
    id: i64,
    /// Whether it is an auto (DCA) order
    is_auto: bool,
    /// Net worth
    net_worth: String,
    /// Product type
    product_type: String,
    /// State
    state: String,
    /// State description
    state_desc: String,
    /// Units
    units: String,
}

/// A keyword block in a fund order detail.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrderKeyword")]
pub struct FundOrderKeyword {
    /// Content
    content: String,
    /// Group
    group: String,
    /// Key
    key: String,
    /// Line strategy
    line_strategy: String,
    /// Title
    title: String,
}

/// A processing stage in a fund order detail.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrderStage")]
pub struct FundOrderStage {
    /// Description
    desc: String,
    /// Key
    key: String,
    /// Link
    link: String,
    /// Link text
    link_text: String,
    /// Progress
    progress: String,
    /// Stage
    stage: String,
}

/// The full information of a fund order.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrderInfo")]
pub struct FundOrderInfo {
    /// Account id
    aaid: i64,
    /// Account channel
    account_channel: String,
    /// Action (buy/sell)
    action: String,
    /// Amount
    amount: String,
    /// Channel
    channel: String,
    /// Fund counter id
    counter_id: String,
    /// Created at (unix seconds)
    created_at: i64,
    /// Currency
    currency: String,
    /// Dividend option
    dividend_option: String,
    /// Equity time (unix seconds)
    eq_at: i64,
    /// Fee
    fee: String,
    /// Fund name
    fund_name: String,
    /// Fund source
    fund_source: String,
    /// Histories
    histories: String,
    /// Order id
    id: i64,
    /// Message
    message: String,
    /// Net worth
    net_worth: String,
    /// Price time (unix seconds)
    price_at: i64,
    /// Processed at (unix seconds)
    processed_at: i64,
    /// Product type
    product_type: String,
    /// Whether repurchaseable
    repurchaseable: bool,
    /// Sale proceeds
    sale_proceeds: String,
    /// Sales charge
    sales_charge: String,
    /// Sales price
    sales_price: String,
    /// Sales unit
    sales_unit: String,
    /// State
    state: String,
    /// State description
    state_desc: String,
    /// Status
    status: i32,
    /// Extended status
    status_ex: i32,
    /// T+ description
    t_description: String,
    /// Time partition
    time_partition: String,
    /// Total amount
    total_amount: String,
    /// Transaction at (unix seconds)
    transaction_at: i64,
    /// Units
    units: String,
    /// Withdraw at (unix seconds)
    withdraw_at: i64,
    /// Whether withdrawable
    withdrawable: bool,
}

/// Fund order detail.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrderDetail")]
pub struct FundOrderDetail {
    /// Keyword blocks
    #[js(array)]
    keywords: Vec<FundOrderKeyword>,
    /// The order
    order: FundOrderInfo,
    /// Processing stages
    #[js(array)]
    stages: Vec<FundOrderStage>,
}

/// A fund transaction / cash-flow record.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundTransaction")]
pub struct FundTransaction {
    /// Amount
    amount: String,
    /// Category
    category: String,
    /// Created at (unix seconds)
    created_at: i64,
    /// Currency
    currency: String,
    /// Description
    description: String,
    /// Detail created at (unix seconds)
    detail_created_at: i64,
    /// Detail type
    detail_type: String,
    /// Done at (unix seconds)
    done_at: i64,
    /// Quantity description
    quantity_description: String,
    /// Redirect page
    redirect_page: String,
    /// Redirect page (v2)
    redirect_page_v2: String,
    /// Reference number
    ref_no: String,
    /// Stock quantity
    stock_quantity: String,
    /// Transaction type
    tx_type: String,
    /// Type name
    type_name: String,
}

/// The result of validating a fund order.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrderValidation")]
pub struct FundOrderValidation {
    /// Auth token to carry into submit
    auth_token: String,
    /// Risk-assessment eval address
    eval_address: String,
    /// Fund risk level
    fund_risk_level: i32,
    /// Message
    msg: String,
    /// User PI status
    user_pi: i32,
    /// User risk level
    user_risk_level: i32,
}

/// The result of submitting a fund order.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::fund::FundOrderSubmitResponse")]
pub struct FundOrderSubmitResponse {
    /// Action (buy/sell)
    action: String,
    /// Amount
    amount: String,
    /// Fund counter id
    counter_id: String,
    /// Created at (unix seconds)
    created_at: i64,
    /// Fund name
    fund_name: String,
    /// Order id
    id: i64,
    /// Message
    msg: String,
    /// Status
    status: i32,
    /// Units
    units: String,
}

// ---------------------------------------------------------------------------
// Hand-written types.
//
// These carry server-defined ("any") JSON structures that the SDK models as
// `serde_json::Value`. The binding exposes each such value as a raw JSON
// `string` (and lists of them as `Array<string>`), so they cannot use the
// `JsObject` derive (which relies on `TryInto` per field).
// ---------------------------------------------------------------------------

/// Fund list filter options. Each list holds server-defined option objects
/// serialized as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundFilters {
    asset_class: Vec<String>,
    company: Vec<String>,
    currency: Vec<String>,
    industry_category_name: Vec<String>,
    risk_level: Vec<String>,
}

impl ::std::convert::TryFrom<longbridge::fund::FundFilters> for FundFilters {
    type Error = ::napi::Error;

    fn try_from(v: longbridge::fund::FundFilters) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self {
            asset_class: values_to_json_strings(v.asset_class),
            company: values_to_json_strings(v.company),
            currency: values_to_json_strings(v.currency),
            industry_category_name: values_to_json_strings(v.industry_category_name),
            risk_level: values_to_json_strings(v.risk_level),
        })
    }
}

#[napi_derive::napi]
impl FundFilters {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Asset class options (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn asset_class(&self) -> Vec<String> {
        self.asset_class.clone()
    }

    /// Company options (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn company(&self) -> Vec<String> {
        self.company.clone()
    }

    /// Currency options (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn currency(&self) -> Vec<String> {
        self.currency.clone()
    }

    /// Industry category options (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn industry_category_name(&self) -> Vec<String> {
        self.industry_category_name.clone()
    }

    /// Risk level options (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn risk_level(&self) -> Vec<String> {
        self.risk_level.clone()
    }
}

impl ToJSON for FundFilters {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                ("assetClass".to_string(), self.asset_class.to_json()),
                ("company".to_string(), self.company.to_json()),
                ("currency".to_string(), self.currency.to_json()),
                (
                    "industryCategoryName".to_string(),
                    self.industry_category_name.to_json(),
                ),
                ("riskLevel".to_string(), self.risk_level.to_json()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// Fund analysis (level 1). The ability/cost breakdowns are server-defined
/// structures exposed as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundAnalysis {
    actual_period: i32,
    cost_level: String,
    return_ability: String,
    risk_ability: String,
    updated_at: String,
    value_for_money: String,
    visible: bool,
}

impl ::std::convert::TryFrom<longbridge::fund::FundAnalysis> for FundAnalysis {
    type Error = ::napi::Error;

    fn try_from(v: longbridge::fund::FundAnalysis) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self {
            actual_period: v.actual_period,
            cost_level: value_to_json_string(v.cost_level),
            return_ability: value_to_json_string(v.return_ability),
            risk_ability: value_to_json_string(v.risk_ability),
            updated_at: v.updated_at,
            value_for_money: value_to_json_string(v.value_for_money),
            visible: v.visible,
        })
    }
}

#[napi_derive::napi]
impl FundAnalysis {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Actual period
    #[napi(getter)]
    #[inline]
    pub fn actual_period(&self) -> i32 {
        self.actual_period
    }

    /// Cost level (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn cost_level(&self) -> String {
        self.cost_level.clone()
    }

    /// Return ability (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn return_ability(&self) -> String {
        self.return_ability.clone()
    }

    /// Risk ability (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn risk_ability(&self) -> String {
        self.risk_ability.clone()
    }

    /// Updated at
    #[napi(getter)]
    #[inline]
    pub fn updated_at(&self) -> String {
        self.updated_at.clone()
    }

    /// Value for money (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn value_for_money(&self) -> String {
        self.value_for_money.clone()
    }

    /// Whether visible
    #[napi(getter)]
    #[inline]
    pub fn visible(&self) -> bool {
        self.visible
    }
}

impl ToJSON for FundAnalysis {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                ("actualPeriod".to_string(), self.actual_period.to_json()),
                ("costLevel".to_string(), self.cost_level.to_json()),
                ("returnAbility".to_string(), self.return_ability.to_json()),
                ("riskAbility".to_string(), self.risk_ability.to_json()),
                ("updatedAt".to_string(), self.updated_at.to_json()),
                ("valueForMoney".to_string(), self.value_for_money.to_json()),
                ("visible".to_string(), self.visible.to_json()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// Fund analysis detail (level 2). The ability/cost breakdowns are
/// server-defined structures exposed as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundAnalysisDetail {
    actual_period: i32,
    available_periods: Vec<i32>,
    cost_level: String,
    return_ability: String,
    risk_ability: String,
    updated_at: String,
    value_for_money: String,
    visible: bool,
}

impl ::std::convert::TryFrom<longbridge::fund::FundAnalysisDetail> for FundAnalysisDetail {
    type Error = ::napi::Error;

    fn try_from(
        v: longbridge::fund::FundAnalysisDetail,
    ) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self {
            actual_period: v.actual_period,
            available_periods: v.available_periods,
            cost_level: value_to_json_string(v.cost_level),
            return_ability: value_to_json_string(v.return_ability),
            risk_ability: value_to_json_string(v.risk_ability),
            updated_at: v.updated_at,
            value_for_money: value_to_json_string(v.value_for_money),
            visible: v.visible,
        })
    }
}

#[napi_derive::napi]
impl FundAnalysisDetail {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Actual period
    #[napi(getter)]
    #[inline]
    pub fn actual_period(&self) -> i32 {
        self.actual_period
    }

    /// Available periods
    #[napi(getter)]
    #[inline]
    pub fn available_periods(&self) -> Vec<i32> {
        self.available_periods.clone()
    }

    /// Cost level (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn cost_level(&self) -> String {
        self.cost_level.clone()
    }

    /// Return ability (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn return_ability(&self) -> String {
        self.return_ability.clone()
    }

    /// Risk ability (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn risk_ability(&self) -> String {
        self.risk_ability.clone()
    }

    /// Updated at
    #[napi(getter)]
    #[inline]
    pub fn updated_at(&self) -> String {
        self.updated_at.clone()
    }

    /// Value for money (raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn value_for_money(&self) -> String {
        self.value_for_money.clone()
    }

    /// Whether visible
    #[napi(getter)]
    #[inline]
    pub fn visible(&self) -> bool {
        self.visible
    }
}

impl ToJSON for FundAnalysisDetail {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                ("actualPeriod".to_string(), self.actual_period.to_json()),
                (
                    "availablePeriods".to_string(),
                    self.available_periods.to_json(),
                ),
                ("costLevel".to_string(), self.cost_level.to_json()),
                ("returnAbility".to_string(), self.return_ability.to_json()),
                ("riskAbility".to_string(), self.risk_ability.to_json()),
                ("updatedAt".to_string(), self.updated_at.to_json()),
                ("valueForMoney".to_string(), self.value_for_money.to_json()),
                ("visible".to_string(), self.visible.to_json()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// A benchmark contrast series in a fund trend chart. The performance points
/// are server-defined structures exposed as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundTrendContrast {
    benchmark_name: String,
    performances: Vec<String>,
}

impl ::std::convert::From<longbridge::fund::FundTrendContrast> for FundTrendContrast {
    fn from(v: longbridge::fund::FundTrendContrast) -> Self {
        Self {
            benchmark_name: v.benchmark_name,
            performances: values_to_json_strings(v.performances),
        }
    }
}

#[napi_derive::napi]
impl FundTrendContrast {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Benchmark name
    #[napi(getter)]
    #[inline]
    pub fn benchmark_name(&self) -> String {
        self.benchmark_name.clone()
    }

    /// Performance points (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn performances(&self) -> Vec<String> {
        self.performances.clone()
    }
}

impl ToJSON for FundTrendContrast {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                ("benchmarkName".to_string(), self.benchmark_name.to_json()),
                ("performances".to_string(), self.performances.to_json()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// Fund trend chart. The performance series are server-defined structures
/// exposed as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundTrend {
    actual_period: i32,
    available_periods: Vec<i32>,
    category_average_performances: Vec<String>,
    contrast_performances: FundTrendContrast,
    fund_performances: Vec<String>,
}

impl ::std::convert::TryFrom<longbridge::fund::FundTrend> for FundTrend {
    type Error = ::napi::Error;

    fn try_from(v: longbridge::fund::FundTrend) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self {
            actual_period: v.actual_period,
            available_periods: v.available_periods,
            category_average_performances: values_to_json_strings(v.category_average_performances),
            contrast_performances: v.contrast_performances.into(),
            fund_performances: values_to_json_strings(v.fund_performances),
        })
    }
}

#[napi_derive::napi]
impl FundTrend {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Actual period
    #[napi(getter)]
    #[inline]
    pub fn actual_period(&self) -> i32 {
        self.actual_period
    }

    /// Available periods
    #[napi(getter)]
    #[inline]
    pub fn available_periods(&self) -> Vec<i32> {
        self.available_periods.clone()
    }

    /// Category average performances (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn category_average_performances(&self) -> Vec<String> {
        self.category_average_performances.clone()
    }

    /// Benchmark contrast performances
    #[napi(getter)]
    #[inline]
    pub fn contrast_performances(&self) -> FundTrendContrast {
        self.contrast_performances.clone()
    }

    /// Fund performances (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn fund_performances(&self) -> Vec<String> {
        self.fund_performances.clone()
    }
}

impl ToJSON for FundTrend {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                ("actualPeriod".to_string(), self.actual_period.to_json()),
                (
                    "availablePeriods".to_string(),
                    self.available_periods.to_json(),
                ),
                (
                    "categoryAveragePerformances".to_string(),
                    self.category_average_performances.to_json(),
                ),
                (
                    "contrastPerformances".to_string(),
                    self.contrast_performances.to_json(),
                ),
                (
                    "fundPerformances".to_string(),
                    self.fund_performances.to_json(),
                ),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// A named contrast performance series. The performance points are
/// server-defined structures exposed as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundNamedContrast {
    name: String,
    performances: Vec<String>,
}

impl ::std::convert::From<longbridge::fund::FundNamedContrast> for FundNamedContrast {
    fn from(v: longbridge::fund::FundNamedContrast) -> Self {
        Self {
            name: v.name,
            performances: values_to_json_strings(v.performances),
        }
    }
}

#[napi_derive::napi]
impl FundNamedContrast {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Series name
    #[napi(getter)]
    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Performance points (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn performances(&self) -> Vec<String> {
        self.performances.clone()
    }
}

impl ToJSON for FundNamedContrast {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                ("name".to_string(), self.name.to_json()),
                ("performances".to_string(), self.performances.to_json()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// Fund performance comparison. The fund performance series is a
/// server-defined structure exposed as raw JSON strings.
#[napi_derive::napi]
#[derive(Debug, Clone)]
pub struct FundPerformanceComparison {
    contrast_performances: Vec<FundNamedContrast>,
    fund_performances: Vec<String>,
}

impl ::std::convert::TryFrom<longbridge::fund::FundPerformanceComparison>
    for FundPerformanceComparison
{
    type Error = ::napi::Error;

    fn try_from(
        v: longbridge::fund::FundPerformanceComparison,
    ) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self {
            contrast_performances: v
                .contrast_performances
                .into_iter()
                .map(FundNamedContrast::from)
                .collect(),
            fund_performances: values_to_json_strings(v.fund_performances),
        })
    }
}

#[napi_derive::napi]
impl FundPerformanceComparison {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as ToJSON>::to_json(self)
    }

    /// Contrast performance series
    #[napi(getter)]
    #[inline]
    pub fn contrast_performances(&self) -> Vec<FundNamedContrast> {
        self.contrast_performances.clone()
    }

    /// Fund performances (each a raw JSON string)
    #[napi(getter)]
    #[inline]
    pub fn fund_performances(&self) -> Vec<String> {
        self.fund_performances.clone()
    }
}

impl ToJSON for FundPerformanceComparison {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                (
                    "contrastPerformances".to_string(),
                    self.contrast_performances.to_json(),
                ),
                (
                    "fundPerformances".to_string(),
                    self.fund_performances.to_json(),
                ),
            ]
            .into_iter()
            .collect(),
        )
    }
}
