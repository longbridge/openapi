use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::pyclass;

use crate::{
    decimal::PyDecimal,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper},
    types::Market,
};

/// Topic type
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::TopicType")]
pub(crate) enum TopicType {
    /// Private notification for trade
    Private,
}

/// Trade
#[pyclass(skip_from_py_object)]
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
    quantity: PyDecimal,
    /// Executed price
    price: PyDecimal,
}

#[pyclass(eq, eq_int, from_py_object)]
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

#[pyclass(eq, eq_int, from_py_object)]
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

#[pyclass(eq, eq_int, from_py_object)]
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
    /// Special Limit Order
    SLO,
}

/// Order tag
#[pyclass(eq, eq_int, skip_from_py_object)]
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
    /// Force Selling
    MarginCall,
    /// OTC
    Offline,
    /// Option Exercise Long
    Creditor,
    /// Option Exercise Short
    Debtor,
    /// Wavier Of Option Exercise
    NonExercise,
    /// Trade Allocation
    AllocatedSub,
}

/// Time in force type
#[pyclass(eq, eq_int, from_py_object)]
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
#[pyclass(eq, eq_int, skip_from_py_object)]
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::OutsideRTH")]
pub(crate) enum OutsideRTH {
    /// Unknown
    Unknown,
    /// Regular trading hour only
    RTHOnly,
    /// Any time
    AnyTime,
    /// Overnight
    Overnight,
}

/// Order
#[pyclass(skip_from_py_object)]
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
    quantity: PyDecimal,
    /// Executed quantity
    executed_quantity: PyDecimal,
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
    /// Limit depth level
    #[py(opt)]
    limit_depth_level: Option<i32>,
    /// Trigger count
    #[py(opt)]
    trigger_count: Option<i32>,
    /// Monitor price
    #[py(opt)]
    monitor_price: Option<PyDecimal>,
    /// Remark
    remark: String,
}

/// Commission-free Status
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::CommissionFreeStatus")]
pub(crate) enum CommissionFreeStatus {
    /// Unknown
    Unknown,
    /// None
    #[py(remote = "None")]
    None_,
    /// Commission-free amount to be calculated
    Calculated,
    /// Pending commission-free
    Pending,
    /// Commission-free applied
    Ready,
}

/// Deduction status
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::DeductionStatus")]
pub(crate) enum DeductionStatus {
    /// Unknown
    Unknown,
    /// Pending Settlement
    #[py(remote = "None")]
    None_,
    /// Settled with no data
    NoData,
    /// Settled and pending distribution
    Pending,
    /// Settled and distributed
    Done,
}

/// Charge category code
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::trade::ChargeCategoryCode")]
pub(crate) enum ChargeCategoryCode {
    /// Unknown
    Unknown,
    /// Broker
    Broker,
    /// Third
    Third,
}

/// Order history detail
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::OrderHistoryDetail")]
pub(crate) struct OrderHistoryDetail {
    /// Executed price for executed orders, submitted price for expired,
    /// canceled, rejected orders, etc.
    price: PyDecimal,
    /// Executed quantity for executed orders, remaining quantity for expired,
    /// canceled, rejected orders, etc.
    quantity: PyDecimal,
    /// Order status
    status: OrderStatus,
    /// Execution or error message
    msg: String,
    /// Occurrence time
    time: PyOffsetDateTimeWrapper,
}

/// Order charge fee
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::OrderChargeFee")]
pub(crate) struct OrderChargeFee {
    /// Charge code
    code: String,
    /// Charge name
    name: String,
    /// Charge amount
    amount: PyDecimal,
    /// Charge currency
    currency: String,
}

/// Order charge item
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::OrderChargeItem")]
pub(crate) struct OrderChargeItem {
    /// Charge category code
    code: ChargeCategoryCode,
    /// Charge category name
    name: String,
    /// Charge details
    #[py(array)]
    fees: Vec<OrderChargeFee>,
}

/// Order charge detail
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::OrderChargeDetail")]
pub(crate) struct OrderChargeDetail {
    /// Total charges amount
    total_amount: PyDecimal,
    /// Settlement currency
    currency: String,
    /// Order charge items
    #[py(array)]
    items: Vec<OrderChargeItem>,
}

/// Order detail
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::OrderDetail")]
pub(crate) struct OrderDetail {
    /// Order ID
    order_id: String,
    /// Order status
    status: OrderStatus,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    quantity: PyDecimal,
    /// Executed quantity
    executed_quantity: PyDecimal,
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
    /// Limit depth level
    #[py(opt)]
    limit_depth_level: Option<i32>,
    /// Trigger count
    #[py(opt)]
    trigger_count: Option<i32>,
    /// Monitor price
    #[py(opt)]
    monitor_price: Option<PyDecimal>,
    /// Remark
    remark: String,
    /// Commission-free Status
    free_status: CommissionFreeStatus,
    /// Commission-free amount
    #[py(opt)]
    free_amount: Option<PyDecimal>,
    /// Commission-free currency
    #[py(opt)]
    free_currency: Option<String>,
    /// Deduction status
    deductions_status: DeductionStatus,
    /// Deduction amount
    #[py(opt)]
    deductions_amount: Option<PyDecimal>,
    /// Deduction currency
    deductions_currency: Option<String>,
    /// Platform fee deduction status
    platform_deducted_status: DeductionStatus,
    /// Platform deduction amount
    #[py(opt)]
    platform_deducted_amount: Option<PyDecimal>,
    /// Platform deduction currency
    #[py(opt)]
    platform_deducted_currency: Option<String>,
    /// Order history details
    #[py(array)]
    history: Vec<OrderHistoryDetail>,
    /// Order charges
    charge_detail: OrderChargeDetail,
}

/// Order changed message
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::PushOrderChanged")]
pub(crate) struct PushOrderChanged {
    /// Order side
    side: OrderSide,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    submitted_quantity: PyDecimal,
    /// Order symbol
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Submitted price
    submitted_price: PyDecimal,
    /// Executed quantity
    executed_quantity: PyDecimal,
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
    /// Last share
    #[py(opt)]
    last_share: Option<PyDecimal>,
    /// Last price
    #[py(opt)]
    last_price: Option<PyDecimal>,
    /// Remark message
    remark: String,
}

/// Response for submit order request
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::SubmitOrderResponse")]
pub(crate) struct SubmitOrderResponse {
    /// Order id
    order_id: String,
}

/// Account balance
#[pyclass(skip_from_py_object)]
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

/// Frozen transaction fee
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::FrozenTransactionFee")]
pub(crate) struct FrozenTransactionFee {
    /// Currency
    pub currency: String,
    /// Frozen transaction fee amount
    pub frozen_transaction_fee: PyDecimal,
}

/// Account balance
#[pyclass(skip_from_py_object)]
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
    /// Buy power
    pub buy_power: PyDecimal,
    /// Frozen transaction fees
    #[py(array)]
    pub frozen_transaction_fees: Vec<FrozenTransactionFee>,
}

#[pyclass(eq, eq_int, from_py_object)]
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

#[pyclass(eq, eq_int, skip_from_py_object)]
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
#[pyclass(skip_from_py_object)]
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::trade::FundPositionsResponse")]
pub(crate) struct FundPositionsResponse {
    #[py(array)]
    channels: Vec<FundPositionChannel>,
}

/// Fund position channel
#[pyclass(skip_from_py_object)]
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
#[pyclass(skip_from_py_object)]
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::StockPositionsResponse")]
pub(crate) struct StockPositionsResponse {
    #[py(array)]
    channels: Vec<StockPositionChannel>,
}

/// Stock position channel
#[pyclass(skip_from_py_object)]
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::StockPosition")]
pub(crate) struct StockPosition {
    /// Stock code
    symbol: String,
    /// Stock name
    symbol_name: String,
    /// The number of holdings
    quantity: PyDecimal,
    /// Available quantity
    available_quantity: PyDecimal,
    /// Currency
    currency: String,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    cost_price: PyDecimal,
    /// Market
    market: Market,
    /// Initial position before market opening
    #[py(opt)]
    pub init_quantity: Option<PyDecimal>,
}

/// Margin ratio
#[pyclass(skip_from_py_object)]
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

/// Response for estimate maximum purchase quantity
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::trade::EstimateMaxPurchaseQuantityResponse")]
pub(crate) struct EstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    pub cash_max_qty: PyDecimal,
    /// Margin available quantity
    pub margin_max_qty: PyDecimal,
}

// ── US-market types ──────────────────────────────────────────────────────────

/// One cash currency entry in USAssetOverview
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USCashEntry {
    pub currency: String,
    pub frozen_buy_cash: String,
    pub outstanding: String,
    pub settled_cash: String,
    pub total_amount: String,
    pub total_cash: String,
}

impl From<longbridge::trade::USCashEntry> for USCashEntry {
    fn from(v: longbridge::trade::USCashEntry) -> Self {
        Self {
            currency: v.currency,
            frozen_buy_cash: v.frozen_buy_cash,
            outstanding: v.outstanding,
            settled_cash: v.settled_cash,
            total_amount: v.total_amount,
            total_cash: v.total_cash,
        }
    }
}

/// One cryptocurrency holding in USAssetOverview
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USCryptoEntry {
    pub asset_type: String,
    pub average_cost: String,
    pub symbol: String,
    pub currency: String,
    pub industry_name: String,
}

impl From<longbridge::trade::USCryptoEntry> for USCryptoEntry {
    fn from(v: longbridge::trade::USCryptoEntry) -> Self {
        Self {
            asset_type: v.asset_type,
            average_cost: v.average_cost,
            symbol: v.symbol,
            currency: v.currency,
            industry_name: v.industry_name,
        }
    }
}

/// US account asset snapshot
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USAssetOverview {
    pub account_type: String,
    pub asset_timestamp: i64,
    pub cash_buy_power: String,
    pub cash_list: Vec<USCashEntry>,
    pub crypto_list: Vec<USCryptoEntry>,
}

impl From<longbridge::trade::USAssetOverview> for USAssetOverview {
    fn from(v: longbridge::trade::USAssetOverview) -> Self {
        Self {
            account_type: v.account_type,
            asset_timestamp: v
                .asset_timestamp
                .map(|t| t.unix_timestamp())
                .unwrap_or_default(),
            cash_buy_power: v.cash_buy_power,
            cash_list: v.cash_list.into_iter().map(Into::into).collect(),
            crypto_list: v.crypto_list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One time-period metric in USRealizedPLEntry
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USRealizedPLMetric {
    pub amount: String,
    pub period: i32,
    pub rate: String,
}

impl From<longbridge::trade::USRealizedPLMetric> for USRealizedPLMetric {
    fn from(v: longbridge::trade::USRealizedPLMetric) -> Self {
        Self {
            amount: v.amount,
            period: v.period,
            rate: v.rate,
        }
    }
}

/// One asset-category entry in USRealizedPL
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USRealizedPLEntry {
    pub category: i32,
    pub currency: String,
    pub metrics: Vec<USRealizedPLMetric>,
}

impl From<longbridge::trade::USRealizedPLEntry> for USRealizedPLEntry {
    fn from(v: longbridge::trade::USRealizedPLEntry) -> Self {
        Self {
            category: v.category,
            currency: v.currency,
            metrics: v.metrics.into_iter().map(Into::into).collect(),
        }
    }
}

/// Realized P&L response for a US account
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USRealizedPL {
    pub realized_pl_list: Vec<USRealizedPLEntry>,
}

impl From<longbridge::trade::USRealizedPL> for USRealizedPL {
    fn from(v: longbridge::trade::USRealizedPL) -> Self {
        Self {
            realized_pl_list: v.realized_pl_list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One order state-transition entry within USOrderDetail.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USOrderHistory {
    pub exec_type: i32,
    pub status: String,
    pub price: String,
    pub qty: String,
    pub time: String,
    pub msg: String,
    pub is_manually: bool,
    pub opp_party_id: String,
    pub trd_match_id: String,
    pub operator: String,
    pub op_entrust_way: String,
    pub cxl_rej_response_to: i32,
    pub withdrawal_reason: String,
    pub opp_name: String,
    pub exec_id: String,
}

impl From<longbridge::trade::USOrderHistory> for USOrderHistory {
    fn from(v: longbridge::trade::USOrderHistory) -> Self {
        Self {
            exec_type: v.exec_type,
            status: v.status,
            price: v.price,
            qty: v.qty,
            time: v.time,
            msg: v.msg,
            is_manually: v.is_manually,
            opp_party_id: v.opp_party_id,
            trd_match_id: v.trd_match_id,
            operator: v.operator,
            op_entrust_way: v.op_entrust_way,
            cxl_rej_response_to: v.cxl_rej_response_to,
            withdrawal_reason: v.withdrawal_reason,
            opp_name: v.opp_name,
            exec_id: v.exec_id,
        }
    }
}

/// Action-button state for an order.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USButtonControl {
    pub withdraw: i32,
    pub replace: i32,
    pub exceptionable: Vec<String>,
}

impl From<longbridge::trade::USButtonControl> for USButtonControl {
    fn from(v: longbridge::trade::USButtonControl) -> Self {
        Self { withdraw: v.withdraw, replace: v.replace, exceptionable: v.exceptionable }
    }
}

/// One fee category within USChargeDetail.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USChargeItem {
    pub code: i32,
    pub name: String,
    pub fees: Vec<String>,
}

impl From<longbridge::trade::USChargeItem> for USChargeItem {
    fn from(v: longbridge::trade::USChargeItem) -> Self {
        Self { code: v.code, name: v.name, fees: v.fees }
    }
}

/// Fee breakdown for an order.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USChargeDetail {
    pub currency: String,
    pub total_amount: String,
    pub items: Vec<USChargeItem>,
}

impl From<longbridge::trade::USChargeDetail> for USChargeDetail {
    fn from(v: longbridge::trade::USChargeDetail) -> Self {
        Self {
            currency: v.currency,
            total_amount: v.total_amount,
            items: v.items.into_iter().map(Into::into).collect(),
        }
    }
}

/// One bracket/conditional sub-order attached to a main order.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USAttachedOrder {
    pub attached_type_display: i32,
    pub executed_qty: String,
    pub quantity: String,
    pub status: String,
    pub trigger_price: String,
    pub order_id: String,
    pub gtd: String,
    pub time_in_force: i32,
    pub tag: i32,
    pub activate_order_type: String,
    pub activate_rth: i32,
    pub submit_price: String,
    pub counter_id: String,
    pub withdrawn: bool,
}

impl From<longbridge::trade::USAttachedOrder> for USAttachedOrder {
    fn from(v: longbridge::trade::USAttachedOrder) -> Self {
        Self {
            attached_type_display: v.attached_type_display,
            executed_qty: v.executed_qty,
            quantity: v.quantity,
            status: v.status,
            trigger_price: v.trigger_price,
            order_id: v.order_id,
            gtd: v.gtd,
            time_in_force: v.time_in_force,
            tag: v.tag,
            activate_order_type: v.activate_order_type,
            activate_rth: v.activate_rth,
            submit_price: v.submit_price,
            counter_id: v.counter_id,
            withdrawn: v.withdrawn,
        }
    }
}

/// Full typed order object within USOrderDetailResponse.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USOrderDetail {
    pub id: String,
    pub aaid: String,
    pub account_channel: String,
    pub action: i32,
    pub counter_id: String,
    pub underlying_counter_id: String,
    pub security_type: String,
    pub name: String,
    pub currency: String,
    pub trade_currency: String,
    pub order_type: String,
    pub status: String,
    pub price: String,
    pub quantity: String,
    pub executed_qty: String,
    pub executed_price: String,
    pub executed_amount: String,
    pub operate_direction: String,
    pub time_in_force: i32,
    pub gtd: String,
    pub tag: i32,
    pub msg: String,
    pub force_only_rth: i32,
    pub submitted_at: String,
    pub done_at: String,
    pub trigger_price: String,
    pub trigger_at: String,
    pub trigger_status: i32,
    pub trigger_exchange: String,
    pub trigger_last_done: String,
    pub trigger_count: i32,
    pub tailing_amount: String,
    pub tailing_percent: String,
    pub limit_offset: String,
    pub limit_depth_level: i32,
    pub market_price: String,
    pub submitted_amount: String,
    pub estimated_fee: String,
    pub free_status: i32,
    pub free_amount: String,
    pub free_currency: String,
    pub deductions_status: i32,
    pub deductions_amount: String,
    pub deductions_currency: String,
    pub platform_deductions_status: i32,
    pub platform_deductions_amount: String,
    pub platform_deductions_currency: String,
    pub display_account: String,
    pub settlement_account: String,
    pub settlement_channel: String,
    pub customer_name: String,
    pub real_name: String,
    pub en_name: String,
    pub joint_real_name: String,
    pub joint_en_name: String,
    pub org_id: String,
    pub bcan: String,
    pub op_entrust_way: i32,
    pub op_entrust_way_name: String,
    pub remark: String,
    pub notice: String,
    pub short_sell_type: i32,
    pub ploy_type: String,
    pub ploy_id: String,
    pub ploy_status: String,
    pub trend: i32,
    pub withdrawal_reason: String,
    pub activate_order_type: String,
    pub activate_rth: i32,
    pub submit_price: String,
    pub contract_direction: String,
    pub strike_price: String,
    pub contract_size: String,
    pub monitor_price: String,
    pub button_control: USButtonControl,
    pub charge_detail: Option<USChargeDetail>,
    pub attached_orders: Vec<USAttachedOrder>,
    pub order_histories: Vec<USOrderHistory>,
}

impl From<longbridge::trade::USOrderDetail> for USOrderDetail {
    fn from(v: longbridge::trade::USOrderDetail) -> Self {
        Self {
            id: v.id, aaid: v.aaid, account_channel: v.account_channel, action: v.action,
            counter_id: v.counter_id, underlying_counter_id: v.underlying_counter_id,
            security_type: v.security_type, name: v.name, currency: v.currency,
            trade_currency: v.trade_currency, order_type: v.order_type, status: v.status,
            price: v.price, quantity: v.quantity, executed_qty: v.executed_qty,
            executed_price: v.executed_price, executed_amount: v.executed_amount,
            operate_direction: v.operate_direction, time_in_force: v.time_in_force,
            gtd: v.gtd, tag: v.tag, msg: v.msg, force_only_rth: v.force_only_rth,
            submitted_at: v.submitted_at, done_at: v.done_at,
            trigger_price: v.trigger_price, trigger_at: v.trigger_at,
            trigger_status: v.trigger_status, trigger_exchange: v.trigger_exchange,
            trigger_last_done: v.trigger_last_done, trigger_count: v.trigger_count,
            tailing_amount: v.tailing_amount, tailing_percent: v.tailing_percent,
            limit_offset: v.limit_offset, limit_depth_level: v.limit_depth_level,
            market_price: v.market_price, submitted_amount: v.submitted_amount,
            estimated_fee: v.estimated_fee, free_status: v.free_status,
            free_amount: v.free_amount, free_currency: v.free_currency,
            deductions_status: v.deductions_status, deductions_amount: v.deductions_amount,
            deductions_currency: v.deductions_currency,
            platform_deductions_status: v.platform_deductions_status,
            platform_deductions_amount: v.platform_deductions_amount,
            platform_deductions_currency: v.platform_deductions_currency,
            display_account: v.display_account, settlement_account: v.settlement_account,
            settlement_channel: v.settlement_channel, customer_name: v.customer_name,
            real_name: v.real_name, en_name: v.en_name, joint_real_name: v.joint_real_name,
            joint_en_name: v.joint_en_name, org_id: v.org_id, bcan: v.bcan,
            op_entrust_way: v.op_entrust_way, op_entrust_way_name: v.op_entrust_way_name,
            remark: v.remark, notice: v.notice, short_sell_type: v.short_sell_type,
            ploy_type: v.ploy_type, ploy_id: v.ploy_id, ploy_status: v.ploy_status,
            trend: v.trend, withdrawal_reason: v.withdrawal_reason,
            activate_order_type: v.activate_order_type, activate_rth: v.activate_rth,
            submit_price: v.submit_price, contract_direction: v.contract_direction,
            strike_price: v.strike_price, contract_size: v.contract_size,
            monitor_price: v.monitor_price,
            button_control: v.button_control.into(),
            charge_detail: v.charge_detail.map(Into::into),
            attached_orders: v.attached_orders.into_iter().map(Into::into).collect(),
            order_histories: v.order_histories.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response for us_order_detail.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct USOrderDetailResponse {
    pub order: Option<USOrderDetail>,
    pub current_attached_order: Option<USOrderDetail>,
    pub current_millisecond: String,
}

impl From<longbridge::trade::USOrderDetailResponse> for USOrderDetailResponse {
    fn from(v: longbridge::trade::USOrderDetailResponse) -> Self {
        Self {
            order: v.order.map(Into::into),
            current_attached_order: v.current_attached_order.map(Into::into),
            current_millisecond: v.current_millisecond,
        }
    }
}
