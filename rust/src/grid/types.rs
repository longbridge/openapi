//! Grid trading types

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::serde_utils;

/// Grid trading rule — parameters for submit / replace.
///
/// Mirrors the `GridTradingRule` message in the gridtrading proto. Prices and
/// quantities are decimals serialized as strings; enum-like fields are raw
/// integers whose code tables are documented inline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GridTradeRule {
    /// Base price the grid is anchored to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_base_price: Option<Decimal>,
    /// Upper price bound
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit_price: Option<Decimal>,
    /// Lower price bound
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit_price: Option<Decimal>,
    /// Trigger price type (only `1` / `2` allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<i32>,
    /// Upward trigger spread (absolute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_spread_up: Option<Decimal>,
    /// Downward trigger spread (absolute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_spread_down: Option<Decimal>,
    /// Upward trigger percent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_percent_up: Option<Decimal>,
    /// Downward trigger percent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_percent_down: Option<Decimal>,
    /// Whether a single grid level may trigger multiple times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_trigger: Option<bool>,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<i32>,
    /// Quantity handled when the upper bound is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit_quantity: Option<Decimal>,
    /// Quantity handled when the lower bound is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit_quantity: Option<Decimal>,
    /// Expiry time (unix seconds), used with GTD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    /// Action when the upper bound is reached (only `1` / `2` allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit_event: Option<i32>,
    /// Action when the lower bound is reached (only `1` / `2` allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit_event: Option<i32>,
    /// Sell-side order-book depth (-5..5, `0` = use `grid_order_type_up`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_sell_depth: Option<i32>,
    /// Buy-side order-book depth (-5..5, `0` = use `grid_order_type_down`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_buy_depth: Option<i32>,
    /// Quantity per trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_quantity: Option<Decimal>,
    /// Whether short selling is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_shortsell: Option<bool>,
    /// Regular trading hours flag (`0` / `1` / `2`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rth: Option<i32>,
    /// Sell-side order type when depth is `0` (`GMO` / `GLO` / `GTG`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_order_type_up: Option<String>,
    /// Buy-side order type when depth is `0` (`GMO` / `GLO` / `GTG`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_order_type_down: Option<String>,
}

/// A grid trading order (element of the list / by-ids responses).
///
/// Fields reflect the gateway JSON; the security is exposed via `symbol`
/// (`700.HK`). Numeric values are returned as strings; unknown fields are
/// ignored (`#[serde(default)]`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridOrder {
    /// Grid master order ID
    pub order_id: String,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: String,
    /// Stock name
    pub stock_name: String,
    /// Market
    pub market: String,
    /// Order status
    pub status: String,
    /// Grid running status
    pub grid_status: String,
    /// Submitted base price
    pub submitted_base_price: String,
    /// Current base price
    pub current_base_price: String,
    /// Base price before the last trigger
    pub pre_trigger_base_price: String,
    /// Base price after the last trigger
    pub post_trigger_base_price: String,
    /// Upper price bound
    pub upper_limit_price: String,
    /// Lower price bound
    pub lower_limit_price: String,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: i32,
    /// Upward trigger spread
    pub trigger_spread_up: String,
    /// Downward trigger spread
    pub trigger_spread_down: String,
    /// Upward trigger percent
    pub trigger_percent_up: String,
    /// Downward trigger percent
    pub trigger_percent_down: String,
    /// Pullback percent
    pub pullback_percent: String,
    /// Pullback spread
    pub pullback_spread: String,
    /// Rebound percent
    pub rebound_percent: String,
    /// Rebound spread
    pub rebound_spread: String,
    /// Sell-side execution order type (e.g. `MO`)
    pub trigger_sell_order_type: String,
    /// Buy-side execution order type (e.g. `MO`)
    pub trigger_buy_order_type: String,
    /// Sell-side order-book depth
    pub trigger_sell_depth: i32,
    /// Buy-side order-book depth
    pub trigger_buy_depth: i32,
    /// Quantity per trigger
    pub trigger_quantity: String,
    /// Quantity per sell trigger
    pub trigger_sell_quantity: String,
    /// Quantity per buy trigger
    pub trigger_buy_quantity: String,
    /// Quantity handled at the upper bound
    pub upper_limit_quantity: String,
    /// Quantity handled at the lower bound
    pub lower_limit_quantity: String,
    /// Action at the upper bound
    pub upper_limit_event: i32,
    /// Action at the lower bound
    pub lower_limit_event: i32,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Number of times the grid has triggered
    pub trigger_times: i32,
    /// Accumulated bought quantity
    pub total_buy_quantity: String,
    /// Accumulated sold quantity
    pub total_sell_quantity: String,
    /// Accumulated profit balance
    pub total_profit_balance: String,
    /// Settlement currency
    pub settlement_currency: String,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    pub gtd: String,
    /// Created time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub created_at: Option<OffsetDateTime>,
    /// Regular trading hours flag
    pub rth: i32,
    /// Whether short selling is allowed
    pub support_shortsell: bool,
    /// Sell-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_up: String,
    /// Buy-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_down: String,
}

/// A triggered sub-order carried in the grid order detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridOrderSubOrder {
    /// Sub-order ID
    pub id: String,
    /// Order price
    pub price: String,
    /// Order type
    pub order_type: String,
    /// Order quantity
    pub quantity: String,
    /// Executed quantity
    pub executed_qty: String,
    /// Buy / sell direction
    pub action: i32,
    /// Order status
    pub status: String,
    /// Submitted time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub submitted_at: Option<OffsetDateTime>,
    /// Regular trading hours flag
    pub rth: i32,
}

/// A grid order lifecycle-history entry carried in the grid order detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridOrderHistory {
    /// History entry ID (paging cursor)
    pub history_id: String,
    /// Created time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub created_at: Option<OffsetDateTime>,
    /// Status at this point
    pub status: String,
    /// Suspend reason, if any
    pub suspend_reason: String,
    /// Additional reason detail, if any
    pub reason: String,
}

/// Detail of a grid trading order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridOrderDetail {
    /// Grid master order ID
    pub order_id: String,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: String,
    /// Stock name
    pub stock_name: String,
    /// Order status
    pub status: String,
    /// Grid running status
    pub grid_status: String,
    /// Suspend reason, if any
    pub suspend_reason: String,
    /// Sleeping reason, if any
    pub sleeping_reason: String,
    /// Submitted base price
    pub submitted_base_price: String,
    /// Current base price
    pub current_base_price: String,
    /// Upper price bound
    pub upper_limit_price: String,
    /// Lower price bound
    pub lower_limit_price: String,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: i32,
    /// Upward trigger spread
    pub trigger_spread_up: String,
    /// Downward trigger spread
    pub trigger_spread_down: String,
    /// Upward trigger percent
    pub trigger_percent_up: String,
    /// Downward trigger percent
    pub trigger_percent_down: String,
    /// Pullback percent
    pub pullback_percent: String,
    /// Pullback spread
    pub pullback_spread: String,
    /// Rebound percent
    pub rebound_percent: String,
    /// Rebound spread
    pub rebound_spread: String,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Quantity per trigger
    pub trigger_quantity: String,
    /// Quantity per sell trigger
    pub trigger_sell_quantity: String,
    /// Quantity per buy trigger
    pub trigger_buy_quantity: String,
    /// Quantity handled at the upper bound
    pub upper_limit_quantity: String,
    /// Quantity handled at the lower bound
    pub lower_limit_quantity: String,
    /// Action at the upper bound
    pub upper_limit_event: i32,
    /// Action at the lower bound
    pub lower_limit_event: i32,
    /// Sell-side order-book depth
    pub trigger_sell_depth: i32,
    /// Buy-side order-book depth
    pub trigger_buy_depth: i32,
    /// Created time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub created_at: Option<OffsetDateTime>,
    /// Last updated time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub updated_at: Option<OffsetDateTime>,
    /// Settlement currency
    pub settlement_currency: String,
    /// Expiry time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub expire_time: Option<OffsetDateTime>,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    pub gtd: String,
    /// Triggered sub-orders
    pub grid_sub_orders: Vec<GridOrderSubOrder>,
    /// Whether there are more sub-orders to page
    pub sub_has_more: bool,
    /// Lifecycle history entries
    pub grid_order_history: Vec<GridOrderHistory>,
    /// Whether there are more history entries to page
    pub history_has_more: bool,
    /// Whether short selling is allowed
    pub support_shortsell: bool,
    /// Regular trading hours flag
    pub rth: i32,
    /// Sell-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_up: String,
    /// Buy-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_down: String,
}

/// A grid trigger-history entry (one triggered order).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TriggerOrder {
    /// Triggered order ID
    pub id: String,
    /// Order status
    pub status: String,
    /// Stock name
    pub name: String,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: String,
    /// Order price
    pub price: String,
    /// Order quantity
    pub quantity: String,
    /// Executed average price
    pub executed_price: String,
    /// Executed total quantity
    pub executed_qty: String,
    /// Submitted time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub submitted_at: Option<OffsetDateTime>,
    /// Buy / sell direction
    pub action: i32,
    /// Order type
    pub order_type: String,
    /// Trigger price
    pub trigger_price: String,
    /// Rejection reason, if any
    pub msg: String,
    /// Settlement currency
    pub currency: String,
    /// Latest quote price
    pub last_done: String,
    /// Last updated time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub updated_at: Option<OffsetDateTime>,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    pub gtd: String,
    /// Trigger time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub trigger_at: Option<OffsetDateTime>,
    /// Conditional trigger status
    pub trigger_status: i32,
}

/// A price-step (bid-size) rule entry from the order-info response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridBidSize {
    /// Range start price (inclusive)
    pub str_proceed: String,
    /// Range end price
    pub end_proceed: String,
    /// Price step within the range
    pub bid_size: String,
}

/// Channel / authorization info nested in the order-info response, holding the
/// fields the grid order window needs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridChannelInfo {
    /// Whether the strategy compliance authorization has been granted
    pub strategy_granted: bool,
    /// Whether the RTH toggle is supported
    pub support_rth: bool,
    /// Trading currency
    pub currency: String,
    /// Supported settlement currencies
    pub settlement_currency: Vec<String>,
}

/// Order info (`/v1/orders/info`) fields used by the grid order window.
///
/// The endpoint takes a `counter_id` query parameter (a symbol such as
/// `700.HK` is accepted).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridOrderInfo {
    /// Security name
    pub name: String,
    /// Latest quote price
    pub last_done: String,
    /// Board lot size
    pub lot_size: String,
    /// Buy-side board lot size
    pub buy_lot_size: String,
    /// Sell-side board lot size
    pub sell_lot_size: String,
    /// Price-step (bid-size) rule table
    pub bid_sizes: Vec<GridBidSize>,
    /// Channel / authorization info (strategy grant, RTH, currencies)
    pub channel_infos: GridChannelInfo,
}
