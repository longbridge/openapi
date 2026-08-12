use std::os::raw::c_char;

use longbridge::grid::{
    GridBidSize, GridChannelInfo, GridOrder, GridOrderDetail, GridOrderHistory, GridOrderSubOrder,
    GridSymbolInfo, SubmitGridOrderResponse, TriggerOrder,
};
use time::OffsetDateTime;

use crate::types::{CDecimal, CMarket, COption, CString, CVec, ToFFI};

// ââ Grid trading types
// âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââ

/// Grid trading rule — parameters for submit / replace.
#[derive(Debug)]
#[repr(C)]
pub struct CGridTradeRule {
    /// Base price the grid is anchored to (can be null)
    pub submitted_base_price: *const CDecimal,
    /// Upper price bound (can be null)
    pub upper_limit_price: *const CDecimal,
    /// Lower price bound (can be null)
    pub lower_limit_price: *const CDecimal,
    /// Trigger price type (only `1` / `2` allowed) (can be null)
    pub trigger_price_type: *const i32,
    /// Upward trigger spread (absolute) (can be null)
    pub trigger_spread_up: *const CDecimal,
    /// Downward trigger spread (absolute) (can be null)
    pub trigger_spread_down: *const CDecimal,
    /// Upward trigger percent (can be null)
    pub trigger_percent_up: *const CDecimal,
    /// Downward trigger percent (can be null)
    pub trigger_percent_down: *const CDecimal,
    /// Whether a single grid level may trigger multiple times (can be null)
    pub multiple_trigger: *const bool,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD) (can be null)
    pub time_in_force: *const i32,
    /// Quantity handled when the upper bound is reached (can be null)
    pub upper_limit_quantity: *const CDecimal,
    /// Quantity handled when the lower bound is reached (can be null)
    pub lower_limit_quantity: *const CDecimal,
    /// Expiry time (unix seconds), used with GTD (can be null)
    pub expire_time: *const i64,
    /// Action when the upper bound is reached (only `1` / `2` allowed) (can be
    /// null)
    pub upper_limit_event: *const i32,
    /// Action when the lower bound is reached (only `1` / `2` allowed) (can be
    /// null)
    pub lower_limit_event: *const i32,
    /// Sell-side order-book depth (-5..5, `0` = use `grid_order_type_up`) (can
    /// be null)
    pub trigger_sell_depth: *const i32,
    /// Buy-side order-book depth (-5..5, `0` = use `grid_order_type_down`) (can
    /// be null)
    pub trigger_buy_depth: *const i32,
    /// Quantity per trigger (can be null)
    pub trigger_quantity: *const CDecimal,
    /// Whether short selling is allowed (can be null)
    pub support_shortsell: *const bool,
    /// Regular trading hours flag (`0` / `1` / `2`) (can be null)
    pub rth: *const i32,
    /// Sell-side order type when depth is `0` (`GMO` / `GLO` / `GTG`) (can be
    /// null)
    pub grid_order_type_up: *const c_char,
    /// Buy-side order type when depth is `0` (`GMO` / `GLO` / `GTG`) (can be
    /// null)
    pub grid_order_type_down: *const c_char,
}

/// Options for submit grid trading order request
#[derive(Debug)]
#[repr(C)]
pub struct CSubmitGridOrderOptions {
    /// Security symbol (e.g. `700.HK`)
    pub symbol: *const c_char,
    /// Settlement currency
    pub settlement_currency: *const c_char,
    /// Grid trading rule
    pub grid_trading_rule: CGridTradeRule,
}

/// Options for replace grid trading order request
#[derive(Debug)]
#[repr(C)]
pub struct CReplaceGridOrderOptions {
    /// Grid master order ID
    pub order_id: *const c_char,
    /// Grid trading rule
    pub grid_trading_rule: CGridTradeRule,
}

/// Options for get grid trading orders (list) request
#[derive(Debug)]
#[repr(C)]
pub struct CGetGridOrdersOptions {
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub limit: *const i32,
    /// Market (can be null)
    pub market: *const CMarket,
    /// Comma-joined status filter (e.g. `Performing,Suspended`) (can be null)
    pub status: *const c_char,
    /// Security symbol filter (e.g. `700.HK`) (can be null)
    pub symbol: *const c_char,
    /// Sort field (can be null)
    pub sort_by: *const c_char,
    /// Sort order (can be null)
    pub sort_order: *const c_char,
}

/// Options for query grid trading orders by IDs request
#[derive(Debug)]
#[repr(C)]
pub struct CGetGridOrdersByIdsOptions {
    /// Grid master order IDs
    pub order_ids: *const *const c_char,
    /// Number of order IDs
    pub num_order_ids: usize,
}

/// Options for get grid trading order detail request
#[derive(Debug)]
#[repr(C)]
pub struct CGetGridOrderDetailOptions {
    /// Grid master order ID
    pub order_id: *const c_char,
    /// History cursor for paging through the trigger history (can be null)
    pub history_id: *const c_char,
    /// Page size (can be null)
    pub limit: *const i32,
}

/// Options for get grid trading trigger history request
#[derive(Debug)]
#[repr(C)]
pub struct CGetGridTriggerHistoryOptions {
    /// Grid master order ID
    pub grid_order_id: *const c_char,
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub limit: *const i32,
}

/// Response for submit grid trading order request
#[repr(C)]
pub struct CSubmitGridOrderResponse {
    /// Grid master order id
    pub order_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CSubmitGridOrderResponseOwned {
    order_id: CString,
}

impl From<SubmitGridOrderResponse> for CSubmitGridOrderResponseOwned {
    fn from(resp: SubmitGridOrderResponse) -> Self {
        CSubmitGridOrderResponseOwned {
            order_id: resp.order_id.into(),
        }
    }
}

impl ToFFI for CSubmitGridOrderResponseOwned {
    type FFIType = CSubmitGridOrderResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CSubmitGridOrderResponse {
            order_id: self.order_id.to_ffi_type(),
        }
    }
}

/// A grid trading order (element of the list / by-ids responses).
#[repr(C)]
pub struct CGridOrder {
    /// Grid master order ID
    pub order_id: *const c_char,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: *const c_char,
    /// Stock name
    pub stock_name: *const c_char,
    /// Market
    pub market: *const c_char,
    /// Order status
    pub status: *const c_char,
    /// Grid running status
    pub grid_status: *const c_char,
    /// Submitted base price (can be null)
    pub submitted_base_price: *const CDecimal,
    /// Current base price (can be null)
    pub current_base_price: *const CDecimal,
    /// Base price before the last trigger (can be null)
    pub pre_trigger_base_price: *const CDecimal,
    /// Base price after the last trigger (can be null)
    pub post_trigger_base_price: *const CDecimal,
    /// Upper price bound (can be null)
    pub upper_limit_price: *const CDecimal,
    /// Lower price bound (can be null)
    pub lower_limit_price: *const CDecimal,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: i32,
    /// Upward trigger spread (can be null)
    pub trigger_spread_up: *const CDecimal,
    /// Downward trigger spread (can be null)
    pub trigger_spread_down: *const CDecimal,
    /// Upward trigger percent (can be null)
    pub trigger_percent_up: *const CDecimal,
    /// Downward trigger percent (can be null)
    pub trigger_percent_down: *const CDecimal,
    /// Pullback percent (can be null)
    pub pullback_percent: *const CDecimal,
    /// Pullback spread (can be null)
    pub pullback_spread: *const CDecimal,
    /// Rebound percent (can be null)
    pub rebound_percent: *const CDecimal,
    /// Rebound spread (can be null)
    pub rebound_spread: *const CDecimal,
    /// Sell-side execution order type (e.g. `MO`)
    pub trigger_sell_order_type: *const c_char,
    /// Buy-side execution order type (e.g. `MO`)
    pub trigger_buy_order_type: *const c_char,
    /// Sell-side order-book depth
    pub trigger_sell_depth: i32,
    /// Buy-side order-book depth
    pub trigger_buy_depth: i32,
    /// Quantity per trigger (can be null)
    pub trigger_quantity: *const CDecimal,
    /// Quantity per sell trigger (can be null)
    pub trigger_sell_quantity: *const CDecimal,
    /// Quantity per buy trigger (can be null)
    pub trigger_buy_quantity: *const CDecimal,
    /// Quantity handled at the upper bound (can be null)
    pub upper_limit_quantity: *const CDecimal,
    /// Quantity handled at the lower bound (can be null)
    pub lower_limit_quantity: *const CDecimal,
    /// Action at the upper bound
    pub upper_limit_event: i32,
    /// Action at the lower bound
    pub lower_limit_event: i32,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Number of times the grid has triggered
    pub trigger_times: i32,
    /// Accumulated bought quantity (can be null)
    pub total_buy_quantity: *const CDecimal,
    /// Accumulated sold quantity (can be null)
    pub total_sell_quantity: *const CDecimal,
    /// Accumulated profit balance (can be null)
    pub total_profit_balance: *const CDecimal,
    /// Settlement currency
    pub settlement_currency: *const c_char,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    pub gtd: *const c_char,
    /// Created time (unix timestamp, maybe null)
    pub created_at: *const i64,
    /// Regular trading hours flag
    pub rth: i32,
    /// Whether short selling is allowed
    pub support_shortsell: bool,
    /// Sell-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_up: *const c_char,
    /// Buy-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_down: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CGridOrderOwned {
    order_id: CString,
    symbol: CString,
    stock_name: CString,
    market: CString,
    status: CString,
    grid_status: CString,
    submitted_base_price: COption<CDecimal>,
    current_base_price: COption<CDecimal>,
    pre_trigger_base_price: COption<CDecimal>,
    post_trigger_base_price: COption<CDecimal>,
    upper_limit_price: COption<CDecimal>,
    lower_limit_price: COption<CDecimal>,
    trigger_price_type: i32,
    trigger_spread_up: COption<CDecimal>,
    trigger_spread_down: COption<CDecimal>,
    trigger_percent_up: COption<CDecimal>,
    trigger_percent_down: COption<CDecimal>,
    pullback_percent: COption<CDecimal>,
    pullback_spread: COption<CDecimal>,
    rebound_percent: COption<CDecimal>,
    rebound_spread: COption<CDecimal>,
    trigger_sell_order_type: CString,
    trigger_buy_order_type: CString,
    trigger_sell_depth: i32,
    trigger_buy_depth: i32,
    trigger_quantity: COption<CDecimal>,
    trigger_sell_quantity: COption<CDecimal>,
    trigger_buy_quantity: COption<CDecimal>,
    upper_limit_quantity: COption<CDecimal>,
    lower_limit_quantity: COption<CDecimal>,
    upper_limit_event: i32,
    lower_limit_event: i32,
    multiple_trigger: bool,
    trigger_times: i32,
    total_buy_quantity: COption<CDecimal>,
    total_sell_quantity: COption<CDecimal>,
    total_profit_balance: COption<CDecimal>,
    settlement_currency: CString,
    time_in_force: i32,
    gtd: CString,
    created_at: Option<i64>,
    rth: i32,
    support_shortsell: bool,
    grid_order_type_up: CString,
    grid_order_type_down: CString,
}

impl From<GridOrder> for CGridOrderOwned {
    fn from(order: GridOrder) -> Self {
        CGridOrderOwned {
            order_id: order.order_id.into(),
            symbol: order.symbol.into(),
            stock_name: order.stock_name.into(),
            market: order.market.into(),
            status: order.status.into(),
            grid_status: order.grid_status.into(),
            submitted_base_price: order.submitted_base_price.into(),
            current_base_price: order.current_base_price.into(),
            pre_trigger_base_price: order.pre_trigger_base_price.into(),
            post_trigger_base_price: order.post_trigger_base_price.into(),
            upper_limit_price: order.upper_limit_price.into(),
            lower_limit_price: order.lower_limit_price.into(),
            trigger_price_type: order.trigger_price_type.into(),
            trigger_spread_up: order.trigger_spread_up.into(),
            trigger_spread_down: order.trigger_spread_down.into(),
            trigger_percent_up: order.trigger_percent_up.into(),
            trigger_percent_down: order.trigger_percent_down.into(),
            pullback_percent: order.pullback_percent.into(),
            pullback_spread: order.pullback_spread.into(),
            rebound_percent: order.rebound_percent.into(),
            rebound_spread: order.rebound_spread.into(),
            trigger_sell_order_type: order.trigger_sell_order_type.into(),
            trigger_buy_order_type: order.trigger_buy_order_type.into(),
            trigger_sell_depth: order.trigger_sell_depth,
            trigger_buy_depth: order.trigger_buy_depth,
            trigger_quantity: order.trigger_quantity.into(),
            trigger_sell_quantity: order.trigger_sell_quantity.into(),
            trigger_buy_quantity: order.trigger_buy_quantity.into(),
            upper_limit_quantity: order.upper_limit_quantity.into(),
            lower_limit_quantity: order.lower_limit_quantity.into(),
            upper_limit_event: order.upper_limit_event.into(),
            lower_limit_event: order.lower_limit_event.into(),
            multiple_trigger: order.multiple_trigger,
            trigger_times: order.trigger_times,
            total_buy_quantity: order.total_buy_quantity.into(),
            total_sell_quantity: order.total_sell_quantity.into(),
            total_profit_balance: order.total_profit_balance.into(),
            settlement_currency: order.settlement_currency.into(),
            time_in_force: order.time_in_force.into(),
            gtd: order.gtd.into(),
            created_at: order.created_at.map(OffsetDateTime::unix_timestamp),
            rth: order.rth,
            support_shortsell: order.support_shortsell,
            grid_order_type_up: order.grid_order_type_up.into(),
            grid_order_type_down: order.grid_order_type_down.into(),
        }
    }
}

impl ToFFI for CGridOrderOwned {
    type FFIType = CGridOrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridOrder {
            order_id: self.order_id.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            stock_name: self.stock_name.to_ffi_type(),
            market: self.market.to_ffi_type(),
            status: self.status.to_ffi_type(),
            grid_status: self.grid_status.to_ffi_type(),
            submitted_base_price: self.submitted_base_price.to_ffi_type().to_ffi_type(),
            current_base_price: self.current_base_price.to_ffi_type().to_ffi_type(),
            pre_trigger_base_price: self.pre_trigger_base_price.to_ffi_type().to_ffi_type(),
            post_trigger_base_price: self.post_trigger_base_price.to_ffi_type().to_ffi_type(),
            upper_limit_price: self.upper_limit_price.to_ffi_type().to_ffi_type(),
            lower_limit_price: self.lower_limit_price.to_ffi_type().to_ffi_type(),
            trigger_price_type: self.trigger_price_type,
            trigger_spread_up: self.trigger_spread_up.to_ffi_type().to_ffi_type(),
            trigger_spread_down: self.trigger_spread_down.to_ffi_type().to_ffi_type(),
            trigger_percent_up: self.trigger_percent_up.to_ffi_type().to_ffi_type(),
            trigger_percent_down: self.trigger_percent_down.to_ffi_type().to_ffi_type(),
            pullback_percent: self.pullback_percent.to_ffi_type().to_ffi_type(),
            pullback_spread: self.pullback_spread.to_ffi_type().to_ffi_type(),
            rebound_percent: self.rebound_percent.to_ffi_type().to_ffi_type(),
            rebound_spread: self.rebound_spread.to_ffi_type().to_ffi_type(),
            trigger_sell_order_type: self.trigger_sell_order_type.to_ffi_type(),
            trigger_buy_order_type: self.trigger_buy_order_type.to_ffi_type(),
            trigger_sell_depth: self.trigger_sell_depth,
            trigger_buy_depth: self.trigger_buy_depth,
            trigger_quantity: self.trigger_quantity.to_ffi_type().to_ffi_type(),
            trigger_sell_quantity: self.trigger_sell_quantity.to_ffi_type().to_ffi_type(),
            trigger_buy_quantity: self.trigger_buy_quantity.to_ffi_type().to_ffi_type(),
            upper_limit_quantity: self.upper_limit_quantity.to_ffi_type().to_ffi_type(),
            lower_limit_quantity: self.lower_limit_quantity.to_ffi_type().to_ffi_type(),
            upper_limit_event: self.upper_limit_event,
            lower_limit_event: self.lower_limit_event,
            multiple_trigger: self.multiple_trigger,
            trigger_times: self.trigger_times,
            total_buy_quantity: self.total_buy_quantity.to_ffi_type().to_ffi_type(),
            total_sell_quantity: self.total_sell_quantity.to_ffi_type().to_ffi_type(),
            total_profit_balance: self.total_profit_balance.to_ffi_type().to_ffi_type(),
            settlement_currency: self.settlement_currency.to_ffi_type(),
            time_in_force: self.time_in_force,
            gtd: self.gtd.to_ffi_type(),
            created_at: self
                .created_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            rth: self.rth,
            support_shortsell: self.support_shortsell,
            grid_order_type_up: self.grid_order_type_up.to_ffi_type(),
            grid_order_type_down: self.grid_order_type_down.to_ffi_type(),
        }
    }
}

/// A triggered sub-order carried in the grid order detail.
#[repr(C)]
pub struct CGridOrderSubOrder {
    /// Sub-order ID
    pub id: *const c_char,
    /// Order price (can be null)
    pub price: *const CDecimal,
    /// Order type
    pub order_type: *const c_char,
    /// Order quantity (can be null)
    pub quantity: *const CDecimal,
    /// Executed quantity (can be null)
    pub executed_qty: *const CDecimal,
    /// Buy / sell direction
    pub action: i32,
    /// Order status
    pub status: *const c_char,
    /// Submitted time (unix timestamp, maybe null)
    pub submitted_at: *const i64,
    /// Regular trading hours flag
    pub rth: i32,
}

#[derive(Debug)]
pub(crate) struct CGridOrderSubOrderOwned {
    id: CString,
    price: COption<CDecimal>,
    order_type: CString,
    quantity: COption<CDecimal>,
    executed_qty: COption<CDecimal>,
    action: i32,
    status: CString,
    submitted_at: Option<i64>,
    rth: i32,
}

impl From<GridOrderSubOrder> for CGridOrderSubOrderOwned {
    fn from(sub: GridOrderSubOrder) -> Self {
        CGridOrderSubOrderOwned {
            id: sub.id.into(),
            price: sub.price.into(),
            order_type: sub.order_type.into(),
            quantity: sub.quantity.into(),
            executed_qty: sub.executed_qty.into(),
            action: sub.action,
            status: sub.status.into(),
            submitted_at: sub.submitted_at.map(OffsetDateTime::unix_timestamp),
            rth: sub.rth,
        }
    }
}

impl ToFFI for CGridOrderSubOrderOwned {
    type FFIType = CGridOrderSubOrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridOrderSubOrder {
            id: self.id.to_ffi_type(),
            price: self.price.to_ffi_type().to_ffi_type(),
            order_type: self.order_type.to_ffi_type(),
            quantity: self.quantity.to_ffi_type().to_ffi_type(),
            executed_qty: self.executed_qty.to_ffi_type().to_ffi_type(),
            action: self.action,
            status: self.status.to_ffi_type(),
            submitted_at: self
                .submitted_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            rth: self.rth,
        }
    }
}

/// A grid order lifecycle-history entry carried in the grid order detail.
#[repr(C)]
pub struct CGridOrderHistory {
    /// History entry ID (paging cursor)
    pub history_id: *const c_char,
    /// Created time (unix timestamp, maybe null)
    pub created_at: *const i64,
    /// Status at this point
    pub status: *const c_char,
    /// Suspend reason, if any
    pub suspend_reason: *const c_char,
    /// Additional reason detail, if any
    pub reason: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CGridOrderHistoryOwned {
    history_id: CString,
    created_at: Option<i64>,
    status: CString,
    suspend_reason: CString,
    reason: CString,
}

impl From<GridOrderHistory> for CGridOrderHistoryOwned {
    fn from(h: GridOrderHistory) -> Self {
        CGridOrderHistoryOwned {
            history_id: h.history_id.into(),
            created_at: h.created_at.map(OffsetDateTime::unix_timestamp),
            status: h.status.into(),
            suspend_reason: h.suspend_reason.into(),
            reason: h.reason.into(),
        }
    }
}

impl ToFFI for CGridOrderHistoryOwned {
    type FFIType = CGridOrderHistory;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridOrderHistory {
            history_id: self.history_id.to_ffi_type(),
            created_at: self
                .created_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            status: self.status.to_ffi_type(),
            suspend_reason: self.suspend_reason.to_ffi_type(),
            reason: self.reason.to_ffi_type(),
        }
    }
}

/// Detail of a grid trading order.
#[repr(C)]
pub struct CGridOrderDetail {
    /// Grid master order ID
    pub order_id: *const c_char,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: *const c_char,
    /// Stock name
    pub stock_name: *const c_char,
    /// Order status
    pub status: *const c_char,
    /// Grid running status
    pub grid_status: *const c_char,
    /// Suspend reason, if any
    pub suspend_reason: *const c_char,
    /// Sleeping reason, if any
    pub sleeping_reason: *const c_char,
    /// Submitted base price (can be null)
    pub submitted_base_price: *const CDecimal,
    /// Current base price (can be null)
    pub current_base_price: *const CDecimal,
    /// Upper price bound (can be null)
    pub upper_limit_price: *const CDecimal,
    /// Lower price bound (can be null)
    pub lower_limit_price: *const CDecimal,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: i32,
    /// Upward trigger spread (can be null)
    pub trigger_spread_up: *const CDecimal,
    /// Downward trigger spread (can be null)
    pub trigger_spread_down: *const CDecimal,
    /// Upward trigger percent (can be null)
    pub trigger_percent_up: *const CDecimal,
    /// Downward trigger percent (can be null)
    pub trigger_percent_down: *const CDecimal,
    /// Pullback percent (can be null)
    pub pullback_percent: *const CDecimal,
    /// Pullback spread (can be null)
    pub pullback_spread: *const CDecimal,
    /// Rebound percent (can be null)
    pub rebound_percent: *const CDecimal,
    /// Rebound spread (can be null)
    pub rebound_spread: *const CDecimal,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Quantity per trigger (can be null)
    pub trigger_quantity: *const CDecimal,
    /// Quantity per sell trigger (can be null)
    pub trigger_sell_quantity: *const CDecimal,
    /// Quantity per buy trigger (can be null)
    pub trigger_buy_quantity: *const CDecimal,
    /// Quantity handled at the upper bound (can be null)
    pub upper_limit_quantity: *const CDecimal,
    /// Quantity handled at the lower bound (can be null)
    pub lower_limit_quantity: *const CDecimal,
    /// Action at the upper bound
    pub upper_limit_event: i32,
    /// Action at the lower bound
    pub lower_limit_event: i32,
    /// Sell-side order-book depth
    pub trigger_sell_depth: i32,
    /// Buy-side order-book depth
    pub trigger_buy_depth: i32,
    /// Created time (unix timestamp, maybe null)
    pub created_at: *const i64,
    /// Last updated time (unix timestamp, maybe null)
    pub updated_at: *const i64,
    /// Settlement currency
    pub settlement_currency: *const c_char,
    /// Expiry time (unix timestamp, maybe null)
    pub expire_time: *const i64,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    pub gtd: *const c_char,
    /// Triggered sub-orders
    pub grid_sub_orders: *const CGridOrderSubOrder,
    /// Number of triggered sub-orders
    pub num_grid_sub_orders: usize,
    /// Whether there are more sub-orders to page
    pub sub_has_more: bool,
    /// Lifecycle history entries
    pub grid_order_history: *const CGridOrderHistory,
    /// Number of lifecycle history entries
    pub num_grid_order_history: usize,
    /// Whether there are more history entries to page
    pub history_has_more: bool,
    /// Whether short selling is allowed
    pub support_shortsell: bool,
    /// Regular trading hours flag
    pub rth: i32,
    /// Sell-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_up: *const c_char,
    /// Buy-side grid order type (`GMO` / `GLO` / `GTG`)
    pub grid_order_type_down: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CGridOrderDetailOwned {
    order_id: CString,
    symbol: CString,
    stock_name: CString,
    status: CString,
    grid_status: CString,
    suspend_reason: CString,
    sleeping_reason: CString,
    submitted_base_price: COption<CDecimal>,
    current_base_price: COption<CDecimal>,
    upper_limit_price: COption<CDecimal>,
    lower_limit_price: COption<CDecimal>,
    trigger_price_type: i32,
    trigger_spread_up: COption<CDecimal>,
    trigger_spread_down: COption<CDecimal>,
    trigger_percent_up: COption<CDecimal>,
    trigger_percent_down: COption<CDecimal>,
    pullback_percent: COption<CDecimal>,
    pullback_spread: COption<CDecimal>,
    rebound_percent: COption<CDecimal>,
    rebound_spread: COption<CDecimal>,
    multiple_trigger: bool,
    time_in_force: i32,
    trigger_quantity: COption<CDecimal>,
    trigger_sell_quantity: COption<CDecimal>,
    trigger_buy_quantity: COption<CDecimal>,
    upper_limit_quantity: COption<CDecimal>,
    lower_limit_quantity: COption<CDecimal>,
    upper_limit_event: i32,
    lower_limit_event: i32,
    trigger_sell_depth: i32,
    trigger_buy_depth: i32,
    created_at: Option<i64>,
    updated_at: Option<i64>,
    settlement_currency: CString,
    expire_time: Option<i64>,
    gtd: CString,
    grid_sub_orders: CVec<CGridOrderSubOrderOwned>,
    sub_has_more: bool,
    grid_order_history: CVec<CGridOrderHistoryOwned>,
    history_has_more: bool,
    support_shortsell: bool,
    rth: i32,
    grid_order_type_up: CString,
    grid_order_type_down: CString,
}

impl From<GridOrderDetail> for CGridOrderDetailOwned {
    fn from(d: GridOrderDetail) -> Self {
        CGridOrderDetailOwned {
            order_id: d.order_id.into(),
            symbol: d.symbol.into(),
            stock_name: d.stock_name.into(),
            status: d.status.into(),
            grid_status: d.grid_status.into(),
            suspend_reason: d.suspend_reason.into(),
            sleeping_reason: d.sleeping_reason.into(),
            submitted_base_price: d.submitted_base_price.into(),
            current_base_price: d.current_base_price.into(),
            upper_limit_price: d.upper_limit_price.into(),
            lower_limit_price: d.lower_limit_price.into(),
            trigger_price_type: d.trigger_price_type.into(),
            trigger_spread_up: d.trigger_spread_up.into(),
            trigger_spread_down: d.trigger_spread_down.into(),
            trigger_percent_up: d.trigger_percent_up.into(),
            trigger_percent_down: d.trigger_percent_down.into(),
            pullback_percent: d.pullback_percent.into(),
            pullback_spread: d.pullback_spread.into(),
            rebound_percent: d.rebound_percent.into(),
            rebound_spread: d.rebound_spread.into(),
            multiple_trigger: d.multiple_trigger,
            time_in_force: d.time_in_force.into(),
            trigger_quantity: d.trigger_quantity.into(),
            trigger_sell_quantity: d.trigger_sell_quantity.into(),
            trigger_buy_quantity: d.trigger_buy_quantity.into(),
            upper_limit_quantity: d.upper_limit_quantity.into(),
            lower_limit_quantity: d.lower_limit_quantity.into(),
            upper_limit_event: d.upper_limit_event.into(),
            lower_limit_event: d.lower_limit_event.into(),
            trigger_sell_depth: d.trigger_sell_depth,
            trigger_buy_depth: d.trigger_buy_depth,
            created_at: d.created_at.map(OffsetDateTime::unix_timestamp),
            updated_at: d.updated_at.map(OffsetDateTime::unix_timestamp),
            settlement_currency: d.settlement_currency.into(),
            expire_time: d.expire_time.map(OffsetDateTime::unix_timestamp),
            gtd: d.gtd.into(),
            grid_sub_orders: d.grid_sub_orders.into(),
            sub_has_more: d.sub_has_more,
            grid_order_history: d.grid_order_history.into(),
            history_has_more: d.history_has_more,
            support_shortsell: d.support_shortsell,
            rth: d.rth,
            grid_order_type_up: d.grid_order_type_up.into(),
            grid_order_type_down: d.grid_order_type_down.into(),
        }
    }
}

impl ToFFI for CGridOrderDetailOwned {
    type FFIType = CGridOrderDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridOrderDetail {
            order_id: self.order_id.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            stock_name: self.stock_name.to_ffi_type(),
            status: self.status.to_ffi_type(),
            grid_status: self.grid_status.to_ffi_type(),
            suspend_reason: self.suspend_reason.to_ffi_type(),
            sleeping_reason: self.sleeping_reason.to_ffi_type(),
            submitted_base_price: self.submitted_base_price.to_ffi_type().to_ffi_type(),
            current_base_price: self.current_base_price.to_ffi_type().to_ffi_type(),
            upper_limit_price: self.upper_limit_price.to_ffi_type().to_ffi_type(),
            lower_limit_price: self.lower_limit_price.to_ffi_type().to_ffi_type(),
            trigger_price_type: self.trigger_price_type,
            trigger_spread_up: self.trigger_spread_up.to_ffi_type().to_ffi_type(),
            trigger_spread_down: self.trigger_spread_down.to_ffi_type().to_ffi_type(),
            trigger_percent_up: self.trigger_percent_up.to_ffi_type().to_ffi_type(),
            trigger_percent_down: self.trigger_percent_down.to_ffi_type().to_ffi_type(),
            pullback_percent: self.pullback_percent.to_ffi_type().to_ffi_type(),
            pullback_spread: self.pullback_spread.to_ffi_type().to_ffi_type(),
            rebound_percent: self.rebound_percent.to_ffi_type().to_ffi_type(),
            rebound_spread: self.rebound_spread.to_ffi_type().to_ffi_type(),
            multiple_trigger: self.multiple_trigger,
            time_in_force: self.time_in_force,
            trigger_quantity: self.trigger_quantity.to_ffi_type().to_ffi_type(),
            trigger_sell_quantity: self.trigger_sell_quantity.to_ffi_type().to_ffi_type(),
            trigger_buy_quantity: self.trigger_buy_quantity.to_ffi_type().to_ffi_type(),
            upper_limit_quantity: self.upper_limit_quantity.to_ffi_type().to_ffi_type(),
            lower_limit_quantity: self.lower_limit_quantity.to_ffi_type().to_ffi_type(),
            upper_limit_event: self.upper_limit_event,
            lower_limit_event: self.lower_limit_event,
            trigger_sell_depth: self.trigger_sell_depth,
            trigger_buy_depth: self.trigger_buy_depth,
            created_at: self
                .created_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            updated_at: self
                .updated_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            settlement_currency: self.settlement_currency.to_ffi_type(),
            expire_time: self
                .expire_time
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            gtd: self.gtd.to_ffi_type(),
            grid_sub_orders: self.grid_sub_orders.to_ffi_type(),
            num_grid_sub_orders: self.grid_sub_orders.len(),
            sub_has_more: self.sub_has_more,
            grid_order_history: self.grid_order_history.to_ffi_type(),
            num_grid_order_history: self.grid_order_history.len(),
            history_has_more: self.history_has_more,
            support_shortsell: self.support_shortsell,
            rth: self.rth,
            grid_order_type_up: self.grid_order_type_up.to_ffi_type(),
            grid_order_type_down: self.grid_order_type_down.to_ffi_type(),
        }
    }
}

/// A grid trigger-history entry (one triggered order).
#[repr(C)]
pub struct CTriggerOrder {
    /// Triggered order ID
    pub id: *const c_char,
    /// Order status
    pub status: *const c_char,
    /// Stock name
    pub name: *const c_char,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: *const c_char,
    /// Order price (can be null)
    pub price: *const CDecimal,
    /// Order quantity (can be null)
    pub quantity: *const CDecimal,
    /// Executed average price (can be null)
    pub executed_price: *const CDecimal,
    /// Executed total quantity (can be null)
    pub executed_qty: *const CDecimal,
    /// Submitted time (unix timestamp, maybe null)
    pub submitted_at: *const i64,
    /// Buy / sell direction
    pub action: i32,
    /// Order type
    pub order_type: *const c_char,
    /// Trigger price (can be null)
    pub trigger_price: *const CDecimal,
    /// Rejection reason, if any
    pub msg: *const c_char,
    /// Settlement currency
    pub currency: *const c_char,
    /// Latest quote price (can be null)
    pub last_done: *const CDecimal,
    /// Last updated time (unix timestamp, maybe null)
    pub updated_at: *const i64,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Expiry date (`YYYY-MM-DD`, GTD)
    pub gtd: *const c_char,
    /// Trigger time (unix timestamp, maybe null)
    pub trigger_at: *const i64,
    /// Conditional trigger status
    pub trigger_status: i32,
}

#[derive(Debug)]
pub(crate) struct CTriggerOrderOwned {
    id: CString,
    status: CString,
    name: CString,
    symbol: CString,
    price: COption<CDecimal>,
    quantity: COption<CDecimal>,
    executed_price: COption<CDecimal>,
    executed_qty: COption<CDecimal>,
    submitted_at: Option<i64>,
    action: i32,
    order_type: CString,
    trigger_price: COption<CDecimal>,
    msg: CString,
    currency: CString,
    last_done: COption<CDecimal>,
    updated_at: Option<i64>,
    time_in_force: i32,
    gtd: CString,
    trigger_at: Option<i64>,
    trigger_status: i32,
}

impl From<TriggerOrder> for CTriggerOrderOwned {
    fn from(t: TriggerOrder) -> Self {
        CTriggerOrderOwned {
            id: t.id.into(),
            status: t.status.into(),
            name: t.name.into(),
            symbol: t.symbol.into(),
            price: t.price.into(),
            quantity: t.quantity.into(),
            executed_price: t.executed_price.into(),
            executed_qty: t.executed_qty.into(),
            submitted_at: t.submitted_at.map(OffsetDateTime::unix_timestamp),
            action: t.action,
            order_type: t.order_type.into(),
            trigger_price: t.trigger_price.into(),
            msg: t.msg.into(),
            currency: t.currency.into(),
            last_done: t.last_done.into(),
            updated_at: t.updated_at.map(OffsetDateTime::unix_timestamp),
            time_in_force: t.time_in_force.into(),
            gtd: t.gtd.into(),
            trigger_at: t.trigger_at.map(OffsetDateTime::unix_timestamp),
            trigger_status: t.trigger_status,
        }
    }
}

impl ToFFI for CTriggerOrderOwned {
    type FFIType = CTriggerOrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        CTriggerOrder {
            id: self.id.to_ffi_type(),
            status: self.status.to_ffi_type(),
            name: self.name.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            price: self.price.to_ffi_type().to_ffi_type(),
            quantity: self.quantity.to_ffi_type().to_ffi_type(),
            executed_price: self.executed_price.to_ffi_type().to_ffi_type(),
            executed_qty: self.executed_qty.to_ffi_type().to_ffi_type(),
            submitted_at: self
                .submitted_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            action: self.action,
            order_type: self.order_type.to_ffi_type(),
            trigger_price: self.trigger_price.to_ffi_type().to_ffi_type(),
            msg: self.msg.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            last_done: self.last_done.to_ffi_type().to_ffi_type(),
            updated_at: self
                .updated_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            time_in_force: self.time_in_force,
            gtd: self.gtd.to_ffi_type(),
            trigger_at: self
                .trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trigger_status: self.trigger_status,
        }
    }
}

/// A price-step (bid-size) rule entry from the order-info response.
#[repr(C)]
pub struct CGridBidSize {
    /// Range start price (inclusive) (can be null)
    pub str_proceed: *const CDecimal,
    /// Range end price (can be null)
    pub end_proceed: *const CDecimal,
    /// Price step within the range (can be null)
    pub bid_size: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CGridBidSizeOwned {
    str_proceed: COption<CDecimal>,
    end_proceed: COption<CDecimal>,
    bid_size: COption<CDecimal>,
}

impl From<GridBidSize> for CGridBidSizeOwned {
    fn from(b: GridBidSize) -> Self {
        CGridBidSizeOwned {
            str_proceed: b.str_proceed.into(),
            end_proceed: b.end_proceed.into(),
            bid_size: b.bid_size.into(),
        }
    }
}

impl ToFFI for CGridBidSizeOwned {
    type FFIType = CGridBidSize;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridBidSize {
            str_proceed: self.str_proceed.to_ffi_type().to_ffi_type(),
            end_proceed: self.end_proceed.to_ffi_type().to_ffi_type(),
            bid_size: self.bid_size.to_ffi_type().to_ffi_type(),
        }
    }
}

/// Channel / authorization info nested in the order-info response.
#[repr(C)]
pub struct CGridChannelInfo {
    /// Whether the strategy compliance authorization has been granted
    pub strategy_granted: bool,
    /// Whether the RTH toggle is supported
    pub support_rth: bool,
    /// Trading currency
    pub currency: *const c_char,
    /// Supported settlement currencies
    pub settlement_currency: *const *const c_char,
    /// Number of supported settlement currencies
    pub num_settlement_currency: usize,
}

#[derive(Debug)]
pub(crate) struct CGridChannelInfoOwned {
    strategy_granted: bool,
    support_rth: bool,
    currency: CString,
    settlement_currency: CVec<CString>,
}

impl From<GridChannelInfo> for CGridChannelInfoOwned {
    fn from(c: GridChannelInfo) -> Self {
        CGridChannelInfoOwned {
            strategy_granted: c.strategy_granted,
            support_rth: c.support_rth,
            currency: c.currency.into(),
            settlement_currency: c.settlement_currency.into(),
        }
    }
}

impl ToFFI for CGridChannelInfoOwned {
    type FFIType = CGridChannelInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridChannelInfo {
            strategy_granted: self.strategy_granted,
            support_rth: self.support_rth,
            currency: self.currency.to_ffi_type(),
            settlement_currency: self.settlement_currency.to_ffi_type(),
            num_settlement_currency: self.settlement_currency.len(),
        }
    }
}

/// Security (symbol) info used to build a grid order.
#[repr(C)]
pub struct CGridSymbolInfo {
    /// Security name
    pub name: *const c_char,
    /// Latest quote price (can be null)
    pub last_done: *const CDecimal,
    /// Board lot size (can be null)
    pub lot_size: *const CDecimal,
    /// Buy-side board lot size (can be null)
    pub buy_lot_size: *const CDecimal,
    /// Sell-side board lot size (can be null)
    pub sell_lot_size: *const CDecimal,
    /// Price-step (bid-size) rule table
    pub bid_sizes: *const CGridBidSize,
    /// Number of bid-size entries
    pub num_bid_sizes: usize,
    /// Channel / authorization info (strategy grant, RTH, currencies)
    pub channel_info: CGridChannelInfo,
}

#[derive(Debug)]
pub(crate) struct CGridSymbolInfoOwned {
    name: CString,
    last_done: COption<CDecimal>,
    lot_size: COption<CDecimal>,
    buy_lot_size: COption<CDecimal>,
    sell_lot_size: COption<CDecimal>,
    bid_sizes: CVec<CGridBidSizeOwned>,
    channel_info: CGridChannelInfoOwned,
}

impl From<GridSymbolInfo> for CGridSymbolInfoOwned {
    fn from(info: GridSymbolInfo) -> Self {
        CGridSymbolInfoOwned {
            name: info.name.into(),
            last_done: info.last_done.into(),
            lot_size: info.lot_size.into(),
            buy_lot_size: info.buy_lot_size.into(),
            sell_lot_size: info.sell_lot_size.into(),
            bid_sizes: info.bid_sizes.into(),
            channel_info: info.channel_info.into(),
        }
    }
}

impl ToFFI for CGridSymbolInfoOwned {
    type FFIType = CGridSymbolInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridSymbolInfo {
            name: self.name.to_ffi_type(),
            last_done: self.last_done.to_ffi_type().to_ffi_type(),
            lot_size: self.lot_size.to_ffi_type().to_ffi_type(),
            buy_lot_size: self.buy_lot_size.to_ffi_type().to_ffi_type(),
            sell_lot_size: self.sell_lot_size.to_ffi_type().to_ffi_type(),
            bid_sizes: self.bid_sizes.to_ffi_type(),
            num_bid_sizes: self.bid_sizes.len(),
            channel_info: self.channel_info.to_ffi_type(),
        }
    }
}

/// Response for get grid trading orders (list) request
#[repr(C)]
pub struct CGridOrdersResponse {
    /// Grid orders
    pub grid_order: *const CGridOrder,
    /// Number of grid orders
    pub num_grid_order: usize,
    /// Whether there are more pages
    pub has_more: bool,
}

#[derive(Debug)]
pub(crate) struct CGridOrdersResponseOwned {
    grid_order: CVec<CGridOrderOwned>,
    has_more: bool,
}

impl CGridOrdersResponseOwned {
    pub(crate) fn new(grid_order: Vec<GridOrder>, has_more: bool) -> Self {
        CGridOrdersResponseOwned {
            grid_order: grid_order.into(),
            has_more,
        }
    }
}

impl ToFFI for CGridOrdersResponseOwned {
    type FFIType = CGridOrdersResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridOrdersResponse {
            grid_order: self.grid_order.to_ffi_type(),
            num_grid_order: self.grid_order.len(),
            has_more: self.has_more,
        }
    }
}

/// Response for get grid trading trigger history request
#[repr(C)]
pub struct CGridTriggerHistoryResponse {
    /// Trigger history entries
    pub trigger_orders: *const CTriggerOrder,
    /// Number of trigger history entries
    pub num_trigger_orders: usize,
    /// Whether there are more pages
    pub has_more: bool,
}

#[derive(Debug)]
pub(crate) struct CGridTriggerHistoryResponseOwned {
    trigger_orders: CVec<CTriggerOrderOwned>,
    has_more: bool,
}

impl CGridTriggerHistoryResponseOwned {
    pub(crate) fn new(trigger_orders: Vec<TriggerOrder>, has_more: bool) -> Self {
        CGridTriggerHistoryResponseOwned {
            trigger_orders: trigger_orders.into(),
            has_more,
        }
    }
}

impl ToFFI for CGridTriggerHistoryResponseOwned {
    type FFIType = CGridTriggerHistoryResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridTriggerHistoryResponse {
            trigger_orders: self.trigger_orders.to_ffi_type(),
            num_trigger_orders: self.trigger_orders.len(),
            has_more: self.has_more,
        }
    }
}
