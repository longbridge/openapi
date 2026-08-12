use chrono::{DateTime, Utc};
use longbridge_nodejs_macros::{JsEnum, JsObject};

use crate::decimal::Decimal;

/// How grid trigger thresholds are interpreted.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::grid::TriggerPriceType")]
pub enum TriggerPriceType {
    /// Unknown / unset
    Unknown,
    /// Trigger by absolute price spread
    Spread,
    /// Trigger by percent
    Percent,
}

/// Time in force for a grid order.
///
/// The underlying SDK models unknown wire values with a catch-all data variant;
/// the binding collapses those to `Unknown`, so the conversions are
/// hand-written instead of derived.
#[napi_derive::napi]
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum GridTimeInForce {
    /// Day order
    Day,
    /// Good-til-canceled
    GoodTilCanceled,
    /// Good-til-date
    GoodTilDate,
    /// Unknown value
    Unknown,
}

impl ::std::convert::From<longbridge::grid::GridTimeInForce> for GridTimeInForce {
    fn from(value: longbridge::grid::GridTimeInForce) -> Self {
        match value {
            longbridge::grid::GridTimeInForce::Day => GridTimeInForce::Day,
            longbridge::grid::GridTimeInForce::GoodTilCanceled => GridTimeInForce::GoodTilCanceled,
            longbridge::grid::GridTimeInForce::GoodTilDate => GridTimeInForce::GoodTilDate,
            longbridge::grid::GridTimeInForce::Unknown(_) => GridTimeInForce::Unknown,
        }
    }
}

impl ::std::convert::From<GridTimeInForce> for longbridge::grid::GridTimeInForce {
    fn from(value: GridTimeInForce) -> Self {
        match value {
            GridTimeInForce::Day => longbridge::grid::GridTimeInForce::Day,
            GridTimeInForce::GoodTilCanceled => longbridge::grid::GridTimeInForce::GoodTilCanceled,
            GridTimeInForce::GoodTilDate => longbridge::grid::GridTimeInForce::GoodTilDate,
            GridTimeInForce::Unknown => longbridge::grid::GridTimeInForce::Unknown(0),
        }
    }
}

impl crate::utils::ToJSON for GridTimeInForce {
    fn to_json(&self) -> serde_json::Value {
        let name = match self {
            GridTimeInForce::Day => "Day",
            GridTimeInForce::GoodTilCanceled => "GoodTilCanceled",
            GridTimeInForce::GoodTilDate => "GoodTilDate",
            GridTimeInForce::Unknown => "Unknown",
        };
        serde_json::Value::String(name.to_string())
    }
}

/// Action taken when a grid boundary is reached.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::grid::GridLimitEvent")]
pub enum GridLimitEvent {
    /// Unknown / unset
    Unknown,
    /// Ignore — keep the grid running
    Ignore,
    /// Close the position at the last price
    CloseAtLast,
}

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
    #[js(opt)]
    submitted_base_price: Option<Decimal>,
    /// Current base price
    #[js(opt)]
    current_base_price: Option<Decimal>,
    /// Base price before the last trigger
    #[js(opt)]
    pre_trigger_base_price: Option<Decimal>,
    /// Base price after the last trigger
    #[js(opt)]
    post_trigger_base_price: Option<Decimal>,
    /// Upper price bound
    #[js(opt)]
    upper_limit_price: Option<Decimal>,
    /// Lower price bound
    #[js(opt)]
    lower_limit_price: Option<Decimal>,
    /// Trigger price type
    trigger_price_type: TriggerPriceType,
    /// Upward trigger spread
    #[js(opt)]
    trigger_spread_up: Option<Decimal>,
    /// Downward trigger spread
    #[js(opt)]
    trigger_spread_down: Option<Decimal>,
    /// Upward trigger percent
    #[js(opt)]
    trigger_percent_up: Option<Decimal>,
    /// Downward trigger percent
    #[js(opt)]
    trigger_percent_down: Option<Decimal>,
    /// Pullback percent
    #[js(opt)]
    pullback_percent: Option<Decimal>,
    /// Pullback spread
    #[js(opt)]
    pullback_spread: Option<Decimal>,
    /// Rebound percent
    #[js(opt)]
    rebound_percent: Option<Decimal>,
    /// Rebound spread
    #[js(opt)]
    rebound_spread: Option<Decimal>,
    /// Sell-side execution order type (e.g. `MO`)
    trigger_sell_order_type: String,
    /// Buy-side execution order type (e.g. `MO`)
    trigger_buy_order_type: String,
    /// Sell-side order-book depth
    trigger_sell_depth: i32,
    /// Buy-side order-book depth
    trigger_buy_depth: i32,
    /// Quantity per trigger
    #[js(opt)]
    trigger_quantity: Option<Decimal>,
    /// Quantity per sell trigger
    #[js(opt)]
    trigger_sell_quantity: Option<Decimal>,
    /// Quantity per buy trigger
    #[js(opt)]
    trigger_buy_quantity: Option<Decimal>,
    /// Quantity handled at the upper bound
    #[js(opt)]
    upper_limit_quantity: Option<Decimal>,
    /// Quantity handled at the lower bound
    #[js(opt)]
    lower_limit_quantity: Option<Decimal>,
    /// Action at the upper bound
    upper_limit_event: GridLimitEvent,
    /// Action at the lower bound
    lower_limit_event: GridLimitEvent,
    /// Whether a single grid level may trigger multiple times
    multiple_trigger: bool,
    /// Number of times the grid has triggered
    trigger_times: i32,
    /// Accumulated bought quantity
    #[js(opt)]
    total_buy_quantity: Option<Decimal>,
    /// Accumulated sold quantity
    #[js(opt)]
    total_sell_quantity: Option<Decimal>,
    /// Accumulated profit balance
    #[js(opt)]
    total_profit_balance: Option<Decimal>,
    /// Settlement currency
    settlement_currency: String,
    /// Time in force
    time_in_force: GridTimeInForce,
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
    #[js(opt)]
    price: Option<Decimal>,
    /// Order type
    order_type: String,
    /// Order quantity
    #[js(opt)]
    quantity: Option<Decimal>,
    /// Executed quantity
    #[js(opt)]
    executed_qty: Option<Decimal>,
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
    #[js(opt)]
    submitted_base_price: Option<Decimal>,
    /// Current base price
    #[js(opt)]
    current_base_price: Option<Decimal>,
    /// Upper price bound
    #[js(opt)]
    upper_limit_price: Option<Decimal>,
    /// Lower price bound
    #[js(opt)]
    lower_limit_price: Option<Decimal>,
    /// Trigger price type
    trigger_price_type: TriggerPriceType,
    /// Upward trigger spread
    #[js(opt)]
    trigger_spread_up: Option<Decimal>,
    /// Downward trigger spread
    #[js(opt)]
    trigger_spread_down: Option<Decimal>,
    /// Upward trigger percent
    #[js(opt)]
    trigger_percent_up: Option<Decimal>,
    /// Downward trigger percent
    #[js(opt)]
    trigger_percent_down: Option<Decimal>,
    /// Pullback percent
    #[js(opt)]
    pullback_percent: Option<Decimal>,
    /// Pullback spread
    #[js(opt)]
    pullback_spread: Option<Decimal>,
    /// Rebound percent
    #[js(opt)]
    rebound_percent: Option<Decimal>,
    /// Rebound spread
    #[js(opt)]
    rebound_spread: Option<Decimal>,
    /// Whether a single grid level may trigger multiple times
    multiple_trigger: bool,
    /// Time in force
    time_in_force: GridTimeInForce,
    /// Quantity per trigger
    #[js(opt)]
    trigger_quantity: Option<Decimal>,
    /// Quantity per sell trigger
    #[js(opt)]
    trigger_sell_quantity: Option<Decimal>,
    /// Quantity per buy trigger
    #[js(opt)]
    trigger_buy_quantity: Option<Decimal>,
    /// Quantity handled at the upper bound
    #[js(opt)]
    upper_limit_quantity: Option<Decimal>,
    /// Quantity handled at the lower bound
    #[js(opt)]
    lower_limit_quantity: Option<Decimal>,
    /// Action at the upper bound
    upper_limit_event: GridLimitEvent,
    /// Action at the lower bound
    lower_limit_event: GridLimitEvent,
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
    #[js(opt)]
    price: Option<Decimal>,
    /// Order quantity
    #[js(opt)]
    quantity: Option<Decimal>,
    /// Executed average price
    #[js(opt)]
    executed_price: Option<Decimal>,
    /// Executed total quantity
    #[js(opt)]
    executed_qty: Option<Decimal>,
    /// Submitted time
    #[js(opt, datetime)]
    submitted_at: Option<DateTime<Utc>>,
    /// Buy / sell direction
    action: i32,
    /// Order type
    order_type: String,
    /// Trigger price
    #[js(opt)]
    trigger_price: Option<Decimal>,
    /// Rejection reason, if any
    msg: String,
    /// Settlement currency
    currency: String,
    /// Latest quote price
    #[js(opt)]
    last_done: Option<Decimal>,
    /// Last updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Time in force
    time_in_force: GridTimeInForce,
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
    #[js(opt)]
    str_proceed: Option<Decimal>,
    /// Range end price
    #[js(opt)]
    end_proceed: Option<Decimal>,
    /// Price step within the range
    #[js(opt)]
    bid_size: Option<Decimal>,
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
    #[js(opt)]
    last_done: Option<Decimal>,
    /// Board lot size
    #[js(opt)]
    lot_size: Option<Decimal>,
    /// Buy-side board lot size
    #[js(opt)]
    buy_lot_size: Option<Decimal>,
    /// Sell-side board lot size
    #[js(opt)]
    sell_lot_size: Option<Decimal>,
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
