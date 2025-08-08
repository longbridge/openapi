use chrono::{DateTime, Utc};
use longport_nodejs_macros::{JsEnum, JsObject};

use crate::{decimal::Decimal, time::NaiveDate, types::Market};

/// Topic type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::TopicType")]
pub enum TopicType {
    /// Private notification for trade
    Private,
}

/// Trade
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::Execution")]
pub struct Execution {
    /// Order ID
    order_id: String,
    /// Execution ID
    trade_id: String,
    /// Security code
    symbol: String,
    /// Trade done time
    #[js(datetime)]
    trade_done_at: DateTime<Utc>,
    /// Executed quantity
    quantity: Decimal,
    /// Executed price
    price: Decimal,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::OrderStatus")]
pub enum OrderStatus {
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

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::OrderSide")]
pub enum OrderSide {
    /// Unknown
    Unknown,
    /// Buy
    Buy,
    /// Sell
    Sell,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::OrderType")]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderType {
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
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::OrderTag")]
pub enum OrderTag {
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
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::TimeInForceType")]
pub enum TimeInForceType {
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
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::TriggerStatus")]
pub enum TriggerStatus {
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
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::OutsideRTH")]
pub enum OutsideRTH {
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
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::Order")]
pub struct Order {
    /// Order ID
    order_id: String,
    /// Order status
    status: OrderStatus,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    quantity: Decimal,
    /// Executed quantity
    executed_quantity: Decimal,
    /// Submitted price
    #[js(opt)]
    price: Option<Decimal>,
    /// Executed price
    #[js(opt)]
    executed_price: Option<Decimal>,
    /// Submitted time
    #[js(datetime)]
    submitted_at: DateTime<Utc>,
    /// Order side
    side: OrderSide,
    /// Security code
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Last done
    #[js(opt)]
    last_done: Option<Decimal>,
    /// `LIT` / `MIT` Order Trigger Price
    #[js(opt)]
    trigger_price: Option<Decimal>,
    /// Rejected Message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Time in force type
    time_in_force: TimeInForceType,
    /// Long term order expire date
    #[js(opt)]
    expire_date: Option<NaiveDate>,
    /// Last updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Conditional order trigger time
    #[js(opt, datetime)]
    trigger_at: Option<DateTime<Utc>>,
    /// `TSMAMT` / `TSLPAMT` order trailing amount
    #[js(opt)]
    trailing_amount: Option<Decimal>,
    /// `TSMPCT` / `TSLPPCT` order trailing percent
    #[js(opt)]
    trailing_percent: Option<Decimal>,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount
    #[js(opt)]
    limit_offset: Option<Decimal>,
    /// Conditional order trigger status
    #[js(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Currency
    currency: String,
    /// Enable or disable outside regular trading hours
    #[js(opt)]
    outside_rth: Option<OutsideRTH>,
    /// Remark
    remark: String,
}

/// Commission-free Status
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::CommissionFreeStatus")]
pub enum CommissionFreeStatus {
    /// Unknown
    Unknown,
    /// None
    None,
    /// Commission-free amount to be calculated
    Calculated,
    /// Pending commission-free
    Pending,
    /// Commission-free applied
    Ready,
}

/// Deduction status
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::DeductionStatus")]
pub enum DeductionStatus {
    /// Unknown
    Unknown,
    /// Pending Settlement
    None,
    /// Settled with no data
    NoData,
    /// Settled and pending distribution
    Pending,
    /// Settled and distributed
    Done,
}

/// Charge category code
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::ChargeCategoryCode")]
pub enum ChargeCategoryCode {
    /// Unknown
    Unknown,
    /// Broker
    Broker,
    /// Third
    Third,
}

/// Order history detail
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::OrderHistoryDetail")]
pub struct OrderHistoryDetail {
    /// Executed price for executed orders, submitted price for expired,
    /// canceled, rejected orders, etc.
    price: Decimal,
    /// Executed quantity for executed orders, remaining quantity for expired,
    /// canceled, rejected orders, etc.
    quantity: Decimal,
    /// Order status
    status: OrderStatus,
    /// Execution or error message
    msg: String,
    /// Occurrence time
    #[js(datetime)]
    time: DateTime<Utc>,
}

/// Order charge fee
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::OrderChargeFee")]
pub struct OrderChargeFee {
    /// Charge code
    code: String,
    /// Charge name
    name: String,
    /// Charge amount
    amount: Decimal,
    /// Charge currency
    currency: String,
}

/// Order charge item
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::OrderChargeItem")]
pub struct OrderChargeItem {
    /// Charge category code
    code: ChargeCategoryCode,
    /// Charge category name
    name: String,
    /// Charge details
    #[js(array)]
    fees: Vec<OrderChargeFee>,
}

/// Order charge detail
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::OrderChargeDetail")]
pub struct OrderChargeDetail {
    /// Total charges amount
    total_amount: Decimal,
    /// Settlement currency
    currency: String,
    /// Order charge items
    #[js(array)]
    items: Vec<OrderChargeItem>,
}

/// Order detail
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::OrderDetail")]
pub struct OrderDetail {
    /// Order ID
    order_id: String,
    /// Order status
    status: OrderStatus,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    quantity: Decimal,
    /// Executed quantity
    executed_quantity: Decimal,
    /// Submitted price
    #[js(opt)]
    price: Option<Decimal>,
    /// Executed price
    #[js(opt)]
    executed_price: Option<Decimal>,
    /// Submitted time
    #[js(datetime)]
    submitted_at: DateTime<Utc>,
    /// Order side
    side: OrderSide,
    /// Security code
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Last done
    #[js(opt)]
    last_done: Option<Decimal>,
    /// `LIT` / `MIT` Order Trigger Price
    #[js(opt)]
    trigger_price: Option<Decimal>,
    /// Rejected Message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Time in force type
    time_in_force: TimeInForceType,
    /// Long term order expire date
    #[js(opt)]
    expire_date: Option<NaiveDate>,
    /// Last updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Conditional order trigger time
    #[js(opt, datetime)]
    trigger_at: Option<DateTime<Utc>>,
    /// `TSMAMT` / `TSLPAMT` order trailing amount
    #[js(opt)]
    trailing_amount: Option<Decimal>,
    /// `TSMPCT` / `TSLPPCT` order trailing percent
    #[js(opt)]
    trailing_percent: Option<Decimal>,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount
    #[js(opt)]
    limit_offset: Option<Decimal>,
    /// Conditional order trigger status
    #[js(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Currency
    currency: String,
    /// Enable or disable outside regular trading hours
    #[js(opt)]
    outside_rth: Option<OutsideRTH>,
    /// Remark
    remark: String,
    /// Commission-free Status
    free_status: CommissionFreeStatus,
    /// Commission-free amount
    #[js(opt)]
    free_amount: Option<Decimal>,
    /// Commission-free currency
    #[js(opt)]
    free_currency: Option<String>,
    /// Deduction status
    deductions_status: DeductionStatus,
    /// Deduction amount
    #[js(opt)]
    deductions_amount: Option<Decimal>,
    /// Deduction currency
    deductions_currency: Option<String>,
    /// Platform fee deduction status
    platform_deducted_status: DeductionStatus,
    /// Platform deduction amount
    #[js(opt)]
    platform_deducted_amount: Option<Decimal>,
    /// Platform deduction currency
    #[js(opt)]
    platform_deducted_currency: Option<String>,
    /// Order history details
    #[js(array)]
    history: Vec<OrderHistoryDetail>,
    /// Order charges
    charge_detail: OrderChargeDetail,
}

/// Order changed message
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::PushOrderChanged")]
pub struct PushOrderChanged {
    /// Order side
    side: OrderSide,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    submitted_quantity: Decimal,
    /// Order symbol
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Submitted price
    submitted_price: Decimal,
    /// Executed quantity
    executed_quantity: Decimal,
    /// Executed price
    #[js(opt)]
    executed_price: Option<Decimal>,
    /// Order ID
    order_id: String,
    /// Currency
    currency: String,
    /// Order status
    status: OrderStatus,
    /// Submitted time
    #[js(datetime)]
    submitted_at: DateTime<Utc>,
    /// Last updated time
    #[js(datetime)]
    updated_at: DateTime<Utc>,
    /// Order trigger price
    #[js(opt)]
    trigger_price: Option<Decimal>,
    /// Rejected message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Conditional order trigger status
    #[js(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Conditional order trigger time
    #[js(opt, datetime)]
    trigger_at: Option<DateTime<Utc>>,
    /// Trailing amount
    #[js(opt)]
    trailing_amount: Option<Decimal>,
    /// Trailing percent
    #[js(opt)]
    trailing_percent: Option<Decimal>,
    /// Limit offset amount
    #[js(opt)]
    limit_offset: Option<Decimal>,
    /// Account no
    account_no: String,
    /// Last share
    #[js(opt)]
    last_share: Option<Decimal>,
    /// Last price
    #[js(opt)]
    last_price: Option<Decimal>,
    /// Remark message
    remark: String,
}

/// Response for submit order request
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::SubmitOrderResponse")]
pub struct SubmitOrderResponse {
    /// Order id
    order_id: String,
}

/// Account balance
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::CashInfo")]
pub struct CashInfo {
    /// Withdraw cash
    withdraw_cash: Decimal,
    /// Available cash
    available_cash: Decimal,
    /// Frozen cash
    frozen_cash: Decimal,
    /// Cash to be settled
    settling_cash: Decimal,
    /// Currency
    currency: String,
}

/// Frozen transaction fee
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::FrozenTransactionFee")]
pub struct FrozenTransactionFee {
    /// Currency
    currency: String,
    /// Frozen transaction fee amount
    frozen_transaction_fee: Decimal,
}

/// Account balance
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::AccountBalance")]
pub struct AccountBalance {
    /// Total cash
    total_cash: Decimal,
    /// Maximum financing amount
    max_finance_amount: Decimal,
    /// Remaining financing amount
    remaining_finance_amount: Decimal,
    /// Risk control level
    risk_level: i32,
    /// Margin call
    margin_call: Decimal,
    /// Currency
    currency: String,
    /// Cash details
    #[js(array)]
    cash_infos: Vec<CashInfo>,
    /// Net assets
    net_assets: Decimal,
    /// Initial margin
    init_margin: Decimal,
    /// Maintenance margin
    maintenance_margin: Decimal,
    /// Buy power
    buy_power: Decimal,
    /// Frozen transaction fees
    #[js(array)]
    frozen_transaction_fees: Vec<FrozenTransactionFee>,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::BalanceType")]
pub enum BalanceType {
    /// Unknown
    Unknown,
    /// Cash
    Cash,
    /// Stock
    Stock,
    /// Fund
    Fund,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::trade::CashFlowDirection")]
pub enum CashFlowDirection {
    /// Unknown
    Unknown,
    /// Out
    Out,
    /// In
    In,
}

/// Account balance
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::CashFlow")]
pub struct CashFlow {
    /// Cash flow name
    transaction_flow_name: String,
    /// Outflow direction
    direction: CashFlowDirection,
    /// Balance type
    business_type: BalanceType,
    /// Cash amount
    balance: Decimal,
    /// Cash currency
    currency: String,
    /// Business time
    #[js(datetime)]
    business_time: DateTime<Utc>,
    /// Associated Stock code information
    symbol: Option<String>,
    /// Cash flow description
    description: String,
}

/// Fund positions response
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::trade::FundPositionsResponse")]
pub struct FundPositionsResponse {
    /// Channels
    #[js(array)]
    channels: Vec<FundPositionChannel>,
}

/// Fund position channel
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::FundPositionChannel")]
pub struct FundPositionChannel {
    /// Account type
    account_channel: String,
    /// Fund positions
    #[js(array)]
    positions: Vec<FundPosition>,
}

/// Fund position
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::FundPosition")]
pub struct FundPosition {
    /// Fund ISIN code
    symbol: String,
    /// Current equity
    current_net_asset_value: Decimal,
    /// Current equity time
    #[js(datetime)]
    net_asset_value_day: DateTime<Utc>,
    /// Fund name
    symbol_name: String,
    /// Currency
    currency: String,
    /// Net cost
    cost_net_asset_value: Decimal,
    /// Holding units
    holding_units: Decimal,
}

/// Stock positions response
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::StockPositionsResponse")]
pub struct StockPositionsResponse {
    /// Channels
    #[js(array)]
    channels: Vec<StockPositionChannel>,
}

/// Stock position channel
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::StockPositionChannel")]
pub struct StockPositionChannel {
    /// Account type
    account_channel: String,
    /// Stock positions
    #[js(array)]
    positions: Vec<StockPosition>,
}

/// Stock position
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::StockPosition")]
pub struct StockPosition {
    /// Stock code
    symbol: String,
    /// Stock name
    symbol_name: String,
    /// The number of holdings
    quantity: Decimal,
    /// Available quantity
    available_quantity: Decimal,
    /// Currency
    currency: String,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    cost_price: Decimal,
    /// Market
    market: Market,
    /// Initial position before market opening
    #[js(opt)]
    init_quantity: Option<Decimal>,
}

/// Margin ratio
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::MarginRatio")]
pub struct MarginRatio {
    /// Initial margin ratio
    im_factor: Decimal,
    /// Maintain the initial margin ratio
    mm_factor: Decimal,
    /// Forced close-out margin ratio
    fm_factor: Decimal,
}

/// Response for estimate maximum purchase quantity
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::trade::EstimateMaxPurchaseQuantityResponse")]
pub struct EstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    cash_max_qty: Decimal,
    /// Margin available quantity
    margin_max_qty: Decimal,
}
