//! Grid trading types

use num_enum::{FromPrimitive, IntoPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::OffsetDateTime;

use crate::serde_utils;

/// Serde helper for response numeric fields: they arrive as strings and an
/// empty string means "no value". Deserializes to `Option<Decimal>` (empty →
/// `None`) and serializes back to a string (`None` → `""`) to preserve the
/// exact wire format.
mod opt_decimal_string {
    use rust_decimal::Decimal;
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<S: Serializer>(
        value: &Option<Decimal>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Decimal>, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            s.parse::<Decimal>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

/// How grid trigger thresholds are interpreted (wire: `i32`).
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TriggerPriceType {
    /// Unknown / unset
    #[num_enum(default)]
    Unknown = 0,
    /// Trigger by absolute price spread
    Spread = 1,
    /// Trigger by percent
    Percent = 2,
}

impl Default for TriggerPriceType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Serialize for TriggerPriceType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        i32::from(*self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TriggerPriceType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(i32::deserialize(deserializer)?))
    }
}

/// Time in force for a grid order (wire: `i32`).
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GridTimeInForce {
    /// Day order
    Day = 0,
    /// Good-til-canceled
    GoodTilCanceled = 1,
    /// Good-til-date
    GoodTilDate = 6,
    /// Unknown value, preserved verbatim
    #[num_enum(catch_all)]
    Unknown(i32),
}

impl Default for GridTimeInForce {
    fn default() -> Self {
        Self::Day
    }
}

impl Serialize for GridTimeInForce {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        i32::from(*self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GridTimeInForce {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(i32::deserialize(deserializer)?))
    }
}

/// Action taken when a grid boundary is reached (wire: `i32`).
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GridLimitEvent {
    /// Unknown / unset
    #[num_enum(default)]
    Unknown = 0,
    /// Ignore — keep the grid running
    Ignore = 1,
    /// Close the position at the last price
    CloseAtLast = 2,
}

impl Default for GridLimitEvent {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Serialize for GridLimitEvent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        i32::from(*self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GridLimitEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(i32::deserialize(deserializer)?))
    }
}

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
    pub trigger_price_type: Option<TriggerPriceType>,
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
    pub time_in_force: Option<GridTimeInForce>,
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
    pub upper_limit_event: Option<GridLimitEvent>,
    /// Action when the lower bound is reached (only `1` / `2` allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit_event: Option<GridLimitEvent>,
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

/// How a grid's up/down trigger thresholds are expressed. Percent and spread
/// are mutually exclusive; modeling them as an enum makes the choice explicit
/// (instead of four independent optional fields).
#[derive(Debug, Clone, Copy)]
pub enum GridTrigger {
    /// Trigger by percent (`up`, `down`)
    Percent {
        /// Upward trigger percent
        up: Decimal,
        /// Downward trigger percent
        down: Decimal,
    },
    /// Trigger by absolute price spread (`up`, `down`)
    Spread {
        /// Upward trigger spread
        up: Decimal,
        /// Downward trigger spread
        down: Decimal,
    },
}

impl GridTradeRule {
    /// Create a rule with the fields a valid grid order requires. The gateway
    /// still validates business rules, but this makes the minimum field set
    /// visible in the type signature instead of leaving all fields optional.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_price: Decimal,
        upper_price: Decimal,
        lower_price: Decimal,
        trigger: GridTrigger,
        quantity: Decimal,
        upper_quantity: Decimal,
        lower_quantity: Decimal,
        time_in_force: GridTimeInForce,
    ) -> Self {
        let mut rule = GridTradeRule {
            submitted_base_price: Some(base_price),
            upper_limit_price: Some(upper_price),
            lower_limit_price: Some(lower_price),
            trigger_quantity: Some(quantity),
            upper_limit_quantity: Some(upper_quantity),
            lower_limit_quantity: Some(lower_quantity),
            time_in_force: Some(time_in_force),
            ..Default::default()
        };
        match trigger {
            GridTrigger::Percent { up, down } => {
                rule.trigger_price_type = Some(TriggerPriceType::Percent);
                rule.trigger_percent_up = Some(up);
                rule.trigger_percent_down = Some(down);
            }
            GridTrigger::Spread { up, down } => {
                rule.trigger_price_type = Some(TriggerPriceType::Spread);
                rule.trigger_spread_up = Some(up);
                rule.trigger_spread_down = Some(down);
            }
        }
        rule
    }

    /// Set the actions taken at the upper / lower bounds.
    #[must_use]
    pub fn limit_events(mut self, upper: GridLimitEvent, lower: GridLimitEvent) -> Self {
        self.upper_limit_event = Some(upper);
        self.lower_limit_event = Some(lower);
        self
    }

    /// Set the sell / buy order-book depths (`0` = use the order type).
    #[must_use]
    pub fn depths(mut self, sell: i32, buy: i32) -> Self {
        self.trigger_sell_depth = Some(sell);
        self.trigger_buy_depth = Some(buy);
        self
    }

    /// Set the sell / buy order types (`GMO` / `GLO` / `GTG`).
    #[must_use]
    pub fn order_types(mut self, up: impl Into<String>, down: impl Into<String>) -> Self {
        self.grid_order_type_up = Some(up.into());
        self.grid_order_type_down = Some(down.into());
        self
    }

    /// Allow a single grid level to trigger multiple times.
    #[must_use]
    pub fn multiple_trigger(mut self, value: bool) -> Self {
        self.multiple_trigger = Some(value);
        self
    }

    /// Allow short selling.
    #[must_use]
    pub fn support_shortsell(mut self, value: bool) -> Self {
        self.support_shortsell = Some(value);
        self
    }

    /// Set the regular-trading-hours flag (`0` / `1` / `2`).
    #[must_use]
    pub fn rth(mut self, value: i32) -> Self {
        self.rth = Some(value);
        self
    }

    /// Set the expiry time (unix seconds), used with a GTD time-in-force.
    #[must_use]
    pub fn expire_time(mut self, unix_seconds: i64) -> Self {
        self.expire_time = Some(unix_seconds);
        self
    }
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
    #[serde(with = "opt_decimal_string")]
    pub submitted_base_price: Option<Decimal>,
    /// Current base price
    #[serde(with = "opt_decimal_string")]
    pub current_base_price: Option<Decimal>,
    /// Base price before the last trigger
    #[serde(with = "opt_decimal_string")]
    pub pre_trigger_base_price: Option<Decimal>,
    /// Base price after the last trigger
    #[serde(with = "opt_decimal_string")]
    pub post_trigger_base_price: Option<Decimal>,
    /// Upper price bound
    #[serde(with = "opt_decimal_string")]
    pub upper_limit_price: Option<Decimal>,
    /// Lower price bound
    #[serde(with = "opt_decimal_string")]
    pub lower_limit_price: Option<Decimal>,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: TriggerPriceType,
    /// Upward trigger spread
    #[serde(with = "opt_decimal_string")]
    pub trigger_spread_up: Option<Decimal>,
    /// Downward trigger spread
    #[serde(with = "opt_decimal_string")]
    pub trigger_spread_down: Option<Decimal>,
    /// Upward trigger percent
    #[serde(with = "opt_decimal_string")]
    pub trigger_percent_up: Option<Decimal>,
    /// Downward trigger percent
    #[serde(with = "opt_decimal_string")]
    pub trigger_percent_down: Option<Decimal>,
    /// Pullback percent
    #[serde(with = "opt_decimal_string")]
    pub pullback_percent: Option<Decimal>,
    /// Pullback spread
    #[serde(with = "opt_decimal_string")]
    pub pullback_spread: Option<Decimal>,
    /// Rebound percent
    #[serde(with = "opt_decimal_string")]
    pub rebound_percent: Option<Decimal>,
    /// Rebound spread
    #[serde(with = "opt_decimal_string")]
    pub rebound_spread: Option<Decimal>,
    /// Sell-side execution order type (e.g. `MO`)
    pub trigger_sell_order_type: String,
    /// Buy-side execution order type (e.g. `MO`)
    pub trigger_buy_order_type: String,
    /// Sell-side order-book depth
    pub trigger_sell_depth: i32,
    /// Buy-side order-book depth
    pub trigger_buy_depth: i32,
    /// Quantity per trigger
    #[serde(with = "opt_decimal_string")]
    pub trigger_quantity: Option<Decimal>,
    /// Quantity per sell trigger
    #[serde(with = "opt_decimal_string")]
    pub trigger_sell_quantity: Option<Decimal>,
    /// Quantity per buy trigger
    #[serde(with = "opt_decimal_string")]
    pub trigger_buy_quantity: Option<Decimal>,
    /// Quantity handled at the upper bound
    #[serde(with = "opt_decimal_string")]
    pub upper_limit_quantity: Option<Decimal>,
    /// Quantity handled at the lower bound
    #[serde(with = "opt_decimal_string")]
    pub lower_limit_quantity: Option<Decimal>,
    /// Action at the upper bound
    pub upper_limit_event: GridLimitEvent,
    /// Action at the lower bound
    pub lower_limit_event: GridLimitEvent,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Number of times the grid has triggered
    pub trigger_times: i32,
    /// Accumulated bought quantity
    #[serde(with = "opt_decimal_string")]
    pub total_buy_quantity: Option<Decimal>,
    /// Accumulated sold quantity
    #[serde(with = "opt_decimal_string")]
    pub total_sell_quantity: Option<Decimal>,
    /// Accumulated profit balance
    #[serde(with = "opt_decimal_string")]
    pub total_profit_balance: Option<Decimal>,
    /// Settlement currency
    pub settlement_currency: String,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: GridTimeInForce,
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
    #[serde(with = "opt_decimal_string")]
    pub price: Option<Decimal>,
    /// Order type
    pub order_type: String,
    /// Order quantity
    #[serde(with = "opt_decimal_string")]
    pub quantity: Option<Decimal>,
    /// Executed quantity
    #[serde(with = "opt_decimal_string")]
    pub executed_qty: Option<Decimal>,
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
    #[serde(with = "opt_decimal_string")]
    pub submitted_base_price: Option<Decimal>,
    /// Current base price
    #[serde(with = "opt_decimal_string")]
    pub current_base_price: Option<Decimal>,
    /// Upper price bound
    #[serde(with = "opt_decimal_string")]
    pub upper_limit_price: Option<Decimal>,
    /// Lower price bound
    #[serde(with = "opt_decimal_string")]
    pub lower_limit_price: Option<Decimal>,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: TriggerPriceType,
    /// Upward trigger spread
    #[serde(with = "opt_decimal_string")]
    pub trigger_spread_up: Option<Decimal>,
    /// Downward trigger spread
    #[serde(with = "opt_decimal_string")]
    pub trigger_spread_down: Option<Decimal>,
    /// Upward trigger percent
    #[serde(with = "opt_decimal_string")]
    pub trigger_percent_up: Option<Decimal>,
    /// Downward trigger percent
    #[serde(with = "opt_decimal_string")]
    pub trigger_percent_down: Option<Decimal>,
    /// Pullback percent
    #[serde(with = "opt_decimal_string")]
    pub pullback_percent: Option<Decimal>,
    /// Pullback spread
    #[serde(with = "opt_decimal_string")]
    pub pullback_spread: Option<Decimal>,
    /// Rebound percent
    #[serde(with = "opt_decimal_string")]
    pub rebound_percent: Option<Decimal>,
    /// Rebound spread
    #[serde(with = "opt_decimal_string")]
    pub rebound_spread: Option<Decimal>,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: GridTimeInForce,
    /// Quantity per trigger
    #[serde(with = "opt_decimal_string")]
    pub trigger_quantity: Option<Decimal>,
    /// Quantity per sell trigger
    #[serde(with = "opt_decimal_string")]
    pub trigger_sell_quantity: Option<Decimal>,
    /// Quantity per buy trigger
    #[serde(with = "opt_decimal_string")]
    pub trigger_buy_quantity: Option<Decimal>,
    /// Quantity handled at the upper bound
    #[serde(with = "opt_decimal_string")]
    pub upper_limit_quantity: Option<Decimal>,
    /// Quantity handled at the lower bound
    #[serde(with = "opt_decimal_string")]
    pub lower_limit_quantity: Option<Decimal>,
    /// Action at the upper bound
    pub upper_limit_event: GridLimitEvent,
    /// Action at the lower bound
    pub lower_limit_event: GridLimitEvent,
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
    #[serde(with = "opt_decimal_string")]
    pub price: Option<Decimal>,
    /// Order quantity
    #[serde(with = "opt_decimal_string")]
    pub quantity: Option<Decimal>,
    /// Executed average price
    #[serde(with = "opt_decimal_string")]
    pub executed_price: Option<Decimal>,
    /// Executed total quantity
    #[serde(with = "opt_decimal_string")]
    pub executed_qty: Option<Decimal>,
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
    #[serde(with = "opt_decimal_string")]
    pub trigger_price: Option<Decimal>,
    /// Rejection reason, if any
    pub msg: String,
    /// Settlement currency
    pub currency: String,
    /// Latest quote price
    #[serde(with = "opt_decimal_string")]
    pub last_done: Option<Decimal>,
    /// Last updated time (RFC3339)
    #[serde(
        deserialize_with = "serde_utils::timestamp_opt::deserialize",
        serialize_with = "serde_utils::rfc3339_opt::serialize"
    )]
    pub updated_at: Option<OffsetDateTime>,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: GridTimeInForce,
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

/// A price-step (bid-size) rule entry from the symbol-info response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridBidSize {
    /// Range start price (inclusive)
    #[serde(with = "opt_decimal_string")]
    pub str_proceed: Option<Decimal>,
    /// Range end price
    #[serde(with = "opt_decimal_string")]
    pub end_proceed: Option<Decimal>,
    /// Price step within the range
    #[serde(with = "opt_decimal_string")]
    pub bid_size: Option<Decimal>,
}

/// Channel / authorization info nested in the symbol-info response, holding the
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

/// Security (symbol) info (`/v1/orders/info`) used to build a grid order.
///
/// Returns the target security's name, latest price, lot sizes, price-step
/// rules and channel / authorization info needed by the grid order window. The
/// endpoint takes a `counter_id` query parameter (a symbol such as `700.HK` is
/// accepted).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GridSymbolInfo {
    /// Security name
    pub name: String,
    /// Latest quote price
    #[serde(with = "opt_decimal_string")]
    pub last_done: Option<Decimal>,
    /// Board lot size
    #[serde(with = "opt_decimal_string")]
    pub lot_size: Option<Decimal>,
    /// Buy-side board lot size
    #[serde(with = "opt_decimal_string")]
    pub buy_lot_size: Option<Decimal>,
    /// Sell-side board lot size
    #[serde(with = "opt_decimal_string")]
    pub sell_lot_size: Option<Decimal>,
    /// Price-step (bid-size) rule table
    pub bid_sizes: Vec<GridBidSize>,
    /// Channel / authorization info (strategy grant, RTH, currencies).
    ///
    /// Uses `alias` (not `rename`) so deserialization still accepts the
    /// server's `channel_infos` key, while serialization emits the clean
    /// `channel_info` name for downstream consumers (e.g. CLI `--format json`).
    #[serde(alias = "channel_infos")]
    pub channel_info: GridChannelInfo,
}
