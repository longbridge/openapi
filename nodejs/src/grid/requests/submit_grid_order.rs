use napi::bindgen_prelude::ClassInstance;

use crate::decimal::Decimal;

/// Grid trading rule — parameters for submit / replace.
///
/// Prices and quantities are decimals; enum-like fields are raw integers whose
/// code tables are documented inline.
#[napi_derive::napi(object)]
pub struct GridTradeRule<'env> {
    /// Base price the grid is anchored to
    pub submitted_base_price: Option<ClassInstance<'env, Decimal>>,
    /// Upper price bound
    pub upper_limit_price: Option<ClassInstance<'env, Decimal>>,
    /// Lower price bound
    pub lower_limit_price: Option<ClassInstance<'env, Decimal>>,
    /// Trigger price type (only `1` / `2` allowed)
    pub trigger_price_type: Option<i32>,
    /// Upward trigger spread (absolute)
    pub trigger_spread_up: Option<ClassInstance<'env, Decimal>>,
    /// Downward trigger spread (absolute)
    pub trigger_spread_down: Option<ClassInstance<'env, Decimal>>,
    /// Upward trigger percent
    pub trigger_percent_up: Option<ClassInstance<'env, Decimal>>,
    /// Downward trigger percent
    pub trigger_percent_down: Option<ClassInstance<'env, Decimal>>,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: Option<bool>,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: Option<i32>,
    /// Quantity handled when the upper bound is reached
    pub upper_limit_quantity: Option<ClassInstance<'env, Decimal>>,
    /// Quantity handled when the lower bound is reached
    pub lower_limit_quantity: Option<ClassInstance<'env, Decimal>>,
    /// Expiry time (unix seconds), used with GTD
    pub expire_time: Option<i64>,
    /// Action when the upper bound is reached (only `1` / `2` allowed)
    pub upper_limit_event: Option<i32>,
    /// Action when the lower bound is reached (only `1` / `2` allowed)
    pub lower_limit_event: Option<i32>,
    /// Sell-side order-book depth (-5..5, `0` = use `grid_order_type_up`)
    pub trigger_sell_depth: Option<i32>,
    /// Buy-side order-book depth (-5..5, `0` = use `grid_order_type_down`)
    pub trigger_buy_depth: Option<i32>,
    /// Quantity per trigger
    pub trigger_quantity: Option<ClassInstance<'env, Decimal>>,
    /// Whether short selling is allowed
    pub support_shortsell: Option<bool>,
    /// Regular trading hours flag (`0` / `1` / `2`)
    pub rth: Option<i32>,
    /// Sell-side order type when depth is `0` (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_up: Option<String>,
    /// Buy-side order type when depth is `0` (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_down: Option<String>,
}

impl<'env> From<GridTradeRule<'env>> for longbridge::grid::GridTradeRule {
    fn from(r: GridTradeRule<'env>) -> Self {
        longbridge::grid::GridTradeRule {
            submitted_base_price: r.submitted_base_price.map(|v| v.0),
            upper_limit_price: r.upper_limit_price.map(|v| v.0),
            lower_limit_price: r.lower_limit_price.map(|v| v.0),
            trigger_price_type: r.trigger_price_type,
            trigger_spread_up: r.trigger_spread_up.map(|v| v.0),
            trigger_spread_down: r.trigger_spread_down.map(|v| v.0),
            trigger_percent_up: r.trigger_percent_up.map(|v| v.0),
            trigger_percent_down: r.trigger_percent_down.map(|v| v.0),
            multiple_trigger: r.multiple_trigger,
            time_in_force: r.time_in_force,
            upper_limit_quantity: r.upper_limit_quantity.map(|v| v.0),
            lower_limit_quantity: r.lower_limit_quantity.map(|v| v.0),
            expire_time: r.expire_time,
            upper_limit_event: r.upper_limit_event,
            lower_limit_event: r.lower_limit_event,
            trigger_sell_depth: r.trigger_sell_depth,
            trigger_buy_depth: r.trigger_buy_depth,
            trigger_quantity: r.trigger_quantity.map(|v| v.0),
            support_shortsell: r.support_shortsell,
            rth: r.rth,
            grid_order_type_up: r.grid_order_type_up,
            grid_order_type_down: r.grid_order_type_down,
        }
    }
}

/// Options for submit grid trading order request
#[napi_derive::napi(object)]
pub struct SubmitGridOrderOptions<'env> {
    /// Security code
    pub symbol: String,
    /// Settlement currency
    pub settlement_currency: String,
    /// Grid trading rule
    pub grid_trading_rule: GridTradeRule<'env>,
}

impl<'env> From<SubmitGridOrderOptions<'env>> for longbridge::grid::SubmitGridOrderOptions {
    #[inline]
    fn from(opts: SubmitGridOrderOptions<'env>) -> Self {
        longbridge::grid::SubmitGridOrderOptions::new(
            opts.symbol,
            opts.settlement_currency,
            opts.grid_trading_rule.into(),
        )
    }
}
