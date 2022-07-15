use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::pyclass;

use crate::{
    decimal::PyDecimal,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper},
    types::Market,
};

/// Topic type
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::TopicType")]
pub(crate) enum TopicType {
    /// Private notification for trade
    Private,
}

/// Trade
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::Execution")]
pub(crate) struct Execution {
    /// Order ID
    order_id: String,
    /// Execution ID
    trade_id: String,
    /// Security code
    symbol: String,
    /// Trade done time
    trade_done_at: PyOffsetDateTimeWrapper,
    /// Executed quantity
    quantity: i64,
    /// Executed price
    price: PyDecimal,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::OrderStatus")]
pub(crate) enum OrderStatus {
    /// Unknown
    Unknown,
    /// Not reported
    NotReported,
    /// Not reported (Replaced Order)
    ReplacedNotReported,
    /// Not reported (Protected Order)
    ProtectedNotReported,
    /// Not reported (Conditional Order)
    VarietiesNotReported,
    /// Filled
    Filled,
    /// Wait To New
    WaitToNew,
    /// New
    New,
    /// Wait To Replace
    WaitToReplace,
    /// Pending Replace
    PendingReplace,
    /// Replaced
    Replaced,
    /// Partial Filled
    PartialFilled,
    /// Wait To Cancel
    WaitToCancel,
    /// Pending Cancel
    PendingCancel,
    /// Rejected
    Rejected,
    /// Canceled
    Canceled,
    /// Expired
    Expired,
    /// Partial Withdrawal
    PartialWithdrawal,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::OrderSide")]
pub(crate) enum OrderSide {
    /// Unknown
    Unknown,
    /// Buy
    Buy,
    /// Sell
    Sell,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::OrderType")]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum OrderType {
    /// Unknown
    Unknown,
    /// Limit Order
    LO,
    /// Enhanced Limit Order
    ELO,
    /// Market Order
    MO,
    /// At-auction Order
    AO,
    /// At-auction Limit Order
    ALO,
    /// Odd Lots
    ODD,
    /// Limit If Touched
    LIT,
    /// Market If Touched
    MIT,
    /// Trailing Limit If Touched (Trailing Amount)
    TSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    TSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    TSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    TSMPCT,
}

/// Order tag
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::OrderTag")]
pub(crate) enum OrderTag {
    /// Unknown
    Unknown,
    /// Normal Order
    Normal,
    /// Long term Order
    LongTerm,
    /// Grey Order
    Grey,
}

/// Time in force type
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::TimeInForceType")]
pub(crate) enum TimeInForceType {
    /// Unknown
    Unknown,
    /// Day Order
    Day,
    /// Good Til Canceled Order
    GoodTilCanceled,
    /// Good Til Date Order
    GoodTilDate,
}

/// Trigger status
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::TriggerStatus")]
pub(crate) enum TriggerStatus {
    /// Unknown
    Unknown,
    /// Deactive
    Deactive,
    /// Active
    Active,
    /// Released
    Released,
}

/// Enable or disable outside regular trading hours
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::OutsideRTH")]
pub(crate) enum OutsideRTH {
    /// Unknown
    Unknown,
    /// Regular trading hour only
    RTHOnly,
    /// Any time
    AnyTime,
}

/// Order
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::Order")]
pub(crate) struct Order {
    /// Order ID
    order_id: String,
    /// Order status
    status: OrderStatus,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    quantity: i64,
    /// Executed quantity
    executed_quantity: i64,
    /// Submitted price
    #[py(opt)]
    price: Option<PyDecimal>,
    /// Executed price
    #[py(opt)]
    executed_price: Option<PyDecimal>,
    /// Submitted time
    submitted_at: PyOffsetDateTimeWrapper,
    /// Order side
    side: OrderSide,
    /// Security code
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Last done
    #[py(opt)]
    last_done: Option<PyDecimal>,
    /// `LIT` / `MIT` Order Trigger Price
    #[py(opt)]
    trigger_price: Option<PyDecimal>,
    /// Rejected Message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Time in force type
    time_in_force: TimeInForceType,
    /// Long term order expire date
    #[py(opt)]
    expire_date: Option<PyDateWrapper>,
    /// Last updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Conditional order trigger time
    #[py(opt)]
    trigger_at: Option<PyOffsetDateTimeWrapper>,
    /// `TSMAMT` / `TSLPAMT` order trailing amount
    #[py(opt)]
    trailing_amount: Option<PyDecimal>,
    /// `TSMPCT` / `TSLPPCT` order trailing percent
    #[py(opt)]
    trailing_percent: Option<PyDecimal>,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount
    #[py(opt)]
    limit_offset: Option<PyDecimal>,
    /// Conditional order trigger status
    #[py(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Currency
    currency: String,
    /// Enable or disable outside regular trading hours
    #[py(opt)]
    outside_rth: Option<OutsideRTH>,
}

/// Order changed message
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::PushOrderChanged")]
pub(crate) struct PushOrderChanged {
    /// Order side
    side: OrderSide,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    submitted_quantity: i64,
    /// Order symbol
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Submitted price
    submitted_price: PyDecimal,
    /// Executed quantity
    executed_quantity: i64,
    /// Executed price
    #[py(opt)]
    executed_price: Option<PyDecimal>,
    /// Order ID
    order_id: String,
    /// Currency
    currency: String,
    /// Order status
    status: OrderStatus,
    /// Submitted time
    submitted_at: PyOffsetDateTimeWrapper,
    /// Last updated time
    updated_at: PyOffsetDateTimeWrapper,
    /// Order trigger price
    #[py(opt)]
    trigger_price: Option<PyDecimal>,
    /// Rejected message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Conditional order trigger status
    #[py(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Conditional order trigger time
    #[py(opt)]
    trigger_at: Option<PyOffsetDateTimeWrapper>,
    /// Trailing amount
    #[py(opt)]
    trailing_amount: Option<PyDecimal>,
    /// Trailing percent
    #[py(opt)]
    trailing_percent: Option<PyDecimal>,
    /// Limit offset amount
    #[py(opt)]
    limit_offset: Option<PyDecimal>,
    /// Account no
    account_no: String,
}

/// Response for submit order request
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::SubmitOrderResponse")]
pub(crate) struct SubmitOrderResponse {
    /// Order id
    order_id: String,
}

/// Account balance
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::CashInfo")]
pub(crate) struct CashInfo {
    /// Withdraw cash
    withdraw_cash: PyDecimal,
    /// Available cash
    available_cash: PyDecimal,
    /// Frozen cash
    frozen_cash: PyDecimal,
    /// Cash to be settled
    settling_cash: PyDecimal,
    /// Currency
    currency: String,
}

/// Account balance
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::AccountBalance")]
pub(crate) struct AccountBalance {
    /// Total cash
    total_cash: PyDecimal,
    /// Maximum financing amount
    max_finance_amount: PyDecimal,
    /// Remaining financing amount
    remaining_finance_amount: PyDecimal,
    /// Risk control level
    risk_level: i32,
    /// Margin call
    margin_call: PyDecimal,
    /// Currency
    currency: String,
    /// Cash details
    #[py(array)]
    cash_infos: Vec<CashInfo>,
    /// Net assets
    pub net_assets: PyDecimal,
    /// Initial margin
    pub init_margin: PyDecimal,
    /// Maintenance margin
    pub maintenance_margin: PyDecimal,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::BalanceType")]
pub(crate) enum BalanceType {
    /// Unknown
    Unknown,
    /// Cash
    Cash,
    /// Stock
    Stock,
    /// Fund
    Fund,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::CashFlowDirection")]
pub(crate) enum CashFlowDirection {
    /// Unknown
    Unknown,
    /// Out
    Out,
    /// In
    In,
}

/// Account balance
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::CashFlow")]
pub(crate) struct CashFlow {
    /// Cash flow name
    transaction_flow_name: String,
    /// Outflow direction
    direction: CashFlowDirection,
    /// Balance type
    business_type: BalanceType,
    /// Cash amount
    balance: PyDecimal,
    /// Cash currency
    currency: String,
    /// Business time
    business_time: PyOffsetDateTimeWrapper,
    /// Associated Stock code information
    symbol: Option<String>,
    /// Cash flow description
    description: String,
}

/// Fund positions response
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::FundPositionsResponse")]
pub(crate) struct FundPositionsResponse {
    #[py(array)]
    channels: Vec<FundPositionChannel>,
}

/// Fund position channel
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::FundPositionChannel")]
pub(crate) struct FundPositionChannel {
    /// Account type
    account_channel: String,
    /// Fund positions
    #[py(array)]
    positions: Vec<FundPosition>,
}

/// Fund position
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::FundPosition")]
pub(crate) struct FundPosition {
    /// Fund ISIN code
    symbol: String,
    /// Current equity
    current_net_asset_value: PyDecimal,
    /// Current equity time
    net_asset_value_day: PyOffsetDateTimeWrapper,
    /// Fund name
    symbol_name: String,
    /// Currency
    currency: String,
    /// Net cost
    cost_net_asset_value: PyDecimal,
    /// Holding units
    holding_units: PyDecimal,
}

/// Stock positions response
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::StockPositionsResponse")]
pub(crate) struct StockPositionsResponse {
    #[py(array)]
    channels: Vec<StockPositionChannel>,
}

/// Stock position channel
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::StockPositionChannel")]
pub(crate) struct StockPositionChannel {
    /// Account type
    account_channel: String,
    /// Stock positions
    #[py(array)]
    positions: Vec<StockPosition>,
}

/// Stock position
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::StockPosition")]
pub(crate) struct StockPosition {
    /// Stock code
    symbol: String,
    /// Stock name
    symbol_name: String,
    /// The number of holdings
    quantity: i64,
    /// Available quantity
    available_quantity: i64,
    /// Currency
    currency: String,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    cost_price: PyDecimal,
    /// Market
    market: Market,
}

/// Margin ratio
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::MarginRatio")]
pub(crate) struct MarginRatio {
    /// Initial margin ratio
    im_factor: PyDecimal,
    /// Maintain the initial margin ratio
    mm_factor: PyDecimal,
    /// Forced close-out margin ratio
    fm_factor: PyDecimal,
}
