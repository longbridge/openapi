use longbridge_python_macros::PyObject;
use pyo3::{pyclass, pymethods};

use crate::{decimal::PyDecimal, time::PyOffsetDateTimeWrapper};

// ── Grid-trading types
// ─────────────────────────────────────────────────────────

/// Grid trading rule — parameters for submit / replace.
///
/// Prices and quantities are decimals; enum-like fields are raw integers whose
/// code tables are documented in the SDK reference.
#[pyclass(from_py_object)]
#[derive(Clone)]
pub(crate) struct GridTradeRule(pub(crate) longbridge::grid::GridTradeRule);

#[pymethods]
impl GridTradeRule {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (*, submitted_base_price=None, upper_limit_price=None, lower_limit_price=None, trigger_price_type=None, trigger_spread_up=None, trigger_spread_down=None, trigger_percent_up=None, trigger_percent_down=None, multiple_trigger=None, time_in_force=None, upper_limit_quantity=None, lower_limit_quantity=None, expire_time=None, upper_limit_event=None, lower_limit_event=None, trigger_sell_depth=None, trigger_buy_depth=None, trigger_quantity=None, support_shortsell=None, rth=None, grid_order_type_up=None, grid_order_type_down=None))]
    fn new(
        submitted_base_price: Option<PyDecimal>,
        upper_limit_price: Option<PyDecimal>,
        lower_limit_price: Option<PyDecimal>,
        trigger_price_type: Option<i32>,
        trigger_spread_up: Option<PyDecimal>,
        trigger_spread_down: Option<PyDecimal>,
        trigger_percent_up: Option<PyDecimal>,
        trigger_percent_down: Option<PyDecimal>,
        multiple_trigger: Option<bool>,
        time_in_force: Option<i32>,
        upper_limit_quantity: Option<PyDecimal>,
        lower_limit_quantity: Option<PyDecimal>,
        expire_time: Option<i64>,
        upper_limit_event: Option<i32>,
        lower_limit_event: Option<i32>,
        trigger_sell_depth: Option<i32>,
        trigger_buy_depth: Option<i32>,
        trigger_quantity: Option<PyDecimal>,
        support_shortsell: Option<bool>,
        rth: Option<i32>,
        grid_order_type_up: Option<String>,
        grid_order_type_down: Option<String>,
    ) -> Self {
        Self(longbridge::grid::GridTradeRule {
            submitted_base_price: submitted_base_price.map(Into::into),
            upper_limit_price: upper_limit_price.map(Into::into),
            lower_limit_price: lower_limit_price.map(Into::into),
            trigger_price_type,
            trigger_spread_up: trigger_spread_up.map(Into::into),
            trigger_spread_down: trigger_spread_down.map(Into::into),
            trigger_percent_up: trigger_percent_up.map(Into::into),
            trigger_percent_down: trigger_percent_down.map(Into::into),
            multiple_trigger,
            time_in_force,
            upper_limit_quantity: upper_limit_quantity.map(Into::into),
            lower_limit_quantity: lower_limit_quantity.map(Into::into),
            expire_time,
            upper_limit_event,
            lower_limit_event,
            trigger_sell_depth,
            trigger_buy_depth,
            trigger_quantity: trigger_quantity.map(Into::into),
            support_shortsell,
            rth,
            grid_order_type_up,
            grid_order_type_down,
        })
    }
}

/// Response for submit grid trading order request
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::grid::SubmitGridOrderResponse")]
pub(crate) struct SubmitGridOrderResponse {
    /// Grid master order id
    order_id: String,
}

/// A grid trading order (element of the list / by-ids responses).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridOrder")]
pub(crate) struct GridOrder {
    /// Grid master order ID
    order_id: String,
    /// Security symbol (e.g. `700.HK`)
    symbol: String,
    /// Stock name
    stock_name: String,
    /// Market
    market: String,
    /// Order status
    status: String,
    /// Grid running status
    grid_status: String,
    /// Submitted base price
    submitted_base_price: String,
    /// Current base price
    current_base_price: String,
    /// Base price before the last trigger
    pre_trigger_base_price: String,
    /// Base price after the last trigger
    post_trigger_base_price: String,
    /// Upper price bound
    upper_limit_price: String,
    /// Lower price bound
    lower_limit_price: String,
    /// Trigger price type (`1` = spread, `2` = percent)
    trigger_price_type: i32,
    /// Upward trigger spread
    trigger_spread_up: String,
    /// Downward trigger spread
    trigger_spread_down: String,
    /// Upward trigger percent
    trigger_percent_up: String,
    /// Downward trigger percent
    trigger_percent_down: String,
    /// Pullback percent
    pullback_percent: String,
    /// Pullback spread
    pullback_spread: String,
    /// Rebound percent
    rebound_percent: String,
    /// Rebound spread
    rebound_spread: String,
    /// Sell-side execution order type (e.g. `MO`)
    trigger_sell_order_type: String,
    /// Buy-side execution order type (e.g. `MO`)
    trigger_buy_order_type: String,
    /// Sell-side order-book depth
    trigger_sell_depth: i32,
    /// Buy-side order-book depth
    trigger_buy_depth: i32,
    /// Quantity per trigger
    trigger_quantity: String,
    /// Quantity per sell trigger
    trigger_sell_quantity: String,
    /// Quantity per buy trigger
    trigger_buy_quantity: String,
    /// Quantity handled at the upper bound
    upper_limit_quantity: String,
    /// Quantity handled at the lower bound
    lower_limit_quantity: String,
    /// Action at the upper bound
    upper_limit_event: i32,
    /// Action at the lower bound
    lower_limit_event: i32,
    /// Whether a single grid level may trigger multiple times
    multiple_trigger: bool,
    /// Number of times the grid has triggered
    trigger_times: i32,
    /// Accumulated bought quantity
    total_buy_quantity: String,
    /// Accumulated sold quantity
    total_sell_quantity: String,
    /// Accumulated profit balance
    total_profit_balance: String,
    /// Settlement currency
    settlement_currency: String,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    gtd: String,
    /// Created time
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Regular trading hours flag
    rth: i32,
    /// Whether short selling is allowed
    support_shortsell: bool,
    /// Sell-side grid order type (`GMO` / `GLO` / `GTG`)
    grid_order_type_up: String,
    /// Buy-side grid order type (`GMO` / `GLO` / `GTG`)
    grid_order_type_down: String,
}

/// A triggered sub-order carried in the grid order detail.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridOrderSubOrder")]
pub(crate) struct GridOrderSubOrder {
    /// Sub-order ID
    id: String,
    /// Order price
    price: String,
    /// Order type
    order_type: String,
    /// Order quantity
    quantity: String,
    /// Executed quantity
    executed_qty: String,
    /// Buy / sell direction
    action: i32,
    /// Order status
    status: String,
    /// Submitted time
    #[py(opt)]
    submitted_at: Option<PyOffsetDateTimeWrapper>,
    /// Regular trading hours flag
    rth: i32,
}

/// A grid order lifecycle-history entry carried in the grid order detail.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridOrderHistory")]
pub(crate) struct GridOrderHistory {
    /// History entry ID (paging cursor)
    history_id: String,
    /// Created time
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Status at this point
    status: String,
    /// Suspend reason, if any
    suspend_reason: String,
    /// Additional reason detail, if any
    reason: String,
}

/// Detail of a grid trading order.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridOrderDetail")]
pub(crate) struct GridOrderDetail {
    /// Grid master order ID
    order_id: String,
    /// Security symbol (e.g. `700.HK`)
    symbol: String,
    /// Stock name
    stock_name: String,
    /// Order status
    status: String,
    /// Grid running status
    grid_status: String,
    /// Suspend reason, if any
    suspend_reason: String,
    /// Sleeping reason, if any
    sleeping_reason: String,
    /// Submitted base price
    submitted_base_price: String,
    /// Current base price
    current_base_price: String,
    /// Upper price bound
    upper_limit_price: String,
    /// Lower price bound
    lower_limit_price: String,
    /// Trigger price type (`1` = spread, `2` = percent)
    trigger_price_type: i32,
    /// Upward trigger spread
    trigger_spread_up: String,
    /// Downward trigger spread
    trigger_spread_down: String,
    /// Upward trigger percent
    trigger_percent_up: String,
    /// Downward trigger percent
    trigger_percent_down: String,
    /// Pullback percent
    pullback_percent: String,
    /// Pullback spread
    pullback_spread: String,
    /// Rebound percent
    rebound_percent: String,
    /// Rebound spread
    rebound_spread: String,
    /// Whether a single grid level may trigger multiple times
    multiple_trigger: bool,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    time_in_force: i32,
    /// Quantity per trigger
    trigger_quantity: String,
    /// Quantity per sell trigger
    trigger_sell_quantity: String,
    /// Quantity per buy trigger
    trigger_buy_quantity: String,
    /// Quantity handled at the upper bound
    upper_limit_quantity: String,
    /// Quantity handled at the lower bound
    lower_limit_quantity: String,
    /// Action at the upper bound
    upper_limit_event: i32,
    /// Action at the lower bound
    lower_limit_event: i32,
    /// Sell-side order-book depth
    trigger_sell_depth: i32,
    /// Buy-side order-book depth
    trigger_buy_depth: i32,
    /// Created time
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Last updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Settlement currency
    settlement_currency: String,
    /// Expiry time
    #[py(opt)]
    expire_time: Option<PyOffsetDateTimeWrapper>,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    gtd: String,
    /// Triggered sub-orders
    #[py(array)]
    grid_sub_orders: Vec<GridOrderSubOrder>,
    /// Whether there are more sub-orders to page
    sub_has_more: bool,
    /// Lifecycle history entries
    #[py(array)]
    grid_order_history: Vec<GridOrderHistory>,
    /// Whether there are more history entries to page
    history_has_more: bool,
    /// Whether short selling is allowed
    support_shortsell: bool,
    /// Regular trading hours flag
    rth: i32,
    /// Sell-side grid order type (`GMO` / `GLO` / `GTG`)
    grid_order_type_up: String,
    /// Buy-side grid order type (`GMO` / `GLO` / `GTG`)
    grid_order_type_down: String,
}

/// A grid trigger-history entry (one triggered order).
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::TriggerOrder")]
pub(crate) struct TriggerOrder {
    /// Triggered order ID
    id: String,
    /// Order status
    status: String,
    /// Stock name
    name: String,
    /// Security symbol (e.g. `700.HK`)
    symbol: String,
    /// Order price
    price: String,
    /// Order quantity
    quantity: String,
    /// Executed average price
    executed_price: String,
    /// Executed total quantity
    executed_qty: String,
    /// Submitted time
    #[py(opt)]
    submitted_at: Option<PyOffsetDateTimeWrapper>,
    /// Buy / sell direction
    action: i32,
    /// Order type
    order_type: String,
    /// Trigger price
    trigger_price: String,
    /// Rejection reason, if any
    msg: String,
    /// Settlement currency
    currency: String,
    /// Latest quote price
    last_done: String,
    /// Last updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    gtd: String,
    /// Trigger time
    #[py(opt)]
    trigger_at: Option<PyOffsetDateTimeWrapper>,
    /// Conditional trigger status
    trigger_status: i32,
}

/// A price-step (bid-size) rule entry from the order-info response.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridBidSize")]
pub(crate) struct GridBidSize {
    /// Range start price (inclusive)
    str_proceed: String,
    /// Range end price
    end_proceed: String,
    /// Price step within the range
    bid_size: String,
}

/// Channel / authorization info nested in the order-info response.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridChannelInfo")]
pub(crate) struct GridChannelInfo {
    /// Whether the strategy compliance authorization has been granted
    strategy_granted: bool,
    /// Whether the RTH toggle is supported
    support_rth: bool,
    /// Trading currency
    currency: String,
    /// Supported settlement currencies
    #[py(array)]
    settlement_currency: Vec<String>,
}

/// Order info (`/v1/orders/info`) fields used by the grid order window.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::grid::GridOrderInfo")]
pub(crate) struct GridOrderInfo {
    /// Security name
    name: String,
    /// Latest quote price
    last_done: String,
    /// Board lot size
    lot_size: String,
    /// Buy-side board lot size
    buy_lot_size: String,
    /// Sell-side board lot size
    sell_lot_size: String,
    /// Price-step (bid-size) rule table
    #[py(array)]
    bid_sizes: Vec<GridBidSize>,
    /// Channel / authorization info (strategy grant, RTH, currencies)
    channel_infos: GridChannelInfo,
}

/// Response for get grid trading orders (list) request
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct GridOrdersResponse {
    /// Grid orders
    pub grid_order: Vec<GridOrder>,
    /// Whether there are more pages
    pub has_more: bool,
}

/// Response for get grid trading trigger history request
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct GridTriggerHistoryResponse {
    /// Trigger history entries
    pub trigger_orders: Vec<TriggerOrder>,
    /// Whether there are more pages
    pub has_more: bool,
}
