use std::os::raw::c_char;

use longbridge::{
    Market,
    trade::{
        AccountBalance, AllExecutionsResponse, AttachedOrderDetail, AttachedOrderType, BalanceType,
        CashFlow, CashFlowDirection, CashInfo, EstimateMaxPurchaseQuantityResponse, Execution,
        FrozenTransactionFee, FundPosition, FundPositionChannel, FundPositionsResponse,
        GridBidSize, GridChannelInfo, GridOrder, GridOrderDetail, GridOrderHistory, GridOrderInfo,
        GridOrderSubOrder, MarginRatio, Order, OrderChargeDetail, OrderChargeFee, OrderChargeItem,
        OrderDetail, OrderHistoryDetail, OrderSide, OrderStatus, OrderTag, OrderType,
        PushGridOrderChanged, PushOrderChanged, StockPosition, StockPositionChannel,
        StockPositionsResponse, SubmitGridOrderResponse, SubmitOrderResponse, TimeInForceType,
        TriggerOrder,
    },
};
use time::OffsetDateTime;

use crate::{
    trade_context::enum_types::{
        CBalanceType, CCashFlowDirection, CChargeCategoryCode, CCommissionFreeStatus,
        CDeductionStatus, COrderSide, COrderStatus, COrderTag, COrderType, COutsideRTH,
        CTimeInForceType, CTriggerStatus,
    },
    types::{CDate, CDecimal, CMarket, CString, CVec, ToFFI},
};

/// Attached order type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAttachedOrderType {
    /// Unknown
    AttachedOrderTypeUnknown = 0,
    /// Take profit
    AttachedOrderTypeProfitTaker = 1,
    /// Stop loss
    AttachedOrderTypeStopLoss = 2,
    /// Bracket order
    AttachedOrderTypeBracket = 3,
}

impl From<AttachedOrderType> for CAttachedOrderType {
    fn from(value: AttachedOrderType) -> Self {
        match value {
            AttachedOrderType::Unknown => CAttachedOrderType::AttachedOrderTypeUnknown,
            AttachedOrderType::ProfitTaker => CAttachedOrderType::AttachedOrderTypeProfitTaker,
            AttachedOrderType::StopLoss => CAttachedOrderType::AttachedOrderTypeStopLoss,
            AttachedOrderType::Bracket => CAttachedOrderType::AttachedOrderTypeBracket,
        }
    }
}

impl From<CAttachedOrderType> for AttachedOrderType {
    fn from(value: CAttachedOrderType) -> Self {
        match value {
            CAttachedOrderType::AttachedOrderTypeUnknown => AttachedOrderType::Unknown,
            CAttachedOrderType::AttachedOrderTypeProfitTaker => AttachedOrderType::ProfitTaker,
            CAttachedOrderType::AttachedOrderTypeStopLoss => AttachedOrderType::StopLoss,
            CAttachedOrderType::AttachedOrderTypeBracket => AttachedOrderType::Bracket,
        }
    }
}

/// Attached order detail
#[repr(C)]
pub struct CAttachedOrderDetail {
    /// Attached order ID
    pub order_id: *const c_char,
    /// Attached order type
    pub attached_type_display: CAttachedOrderType,
    /// Trigger price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Quantity
    pub quantity: *const CDecimal,
    /// Executed quantity
    pub executed_qty: *const CDecimal,
    /// Order status
    pub status: COrderStatus,
    /// Last updated time (unix timestamp)
    pub updated_at: i64,
    /// Whether withdrawn
    pub withdrawn: bool,
    /// GTD date (maybe null)
    pub gtd: *const CDate,
    /// Time in force type
    pub time_in_force: CTimeInForceType,
    /// Counter order ID
    pub counter_id: *const c_char,
    /// Trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Executed amount
    pub executed_amount: *const CDecimal,
    /// Tag
    pub tag: COrderTag,
    /// Submitted time (unix timestamp)
    pub submitted_at: i64,
    /// Executed price
    pub executed_price: *const CDecimal,
    /// Force RTH only (maybe null)
    pub force_only_rth: *const COutsideRTH,
    /// Whether reviewed
    pub reviewed: bool,
    /// Order type to submit after trigger
    pub activate_order_type: COrderType,
    /// RTH setting for activated order (maybe null)
    pub activate_rth: *const COutsideRTH,
    /// Submit price (maybe null)
    pub submit_price: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CAttachedOrderDetailOwned {
    order_id: CString,
    attached_type_display: AttachedOrderType,
    trigger_price: Option<CDecimal>,
    quantity: CDecimal,
    executed_qty: CDecimal,
    status: OrderStatus,
    updated_at: i64,
    withdrawn: bool,
    gtd: Option<CDate>,
    time_in_force: TimeInForceType,
    counter_id: CString,
    trigger_status: Option<CTriggerStatus>,
    executed_amount: CDecimal,
    tag: OrderTag,
    submitted_at: i64,
    executed_price: Option<CDecimal>,
    force_only_rth: Option<COutsideRTH>,
    reviewed: bool,
    activate_order_type: OrderType,
    activate_rth: Option<COutsideRTH>,
    submit_price: Option<CDecimal>,
}

impl From<AttachedOrderDetail> for CAttachedOrderDetailOwned {
    fn from(detail: AttachedOrderDetail) -> Self {
        let AttachedOrderDetail {
            order_id,
            attached_type_display,
            trigger_price,
            quantity,
            executed_qty,
            status,
            updated_at,
            withdrawn,
            gtd,
            time_in_force,
            counter_id,
            trigger_status,
            executed_amount,
            tag,
            submitted_at,
            executed_price,
            force_only_rth,
            reviewed,
            activate_order_type,
            activate_rth,
            submit_price,
        } = detail;
        Self {
            order_id: order_id.into(),
            attached_type_display,
            trigger_price: trigger_price.map(Into::into),
            quantity: quantity.into(),
            executed_qty: executed_qty.into(),
            status,
            updated_at: updated_at.unix_timestamp(),
            withdrawn,
            gtd: gtd.map(Into::into),
            time_in_force,
            counter_id: counter_id.into(),
            trigger_status: trigger_status.map(Into::into),
            executed_amount: executed_amount.into(),
            tag,
            submitted_at: submitted_at.unix_timestamp(),
            executed_price: executed_price.map(Into::into),
            force_only_rth: force_only_rth.map(Into::into),
            reviewed,
            activate_order_type,
            activate_rth: activate_rth.map(Into::into),
            submit_price: submit_price.map(Into::into),
        }
    }
}

impl ToFFI for CAttachedOrderDetailOwned {
    type FFIType = CAttachedOrderDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAttachedOrderDetailOwned {
            order_id,
            attached_type_display,
            trigger_price,
            quantity,
            executed_qty,
            status,
            updated_at,
            withdrawn,
            gtd,
            time_in_force,
            counter_id,
            trigger_status,
            executed_amount,
            tag,
            submitted_at,
            executed_price,
            force_only_rth,
            reviewed,
            activate_order_type,
            activate_rth,
            submit_price,
        } = self;
        CAttachedOrderDetail {
            order_id: order_id.to_ffi_type(),
            attached_type_display: (*attached_type_display).into(),
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            quantity: quantity.to_ffi_type(),
            executed_qty: executed_qty.to_ffi_type(),
            status: (*status).into(),
            updated_at: *updated_at,
            withdrawn: *withdrawn,
            gtd: gtd
                .as_ref()
                .map(|value| value as *const CDate)
                .unwrap_or(std::ptr::null()),
            time_in_force: (*time_in_force).into(),
            counter_id: counter_id.to_ffi_type(),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            executed_amount: executed_amount.to_ffi_type(),
            tag: (*tag).into(),
            submitted_at: *submitted_at,
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            force_only_rth: force_only_rth
                .as_ref()
                .map(|value| value as *const COutsideRTH)
                .unwrap_or(std::ptr::null()),
            reviewed: *reviewed,
            activate_order_type: (*activate_order_type).into(),
            activate_rth: activate_rth
                .as_ref()
                .map(|value| value as *const COutsideRTH)
                .unwrap_or(std::ptr::null()),
            submit_price: submit_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
        }
    }
}

/// Options for submit attached order params
#[derive(Debug)]
#[repr(C)]
pub struct CSubmitAttachedParams {
    /// Attached order type
    pub attached_order_type: CAttachedOrderType,
    /// Take-profit trigger price (can be null)
    pub profit_taker_price: *const CDecimal,
    /// Stop-loss trigger price (can be null)
    pub stop_loss_price: *const CDecimal,
    /// Time in force type (can be null)
    pub time_in_force: *const CTimeInForceType,
    /// Expiry time unix timestamp (can be null)
    pub expire_time: *const i64,
    /// Order type to submit after trigger (can be null)
    pub activate_order_type: *const COrderType,
    /// Take-profit limit price (can be null)
    pub profit_taker_submit_price: *const CDecimal,
    /// Stop-loss limit price (can be null)
    pub stop_loss_submit_price: *const CDecimal,
    /// RTH setting for activated order (can be null)
    pub activate_rth: *const COutsideRTH,
}

/// Options for replace attached order params
#[derive(Debug)]
#[repr(C)]
pub struct CReplaceAttachedParams {
    /// Attached order type
    pub attached_order_type: CAttachedOrderType,
    /// Take-profit trigger price (can be null)
    pub profit_taker_price: *const CDecimal,
    /// Stop-loss trigger price (can be null)
    pub stop_loss_price: *const CDecimal,
    /// Time in force type (can be null)
    pub time_in_force: *const CTimeInForceType,
    /// Expiry time unix timestamp (can be null)
    pub expire_time: *const i64,
    /// Order type to submit after trigger (can be null)
    pub activate_order_type: *const COrderType,
    /// Take-profit limit price (can be null)
    pub profit_taker_submit_price: *const CDecimal,
    /// Stop-loss limit price (can be null)
    pub stop_loss_submit_price: *const CDecimal,
    /// RTH setting for activated order (can be null)
    pub activate_rth: *const COutsideRTH,
    /// Take-profit order ID (can be null)
    pub profit_taker_id: *const i64,
    /// Stop-loss order ID (can be null)
    pub stop_loss_id: *const i64,
    /// Cancel all attached orders flag (can be null)
    pub cancel_all_attached: *const bool,
    /// Main order ID (can be null)
    pub main_id: *const i64,
    /// Quantity (can be null)
    pub quantity: *const CDecimal,
    /// Market price (can be null)
    pub market_price: *const CDecimal,
}

/// Order changed message
#[repr(C)]
pub struct CPushOrderChanged {
    /// Order side
    pub side: COrderSide,
    /// Stock name
    pub stock_name: *const c_char,
    /// Submitted quantity
    pub submitted_quantity: *const CDecimal,
    /// Order symbol
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Submitted price
    pub submitted_price: *const CDecimal,
    /// Executed quantity
    pub executed_quantity: *const CDecimal,
    /// Executed price (maybe null)
    pub executed_price: *const CDecimal,
    /// Order ID
    pub order_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Order status
    pub status: COrderStatus,
    /// Submitted time
    pub submitted_at: i64,
    /// Last updated time
    pub updated_at: i64,
    /// Order trigger price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Rejected message or remark
    pub msg: *const c_char,
    /// Order tag
    pub tag: COrderTag,
    /// Conditional order trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Conditional order trigger time (maybe null)
    pub trigger_at: *const i64,
    /// Trailing amount (maybe null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (maybe null)
    pub trailing_percent: *const CDecimal,
    /// Limit offset amount (maybe null)
    pub limit_offset: *const CDecimal,
    /// Account no
    pub account_no: *const c_char,
    /// Last share (maybe null)
    pub last_share: *const CDecimal,
    /// Last price (maybe null)
    pub last_price: *const CDecimal,
    /// Remark message
    pub remark: *const c_char,
}

pub struct CPushOrderChangedOwned {
    side: OrderSide,
    stock_name: CString,
    submitted_quantity: CDecimal,
    symbol: CString,
    order_type: OrderType,
    submitted_price: CDecimal,
    executed_quantity: CDecimal,
    executed_price: Option<CDecimal>,
    order_id: CString,
    currency: CString,
    status: OrderStatus,
    submitted_at: i64,
    updated_at: i64,
    trigger_price: Option<CDecimal>,
    msg: CString,
    tag: OrderTag,
    trigger_status: Option<CTriggerStatus>,
    trigger_at: Option<i64>,
    trailing_amount: Option<CDecimal>,
    trailing_percent: Option<CDecimal>,
    limit_offset: Option<CDecimal>,
    account_no: CString,
    last_share: Option<CDecimal>,
    last_price: Option<CDecimal>,
    /// Remark message
    pub remark: CString,
}

impl From<PushOrderChanged> for CPushOrderChangedOwned {
    fn from(order_changed: PushOrderChanged) -> Self {
        let PushOrderChanged {
            side,
            stock_name,
            submitted_quantity,
            symbol,
            order_type,
            submitted_price,
            executed_quantity,
            executed_price,
            order_id,
            currency,
            status,
            submitted_at,
            updated_at,
            trigger_price,
            msg,
            tag,
            trigger_status,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            account_no,
            last_share,
            last_price,
            remark,
        } = order_changed;
        CPushOrderChangedOwned {
            side,
            stock_name: stock_name.into(),
            submitted_quantity: submitted_quantity.into(),
            symbol: symbol.into(),
            order_type,
            submitted_price: submitted_price.into(),
            executed_quantity: executed_quantity.into(),
            executed_price: executed_price.map(Into::into),
            order_id: order_id.into(),
            currency: currency.into(),
            status,
            submitted_at: submitted_at.unix_timestamp(),
            updated_at: updated_at.unix_timestamp(),
            trigger_price: trigger_price.map(Into::into),
            msg: msg.into(),
            tag,
            trigger_status: trigger_status.map(Into::into),
            trigger_at: trigger_at.map(OffsetDateTime::unix_timestamp),
            trailing_amount: trailing_amount.map(Into::into),
            trailing_percent: trailing_percent.map(Into::into),
            limit_offset: limit_offset.map(Into::into),
            account_no: account_no.into(),
            last_share: last_share.map(Into::into),
            last_price: last_price.map(Into::into),
            remark: remark.into(),
        }
    }
}

impl ToFFI for CPushOrderChangedOwned {
    type FFIType = CPushOrderChanged;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushOrderChangedOwned {
            side,
            stock_name,
            submitted_quantity,
            symbol,
            order_type,
            submitted_price,
            executed_quantity,
            executed_price,
            order_id,
            currency,
            status,
            submitted_at,
            updated_at,
            trigger_price,
            msg,
            tag,
            trigger_status,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            account_no,
            last_share,
            last_price,
            remark,
        } = self;
        CPushOrderChanged {
            side: (*side).into(),
            stock_name: stock_name.to_ffi_type(),
            submitted_quantity: submitted_quantity.to_ffi_type(),
            symbol: symbol.to_ffi_type(),
            order_type: (*order_type).into(),
            submitted_price: submitted_price.to_ffi_type(),
            executed_quantity: executed_quantity.to_ffi_type(),
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            order_id: order_id.to_ffi_type(),
            currency: currency.to_ffi_type(),
            status: (*status).into(),
            submitted_at: *submitted_at,
            updated_at: *updated_at,
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            msg: msg.to_ffi_type(),
            tag: (*tag).into(),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            trigger_at: trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trailing_amount: trailing_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trailing_percent: trailing_percent
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            limit_offset: limit_offset
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            account_no: account_no.to_ffi_type(),
            last_share: last_share
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            last_price: last_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            remark: remark.to_ffi_type(),
        }
    }
}

/// Execution
#[repr(C)]
pub struct CExecution {
    /// Order ID
    pub order_id: *const c_char,
    /// Execution ID
    pub trade_id: *const c_char,
    /// Security code
    pub symbol: *const c_char,
    /// Trade done time
    pub trade_done_at: i64,
    /// Executed quantity
    pub quantity: *const CDecimal,
    /// Executed price
    pub price: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CExecutionOwned {
    order_id: CString,
    trade_id: CString,
    symbol: CString,
    trade_done_at: i64,
    quantity: CDecimal,
    price: CDecimal,
}

impl From<Execution> for CExecutionOwned {
    fn from(execution: Execution) -> Self {
        let Execution {
            order_id,
            trade_id,
            symbol,
            trade_done_at,
            quantity,
            price,
        } = execution;
        CExecutionOwned {
            order_id: order_id.into(),
            trade_id: trade_id.into(),
            symbol: symbol.into(),
            trade_done_at: trade_done_at.unix_timestamp(),
            quantity: quantity.into(),
            price: price.into(),
        }
    }
}

impl ToFFI for CExecutionOwned {
    type FFIType = CExecution;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CExecutionOwned {
            order_id,
            trade_id,
            symbol,
            trade_done_at,
            quantity,
            price,
        } = self;
        CExecution {
            order_id: order_id.to_ffi_type(),
            trade_id: trade_id.to_ffi_type(),
            symbol: symbol.to_ffi_type(),
            trade_done_at: *trade_done_at,
            quantity: quantity.to_ffi_type(),
            price: price.to_ffi_type(),
        }
    }
}

/// Options for get history executions request
#[repr(C)]
pub struct CGetHistoryExecutionsOptions {
    /// Start time (can be null)
    pub start_at: *const i64,
    /// End time (can be null)
    pub end_at: *const i64,
    /// Security code (can be null)
    pub symbol: *const c_char,
}

/// Options for get today executions request
#[repr(C)]
pub struct CGetTodayExecutionsOptions {
    /// Security code (can be null)
    pub symbol: *const c_char,
    /// Order id (can be null)
    pub order_id: *const c_char,
}

/// Options for get all executions request
#[repr(C)]
pub struct CGetAllExecutionsOptions {
    /// Security code (can be null)
    pub symbol: *const c_char,
    /// Order id (can be null)
    pub order_id: *const c_char,
    /// Start time (can be null)
    pub start_at: *const i64,
    /// End time (can be null)
    pub end_at: *const i64,
    /// Page number (can be null)
    pub page: *const u64,
}

/// All executions response
#[repr(C)]
pub struct CAllExecutionsResponse {
    /// Has more records
    pub has_more: bool,
    /// Executions
    pub trades: *const CExecution,
    /// Number of executions
    pub num_trades: usize,
}

pub(crate) struct CAllExecutionsResponseOwned {
    pub has_more: bool,
    pub trades: CVec<CExecutionOwned>,
}

impl From<AllExecutionsResponse> for CAllExecutionsResponseOwned {
    fn from(resp: AllExecutionsResponse) -> Self {
        let AllExecutionsResponse { has_more, trades } = resp;
        Self {
            has_more,
            trades: trades.into(),
        }
    }
}

impl ToFFI for CAllExecutionsResponseOwned {
    type FFIType = CAllExecutionsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAllExecutionsResponseOwned { has_more, trades } = self;
        CAllExecutionsResponse {
            has_more: *has_more,
            trades: trades.to_ffi_type(),
            num_trades: trades.len(),
        }
    }
}

/// Order
#[repr(C)]
pub struct COrder {
    /// Order ID
    pub order_id: *const c_char,
    /// Order status
    pub status: COrderStatus,
    /// Stock name
    pub stock_name: *const c_char,
    /// Submitted quantity
    pub quantity: *const CDecimal,
    /// Executed quantity
    pub executed_quantity: *const CDecimal,
    /// Submitted price (maybe null)
    pub price: *const CDecimal,
    /// Executed price (maybe null)
    pub executed_price: *const CDecimal,
    /// Submitted time
    pub submitted_at: i64,
    /// Order side
    pub side: COrderSide,
    /// Security code
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Last done (maybe null)
    pub last_done: *const CDecimal,
    /// `LIT` / `MIT` Order Trigger Price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Rejected Message or remark
    pub msg: *const c_char,
    /// Order tag
    pub tag: COrderTag,
    /// Time in force type
    pub time_in_force: CTimeInForceType,
    /// Long term order expire date (maybe null)
    pub expire_date: *const CDate,
    /// Last updated time (maybe null)
    pub updated_at: *const i64,
    /// Conditional order trigger time (maybe null)
    pub trigger_at: *const i64,
    /// `TSMAMT` / `TSLPAMT` order trailing amount (maybe null)
    pub trailing_amount: *const CDecimal,
    /// `TSMPCT` / `TSLPPCT` order trailing percent (maybe null)
    pub trailing_percent: *const CDecimal,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount (maybe null)
    pub limit_offset: *const CDecimal,
    /// Conditional order trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Currency
    pub currency: *const c_char,
    /// Enable or disable outside regular trading hours (maybe null)
    pub outside_rth: *const COutsideRTH,
    /// Limit depth level (maybe null)
    pub limit_depth_level: *const i32,
    /// Trigger count (maybe null)
    pub trigger_count: *const i32,
    /// Monitor price (maybe null)
    pub monitor_price: *const CDecimal,
    /// Remark
    pub remark: *const c_char,
    /// Attached orders
    pub attached_orders: *const CAttachedOrderDetail,
    /// Number of attached orders
    pub num_attached_orders: usize,
}

#[derive(Debug)]
pub(crate) struct COrderOwned {
    order_id: CString,
    status: OrderStatus,
    stock_name: CString,
    quantity: CDecimal,
    executed_quantity: CDecimal,
    price: Option<CDecimal>,
    executed_price: Option<CDecimal>,
    submitted_at: OffsetDateTime,
    side: OrderSide,
    symbol: CString,
    order_type: OrderType,
    last_done: Option<CDecimal>,
    trigger_price: Option<CDecimal>,
    msg: CString,
    tag: OrderTag,
    time_in_force: TimeInForceType,
    expire_date: Option<CDate>,
    updated_at: Option<i64>,
    trigger_at: Option<i64>,
    trailing_amount: Option<CDecimal>,
    trailing_percent: Option<CDecimal>,
    limit_offset: Option<CDecimal>,
    trigger_status: Option<CTriggerStatus>,
    currency: CString,
    outside_rth: Option<COutsideRTH>,
    limit_depth_level: Option<i32>,
    trigger_count: Option<i32>,
    monitor_price: Option<CDecimal>,
    remark: CString,
    attached_orders: CVec<CAttachedOrderDetailOwned>,
}

impl From<Order> for COrderOwned {
    fn from(order: Order) -> Self {
        let Order {
            order_id,
            status,
            stock_name,
            quantity,
            executed_quantity,
            price,
            executed_price,
            submitted_at,
            side,
            symbol,
            order_type,
            last_done,
            trigger_price,
            msg,
            tag,
            time_in_force,
            expire_date,
            updated_at,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            trigger_status,
            currency,
            outside_rth,
            limit_depth_level,
            trigger_count,
            monitor_price,
            remark,
            attached_orders,
        } = order;
        COrderOwned {
            order_id: order_id.into(),
            status,
            stock_name: stock_name.into(),
            quantity: quantity.into(),
            executed_quantity: executed_quantity.into(),
            price: price.map(Into::into),
            executed_price: executed_price.map(Into::into),
            submitted_at,
            side,
            symbol: symbol.into(),
            order_type,
            last_done: last_done.map(Into::into),
            trigger_price: trigger_price.map(Into::into),
            msg: msg.into(),
            tag,
            time_in_force,
            expire_date: expire_date.map(Into::into),
            updated_at: updated_at.map(OffsetDateTime::unix_timestamp),
            trigger_at: trigger_at.map(OffsetDateTime::unix_timestamp),
            trailing_amount: trailing_amount.map(Into::into),
            trailing_percent: trailing_percent.map(Into::into),
            limit_offset: limit_offset.map(Into::into),
            trigger_status: trigger_status.map(Into::into),
            currency: currency.into(),
            outside_rth: outside_rth.map(Into::into),
            limit_depth_level,
            trigger_count,
            monitor_price: monitor_price.map(Into::into),
            remark: remark.into(),
            attached_orders: attached_orders.into(),
        }
    }
}

impl ToFFI for COrderOwned {
    type FFIType = COrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COrderOwned {
            order_id,
            status,
            stock_name,
            quantity,
            executed_quantity,
            price,
            executed_price,
            submitted_at,
            side,
            symbol,
            order_type,
            last_done,
            trigger_price,
            msg,
            tag,
            time_in_force,
            expire_date,
            updated_at,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            trigger_status,
            currency,
            outside_rth,
            limit_depth_level,
            trigger_count,
            monitor_price,
            remark,
            attached_orders,
        } = self;
        COrder {
            order_id: order_id.to_ffi_type(),
            status: (*status).into(),
            stock_name: stock_name.to_ffi_type(),
            quantity: quantity.to_ffi_type(),
            executed_quantity: executed_quantity.to_ffi_type(),
            price: price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            submitted_at: submitted_at.unix_timestamp(),
            side: (*side).into(),
            symbol: symbol.to_ffi_type(),
            order_type: (*order_type).into(),
            last_done: last_done
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            msg: msg.to_ffi_type(),
            tag: (*tag).into(),
            time_in_force: (*time_in_force).into(),
            expire_date: expire_date
                .as_ref()
                .map(|value| value as *const CDate)
                .unwrap_or(std::ptr::null()),
            updated_at: updated_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trigger_at: trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trailing_amount: trailing_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trailing_percent: trailing_percent
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            limit_offset: limit_offset
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            currency: currency.to_ffi_type(),
            outside_rth: outside_rth
                .as_ref()
                .map(|value| value as *const COutsideRTH)
                .unwrap_or(std::ptr::null()),
            limit_depth_level: limit_depth_level
                .as_ref()
                .map(|value| value as *const i32)
                .unwrap_or(std::ptr::null()),
            trigger_count: trigger_count
                .as_ref()
                .map(|value| value as *const i32)
                .unwrap_or(std::ptr::null()),
            monitor_price: monitor_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            remark: remark.to_ffi_type(),
            attached_orders: attached_orders.to_ffi_type(),
            num_attached_orders: attached_orders.len(),
        }
    }
}

/// Options for get history orders request
#[derive(Debug)]
#[repr(C)]
pub struct CGetHistoryOrdersOptions {
    /// Security symbol (can be null)
    pub symbol: *const c_char,
    /// Order status (can be null)
    pub status: *const COrderStatus,
    /// Number of order status
    pub num_status: usize,
    /// Order side (can be null)
    pub side: *const COrderSide,
    /// Market (can be null)
    pub market: *const CMarket,
    /// Start time (can be null)
    pub start_at: *const i64,
    /// End time (can be null)
    pub end_at: *const i64,
}

/// Options for get today orders request
#[derive(Debug)]
#[repr(C)]
pub struct CGetTodayOrdersOptions {
    /// Security symbol (can be null)
    pub symbol: *const c_char,
    /// Order status (can be null)
    pub status: *const COrderStatus,
    /// Number of order status
    pub num_status: usize,
    /// Order side (can be null)
    pub side: *const COrderSide,
    /// Market (can be null)
    pub market: *const CMarket,
    /// Order id (can be null)
    pub order_id: *const c_char,
    /// Filter by attached order (can be null)
    pub is_attached: *const bool,
}

/// Options for replace order request
#[derive(Debug)]
#[repr(C)]
pub struct CReplaceOrderOptions {
    /// Order ID
    pub order_id: *const c_char,
    /// Quantity
    pub quantity: *const CDecimal,
    /// Price (can be null)
    pub price: *const CDecimal,
    /// Trigger price (can be null)
    pub trigger_price: *const CDecimal,
    /// Limit offset (can be null)
    pub limit_offset: *const CDecimal,
    /// Trailing amount (can be null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (can be null)
    pub trailing_percent: *const CDecimal,
    /// Limit depth level (can be null)
    pub limit_depth_level: *const i32,
    /// Trigger count (can be null)
    pub trigger_count: *const i32,
    /// Monitor price (can be null)
    pub monitor_price: *const CDecimal,
    /// Remark (can be null)
    pub remark: *const c_char,
    /// Attached order parameters (can be null)
    pub attached_params: *const CReplaceAttachedParams,
}

/// Options for submit order request
#[derive(Debug)]
#[repr(C)]
pub struct CSubmitOrderOptions {
    /// Security symbol
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Order side
    pub side: COrderSide,
    /// Submitted price
    pub submitted_quantity: *const CDecimal,
    /// Time in force type
    pub time_in_force: CTimeInForceType,
    /// Submitted price (can be null)
    pub submitted_price: *const CDecimal,
    /// Trigger price (`LIT` / `MIT` Required) (can be null)
    pub trigger_price: *const CDecimal,
    /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required) (can be null)
    pub limit_offset: *const CDecimal,
    /// Trailing amount (`TSLPAMT` / `TSMAMT` Required) (can be null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required) (can be null)
    pub trailing_percent: *const CDecimal,
    /// Long term order expire date (Required when `time_in_force` is
    /// `GoodTilDate`) (can be null)
    pub expire_date: *const CDate,
    /// Enable or disable outside regular trading hours (can be null)
    pub outside_rth: *const COutsideRTH,
    /// Limit depth level (can be null)
    pub limit_depth_level: *const i32,
    /// Trigger count (can be null)
    pub trigger_count: *const i32,
    /// Monitor price (can be null)
    pub monitor_price: *const CDecimal,
    /// Remark (Maximum 64 characters) (can be null)
    pub remark: *const c_char,
    /// Idempotent request ID for preventing duplicate orders (can be null).
    /// If not specified, idempotency control is skipped.
    /// The server caches this ID for 10 minutes.
    pub client_request_id: *const c_char,
    /// Attached order parameters (can be null)
    pub attached_params: *const CSubmitAttachedParams,
}

/// Response for submit order request
#[repr(C)]
pub struct CSubmitOrderResponse {
    /// Order id
    pub order_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CSubmitOrderResponseOwned {
    order_id: CString,
}

impl From<SubmitOrderResponse> for CSubmitOrderResponseOwned {
    fn from(resp: SubmitOrderResponse) -> Self {
        CSubmitOrderResponseOwned {
            order_id: resp.order_id.into(),
        }
    }
}

impl ToFFI for CSubmitOrderResponseOwned {
    type FFIType = CSubmitOrderResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CSubmitOrderResponse {
            order_id: self.order_id.to_ffi_type(),
        }
    }
}

/// Account balance
#[repr(C)]
pub struct CCashInfo {
    /// Withdraw cash
    pub withdraw_cash: *const CDecimal,
    /// Available cash
    pub available_cash: *const CDecimal,
    /// Frozen cash
    pub frozen_cash: *const CDecimal,
    /// Cash to be settled
    pub settling_cash: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCashInfoOwned {
    withdraw_cash: CDecimal,
    available_cash: CDecimal,
    frozen_cash: CDecimal,
    settling_cash: CDecimal,
    currency: CString,
}

impl From<CashInfo> for CCashInfoOwned {
    fn from(info: CashInfo) -> Self {
        let CashInfo {
            withdraw_cash,
            available_cash,
            frozen_cash,
            settling_cash,
            currency,
        } = info;
        Self {
            withdraw_cash: withdraw_cash.into(),
            available_cash: available_cash.into(),
            frozen_cash: frozen_cash.into(),
            settling_cash: settling_cash.into(),
            currency: currency.into(),
        }
    }
}

impl ToFFI for CCashInfoOwned {
    type FFIType = CCashInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCashInfoOwned {
            withdraw_cash,
            available_cash,
            frozen_cash,
            settling_cash,
            currency,
        } = self;
        CCashInfo {
            withdraw_cash: withdraw_cash.to_ffi_type(),
            available_cash: available_cash.to_ffi_type(),
            frozen_cash: frozen_cash.to_ffi_type(),
            settling_cash: settling_cash.to_ffi_type(),
            currency: currency.to_ffi_type(),
        }
    }
}

/// Frozen transaction fee entry for a given currency
#[repr(C)]
pub struct CFrozenTransactionFee {
    /// Currency of the frozen fee
    pub currency: *const c_char,
    /// Amount of transaction fee frozen for pending orders
    pub frozen_transaction_fee: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CFrozenTransactionFeeOwned {
    currency: CString,
    frozen_transaction_fee: CDecimal,
}

impl From<FrozenTransactionFee> for CFrozenTransactionFeeOwned {
    fn from(frozen_fee: FrozenTransactionFee) -> Self {
        let FrozenTransactionFee {
            currency,
            frozen_transaction_fee,
        } = frozen_fee;
        Self {
            currency: currency.into(),
            frozen_transaction_fee: frozen_transaction_fee.into(),
        }
    }
}

impl ToFFI for CFrozenTransactionFeeOwned {
    type FFIType = CFrozenTransactionFee;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFrozenTransactionFeeOwned {
            currency,
            frozen_transaction_fee,
        } = self;
        CFrozenTransactionFee {
            currency: currency.to_ffi_type(),
            frozen_transaction_fee: frozen_transaction_fee.to_ffi_type(),
        }
    }
}

/// Account balance
#[repr(C)]
pub struct CAccountBalance {
    /// Total cash
    pub total_cash: *const CDecimal,
    /// Maximum financing amount
    pub max_finance_amount: *const CDecimal,
    /// Remaining financing amount
    pub remaining_finance_amount: *const CDecimal,
    /// Risk control level
    pub risk_level: i32,
    /// Margin call
    pub margin_call: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Cash details
    pub cash_infos: *const CCashInfo,
    /// Number of cash details
    pub num_cash_infos: usize,
    /// Net assets
    pub net_assets: *const CDecimal,
    /// Initial margin
    pub init_margin: *const CDecimal,
    /// Maintenance margin
    pub maintenance_margin: *const CDecimal,
    /// Buy power
    pub buy_power: *const CDecimal,
    /// Frozen transaction fees
    pub frozen_transaction_fees: *const CFrozenTransactionFee,
    /// Number of frozen transaction fees
    pub num_frozen_transaction_fees: usize,
}

#[derive(Debug)]
pub(crate) struct CAccountBalanceOwned {
    total_cash: CDecimal,
    max_finance_amount: CDecimal,
    remaining_finance_amount: CDecimal,
    risk_level: i32,
    margin_call: CDecimal,
    currency: CString,
    cash_infos: CVec<CCashInfoOwned>,
    net_assets: CDecimal,
    init_margin: CDecimal,
    maintenance_margin: CDecimal,
    buy_power: CDecimal,
    frozen_transaction_fees: CVec<CFrozenTransactionFeeOwned>,
}

impl From<AccountBalance> for CAccountBalanceOwned {
    fn from(info: AccountBalance) -> Self {
        let AccountBalance {
            total_cash,
            max_finance_amount,
            remaining_finance_amount,
            risk_level,
            margin_call,
            currency,
            cash_infos,
            net_assets,
            init_margin,
            maintenance_margin,
            buy_power,
            frozen_transaction_fees,
        } = info;
        Self {
            total_cash: total_cash.into(),
            max_finance_amount: max_finance_amount.into(),
            remaining_finance_amount: remaining_finance_amount.into(),
            risk_level,
            margin_call: margin_call.into(),
            currency: currency.into(),
            cash_infos: cash_infos.into(),
            net_assets: net_assets.into(),
            init_margin: init_margin.into(),
            maintenance_margin: maintenance_margin.into(),
            buy_power: buy_power.into(),
            frozen_transaction_fees: frozen_transaction_fees.into(),
        }
    }
}

impl ToFFI for CAccountBalanceOwned {
    type FFIType = CAccountBalance;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAccountBalanceOwned {
            total_cash,
            max_finance_amount,
            remaining_finance_amount,
            risk_level,
            margin_call,
            currency,
            cash_infos,
            net_assets,
            init_margin,
            maintenance_margin,
            buy_power,
            frozen_transaction_fees,
        } = self;
        CAccountBalance {
            total_cash: total_cash.to_ffi_type(),
            max_finance_amount: max_finance_amount.to_ffi_type(),
            remaining_finance_amount: remaining_finance_amount.to_ffi_type(),
            risk_level: *risk_level,
            margin_call: margin_call.to_ffi_type(),
            currency: currency.to_ffi_type(),
            cash_infos: cash_infos.to_ffi_type(),
            num_cash_infos: cash_infos.len(),
            net_assets: net_assets.to_ffi_type(),
            init_margin: init_margin.to_ffi_type(),
            maintenance_margin: maintenance_margin.to_ffi_type(),
            buy_power: buy_power.to_ffi_type(),
            frozen_transaction_fees: frozen_transaction_fees.to_ffi_type(),
            num_frozen_transaction_fees: frozen_transaction_fees.len(),
        }
    }
}

/// Cash flow
#[repr(C)]
pub struct CCashFlow {
    /// Cash flow name
    pub transaction_flow_name: *const c_char,
    /// Outflow direction
    pub direction: CCashFlowDirection,
    /// Balance type
    pub business_type: CBalanceType,
    /// Cash amount
    pub balance: *const CDecimal,
    /// Cash currency
    pub currency: *const c_char,
    /// Business time
    pub business_time: i64,
    /// Associated Stock code information (maybe null)
    pub symbol: *const c_char,
    /// Cash flow description
    pub description: *const c_char,
}

/// Cash flow
#[repr(C)]
pub(crate) struct CCashFlowOwned {
    transaction_flow_name: CString,
    direction: CashFlowDirection,
    business_type: BalanceType,
    balance: CDecimal,
    currency: CString,
    business_time: i64,
    symbol: Option<CString>,
    description: CString,
}

impl From<CashFlow> for CCashFlowOwned {
    fn from(cash_flow: CashFlow) -> Self {
        let CashFlow {
            transaction_flow_name,
            direction,
            business_type,
            balance,
            currency,
            business_time,
            symbol,
            description,
        } = cash_flow;
        CCashFlowOwned {
            transaction_flow_name: transaction_flow_name.into(),
            direction,
            business_type,
            balance: balance.into(),
            currency: currency.into(),
            business_time: business_time.unix_timestamp(),
            symbol: symbol.map(Into::into),
            description: description.into(),
        }
    }
}

impl ToFFI for CCashFlowOwned {
    type FFIType = CCashFlow;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCashFlowOwned {
            transaction_flow_name,
            direction,
            business_type,
            balance,
            currency,
            business_time,
            symbol,
            description,
        } = self;
        CCashFlow {
            transaction_flow_name: transaction_flow_name.to_ffi_type(),
            direction: (*direction).into(),
            business_type: (*business_type).into(),
            balance: balance.to_ffi_type(),
            currency: currency.to_ffi_type(),
            business_time: *business_time,
            symbol: match symbol {
                Some(symbol) => symbol.to_ffi_type(),
                None => std::ptr::null(),
            },
            description: description.to_ffi_type(),
        }
    }
}

/// Options for get cash flow request
#[repr(C)]
pub struct CGetCashFlowOptions {
    /// Start time
    pub start_at: i64,
    /// End time
    pub end_at: i64,
    /// Business type (can be null)
    pub business_type: *const CBalanceType,
    /// Security symbol (can be null)
    pub symbol: *const c_char,
    /// Page number (can be null)
    pub page: *const usize,
    /// Page size (can be null)
    pub size: *const usize,
}

/// Options for get fund positions request
#[repr(C)]
pub struct CGetFundPositionsOptions {
    /// Fund symbols (can be null)
    pub symbols: *const *const c_char,
    /// Number of fund symbols
    pub num_symbols: usize,
}

/// Fund positions response
#[repr(C)]
pub struct CFundPositionsResponse {
    /// Channels
    pub channels: *const CFundPositionChannel,
    /// Number of channels
    pub num_channels: usize,
}

pub(crate) struct CFundPositionsResponseOwned {
    pub channels: CVec<CFundPositionChannelOwned>,
}

impl From<FundPositionsResponse> for CFundPositionsResponseOwned {
    fn from(resp: FundPositionsResponse) -> Self {
        let FundPositionsResponse { channels } = resp;
        Self {
            channels: channels.into(),
        }
    }
}

impl ToFFI for CFundPositionsResponseOwned {
    type FFIType = CFundPositionsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFundPositionsResponseOwned { channels } = self;
        CFundPositionsResponse {
            channels: channels.to_ffi_type(),
            num_channels: channels.len(),
        }
    }
}

/// Fund position channel
#[repr(C)]
pub struct CFundPositionChannel {
    /// Account type
    pub account_channel: *const c_char,
    /// Fund positions
    pub positions: *const CFundPosition,
    /// Number of fund positions
    pub num_positions: usize,
}

pub(crate) struct CFundPositionChannelOwned {
    account_channel: CString,
    positions: CVec<CFundPositionOwned>,
}

impl From<FundPositionChannel> for CFundPositionChannelOwned {
    fn from(channel: FundPositionChannel) -> Self {
        let FundPositionChannel {
            account_channel,
            positions,
        } = channel;
        CFundPositionChannelOwned {
            account_channel: account_channel.into(),
            positions: positions.into(),
        }
    }
}

impl ToFFI for CFundPositionChannelOwned {
    type FFIType = CFundPositionChannel;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFundPositionChannelOwned {
            account_channel,
            positions,
        } = self;
        CFundPositionChannel {
            account_channel: account_channel.to_ffi_type(),
            positions: positions.to_ffi_type(),
            num_positions: positions.len(),
        }
    }
}

/// Fund position
#[repr(C)]
pub struct CFundPosition {
    /// Fund ISIN code
    pub symbol: *const c_char,
    /// Current equity
    pub current_net_asset_value: *const CDecimal,
    /// Current equity time
    pub net_asset_value_day: i64,
    /// Fund name
    pub symbol_name: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Net cost
    pub cost_net_asset_value: *const CDecimal,
    /// Holding units
    pub holding_units: *const CDecimal,
}

pub(crate) struct CFundPositionOwned {
    symbol: CString,
    current_net_asset_value: CDecimal,
    net_asset_value_day: i64,
    symbol_name: CString,
    currency: CString,
    cost_net_asset_value: CDecimal,
    holding_units: CDecimal,
}

impl From<FundPosition> for CFundPositionOwned {
    fn from(position: FundPosition) -> Self {
        let FundPosition {
            symbol,
            current_net_asset_value,
            net_asset_value_day,
            symbol_name,
            currency,
            cost_net_asset_value,
            holding_units,
        } = position;
        Self {
            symbol: symbol.into(),
            current_net_asset_value: current_net_asset_value.into(),
            net_asset_value_day: net_asset_value_day.unix_timestamp(),
            symbol_name: symbol_name.into(),
            currency: currency.into(),
            cost_net_asset_value: cost_net_asset_value.into(),
            holding_units: holding_units.into(),
        }
    }
}

impl ToFFI for CFundPositionOwned {
    type FFIType = CFundPosition;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFundPositionOwned {
            symbol,
            current_net_asset_value,
            net_asset_value_day,
            symbol_name,
            currency,
            cost_net_asset_value,
            holding_units,
        } = self;
        CFundPosition {
            symbol: symbol.to_ffi_type(),
            current_net_asset_value: current_net_asset_value.to_ffi_type(),
            net_asset_value_day: *net_asset_value_day,
            symbol_name: symbol_name.to_ffi_type(),
            currency: currency.to_ffi_type(),
            cost_net_asset_value: cost_net_asset_value.to_ffi_type(),
            holding_units: holding_units.to_ffi_type(),
        }
    }
}

/// Stock position
#[repr(C)]
pub struct CStockPosition {
    /// Stock code
    pub symbol: *const c_char,
    /// Stock name
    pub symbol_name: *const c_char,
    /// The number of holdings
    pub quantity: *const CDecimal,
    /// Available quantity
    pub available_quantity: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    pub cost_price: *const CDecimal,
    /// Market
    pub market: CMarket,
    /// Initial position before market opening
    init_quantity: *const CDecimal,
}

pub(crate) struct CStockPositionOwned {
    /// Stock code
    symbol: CString,
    /// Stock name
    symbol_name: CString,
    /// The number of holdings
    quantity: CDecimal,
    /// Available quantity
    available_quantity: CDecimal,
    /// Currency
    currency: CString,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    cost_price: CDecimal,
    /// Market
    market: Market,
    /// Initial position before market opening
    init_quantity: Option<CDecimal>,
}

impl From<StockPosition> for CStockPositionOwned {
    fn from(position: StockPosition) -> Self {
        let StockPosition {
            symbol,
            symbol_name,
            quantity,
            available_quantity,
            currency,
            cost_price,
            market,
            init_quantity,
        } = position;
        Self {
            symbol: symbol.into(),
            symbol_name: symbol_name.into(),
            quantity: quantity.into(),
            available_quantity: available_quantity.into(),
            currency: currency.into(),
            cost_price: cost_price.into(),
            market,
            init_quantity: init_quantity.map(Into::into),
        }
    }
}

impl ToFFI for CStockPositionOwned {
    type FFIType = CStockPosition;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStockPositionOwned {
            symbol,
            symbol_name,
            quantity,
            available_quantity,
            currency,
            cost_price,
            market,
            init_quantity,
        } = self;
        CStockPosition {
            symbol: symbol.to_ffi_type(),
            symbol_name: symbol_name.to_ffi_type(),
            quantity: quantity.to_ffi_type(),
            available_quantity: available_quantity.to_ffi_type(),
            currency: currency.to_ffi_type(),
            cost_price: cost_price.to_ffi_type(),
            market: (*market).into(),
            init_quantity: init_quantity
                .as_ref()
                .map(|value| value.to_ffi_type())
                .unwrap_or(std::ptr::null()),
        }
    }
}

/// Stock position channel
#[repr(C)]
pub struct CStockPositionChannel {
    /// Account type
    pub account_channel: *const c_char,
    /// Stock positions
    pub positions: *const CStockPosition,
    /// Number of stock positions
    pub num_positions: usize,
}

pub(crate) struct CStockPositionChannelOwned {
    account_channel: CString,
    positions: CVec<CStockPositionOwned>,
}

impl From<StockPositionChannel> for CStockPositionChannelOwned {
    fn from(channel: StockPositionChannel) -> Self {
        let StockPositionChannel {
            account_channel,
            positions,
        } = channel;
        Self {
            account_channel: account_channel.into(),
            positions: positions.into(),
        }
    }
}

impl ToFFI for CStockPositionChannelOwned {
    type FFIType = CStockPositionChannel;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStockPositionChannelOwned {
            account_channel,
            positions,
        } = self;
        CStockPositionChannel {
            account_channel: account_channel.to_ffi_type(),
            positions: positions.to_ffi_type(),
            num_positions: positions.len(),
        }
    }
}

/// Stock positions response
#[repr(C)]
pub struct CStockPositionsResponse {
    /// Channels
    pub channels: *const CStockPositionChannel,
    /// Number of channels
    pub num_channels: usize,
}

pub(crate) struct CStockPositionsResponseOwned {
    channels: CVec<CStockPositionChannelOwned>,
}

impl From<StockPositionsResponse> for CStockPositionsResponseOwned {
    fn from(resp: StockPositionsResponse) -> Self {
        let StockPositionsResponse { channels } = resp;
        CStockPositionsResponseOwned {
            channels: channels.into(),
        }
    }
}

impl ToFFI for CStockPositionsResponseOwned {
    type FFIType = CStockPositionsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStockPositionsResponseOwned { channels } = self;
        CStockPositionsResponse {
            channels: channels.to_ffi_type(),
            num_channels: channels.len(),
        }
    }
}

/// Options for get stock positions request
#[repr(C)]
pub struct CGetStockPositionsOptions {
    /// Fund symbols (can be null)
    pub symbols: *const *const c_char,
    /// Number of stock symbols
    pub num_symbols: usize,
}

/// Margin ratio
#[repr(C)]
pub struct CMarginRatio {
    /// Initial margin ratio
    pub im_factor: *const CDecimal,
    /// Maintain the initial margin ratio
    pub mm_factor: *const CDecimal,
    /// Forced close-out margin ratio
    pub fm_factor: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CMarginRatioOwned {
    im_factor: CDecimal,
    mm_factor: CDecimal,
    fm_factor: CDecimal,
}

impl From<MarginRatio> for CMarginRatioOwned {
    fn from(resp: MarginRatio) -> Self {
        let MarginRatio {
            im_factor,
            mm_factor,
            fm_factor,
        } = resp;
        CMarginRatioOwned {
            im_factor: im_factor.into(),
            mm_factor: mm_factor.into(),
            fm_factor: fm_factor.into(),
        }
    }
}

impl ToFFI for CMarginRatioOwned {
    type FFIType = CMarginRatio;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CMarginRatioOwned {
            im_factor,
            mm_factor,
            fm_factor,
        } = self;
        CMarginRatio {
            im_factor: im_factor.to_ffi_type(),
            mm_factor: mm_factor.to_ffi_type(),
            fm_factor: fm_factor.to_ffi_type(),
        }
    }
}

/// Historical status record for a single order transition
#[repr(C)]
pub struct COrderHistoryDetail {
    /// Order price at the time of this status transition
    pub price: *const CDecimal,
    /// Order quantity at the time of this status transition
    pub quantity: *const CDecimal,
    /// Order status for this history entry
    pub status: COrderStatus,
    /// Rejection or remark message associated with this transition
    pub msg: *const c_char,
    /// Unix timestamp of this status transition
    pub time: i64,
}

#[derive(Debug)]
pub(crate) struct COrderHistoryDetailOwned {
    price: CDecimal,
    quantity: CDecimal,
    status: COrderStatus,
    msg: CString,
    time: i64,
}

impl From<OrderHistoryDetail> for COrderHistoryDetailOwned {
    fn from(value: OrderHistoryDetail) -> Self {
        COrderHistoryDetailOwned {
            price: value.price.into(),
            quantity: value.quantity.into(),
            status: value.status.into(),
            msg: value.msg.into(),
            time: value.time.unix_timestamp(),
        }
    }
}

impl ToFFI for COrderHistoryDetailOwned {
    type FFIType = COrderHistoryDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COrderHistoryDetailOwned {
            price,
            quantity,
            status,
            msg,
            time,
        } = self;
        COrderHistoryDetail {
            price: price.to_ffi_type(),
            quantity: quantity.to_ffi_type(),
            status: *status,
            msg: msg.to_ffi_type(),
            time: *time,
        }
    }
}

/// Order charge fee
#[repr(C)]
pub struct COrderChargeFee {
    /// Charge code
    pub code: *const c_char,
    /// Charge name
    pub name: *const c_char,
    /// Charge amount
    pub amount: *const CDecimal,
    /// Charge currency
    pub currency: *const c_char,
}

#[derive(Debug)]
pub(crate) struct COrderChargeFeeOwned {
    code: CString,
    name: CString,
    amount: CDecimal,
    currency: CString,
}

impl From<OrderChargeFee> for COrderChargeFeeOwned {
    fn from(value: OrderChargeFee) -> Self {
        COrderChargeFeeOwned {
            code: value.code.into(),
            name: value.name.into(),
            amount: value.amount.into(),
            currency: value.currency.into(),
        }
    }
}

impl ToFFI for COrderChargeFeeOwned {
    type FFIType = COrderChargeFee;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COrderChargeFeeOwned {
            code,
            name,
            amount,
            currency,
        } = self;
        COrderChargeFee {
            code: code.to_ffi_type(),
            name: name.to_ffi_type(),
            amount: amount.to_ffi_type(),
            currency: currency.to_ffi_type(),
        }
    }
}

/// Order charge item
#[repr(C)]
pub struct COrderChargeItem {
    /// Charge category code
    pub code: CChargeCategoryCode,
    /// Charge category name
    pub name: *const c_char,
    /// Charge details
    pub fees: *const COrderChargeFee,
    /// Number of charge details
    pub num_fees: usize,
}

#[derive(Debug)]
pub(crate) struct COrderChargeItemOwned {
    code: CChargeCategoryCode,
    name: CString,
    fees: CVec<COrderChargeFeeOwned>,
}

impl From<OrderChargeItem> for COrderChargeItemOwned {
    fn from(value: OrderChargeItem) -> Self {
        COrderChargeItemOwned {
            code: value.code.into(),
            name: value.name.into(),
            fees: value.fees.into(),
        }
    }
}

impl ToFFI for COrderChargeItemOwned {
    type FFIType = COrderChargeItem;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COrderChargeItemOwned { code, name, fees } = self;
        COrderChargeItem {
            code: *code,
            name: name.to_ffi_type(),
            fees: fees.to_ffi_type(),
            num_fees: fees.len(),
        }
    }
}

/// Order charge detail
#[derive(Clone)]
#[repr(C)]
pub struct COrderChargeDetail {
    /// Total charges amount
    pub total_amount: *const CDecimal,
    /// Settlement currency
    pub currency: *const c_char,
    /// Order charge items
    pub items: *const COrderChargeItem,
    /// Number of items
    pub num_items: usize,
}

impl Default for COrderChargeDetail {
    fn default() -> Self {
        Self {
            total_amount: std::ptr::null(),
            currency: std::ptr::null(),
            items: std::ptr::null(),
            num_items: 0,
        }
    }
}

#[derive(Debug)]
pub(crate) struct COrderChargeDetailOwned {
    /// Total charges amount
    total_amount: CDecimal,
    /// Settlement currency
    currency: CString,
    /// Order charge items
    items: CVec<COrderChargeItemOwned>,
}

impl From<OrderChargeDetail> for COrderChargeDetailOwned {
    fn from(value: OrderChargeDetail) -> Self {
        Self {
            total_amount: value.total_amount.into(),
            currency: value.currency.into(),
            items: value.items.into(),
        }
    }
}

impl ToFFI for COrderChargeDetailOwned {
    type FFIType = COrderChargeDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        COrderChargeDetail {
            total_amount: self.total_amount.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            items: self.items.to_ffi_type(),
            num_items: self.items.len(),
        }
    }
}

/// Order detail
#[repr(C)]
pub struct COrderDetail {
    /// Order ID
    pub order_id: *const c_char,
    /// Order status
    pub status: COrderStatus,
    /// Stock name
    pub stock_name: *const c_char,
    /// Submitted quantity
    pub quantity: *const CDecimal,
    /// Executed quantity
    pub executed_quantity: *const CDecimal,
    /// Submitted price (maybe null)
    pub price: *const CDecimal,
    /// Executed price (maybe null)
    pub executed_price: *const CDecimal,
    /// Submitted time
    pub submitted_at: i64,
    /// Order side
    pub side: COrderSide,
    /// Security code
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Last done (maybe null)
    pub last_done: *const CDecimal,
    /// `LIT` / `MIT` Order Trigger Price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Rejected Message or remark
    pub msg: *const c_char,
    /// Order tag
    pub tag: COrderTag,
    /// Time in force type
    pub time_in_force: CTimeInForceType,
    /// Long term order expire date (maybe null)
    pub expire_date: *const CDate,
    /// Last updated time (maybe null)
    pub updated_at: *const i64,
    /// Conditional order trigger time (maybe null)
    pub trigger_at: *const i64,
    /// `TSMAMT` / `TSLPAMT` order trailing amount (maybe null)
    pub trailing_amount: *const CDecimal,
    /// `TSMPCT` / `TSLPPCT` order trailing percent (maybe null)
    pub trailing_percent: *const CDecimal,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount (maybe null)
    pub limit_offset: *const CDecimal,
    /// Conditional order trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Currency
    pub currency: *const c_char,
    /// Enable or disable outside regular trading hours (maybe null)
    pub outside_rth: *const COutsideRTH,
    /// Limit depth level (maybe null)
    pub limit_depth_level: *const i32,
    /// Trigger count (maybe null)
    pub trigger_count: *const i32,
    /// Monitor price (maybe null)
    pub monitor_price: *const CDecimal,
    /// Remark
    pub remark: *const c_char,
    /// Commission-free Status
    pub free_status: CCommissionFreeStatus,
    /// Commission-free amount
    pub free_amount: *const CDecimal,
    /// Commission-free currency
    pub free_currency: *const c_char,
    /// Deduction status
    pub deductions_status: CDeductionStatus,
    /// Deduction amount
    pub deductions_amount: *const CDecimal,
    /// Deduction currency
    pub deductions_currency: *const c_char,
    /// Platform fee deduction status
    pub platform_deducted_status: CDeductionStatus,
    /// Platform deduction amount
    pub platform_deducted_amount: *const CDecimal,
    /// Platform deduction currency
    pub platform_deducted_currency: *const c_char,
    /// Order history details
    pub history: *const COrderHistoryDetail,
    /// Number of history
    pub num_history: usize,
    /// Whether charge_detail is valid (false when the order has no charge info)
    pub has_charge_detail: bool,
    /// Order charges (only valid when has_charge_detail is true)
    pub charge_detail: COrderChargeDetail,
    /// Attached orders
    pub attached_orders: *const CAttachedOrderDetail,
    /// Number of attached orders
    pub num_attached_orders: usize,
}

#[derive(Debug)]
pub(crate) struct COrderDetailOwned {
    order_id: CString,
    status: OrderStatus,
    stock_name: CString,
    quantity: CDecimal,
    executed_quantity: CDecimal,
    price: Option<CDecimal>,
    executed_price: Option<CDecimal>,
    submitted_at: OffsetDateTime,
    side: OrderSide,
    symbol: CString,
    order_type: OrderType,
    last_done: Option<CDecimal>,
    trigger_price: Option<CDecimal>,
    msg: CString,
    tag: OrderTag,
    time_in_force: TimeInForceType,
    expire_date: Option<CDate>,
    updated_at: Option<i64>,
    trigger_at: Option<i64>,
    trailing_amount: Option<CDecimal>,
    trailing_percent: Option<CDecimal>,
    limit_offset: Option<CDecimal>,
    trigger_status: Option<CTriggerStatus>,
    currency: CString,
    outside_rth: Option<COutsideRTH>,
    limit_depth_level: Option<i32>,
    trigger_count: Option<i32>,
    monitor_price: Option<CDecimal>,
    remark: CString,
    free_status: CCommissionFreeStatus,
    free_amount: Option<CDecimal>,
    free_currency: Option<CString>,
    deductions_status: CDeductionStatus,
    deductions_amount: Option<CDecimal>,
    deductions_currency: Option<CString>,
    platform_deducted_status: CDeductionStatus,
    platform_deducted_amount: Option<CDecimal>,
    platform_deducted_currency: Option<CString>,
    history: CVec<COrderHistoryDetailOwned>,
    charge_detail: Option<COrderChargeDetailOwned>,
    attached_orders: CVec<CAttachedOrderDetailOwned>,
}

impl From<OrderDetail> for COrderDetailOwned {
    fn from(order: OrderDetail) -> Self {
        let OrderDetail {
            order_id,
            status,
            stock_name,
            quantity,
            executed_quantity,
            price,
            executed_price,
            submitted_at,
            side,
            symbol,
            order_type,
            last_done,
            trigger_price,
            msg,
            tag,
            time_in_force,
            expire_date,
            updated_at,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            trigger_status,
            currency,
            outside_rth,
            limit_depth_level,
            trigger_count,
            monitor_price,
            remark,
            free_status,
            free_amount,
            free_currency,
            deductions_status,
            deductions_amount,
            deductions_currency,
            platform_deducted_status,
            platform_deducted_amount,
            platform_deducted_currency,
            history,
            charge_detail,
            attached_orders,
        } = order;
        COrderDetailOwned {
            order_id: order_id.into(),
            status,
            stock_name: stock_name.into(),
            quantity: quantity.into(),
            executed_quantity: executed_quantity.into(),
            price: price.map(Into::into),
            executed_price: executed_price.map(Into::into),
            submitted_at,
            side,
            symbol: symbol.into(),
            order_type,
            last_done: last_done.map(Into::into),
            trigger_price: trigger_price.map(Into::into),
            msg: msg.into(),
            tag,
            time_in_force,
            expire_date: expire_date.map(Into::into),
            updated_at: updated_at.map(OffsetDateTime::unix_timestamp),
            trigger_at: trigger_at.map(OffsetDateTime::unix_timestamp),
            trailing_amount: trailing_amount.map(Into::into),
            trailing_percent: trailing_percent.map(Into::into),
            limit_offset: limit_offset.map(Into::into),
            trigger_status: trigger_status.map(Into::into),
            currency: currency.into(),
            outside_rth: outside_rth.map(Into::into),
            limit_depth_level,
            trigger_count,
            monitor_price: monitor_price.map(Into::into),
            remark: remark.into(),
            free_status: free_status.into(),
            free_amount: free_amount.map(Into::into),
            free_currency: free_currency.map(Into::into),
            deductions_status: deductions_status.into(),
            deductions_amount: deductions_amount.map(Into::into),
            deductions_currency: deductions_currency.map(Into::into),
            platform_deducted_status: platform_deducted_status.into(),
            platform_deducted_amount: platform_deducted_amount.map(Into::into),
            platform_deducted_currency: platform_deducted_currency.map(Into::into),
            history: history.into(),
            charge_detail: charge_detail.map(Into::into),
            attached_orders: attached_orders.into(),
        }
    }
}

impl ToFFI for COrderDetailOwned {
    type FFIType = COrderDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COrderDetailOwned {
            order_id,
            status,
            stock_name,
            quantity,
            executed_quantity,
            price,
            executed_price,
            submitted_at,
            side,
            symbol,
            order_type,
            last_done,
            trigger_price,
            msg,
            tag,
            time_in_force,
            expire_date,
            updated_at,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            trigger_status,
            currency,
            outside_rth,
            limit_depth_level,
            trigger_count,
            monitor_price,
            remark,
            free_status,
            free_amount,
            free_currency,
            deductions_status,
            deductions_amount,
            deductions_currency,
            platform_deducted_status,
            platform_deducted_amount,
            platform_deducted_currency,
            history,
            charge_detail,
            attached_orders,
        } = self;
        COrderDetail {
            order_id: order_id.to_ffi_type(),
            status: (*status).into(),
            stock_name: stock_name.to_ffi_type(),
            quantity: quantity.to_ffi_type(),
            executed_quantity: executed_quantity.to_ffi_type(),
            price: price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            submitted_at: submitted_at.unix_timestamp(),
            side: (*side).into(),
            symbol: symbol.to_ffi_type(),
            order_type: (*order_type).into(),
            last_done: last_done
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            msg: msg.to_ffi_type(),
            tag: (*tag).into(),
            time_in_force: (*time_in_force).into(),
            expire_date: expire_date
                .as_ref()
                .map(|value| value as *const CDate)
                .unwrap_or(std::ptr::null()),
            updated_at: updated_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trigger_at: trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trailing_amount: trailing_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trailing_percent: trailing_percent
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            limit_offset: limit_offset
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            currency: currency.to_ffi_type(),
            outside_rth: outside_rth
                .as_ref()
                .map(|value| value as *const COutsideRTH)
                .unwrap_or(std::ptr::null()),
            limit_depth_level: limit_depth_level
                .as_ref()
                .map(|value| value as *const i32)
                .unwrap_or(std::ptr::null()),
            trigger_count: trigger_count
                .as_ref()
                .map(|value| value as *const i32)
                .unwrap_or(std::ptr::null()),
            monitor_price: monitor_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            remark: remark.to_ffi_type(),
            free_status: *free_status,
            free_amount: free_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            free_currency: free_currency
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            deductions_status: *deductions_status,
            deductions_amount: deductions_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            deductions_currency: deductions_currency
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            platform_deducted_status: *platform_deducted_status,
            platform_deducted_amount: platform_deducted_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            platform_deducted_currency: platform_deducted_currency
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            history: history.to_ffi_type(),
            num_history: history.len(),
            has_charge_detail: charge_detail.is_some(),
            charge_detail: charge_detail
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or_default(),
            attached_orders: attached_orders.to_ffi_type(),
            num_attached_orders: attached_orders.len(),
        }
    }
}

/// Options for estimate maximum purchase quantity
#[derive(Debug)]
#[repr(C)]
pub struct CEstimateMaxPurchaseQuantityOptions {
    /// Security symbol to estimate for
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Order price; may be null for market orders
    pub price: *const CDecimal,
    /// Order side (buy or sell)
    pub side: COrderSide,
    /// Settlement currency to use for the estimate (can be null)
    pub currency: *const c_char,
    /// Existing order ID to exclude from available funds calculation (can be
    /// null)
    pub order_id: *const c_char,
    /// Whether to allow fractional share quantities in the result
    pub fractional_shares: bool,
}

/// Options for estimate maximum purchase quantity
#[repr(C)]
pub struct CEstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    pub cash_max_qty: *const CDecimal,
    /// Margin available quantity
    pub margin_max_qty: *const CDecimal,
}

impl From<EstimateMaxPurchaseQuantityResponse> for CEstimateMaxPurchaseQuantityResponseOwned {
    fn from(value: EstimateMaxPurchaseQuantityResponse) -> Self {
        CEstimateMaxPurchaseQuantityResponseOwned {
            cash_max_qty: value.cash_max_qty.into(),
            margin_max_qty: value.margin_max_qty.into(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CEstimateMaxPurchaseQuantityResponseOwned {
    /// Cash available quantity
    pub cash_max_qty: CDecimal,
    /// Margin available quantity
    pub margin_max_qty: CDecimal,
}

impl ToFFI for CEstimateMaxPurchaseQuantityResponseOwned {
    type FFIType = CEstimateMaxPurchaseQuantityResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CEstimateMaxPurchaseQuantityResponse {
            cash_max_qty: self.cash_max_qty.to_ffi_type(),
            margin_max_qty: self.margin_max_qty.to_ffi_type(),
        }
    }
}

// ── Grid trading types
// ───────────────────────────────────────────────────────────

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
    /// Submitted base price
    pub submitted_base_price: *const c_char,
    /// Current base price
    pub current_base_price: *const c_char,
    /// Base price before the last trigger
    pub pre_trigger_base_price: *const c_char,
    /// Base price after the last trigger
    pub post_trigger_base_price: *const c_char,
    /// Upper price bound
    pub upper_limit_price: *const c_char,
    /// Lower price bound
    pub lower_limit_price: *const c_char,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: i32,
    /// Upward trigger spread
    pub trigger_spread_up: *const c_char,
    /// Downward trigger spread
    pub trigger_spread_down: *const c_char,
    /// Upward trigger percent
    pub trigger_percent_up: *const c_char,
    /// Downward trigger percent
    pub trigger_percent_down: *const c_char,
    /// Pullback percent
    pub pullback_percent: *const c_char,
    /// Pullback spread
    pub pullback_spread: *const c_char,
    /// Rebound percent
    pub rebound_percent: *const c_char,
    /// Rebound spread
    pub rebound_spread: *const c_char,
    /// Sell-side execution order type (e.g. `MO`)
    pub trigger_sell_order_type: *const c_char,
    /// Buy-side execution order type (e.g. `MO`)
    pub trigger_buy_order_type: *const c_char,
    /// Sell-side order-book depth
    pub trigger_sell_depth: i32,
    /// Buy-side order-book depth
    pub trigger_buy_depth: i32,
    /// Quantity per trigger
    pub trigger_quantity: *const c_char,
    /// Quantity per sell trigger
    pub trigger_sell_quantity: *const c_char,
    /// Quantity per buy trigger
    pub trigger_buy_quantity: *const c_char,
    /// Quantity handled at the upper bound
    pub upper_limit_quantity: *const c_char,
    /// Quantity handled at the lower bound
    pub lower_limit_quantity: *const c_char,
    /// Action at the upper bound
    pub upper_limit_event: i32,
    /// Action at the lower bound
    pub lower_limit_event: i32,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Number of times the grid has triggered
    pub trigger_times: i32,
    /// Accumulated bought quantity
    pub total_buy_quantity: *const c_char,
    /// Accumulated sold quantity
    pub total_sell_quantity: *const c_char,
    /// Accumulated profit balance
    pub total_profit_balance: *const c_char,
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
    submitted_base_price: CString,
    current_base_price: CString,
    pre_trigger_base_price: CString,
    post_trigger_base_price: CString,
    upper_limit_price: CString,
    lower_limit_price: CString,
    trigger_price_type: i32,
    trigger_spread_up: CString,
    trigger_spread_down: CString,
    trigger_percent_up: CString,
    trigger_percent_down: CString,
    pullback_percent: CString,
    pullback_spread: CString,
    rebound_percent: CString,
    rebound_spread: CString,
    trigger_sell_order_type: CString,
    trigger_buy_order_type: CString,
    trigger_sell_depth: i32,
    trigger_buy_depth: i32,
    trigger_quantity: CString,
    trigger_sell_quantity: CString,
    trigger_buy_quantity: CString,
    upper_limit_quantity: CString,
    lower_limit_quantity: CString,
    upper_limit_event: i32,
    lower_limit_event: i32,
    multiple_trigger: bool,
    trigger_times: i32,
    total_buy_quantity: CString,
    total_sell_quantity: CString,
    total_profit_balance: CString,
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
            trigger_price_type: order.trigger_price_type,
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
            upper_limit_event: order.upper_limit_event,
            lower_limit_event: order.lower_limit_event,
            multiple_trigger: order.multiple_trigger,
            trigger_times: order.trigger_times,
            total_buy_quantity: order.total_buy_quantity.into(),
            total_sell_quantity: order.total_sell_quantity.into(),
            total_profit_balance: order.total_profit_balance.into(),
            settlement_currency: order.settlement_currency.into(),
            time_in_force: order.time_in_force,
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
            submitted_base_price: self.submitted_base_price.to_ffi_type(),
            current_base_price: self.current_base_price.to_ffi_type(),
            pre_trigger_base_price: self.pre_trigger_base_price.to_ffi_type(),
            post_trigger_base_price: self.post_trigger_base_price.to_ffi_type(),
            upper_limit_price: self.upper_limit_price.to_ffi_type(),
            lower_limit_price: self.lower_limit_price.to_ffi_type(),
            trigger_price_type: self.trigger_price_type,
            trigger_spread_up: self.trigger_spread_up.to_ffi_type(),
            trigger_spread_down: self.trigger_spread_down.to_ffi_type(),
            trigger_percent_up: self.trigger_percent_up.to_ffi_type(),
            trigger_percent_down: self.trigger_percent_down.to_ffi_type(),
            pullback_percent: self.pullback_percent.to_ffi_type(),
            pullback_spread: self.pullback_spread.to_ffi_type(),
            rebound_percent: self.rebound_percent.to_ffi_type(),
            rebound_spread: self.rebound_spread.to_ffi_type(),
            trigger_sell_order_type: self.trigger_sell_order_type.to_ffi_type(),
            trigger_buy_order_type: self.trigger_buy_order_type.to_ffi_type(),
            trigger_sell_depth: self.trigger_sell_depth,
            trigger_buy_depth: self.trigger_buy_depth,
            trigger_quantity: self.trigger_quantity.to_ffi_type(),
            trigger_sell_quantity: self.trigger_sell_quantity.to_ffi_type(),
            trigger_buy_quantity: self.trigger_buy_quantity.to_ffi_type(),
            upper_limit_quantity: self.upper_limit_quantity.to_ffi_type(),
            lower_limit_quantity: self.lower_limit_quantity.to_ffi_type(),
            upper_limit_event: self.upper_limit_event,
            lower_limit_event: self.lower_limit_event,
            multiple_trigger: self.multiple_trigger,
            trigger_times: self.trigger_times,
            total_buy_quantity: self.total_buy_quantity.to_ffi_type(),
            total_sell_quantity: self.total_sell_quantity.to_ffi_type(),
            total_profit_balance: self.total_profit_balance.to_ffi_type(),
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
    /// Order price
    pub price: *const c_char,
    /// Order type
    pub order_type: *const c_char,
    /// Order quantity
    pub quantity: *const c_char,
    /// Executed quantity
    pub executed_qty: *const c_char,
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
    price: CString,
    order_type: CString,
    quantity: CString,
    executed_qty: CString,
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
            price: self.price.to_ffi_type(),
            order_type: self.order_type.to_ffi_type(),
            quantity: self.quantity.to_ffi_type(),
            executed_qty: self.executed_qty.to_ffi_type(),
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
    /// Submitted base price
    pub submitted_base_price: *const c_char,
    /// Current base price
    pub current_base_price: *const c_char,
    /// Upper price bound
    pub upper_limit_price: *const c_char,
    /// Lower price bound
    pub lower_limit_price: *const c_char,
    /// Trigger price type (`1` = spread, `2` = percent)
    pub trigger_price_type: i32,
    /// Upward trigger spread
    pub trigger_spread_up: *const c_char,
    /// Downward trigger spread
    pub trigger_spread_down: *const c_char,
    /// Upward trigger percent
    pub trigger_percent_up: *const c_char,
    /// Downward trigger percent
    pub trigger_percent_down: *const c_char,
    /// Pullback percent
    pub pullback_percent: *const c_char,
    /// Pullback spread
    pub pullback_spread: *const c_char,
    /// Rebound percent
    pub rebound_percent: *const c_char,
    /// Rebound spread
    pub rebound_spread: *const c_char,
    /// Whether a single grid level may trigger multiple times
    pub multiple_trigger: bool,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Quantity per trigger
    pub trigger_quantity: *const c_char,
    /// Quantity per sell trigger
    pub trigger_sell_quantity: *const c_char,
    /// Quantity per buy trigger
    pub trigger_buy_quantity: *const c_char,
    /// Quantity handled at the upper bound
    pub upper_limit_quantity: *const c_char,
    /// Quantity handled at the lower bound
    pub lower_limit_quantity: *const c_char,
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
    submitted_base_price: CString,
    current_base_price: CString,
    upper_limit_price: CString,
    lower_limit_price: CString,
    trigger_price_type: i32,
    trigger_spread_up: CString,
    trigger_spread_down: CString,
    trigger_percent_up: CString,
    trigger_percent_down: CString,
    pullback_percent: CString,
    pullback_spread: CString,
    rebound_percent: CString,
    rebound_spread: CString,
    multiple_trigger: bool,
    time_in_force: i32,
    trigger_quantity: CString,
    trigger_sell_quantity: CString,
    trigger_buy_quantity: CString,
    upper_limit_quantity: CString,
    lower_limit_quantity: CString,
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
            trigger_price_type: d.trigger_price_type,
            trigger_spread_up: d.trigger_spread_up.into(),
            trigger_spread_down: d.trigger_spread_down.into(),
            trigger_percent_up: d.trigger_percent_up.into(),
            trigger_percent_down: d.trigger_percent_down.into(),
            pullback_percent: d.pullback_percent.into(),
            pullback_spread: d.pullback_spread.into(),
            rebound_percent: d.rebound_percent.into(),
            rebound_spread: d.rebound_spread.into(),
            multiple_trigger: d.multiple_trigger,
            time_in_force: d.time_in_force,
            trigger_quantity: d.trigger_quantity.into(),
            trigger_sell_quantity: d.trigger_sell_quantity.into(),
            trigger_buy_quantity: d.trigger_buy_quantity.into(),
            upper_limit_quantity: d.upper_limit_quantity.into(),
            lower_limit_quantity: d.lower_limit_quantity.into(),
            upper_limit_event: d.upper_limit_event,
            lower_limit_event: d.lower_limit_event,
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
            submitted_base_price: self.submitted_base_price.to_ffi_type(),
            current_base_price: self.current_base_price.to_ffi_type(),
            upper_limit_price: self.upper_limit_price.to_ffi_type(),
            lower_limit_price: self.lower_limit_price.to_ffi_type(),
            trigger_price_type: self.trigger_price_type,
            trigger_spread_up: self.trigger_spread_up.to_ffi_type(),
            trigger_spread_down: self.trigger_spread_down.to_ffi_type(),
            trigger_percent_up: self.trigger_percent_up.to_ffi_type(),
            trigger_percent_down: self.trigger_percent_down.to_ffi_type(),
            pullback_percent: self.pullback_percent.to_ffi_type(),
            pullback_spread: self.pullback_spread.to_ffi_type(),
            rebound_percent: self.rebound_percent.to_ffi_type(),
            rebound_spread: self.rebound_spread.to_ffi_type(),
            multiple_trigger: self.multiple_trigger,
            time_in_force: self.time_in_force,
            trigger_quantity: self.trigger_quantity.to_ffi_type(),
            trigger_sell_quantity: self.trigger_sell_quantity.to_ffi_type(),
            trigger_buy_quantity: self.trigger_buy_quantity.to_ffi_type(),
            upper_limit_quantity: self.upper_limit_quantity.to_ffi_type(),
            lower_limit_quantity: self.lower_limit_quantity.to_ffi_type(),
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
    /// Order price
    pub price: *const c_char,
    /// Order quantity
    pub quantity: *const c_char,
    /// Executed average price
    pub executed_price: *const c_char,
    /// Executed total quantity
    pub executed_qty: *const c_char,
    /// Submitted time (unix timestamp, maybe null)
    pub submitted_at: *const i64,
    /// Buy / sell direction
    pub action: i32,
    /// Order type
    pub order_type: *const c_char,
    /// Trigger price
    pub trigger_price: *const c_char,
    /// Rejection reason, if any
    pub msg: *const c_char,
    /// Settlement currency
    pub currency: *const c_char,
    /// Latest quote price
    pub last_done: *const c_char,
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
    price: CString,
    quantity: CString,
    executed_price: CString,
    executed_qty: CString,
    submitted_at: Option<i64>,
    action: i32,
    order_type: CString,
    trigger_price: CString,
    msg: CString,
    currency: CString,
    last_done: CString,
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
            time_in_force: t.time_in_force,
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
            price: self.price.to_ffi_type(),
            quantity: self.quantity.to_ffi_type(),
            executed_price: self.executed_price.to_ffi_type(),
            executed_qty: self.executed_qty.to_ffi_type(),
            submitted_at: self
                .submitted_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            action: self.action,
            order_type: self.order_type.to_ffi_type(),
            trigger_price: self.trigger_price.to_ffi_type(),
            msg: self.msg.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            last_done: self.last_done.to_ffi_type(),
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
    /// Range start price (inclusive)
    pub str_proceed: *const c_char,
    /// Range end price
    pub end_proceed: *const c_char,
    /// Price step within the range
    pub bid_size: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CGridBidSizeOwned {
    str_proceed: CString,
    end_proceed: CString,
    bid_size: CString,
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
            str_proceed: self.str_proceed.to_ffi_type(),
            end_proceed: self.end_proceed.to_ffi_type(),
            bid_size: self.bid_size.to_ffi_type(),
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

/// Order info fields used by the grid order window.
#[repr(C)]
pub struct CGridOrderInfo {
    /// Security name
    pub name: *const c_char,
    /// Latest quote price
    pub last_done: *const c_char,
    /// Board lot size
    pub lot_size: *const c_char,
    /// Buy-side board lot size
    pub buy_lot_size: *const c_char,
    /// Sell-side board lot size
    pub sell_lot_size: *const c_char,
    /// Price-step (bid-size) rule table
    pub bid_sizes: *const CGridBidSize,
    /// Number of bid-size entries
    pub num_bid_sizes: usize,
    /// Channel / authorization info (strategy grant, RTH, currencies)
    pub channel_infos: CGridChannelInfo,
}

#[derive(Debug)]
pub(crate) struct CGridOrderInfoOwned {
    name: CString,
    last_done: CString,
    lot_size: CString,
    buy_lot_size: CString,
    sell_lot_size: CString,
    bid_sizes: CVec<CGridBidSizeOwned>,
    channel_infos: CGridChannelInfoOwned,
}

impl From<GridOrderInfo> for CGridOrderInfoOwned {
    fn from(info: GridOrderInfo) -> Self {
        CGridOrderInfoOwned {
            name: info.name.into(),
            last_done: info.last_done.into(),
            lot_size: info.lot_size.into(),
            buy_lot_size: info.buy_lot_size.into(),
            sell_lot_size: info.sell_lot_size.into(),
            bid_sizes: info.bid_sizes.into(),
            channel_infos: info.channel_infos.into(),
        }
    }
}

impl ToFFI for CGridOrderInfoOwned {
    type FFIType = CGridOrderInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        CGridOrderInfo {
            name: self.name.to_ffi_type(),
            last_done: self.last_done.to_ffi_type(),
            lot_size: self.lot_size.to_ffi_type(),
            buy_lot_size: self.buy_lot_size.to_ffi_type(),
            sell_lot_size: self.sell_lot_size.to_ffi_type(),
            bid_sizes: self.bid_sizes.to_ffi_type(),
            num_bid_sizes: self.bid_sizes.len(),
            channel_infos: self.channel_infos.to_ffi_type(),
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

/// Grid trading master-order changed message.
#[repr(C)]
pub struct CPushGridOrderChanged {
    /// Grid master order ID
    pub order_id: *const c_char,
    /// Order status
    pub status: *const c_char,
    /// Security symbol (e.g. `700.HK`)
    pub symbol: *const c_char,
    /// Suspend reason, if any
    pub suspend_reason: *const c_char,
    /// Submitted base price
    pub submitted_base_price: *const c_char,
    /// Current base price
    pub current_base_price: *const c_char,
    /// Upper price bound
    pub upper_limit_price: *const c_char,
    /// Lower price bound
    pub lower_limit_price: *const c_char,
    /// Trigger price type
    pub trigger_price_type: i32,
    /// Quantity per trigger
    pub trigger_quantity: *const c_char,
    /// Settlement currency
    pub settlement_currency: *const c_char,
    /// Time in force (`0` = Day, `1` = GTC, `6` = GTD)
    pub time_in_force: i32,
    /// Regular trading hours flag
    pub rth: i32,
    /// Sell-side order type when depth is 0
    pub grid_order_type_up: *const c_char,
    /// Buy-side order type when depth is 0
    pub grid_order_type_down: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CPushGridOrderChangedOwned {
    order_id: CString,
    status: CString,
    symbol: CString,
    suspend_reason: CString,
    submitted_base_price: CString,
    current_base_price: CString,
    upper_limit_price: CString,
    lower_limit_price: CString,
    trigger_price_type: i32,
    trigger_quantity: CString,
    settlement_currency: CString,
    time_in_force: i32,
    rth: i32,
    grid_order_type_up: CString,
    grid_order_type_down: CString,
}

impl From<PushGridOrderChanged> for CPushGridOrderChangedOwned {
    fn from(p: PushGridOrderChanged) -> Self {
        CPushGridOrderChangedOwned {
            order_id: p.order_id.into(),
            status: p.status.into(),
            symbol: p.symbol.into(),
            suspend_reason: p.suspend_reason.into(),
            submitted_base_price: p.submitted_base_price.into(),
            current_base_price: p.current_base_price.into(),
            upper_limit_price: p.upper_limit_price.into(),
            lower_limit_price: p.lower_limit_price.into(),
            trigger_price_type: p.trigger_price_type,
            trigger_quantity: p.trigger_quantity.into(),
            settlement_currency: p.settlement_currency.into(),
            time_in_force: p.time_in_force,
            rth: p.rth,
            grid_order_type_up: p.grid_order_type_up.into(),
            grid_order_type_down: p.grid_order_type_down.into(),
        }
    }
}

impl ToFFI for CPushGridOrderChangedOwned {
    type FFIType = CPushGridOrderChanged;

    fn to_ffi_type(&self) -> Self::FFIType {
        CPushGridOrderChanged {
            order_id: self.order_id.to_ffi_type(),
            status: self.status.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            suspend_reason: self.suspend_reason.to_ffi_type(),
            submitted_base_price: self.submitted_base_price.to_ffi_type(),
            current_base_price: self.current_base_price.to_ffi_type(),
            upper_limit_price: self.upper_limit_price.to_ffi_type(),
            lower_limit_price: self.lower_limit_price.to_ffi_type(),
            trigger_price_type: self.trigger_price_type,
            trigger_quantity: self.trigger_quantity.to_ffi_type(),
            settlement_currency: self.settlement_currency.to_ffi_type(),
            time_in_force: self.time_in_force,
            rth: self.rth,
            grid_order_type_up: self.grid_order_type_up.to_ffi_type(),
            grid_order_type_down: self.grid_order_type_down.to_ffi_type(),
        }
    }
}
