use chrono::{DateTime, Utc};
use longbridge_nodejs_macros::JsObject;

/// Response for submit grid trading order request
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::SubmitGridOrderResponse")]
pub struct SubmitGridOrderResponse {
    /// Grid master order id
    order_id: String,
}

/// A grid trading order (element of the list / by-ids responses).
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridOrder")]
pub struct GridOrder {
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
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
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
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridOrderSubOrder")]
pub struct GridOrderSubOrder {
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
    #[js(opt, datetime)]
    submitted_at: Option<DateTime<Utc>>,
    /// Regular trading hours flag
    rth: i32,
}

/// A grid order lifecycle-history entry carried in the grid order detail.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridOrderHistory")]
pub struct GridOrderHistory {
    /// History entry ID (paging cursor)
    history_id: String,
    /// Created time
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Status at this point
    status: String,
    /// Suspend reason, if any
    suspend_reason: String,
    /// Additional reason detail, if any
    reason: String,
}

/// Detail of a grid trading order.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridOrderDetail")]
pub struct GridOrderDetail {
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
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Last updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Settlement currency
    settlement_currency: String,
    /// Expiry time
    #[js(opt, datetime)]
    expire_time: Option<DateTime<Utc>>,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    gtd: String,
    /// Triggered sub-orders
    #[js(array)]
    grid_sub_orders: Vec<GridOrderSubOrder>,
    /// Whether there are more sub-orders to page
    sub_has_more: bool,
    /// Lifecycle history entries
    #[js(array)]
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
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::TriggerOrder")]
pub struct TriggerOrder {
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
    #[js(opt, datetime)]
    submitted_at: Option<DateTime<Utc>>,
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
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    gtd: String,
    /// Trigger time
    #[js(opt, datetime)]
    trigger_at: Option<DateTime<Utc>>,
    /// Conditional trigger status
    trigger_status: i32,
}

/// A price-step (bid-size) rule entry from the order-info response.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridBidSize")]
pub struct GridBidSize {
    /// Range start price (inclusive)
    str_proceed: String,
    /// Range end price
    end_proceed: String,
    /// Price step within the range
    bid_size: String,
}

/// Channel / authorization info nested in the order-info response.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridChannelInfo")]
pub struct GridChannelInfo {
    /// Whether the strategy compliance authorization has been granted
    strategy_granted: bool,
    /// Whether the RTH toggle is supported
    support_rth: bool,
    /// Trading currency
    currency: String,
    /// Supported settlement currencies
    #[js(array)]
    settlement_currency: Vec<String>,
}

/// Order info fields used by the grid order window.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::grid::GridOrderInfo")]
pub struct GridOrderInfo {
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
    #[js(array)]
    bid_sizes: Vec<GridBidSize>,
    /// Channel / authorization info (strategy grant, RTH, currencies)
    channel_infos: GridChannelInfo,
}

/// Response for get grid trading orders (list) request.
///
/// Hand-written because the underlying `longbridge::grid::GridOrdersResponse`
/// is not re-exported from the SDK; the wrapper is built directly in the
/// context method.
#[napi_derive::napi]
#[derive(Debug)]
pub struct GridOrdersResponse {
    grid_order: Vec<GridOrder>,
    has_more: bool,
}

impl GridOrdersResponse {
    pub(crate) fn new(grid_order: Vec<GridOrder>, has_more: bool) -> Self {
        Self {
            grid_order,
            has_more,
        }
    }
}

#[napi_derive::napi]
impl GridOrdersResponse {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as crate::utils::ToJSON>::to_json(self)
    }

    /// Grid orders
    #[napi(getter)]
    #[inline]
    pub fn grid_order(&self) -> Vec<GridOrder> {
        self.grid_order.clone()
    }

    /// Whether there are more pages
    #[napi(getter)]
    #[inline]
    pub fn has_more(&self) -> bool {
        self.has_more
    }
}

impl crate::utils::ToJSON for GridOrdersResponse {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                (
                    "gridOrder".to_string(),
                    <Vec<GridOrder> as crate::utils::ToJSON>::to_json(&self.grid_order),
                ),
                (
                    "hasMore".to_string(),
                    <bool as crate::utils::ToJSON>::to_json(&self.has_more),
                ),
            ]
            .into_iter()
            .collect(),
        )
    }
}

/// Response for get grid trading trigger history request.
///
/// Hand-written because the underlying
/// `longbridge::grid::GridTriggerHistoryResponse` is not re-exported from the
/// SDK; the wrapper is built directly in the context method.
#[napi_derive::napi]
#[derive(Debug)]
pub struct GridTriggerHistoryResponse {
    trigger_orders: Vec<TriggerOrder>,
    has_more: bool,
}

impl GridTriggerHistoryResponse {
    pub(crate) fn new(trigger_orders: Vec<TriggerOrder>, has_more: bool) -> Self {
        Self {
            trigger_orders,
            has_more,
        }
    }
}

#[napi_derive::napi]
impl GridTriggerHistoryResponse {
    #[napi]
    pub fn to_string(&self) -> String {
        ::std::format!("{:?}", self)
    }

    #[napi(js_name = "toJSON")]
    pub fn to_json(&self) -> serde_json::Value {
        <Self as crate::utils::ToJSON>::to_json(self)
    }

    /// Trigger history entries
    #[napi(getter)]
    #[inline]
    pub fn trigger_orders(&self) -> Vec<TriggerOrder> {
        self.trigger_orders.clone()
    }

    /// Whether there are more pages
    #[napi(getter)]
    #[inline]
    pub fn has_more(&self) -> bool {
        self.has_more
    }
}

impl crate::utils::ToJSON for GridTriggerHistoryResponse {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            [
                (
                    "triggerOrders".to_string(),
                    <Vec<TriggerOrder> as crate::utils::ToJSON>::to_json(&self.trigger_orders),
                ),
                (
                    "hasMore".to_string(),
                    <bool as crate::utils::ToJSON>::to_json(&self.has_more),
                ),
            ]
            .into_iter()
            .collect(),
        )
    }
}
