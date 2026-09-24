use std::{
    convert::Infallible,
    fmt::{self, Debug},
};

use longbridge_python_macros::PyObject;
use pyo3::{
    Bound, IntoPyObject, Python, pyclass,
    types::{PyString, PyStringMethods},
};
use serde_json::Value;

// ── Fund (mutual fund) types
// ─────────────────────────────────────────────────────

/// A raw, server-defined JSON value exposed to Python as a JSON `str`.
///
/// Several fund endpoints return objects whose schema is defined by the server
/// and may change over time (filter option lists, analysis blocks, trend /
/// comparison performance series). To avoid pinning these to a fixed shape the
/// Rust core keeps them as `serde_json::Value`; here they are surfaced verbatim
/// to Python as JSON-encoded strings that the caller can `json.loads(...)`.
#[derive(Clone)]
pub(crate) struct PyJson(pub(crate) String);

impl Debug for PyJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<Value> for PyJson {
    type Error = pyo3::PyErr;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(PyJson(value.to_string()))
    }
}

impl<'py> IntoPyObject<'py> for PyJson {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, &self.0))
    }
}

impl<'a, 'py> pyo3::FromPyObject<'a, 'py> for PyJson {
    type Error = pyo3::PyErr;

    fn extract(ob: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let s: Bound<PyString> = ob.extract()?;
        Ok(PyJson(s.to_str()?.to_string()))
    }
}

/// A fund net-asset-value data point (latest / historical).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundNavValue")]
pub(crate) struct FundNavValue {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPerformancePoint")]
pub(crate) struct FundPerformancePoint {
    /// Date
    date: String,
    /// Last done value
    last_done: String,
}

/// A hot-selling fund entry.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::HotFund")]
pub(crate) struct HotFund {
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
    #[py(array)]
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundBrief")]
pub(crate) struct FundBrief {
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

/// Fund list filter options. Each list holds server-defined option objects
/// exposed as raw JSON strings.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundFilters")]
pub(crate) struct FundFilters {
    /// Asset class options (raw JSON strings)
    #[py(array)]
    asset_class: Vec<PyJson>,
    /// Company options (raw JSON strings)
    #[py(array)]
    company: Vec<PyJson>,
    /// Currency options (raw JSON strings)
    #[py(array)]
    currency: Vec<PyJson>,
    /// Industry category options (raw JSON strings)
    #[py(array)]
    industry_category_name: Vec<PyJson>,
    /// Risk level options (raw JSON strings)
    #[py(array)]
    risk_level: Vec<PyJson>,
}

/// A single holding entry inside a fund's asset allocation.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundAssetAllocationItem")]
pub(crate) struct FundAssetAllocationItem {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundAssetAllocation")]
pub(crate) struct FundAssetAllocation {
    /// Asset type
    asset_type: i32,
    /// Allocation entries
    #[py(array)]
    lists: Vec<FundAssetAllocationItem>,
    /// Report date
    report_date: String,
}

/// Fund detail.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundDetail")]
pub(crate) struct FundDetail {
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

/// Fund analysis (level 1).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundAnalysis")]
pub(crate) struct FundAnalysis {
    /// Actual period
    actual_period: i32,
    /// Cost level (server-defined structure, raw JSON string)
    cost_level: PyJson,
    /// Return ability (server-defined structure, raw JSON string)
    return_ability: PyJson,
    /// Risk ability (server-defined structure, raw JSON string)
    risk_ability: PyJson,
    /// Updated at
    updated_at: String,
    /// Value for money (server-defined structure, raw JSON string)
    value_for_money: PyJson,
    /// Whether visible
    visible: bool,
}

/// Fund analysis detail (level 2).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundAnalysisDetail")]
pub(crate) struct FundAnalysisDetail {
    /// Actual period
    actual_period: i32,
    /// Available periods
    available_periods: Vec<i32>,
    /// Cost level (server-defined structure, raw JSON string)
    cost_level: PyJson,
    /// Return ability (server-defined structure, raw JSON string)
    return_ability: PyJson,
    /// Risk ability (server-defined structure, raw JSON string)
    risk_ability: PyJson,
    /// Updated at
    updated_at: String,
    /// Value for money (server-defined structure, raw JSON string)
    value_for_money: PyJson,
    /// Whether visible
    visible: bool,
}

/// A benchmark contrast series in a fund trend chart.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundTrendContrast")]
pub(crate) struct FundTrendContrast {
    /// Benchmark name
    benchmark_name: String,
    /// Performance points (server-defined structure, raw JSON strings)
    #[py(array)]
    performances: Vec<PyJson>,
}

/// Fund trend chart.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundTrend")]
pub(crate) struct FundTrend {
    /// Actual period
    actual_period: i32,
    /// Available periods
    available_periods: Vec<i32>,
    /// Category average performances (server-defined structure, raw JSON
    /// strings)
    #[py(array)]
    category_average_performances: Vec<PyJson>,
    /// Benchmark contrast performances
    contrast_performances: FundTrendContrast,
    /// Fund performances (server-defined structure, raw JSON strings)
    #[py(array)]
    fund_performances: Vec<PyJson>,
}

/// A named contrast performance series.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundNamedContrast")]
pub(crate) struct FundNamedContrast {
    /// Series name
    name: String,
    /// Performance points (server-defined structure, raw JSON strings)
    #[py(array)]
    performances: Vec<PyJson>,
}

/// Fund performance comparison.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPerformanceComparison")]
pub(crate) struct FundPerformanceComparison {
    /// Contrast performance series
    #[py(array)]
    contrast_performances: Vec<FundNamedContrast>,
    /// Fund performances (server-defined structure, raw JSON strings)
    #[py(array)]
    fund_performances: Vec<PyJson>,
}

/// A fund annual return entry.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundAnnualReturn")]
pub(crate) struct FundAnnualReturn {
    /// Change percent
    change_percent: String,
    /// Year
    year: i32,
}

/// A fund quarterly return entry.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundQuarterlyReturn")]
pub(crate) struct FundQuarterlyReturn {
    /// Change percent
    change_percent: String,
    /// Quarter
    quarter: i32,
    /// Year
    year: i32,
}

/// A fund's detailed performance figures.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPerformance")]
pub(crate) struct FundPerformance {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundHolding")]
pub(crate) struct FundHolding {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundHoldings")]
pub(crate) struct FundHoldings {
    /// Holding entries
    #[py(array)]
    holdings: Vec<FundHolding>,
    /// Report date
    report_date: String,
    /// Total weighting
    weighting: String,
}

/// A stock held by the fund (reverse lookup).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundStockHolding")]
pub(crate) struct FundStockHolding {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPosition")]
pub(crate) struct FundPosition {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPositions")]
pub(crate) struct FundPositions {
    /// Account channel
    account_channel: String,
    /// Position entries
    #[py(array)]
    list: Vec<FundPosition>,
    /// Pending buy orders amount
    pending_buy_orders: String,
    /// Recent trading day (unix seconds)
    recent_trading_day: i64,
    /// Sold pending credit orders amount
    sold_pending_credit_orders: String,
}

/// A dated value point.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundDatedValue")]
pub(crate) struct FundDatedValue {
    /// Date (unix seconds)
    date: i64,
    /// Value
    value: String,
}

/// A fund unit-value point (position view).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundUnitValue")]
pub(crate) struct FundUnitValue {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPositionDetailValues")]
pub(crate) struct FundPositionDetailValues {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPositionDetail")]
pub(crate) struct FundPositionDetail {
    /// Detail values
    detail_values: FundPositionDetailValues,
    /// Accumulated profit series
    #[py(array)]
    sum_profit: Vec<FundDatedValue>,
    /// Unit value series
    #[py(array)]
    ut_value: Vec<FundUnitValue>,
}

/// Performance figures for a held fund.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPositionPerformance")]
pub(crate) struct FundPositionPerformance {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPositionProfits")]
pub(crate) struct FundPositionProfits {
    /// Currency
    currency: String,
    /// Profit series
    #[py(array)]
    history_value: Vec<FundDatedValue>,
    /// Last update time (unix seconds)
    last_update_time: i64,
    /// Total profit
    sum_profit: String,
}

/// A held-fund net-value point (position view).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundPositionNav")]
pub(crate) struct FundPositionNav {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundDividend")]
pub(crate) struct FundDividend {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundDividends")]
pub(crate) struct FundDividends {
    /// Currency
    currency: String,
    /// Dividend records
    #[py(array)]
    div_cash_infos: Vec<FundDividend>,
    /// Latest dividend date (unix seconds)
    lastest_date: i64,
    /// Total cash dividend
    total_div_cash: String,
}

/// A fund order (list view).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrder")]
pub(crate) struct FundOrder {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrderKeyword")]
pub(crate) struct FundOrderKeyword {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrderStage")]
pub(crate) struct FundOrderStage {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrderInfo")]
pub(crate) struct FundOrderInfo {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrderDetail")]
pub(crate) struct FundOrderDetail {
    /// Keyword blocks
    #[py(array)]
    keywords: Vec<FundOrderKeyword>,
    /// The order
    order: FundOrderInfo,
    /// Processing stages
    #[py(array)]
    stages: Vec<FundOrderStage>,
}

/// A fund transaction / cash-flow record.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundTransaction")]
pub(crate) struct FundTransaction {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrderValidation")]
pub(crate) struct FundOrderValidation {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::fund::FundOrderSubmitResponse")]
pub(crate) struct FundOrderSubmitResponse {
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
