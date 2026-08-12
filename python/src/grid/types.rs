use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::{pyclass, pymethods};

use crate::{decimal::PyDecimal, time::PyOffsetDateTimeWrapper};

// ── Grid-trading types
// ─────────────────────────────────────────────────────────

/// How grid trigger thresholds are interpreted.
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::grid::TriggerPriceType")]
pub(crate) enum TriggerPriceType {
    /// Unknown / unset
    Unknown,
    /// Trigger by absolute price spread
    Spread,
    /// Trigger by percent
    Percent,
}

/// Time in force for a grid order.
///
/// The core enum carries an `Unknown(i32)` catch-all variant, which cannot be
/// mirrored one-to-one by the macro-based enum derive; the conversions are
/// hand-written below (any unknown wire value maps to [`GridTimeInForce::Unknown`],
/// which serializes back as `0`).
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::grid::GridTimeInForce", from = false, into = false)]
pub(crate) enum GridTimeInForce {
    /// Day order
    Day,
    /// Good-til-canceled
    GoodTilCanceled,
    /// Good-til-date
    GoodTilDate,
    /// Unknown value
    Unknown,
}

impl From<longbridge::grid::GridTimeInForce> for GridTimeInForce {
    fn from(value: longbridge::grid::GridTimeInForce) -> Self {
        use longbridge::grid::GridTimeInForce as Remote;
        match value {
            Remote::Day => GridTimeInForce::Day,
            Remote::GoodTilCanceled => GridTimeInForce::GoodTilCanceled,
            Remote::GoodTilDate => GridTimeInForce::GoodTilDate,
            Remote::Unknown(_) => GridTimeInForce::Unknown,
        }
    }
}

impl From<GridTimeInForce> for longbridge::grid::GridTimeInForce {
    fn from(value: GridTimeInForce) -> Self {
        use longbridge::grid::GridTimeInForce as Remote;
        match value {
            GridTimeInForce::Day => Remote::Day,
            GridTimeInForce::GoodTilCanceled => Remote::GoodTilCanceled,
            GridTimeInForce::GoodTilDate => Remote::GoodTilDate,
            GridTimeInForce::Unknown => Remote::Unknown(0),
        }
    }
}

/// Action taken when a grid boundary is reached.
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::grid::GridLimitEvent")]
pub(crate) enum GridLimitEvent {
    /// Unknown / unset
    Unknown,
    /// Ignore — keep the grid running
    Ignore,
    /// Close the position at the last price
    CloseAtLast,
}

/// Grid trading rule — parameters for submit / replace.
///
/// The constructor takes the minimum field set a valid grid order requires as
/// positional arguments; the remaining fields are optional keyword arguments.
/// The trigger thresholds are expressed as a `trigger_price_type`
/// ([`TriggerPriceType.Spread`] or [`TriggerPriceType.Percent`]) plus the
/// up / down values that go with it.
#[pyclass(from_py_object)]
#[derive(Clone)]
pub(crate) struct GridTradeRule(pub(crate) longbridge::grid::GridTradeRule);

#[pymethods]
impl GridTradeRule {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        base_price,
        upper_price,
        lower_price,
        trigger_price_type,
        trigger_up,
        trigger_down,
        quantity,
        upper_quantity,
        lower_quantity,
        time_in_force,
        *,
        upper_limit_event=None,
        lower_limit_event=None,
        trigger_sell_depth=None,
        trigger_buy_depth=None,
        grid_order_type_up=None,
        grid_order_type_down=None,
        multiple_trigger=None,
        support_shortsell=None,
        rth=None,
        expire_time=None,
    ))]
    fn new(
        base_price: PyDecimal,
        upper_price: PyDecimal,
        lower_price: PyDecimal,
        trigger_price_type: TriggerPriceType,
        trigger_up: PyDecimal,
        trigger_down: PyDecimal,
        quantity: PyDecimal,
        upper_quantity: PyDecimal,
        lower_quantity: PyDecimal,
        time_in_force: GridTimeInForce,
        upper_limit_event: Option<GridLimitEvent>,
        lower_limit_event: Option<GridLimitEvent>,
        trigger_sell_depth: Option<i32>,
        trigger_buy_depth: Option<i32>,
        grid_order_type_up: Option<String>,
        grid_order_type_down: Option<String>,
        multiple_trigger: Option<bool>,
        support_shortsell: Option<bool>,
        rth: Option<i32>,
        expire_time: Option<i64>,
    ) -> Self {
        let trigger = match trigger_price_type {
            TriggerPriceType::Spread => longbridge::grid::GridTrigger::Spread {
                up: trigger_up.into(),
                down: trigger_down.into(),
            },
            _ => longbridge::grid::GridTrigger::Percent {
                up: trigger_up.into(),
                down: trigger_down.into(),
            },
        };
        let mut rule = longbridge::grid::GridTradeRule::new(
            base_price.into(),
            upper_price.into(),
            lower_price.into(),
            trigger,
            quantity.into(),
            upper_quantity.into(),
            lower_quantity.into(),
            time_in_force.into(),
        );
        if let (Some(upper), Some(lower)) = (upper_limit_event, lower_limit_event) {
            rule = rule.limit_events(upper.into(), lower.into());
        }
        if let (Some(sell), Some(buy)) = (trigger_sell_depth, trigger_buy_depth) {
            rule = rule.depths(sell, buy);
        }
        if let (Some(up), Some(down)) = (grid_order_type_up, grid_order_type_down) {
            rule = rule.order_types(up, down);
        }
        if let Some(value) = multiple_trigger {
            rule = rule.multiple_trigger(value);
        }
        if let Some(value) = support_shortsell {
            rule = rule.support_shortsell(value);
        }
        if let Some(value) = rth {
            rule = rule.rth(value);
        }
        if let Some(value) = expire_time {
            rule = rule.expire_time(value);
        }
        Self(rule)
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
    #[py(opt)]
    submitted_base_price: Option<PyDecimal>,
    /// Current base price
    #[py(opt)]
    current_base_price: Option<PyDecimal>,
    /// Base price before the last trigger
    #[py(opt)]
    pre_trigger_base_price: Option<PyDecimal>,
    /// Base price after the last trigger
    #[py(opt)]
    post_trigger_base_price: Option<PyDecimal>,
    /// Upper price bound
    #[py(opt)]
    upper_limit_price: Option<PyDecimal>,
    /// Lower price bound
    #[py(opt)]
    lower_limit_price: Option<PyDecimal>,
    /// Trigger price type
    trigger_price_type: TriggerPriceType,
    /// Upward trigger spread
    #[py(opt)]
    trigger_spread_up: Option<PyDecimal>,
    /// Downward trigger spread
    #[py(opt)]
    trigger_spread_down: Option<PyDecimal>,
    /// Upward trigger percent
    #[py(opt)]
    trigger_percent_up: Option<PyDecimal>,
    /// Downward trigger percent
    #[py(opt)]
    trigger_percent_down: Option<PyDecimal>,
    /// Pullback percent
    #[py(opt)]
    pullback_percent: Option<PyDecimal>,
    /// Pullback spread
    #[py(opt)]
    pullback_spread: Option<PyDecimal>,
    /// Rebound percent
    #[py(opt)]
    rebound_percent: Option<PyDecimal>,
    /// Rebound spread
    #[py(opt)]
    rebound_spread: Option<PyDecimal>,
    /// Sell-side execution order type (e.g. `MO`)
    trigger_sell_order_type: String,
    /// Buy-side execution order type (e.g. `MO`)
    trigger_buy_order_type: String,
    /// Sell-side order-book depth
    trigger_sell_depth: i32,
    /// Buy-side order-book depth
    trigger_buy_depth: i32,
    /// Quantity per trigger
    #[py(opt)]
    trigger_quantity: Option<PyDecimal>,
    /// Quantity per sell trigger
    #[py(opt)]
    trigger_sell_quantity: Option<PyDecimal>,
    /// Quantity per buy trigger
    #[py(opt)]
    trigger_buy_quantity: Option<PyDecimal>,
    /// Quantity handled at the upper bound
    #[py(opt)]
    upper_limit_quantity: Option<PyDecimal>,
    /// Quantity handled at the lower bound
    #[py(opt)]
    lower_limit_quantity: Option<PyDecimal>,
    /// Action at the upper bound
    upper_limit_event: GridLimitEvent,
    /// Action at the lower bound
    lower_limit_event: GridLimitEvent,
    /// Whether a single grid level may trigger multiple times
    multiple_trigger: bool,
    /// Number of times the grid has triggered
    trigger_times: i32,
    /// Accumulated bought quantity
    #[py(opt)]
    total_buy_quantity: Option<PyDecimal>,
    /// Accumulated sold quantity
    #[py(opt)]
    total_sell_quantity: Option<PyDecimal>,
    /// Accumulated profit balance
    #[py(opt)]
    total_profit_balance: Option<PyDecimal>,
    /// Settlement currency
    settlement_currency: String,
    /// Time in force
    time_in_force: GridTimeInForce,
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
    #[py(opt)]
    price: Option<PyDecimal>,
    /// Order type
    order_type: String,
    /// Order quantity
    #[py(opt)]
    quantity: Option<PyDecimal>,
    /// Executed quantity
    #[py(opt)]
    executed_qty: Option<PyDecimal>,
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
    #[py(opt)]
    submitted_base_price: Option<PyDecimal>,
    /// Current base price
    #[py(opt)]
    current_base_price: Option<PyDecimal>,
    /// Upper price bound
    #[py(opt)]
    upper_limit_price: Option<PyDecimal>,
    /// Lower price bound
    #[py(opt)]
    lower_limit_price: Option<PyDecimal>,
    /// Trigger price type
    trigger_price_type: TriggerPriceType,
    /// Upward trigger spread
    #[py(opt)]
    trigger_spread_up: Option<PyDecimal>,
    /// Downward trigger spread
    #[py(opt)]
    trigger_spread_down: Option<PyDecimal>,
    /// Upward trigger percent
    #[py(opt)]
    trigger_percent_up: Option<PyDecimal>,
    /// Downward trigger percent
    #[py(opt)]
    trigger_percent_down: Option<PyDecimal>,
    /// Pullback percent
    #[py(opt)]
    pullback_percent: Option<PyDecimal>,
    /// Pullback spread
    #[py(opt)]
    pullback_spread: Option<PyDecimal>,
    /// Rebound percent
    #[py(opt)]
    rebound_percent: Option<PyDecimal>,
    /// Rebound spread
    #[py(opt)]
    rebound_spread: Option<PyDecimal>,
    /// Whether a single grid level may trigger multiple times
    multiple_trigger: bool,
    /// Time in force
    time_in_force: GridTimeInForce,
    /// Quantity per trigger
    #[py(opt)]
    trigger_quantity: Option<PyDecimal>,
    /// Quantity per sell trigger
    #[py(opt)]
    trigger_sell_quantity: Option<PyDecimal>,
    /// Quantity per buy trigger
    #[py(opt)]
    trigger_buy_quantity: Option<PyDecimal>,
    /// Quantity handled at the upper bound
    #[py(opt)]
    upper_limit_quantity: Option<PyDecimal>,
    /// Quantity handled at the lower bound
    #[py(opt)]
    lower_limit_quantity: Option<PyDecimal>,
    /// Action at the upper bound
    upper_limit_event: GridLimitEvent,
    /// Action at the lower bound
    lower_limit_event: GridLimitEvent,
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
    #[py(opt)]
    price: Option<PyDecimal>,
    /// Order quantity
    #[py(opt)]
    quantity: Option<PyDecimal>,
    /// Executed average price
    #[py(opt)]
    executed_price: Option<PyDecimal>,
    /// Executed total quantity
    #[py(opt)]
    executed_qty: Option<PyDecimal>,
    /// Submitted time
    #[py(opt)]
    submitted_at: Option<PyOffsetDateTimeWrapper>,
    /// Buy / sell direction
    action: i32,
    /// Order type
    order_type: String,
    /// Trigger price
    #[py(opt)]
    trigger_price: Option<PyDecimal>,
    /// Rejection reason, if any
    msg: String,
    /// Settlement currency
    currency: String,
    /// Latest quote price
    #[py(opt)]
    last_done: Option<PyDecimal>,
    /// Last updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Time in force
    time_in_force: GridTimeInForce,
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
    #[py(opt)]
    str_proceed: Option<PyDecimal>,
    /// Range end price
    #[py(opt)]
    end_proceed: Option<PyDecimal>,
    /// Price step within the range
    #[py(opt)]
    bid_size: Option<PyDecimal>,
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
    #[py(opt)]
    last_done: Option<PyDecimal>,
    /// Board lot size
    #[py(opt)]
    lot_size: Option<PyDecimal>,
    /// Buy-side board lot size
    #[py(opt)]
    buy_lot_size: Option<PyDecimal>,
    /// Sell-side board lot size
    #[py(opt)]
    sell_lot_size: Option<PyDecimal>,
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
