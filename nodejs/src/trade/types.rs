use chrono::{DateTime, Utc};
use longbridge_nodejs_macros::{JsEnum, JsObject};

use crate::{decimal::Decimal, time::NaiveDate, types::Market};

/// Topic type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::trade::TopicType")]
pub enum TopicType {
    /// Private notification for trade
    Private,
}

/// Trade
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::trade::Execution")]
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
#[js(remote = "longbridge::trade::OrderStatus")]
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
#[js(remote = "longbridge::trade::OrderSide")]
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
#[js(remote = "longbridge::trade::OrderType")]
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
#[js(remote = "longbridge::trade::OrderTag")]
pub enum OrderTag {
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
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::trade::TimeInForceType")]
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
#[js(remote = "longbridge::trade::TriggerStatus")]
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
#[js(remote = "longbridge::trade::OutsideRTH")]
pub enum OutsideRTH {
    /// Unknown
    Unknown,
    /// Regular trading hour only
    RTHOnly,
    /// Any time
    AnyTime,
    /// Overnight
    Overnight,
    /// Overnight option
    OptionPreMarket,
}

/// Response for get all executions request
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::trade::AllExecutionsResponse")]
pub struct AllExecutionsResponse {
    /// Has more records
    has_more: bool,
    /// Execution list
    #[js(array)]
    trades: Vec<Execution>,
}

/// Attached order type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::trade::AttachedOrderType")]
pub enum AttachedOrderType {
    /// Profit taker
    ProfitTaker,
    /// Stop loss
    StopLoss,
    /// Bracket
    Bracket,
}

/// Attached order detail
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::trade::AttachedOrderDetail")]
pub struct AttachedOrderDetail {
    /// Order ID
    order_id: String,
    /// Attached type display
    attached_type_display: i32,
    /// Trigger price
    #[js(opt)]
    trigger_price: Option<Decimal>,
    /// Submitted quantity
    quantity: Decimal,
    /// Executed quantity
    executed_qty: Decimal,
    /// Order status
    status: OrderStatus,
    /// Last updated time
    #[js(datetime)]
    updated_at: DateTime<Utc>,
    /// Whether withdrawn
    withdrawn: bool,
    /// Good till date
    #[js(opt)]
    gtd: Option<NaiveDate>,
    /// Time in force type
    time_in_force: TimeInForceType,
    /// Counter ID
    counter_id: String,
    /// Trigger status
    trigger_status: i32,
    /// Executed amount
    executed_amount: Decimal,
    /// Tag
    tag: i32,
    /// Submitted time
    #[js(datetime)]
    submitted_at: DateTime<Utc>,
    /// Executed price
    executed_price: Decimal,
    /// Force only RTH
    #[js(opt)]
    force_only_rth: Option<OutsideRTH>,
    /// Reviewed
    reviewed: bool,
    /// Activate order type
    activate_order_type: OrderType,
    /// Activate RTH
    #[js(opt)]
    activate_rth: Option<OutsideRTH>,
    /// Submit price
    #[js(opt)]
    submit_price: Option<Decimal>,
}

/// Order
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::trade::Order")]
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
    /// Limit depth level
    #[js(opt)]
    limit_depth_level: Option<i32>,
    /// Trigger count
    #[js(opt)]
    trigger_count: Option<i32>,
    /// Monitor price
    #[js(opt)]
    monitor_price: Option<Decimal>,
    /// Remark
    remark: String,
    /// Attached orders
    #[js(array)]
    attached_orders: Vec<AttachedOrderDetail>,
}

/// Commission-free Status
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::trade::CommissionFreeStatus")]
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
#[js(remote = "longbridge::trade::DeductionStatus")]
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
#[js(remote = "longbridge::trade::ChargeCategoryCode")]
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
#[js(remote = "longbridge::trade::OrderHistoryDetail")]
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
#[js(remote = "longbridge::trade::OrderChargeFee")]
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
#[js(remote = "longbridge::trade::OrderChargeItem")]
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
#[js(remote = "longbridge::trade::OrderChargeDetail")]
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
#[js(remote = "longbridge::trade::OrderDetail")]
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
    /// Limit depth level
    #[js(opt)]
    limit_depth_level: Option<i32>,
    /// Trigger count
    #[js(opt)]
    trigger_count: Option<i32>,
    /// Monitor price
    #[js(opt)]
    monitor_price: Option<Decimal>,
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
    /// Attached orders
    #[js(array)]
    attached_orders: Vec<AttachedOrderDetail>,
}

/// Order changed message
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::trade::PushOrderChanged")]
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
#[js(remote = "longbridge::trade::SubmitOrderResponse")]
pub struct SubmitOrderResponse {
    /// Order id
    order_id: String,
}

/// Account balance
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::trade::CashInfo")]
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
#[js(remote = "longbridge::trade::FrozenTransactionFee")]
pub struct FrozenTransactionFee {
    /// Currency
    currency: String,
    /// Frozen transaction fee amount
    frozen_transaction_fee: Decimal,
}

/// Account balance
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::trade::AccountBalance")]
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
#[js(remote = "longbridge::trade::BalanceType")]
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
#[js(remote = "longbridge::trade::CashFlowDirection")]
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
#[js(remote = "longbridge::trade::CashFlow")]
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
#[js(remote = "longbridge::trade::FundPositionsResponse")]
pub struct FundPositionsResponse {
    /// Channels
    #[js(array)]
    channels: Vec<FundPositionChannel>,
}

/// Fund position channel
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::trade::FundPositionChannel")]
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
#[js(remote = "longbridge::trade::FundPosition")]
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
#[js(remote = "longbridge::trade::StockPositionsResponse")]
pub struct StockPositionsResponse {
    /// Channels
    #[js(array)]
    channels: Vec<StockPositionChannel>,
}

/// Stock position channel
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::trade::StockPositionChannel")]
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
#[js(remote = "longbridge::trade::StockPosition")]
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
#[js(remote = "longbridge::trade::MarginRatio")]
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
#[js(remote = "longbridge::trade::EstimateMaxPurchaseQuantityResponse")]
pub struct EstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    cash_max_qty: Decimal,
    /// Margin available quantity
    margin_max_qty: Decimal,
}

// ── US-market types ──────────────────────────────────────────────────────────

/// One cash currency entry in USAssetOverview
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USCashEntry {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USCryptoEntry {
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

/// One stock/equity position in USAssetOverview
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USStockEntry {
    pub symbol: String,
    pub full_symbol: String,
    pub asset_type: String,
    pub quantity: String,
    pub currency: String,
    pub average_cost: String,
    pub market: String,
    pub trade_status: String,
    pub prev_close: String,
    pub last_done: String,
    pub market_price: String,
    pub pretrade_close: String,
    pub stock_invest_of_today: String,
    pub today_pl: String,
    pub pretrade_stock_invest_of_today: String,
    pub pretrade_today_pl: String,
    pub night_last_done: String,
    pub night_prev_close: String,
    pub position_side: String,
    pub open_position_time: String,
    pub name: String,
    pub industry_counter_id: String,
    pub industry_name: String,
}

impl From<longbridge::trade::USStockEntry> for USStockEntry {
    fn from(v: longbridge::trade::USStockEntry) -> Self {
        Self {
            symbol: v.symbol,
            full_symbol: v.full_symbol,
            asset_type: v.asset_type,
            quantity: v.quantity,
            currency: v.currency,
            average_cost: v.average_cost,
            market: v.market,
            trade_status: v.trade_status,
            prev_close: v.prev_close,
            last_done: v.last_done,
            market_price: v.market_price,
            pretrade_close: v.pretrade_close,
            stock_invest_of_today: v.stock_invest_of_today,
            today_pl: v.today_pl,
            pretrade_stock_invest_of_today: v.pretrade_stock_invest_of_today,
            pretrade_today_pl: v.pretrade_today_pl,
            night_last_done: v.night_last_done,
            night_prev_close: v.night_prev_close,
            position_side: v.position_side,
            open_position_time: v.open_position_time,
            name: v.name,
            industry_counter_id: v.industry_counter_id,
            industry_name: v.industry_name,
        }
    }
}

/// US account asset snapshot
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USAssetOverview {
    pub account_type: String,
    pub asset_timestamp: i64,
    pub cash_buy_power: String,
    pub overnight_buy_power: String,
    pub currency: String,
    pub cash_list: Vec<USCashEntry>,
    pub stock_list: Vec<USStockEntry>,
    pub option_list: Vec<serde_json::Value>,
    pub crypto_list: Vec<USCryptoEntry>,
    pub multi_leg: serde_json::Value,
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
            overnight_buy_power: v.overnight_buy_power,
            currency: v.currency,
            cash_list: v.cash_list.into_iter().map(Into::into).collect(),
            stock_list: v.stock_list.into_iter().map(Into::into).collect(),
            option_list: v.option_list,
            crypto_list: v.crypto_list.into_iter().map(Into::into).collect(),
            multi_leg: v.multi_leg,
        }
    }
}

/// One time-period metric in USRealizedPLEntry
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USRealizedPLMetric {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USRealizedPLEntry {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USRealizedPL {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USOrderHistory {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USButtonControl {
    pub withdraw: i32,
    pub replace: i32,
    pub exceptionable: Vec<String>,
}

impl From<longbridge::trade::USButtonControl> for USButtonControl {
    fn from(v: longbridge::trade::USButtonControl) -> Self {
        Self {
            withdraw: v.withdraw,
            replace: v.replace,
            exceptionable: v.exceptionable,
        }
    }
}

/// One fee category within USChargeDetail.
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USChargeItem {
    pub code: i32,
    pub name: String,
    pub fees: Vec<String>,
}

impl From<longbridge::trade::USChargeItem> for USChargeItem {
    fn from(v: longbridge::trade::USChargeItem) -> Self {
        Self {
            code: v.code,
            name: v.name,
            fees: v.fees,
        }
    }
}

/// Fee breakdown for an order.
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USChargeDetail {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USAttachedOrder {
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
    pub symbol: String,
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
            symbol: v.symbol,
            withdrawn: v.withdrawn,
        }
    }
}

/// Full typed order object within USOrderDetailResponse.
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USOrderDetail {
    pub id: String,
    pub aaid: String,
    pub account_channel: String,
    pub action: i32,
    pub symbol: String,
    pub underlying_symbol: String,
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
            id: v.id,
            aaid: v.aaid,
            account_channel: v.account_channel,
            action: v.action,
            symbol: v.symbol,
            underlying_symbol: v.underlying_symbol,
            security_type: v.security_type,
            name: v.name,
            currency: v.currency,
            trade_currency: v.trade_currency,
            order_type: v.order_type,
            status: v.status,
            price: v.price,
            quantity: v.quantity,
            executed_qty: v.executed_qty,
            executed_price: v.executed_price,
            executed_amount: v.executed_amount,
            operate_direction: v.operate_direction,
            time_in_force: v.time_in_force,
            gtd: v.gtd,
            tag: v.tag,
            msg: v.msg,
            force_only_rth: v.force_only_rth,
            submitted_at: v.submitted_at,
            done_at: v.done_at,
            trigger_price: v.trigger_price,
            trigger_at: v.trigger_at,
            trigger_status: v.trigger_status,
            trigger_exchange: v.trigger_exchange,
            trigger_last_done: v.trigger_last_done,
            trigger_count: v.trigger_count,
            tailing_amount: v.tailing_amount,
            tailing_percent: v.tailing_percent,
            limit_offset: v.limit_offset,
            limit_depth_level: v.limit_depth_level,
            market_price: v.market_price,
            submitted_amount: v.submitted_amount,
            estimated_fee: v.estimated_fee,
            free_status: v.free_status,
            free_amount: v.free_amount,
            free_currency: v.free_currency,
            deductions_status: v.deductions_status,
            deductions_amount: v.deductions_amount,
            deductions_currency: v.deductions_currency,
            platform_deductions_status: v.platform_deductions_status,
            platform_deductions_amount: v.platform_deductions_amount,
            platform_deductions_currency: v.platform_deductions_currency,
            display_account: v.display_account,
            settlement_account: v.settlement_account,
            settlement_channel: v.settlement_channel,
            customer_name: v.customer_name,
            real_name: v.real_name,
            en_name: v.en_name,
            joint_real_name: v.joint_real_name,
            joint_en_name: v.joint_en_name,
            org_id: v.org_id,
            bcan: v.bcan,
            op_entrust_way: v.op_entrust_way,
            op_entrust_way_name: v.op_entrust_way_name,
            remark: v.remark,
            notice: v.notice,
            short_sell_type: v.short_sell_type,
            ploy_type: v.ploy_type,
            ploy_id: v.ploy_id,
            ploy_status: v.ploy_status,
            trend: v.trend,
            withdrawal_reason: v.withdrawal_reason,
            activate_order_type: v.activate_order_type,
            activate_rth: v.activate_rth,
            submit_price: v.submit_price,
            contract_direction: v.contract_direction,
            strike_price: v.strike_price,
            contract_size: v.contract_size,
            monitor_price: v.monitor_price,
            button_control: v.button_control.into(),
            charge_detail: v.charge_detail.map(Into::into),
            attached_orders: v.attached_orders.into_iter().map(Into::into).collect(),
            order_histories: v.order_histories.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response for us_order_detail.
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct USOrderDetailResponse {
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
