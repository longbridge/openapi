use chrono::{DateTime, Utc};
use longport::quote::SubFlags;
use longport_nodejs_macros::{JsEnum, JsObject};

use crate::{
    decimal::Decimal,
    time::{NaiveDate, Time},
    types::Market,
    utils::ToJSON,
};

/// Subscription
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::Subscription")]
pub struct Subscription {
    symbol: String,
    #[js(sub_types)]
    sub_types: Vec<SubType>,
    #[js(array)]
    candlesticks: Vec<Period>,
}

/// Derivative type
#[napi_derive::napi]
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum DerivativeType {
    /// US stock options
    Option,
    /// HK warrants
    Warrant,
}

impl ToJSON for DerivativeType {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(
            match self {
                DerivativeType::Option => "Option",
                DerivativeType::Warrant => "Warrant",
            }
            .to_string(),
        )
    }
}

struct DerivativeTypes(Vec<DerivativeType>);

impl From<longport::quote::DerivativeType> for DerivativeTypes {
    fn from(ty: longport::quote::DerivativeType) -> Self {
        let mut res = Vec::new();
        if ty.contains(longport::quote::DerivativeType::OPTION) {
            res.push(DerivativeType::Option);
        }
        if ty.contains(longport::quote::DerivativeType::WARRANT) {
            res.push(DerivativeType::Warrant);
        }
        DerivativeTypes(res)
    }
}

#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::TradeStatus")]
pub enum TradeStatus {
    /// Normal
    Normal,
    /// Suspension
    Halted,
    /// Delisted
    Delisted,
    /// Fuse
    Fuse,
    /// Prepare List
    PrepareList,
    /// Code Moved
    CodeMoved,
    /// To Be Opened
    ToBeOpened,
    /// Split Stock Halts
    SplitStockHalts,
    /// Expired
    Expired,
    /// Warrant To BeListed
    WarrantPrepareList,
    /// Warrant To BeListed
    #[js(remote = "SuspendTrade")]
    Suspend,
}

/// Trade session
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::TradeSession")]
pub enum TradeSession {
    /// Intraday
    Intraday,
    /// Pre-Market
    Pre,
    /// Post-Market
    Post,
    /// Overnight
    Overnight,
}

/// Quote type of subscription
#[napi_derive::napi]
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum SubType {
    /// Quote
    Quote,
    /// Depth
    Depth,
    /// Brokers
    Brokers,
    /// Trade
    Trade,
}

impl ToJSON for SubType {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(
            match self {
                SubType::Quote => "Quote",
                SubType::Depth => "Depth",
                SubType::Brokers => "Brokers",
                SubType::Trade => "Trade",
            }
            .to_string(),
        )
    }
}

pub struct SubTypes(pub Vec<SubType>);

impl From<SubTypes> for SubFlags {
    fn from(types: SubTypes) -> Self {
        types
            .0
            .into_iter()
            .map(|ty| match ty {
                SubType::Quote => SubFlags::QUOTE,
                SubType::Depth => SubFlags::DEPTH,
                SubType::Brokers => SubFlags::BROKER,
                SubType::Trade => SubFlags::TRADE,
            })
            .fold(SubFlags::empty(), |mut acc, flag| {
                acc |= flag;
                acc
            })
    }
}

impl From<SubFlags> for SubTypes {
    fn from(flags: SubFlags) -> Self {
        let mut res = Vec::new();
        if flags.contains(SubFlags::QUOTE) {
            res.push(SubType::Quote);
        }
        if flags.contains(SubFlags::DEPTH) {
            res.push(SubType::Quote);
        }
        if flags.contains(SubFlags::BROKER) {
            res.push(SubType::Quote);
        }
        if flags.contains(SubFlags::TRADE) {
            res.push(SubType::Quote);
        }
        SubTypes(res)
    }
}

/// Trade direction
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::TradeDirection")]
pub enum TradeDirection {
    /// Neutral
    Neutral,
    /// Down
    Down,
    /// Up
    Up,
}

/// Option type
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::OptionType")]
pub enum OptionType {
    /// Unknown
    Unknown,
    /// American
    American,
    /// Europe
    Europe,
}

/// Option direction
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::OptionDirection")]
pub enum OptionDirection {
    /// Unknown
    Unknown,
    /// Put
    Put,
    /// Call
    Call,
}

/// Warrant type
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::WarrantType")]
pub enum WarrantType {
    /// Unknown
    Unknown,
    /// Call
    Call,
    /// Put
    Put,
    /// Bull
    Bull,
    /// Bear
    Bear,
    /// Inline
    Inline,
}

/// Candlestick period
#[napi_derive::napi]
#[allow(non_camel_case_types)]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::Period")]
pub enum Period {
    /// Unknown
    #[js(remote = "UnknownPeriod")]
    Unknown,
    /// One Minute
    #[js(remote = "OneMinute")]
    Min_1,
    /// Two Minutes
    #[js(remote = "TwoMinute")]
    Min_2,
    /// Three Minutes
    #[js(remote = "ThreeMinute")]
    Min_3,
    /// Five Minutes
    #[js(remote = "FiveMinute")]
    Min_5,
    /// Ten Minutes
    #[js(remote = "TenMinute")]
    Min_10,
    /// Fifteen Minutes
    #[js(remote = "FifteenMinute")]
    Min_15,
    /// Twenty Minutes
    #[js(remote = "TwentyMinute")]
    Min_20,
    /// Thirty Minutes
    #[js(remote = "ThirtyMinute")]
    Min_30,
    /// Forty-Five Minutes
    #[js(remote = "FortyFiveMinute")]
    Min_45,
    /// One Hour
    #[js(remote = "SixtyMinute")]
    Min_60,
    /// Two Hours
    #[js(remote = "TwoHour")]
    Min_120,
    /// Three Hours
    #[js(remote = "ThreeHour")]
    Min_180,
    /// Four Hours
    #[js(remote = "FourHour")]
    Min_240,
    /// Daily
    Day,
    /// Weekly
    Week,
    /// Monthly
    Month,
    /// Quarterly
    #[js(remote = "Quarter")]
    Quarter,
    /// Yearly
    Year,
}

/// Candlestick adjustment type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::AdjustType", from = false)]
pub enum AdjustType {
    /// Actual
    NoAdjust,
    /// Adjust forward
    ForwardAdjust,
}

/// Security board
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::SecurityBoard")]
#[allow(clippy::upper_case_acronyms)]
pub enum SecurityBoard {
    /// Unknown
    Unknown,
    /// US Main Board
    USMain,
    /// US Pink Board
    USPink,
    /// Dow Jones Industrial Average
    USDJI,
    /// Nasdsaq Index
    USNSDQ,
    /// US Industry Board
    USSector,
    /// US Option
    USOption,
    /// US Sepecial Option
    USOptionS,
    /// Hong Kong Equity Securities
    HKEquity,
    /// HK PreIPO Security
    HKPreIPO,
    /// HK Warrant
    HKWarrant,
    /// Hang Seng Index
    HKHS,
    /// HK Industry Board
    HKSector,
    /// SH Main Board(Connect)
    SHMainConnect,
    /// SH Main Board(Non Connect)
    SHMainNonConnect,
    /// SH Science and Technology Innovation Board
    SHSTAR,
    /// CN Index
    CNIX,
    /// CN Industry Board
    CNSector,
    /// SZ Main Board(Connect)
    SZMainConnect,
    /// SZ Main Board(Non Connect)
    SZMainNonConnect,
    /// SZ Gem Board(Connect)
    SZGEMConnect,
    /// SZ Gem Board(Non Connect)
    SZGEMNonConnect,
    /// SG Main Board
    SGMain,
    /// Singapore Straits Index
    STI,
    /// SG Industry Board
    SGSector,
}

/// The basic information of securities
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::SecurityStaticInfo")]
pub struct SecurityStaticInfo {
    /// Security code
    symbol: String,
    /// Security name (zh-CN)
    name_cn: String,
    /// Security name (en)
    name_en: String,
    /// Security name (zh-HK)
    name_hk: String,
    /// Exchange which the security belongs to
    exchange: String,
    /// Trading currency
    currency: String,
    /// Lot size
    lot_size: i32,
    /// Total shares
    total_shares: i64,
    /// Circulating shares
    circulating_shares: i64,
    /// HK shares (only HK stocks)
    hk_shares: i64,
    /// Earnings per share
    eps: Decimal,
    /// Earnings per share (TTM)
    eps_ttm: Decimal,
    /// Net assets per share
    bps: Decimal,
    /// Dividend yield
    dividend_yield: Decimal,
    /// Types of supported derivatives
    #[js(derivative_types)]
    stock_derivatives: Vec<DerivativeType>,
    /// Board
    board: SecurityBoard,
}

/// Quote of US pre/post market
#[napi_derive::napi]
#[derive(Debug, JsObject, Copy, Clone)]
#[js(remote = "longport::quote::PrePostQuote")]
pub struct PrePostQuote {
    /// Latest price
    last_done: Decimal,
    /// Time of latest price
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// High
    high: Decimal,
    /// Low
    low: Decimal,
    /// Close of the last trade session
    prev_close: Decimal,
}

/// Quote of securitity
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::SecurityQuote")]
pub struct SecurityQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: Decimal,
    /// Yesterday's close
    prev_close: Decimal,
    /// Open
    open: Decimal,
    /// High
    high: Decimal,
    /// Low
    low: Decimal,
    /// Time of latest price
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Quote of US pre market
    #[js(opt)]
    pre_market_quote: Option<PrePostQuote>,
    /// Quote of US post market
    #[js(opt)]
    post_market_quote: Option<PrePostQuote>,
    /// Quote of US overnight market
    #[js(opt)]
    overnight_quote: Option<PrePostQuote>,
}

/// Quote of option
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::OptionQuote")]
pub struct OptionQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: Decimal,
    /// Yesterday's close
    prev_close: Decimal,
    /// Open
    open: Decimal,
    /// High
    high: Decimal,
    /// Low
    low: Decimal,
    /// Time of latest price
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Implied volatility
    implied_volatility: Decimal,
    /// Number of open positions
    open_interest: i64,
    /// Exprity date
    expiry_date: NaiveDate,
    /// Strike price
    strike_price: Decimal,
    /// Contract multiplier
    contract_multiplier: Decimal,
    /// Option type
    contract_type: OptionType,
    /// Contract size
    contract_size: Decimal,
    /// Option direction
    direction: OptionDirection,
    /// Underlying security historical volatility of the option
    historical_volatility: Decimal,
    /// Underlying security symbol of the option
    underlying_symbol: String,
}

/// Quote of warrant
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::WarrantQuote")]
pub struct WarrantQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: Decimal,
    /// Yesterday's close
    prev_close: Decimal,
    /// Open
    open: Decimal,
    /// High
    high: Decimal,
    /// Low
    low: Decimal,
    /// Time of latest price
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Implied volatility
    implied_volatility: Decimal,
    /// Exprity date
    expiry_date: NaiveDate,
    /// Last tradalbe date
    last_trade_date: NaiveDate,
    /// Outstanding ratio
    outstanding_ratio: Decimal,
    /// Outstanding quantity
    outstanding_quantity: i64,
    /// Conversion ratio
    conversion_ratio: Decimal,
    /// Warrant type
    category: WarrantType,
    /// Strike price
    strike_price: Decimal,
    /// Upper bound price
    upper_strike_price: Decimal,
    /// Lower bound price
    lower_strike_price: Decimal,
    /// Call price
    call_price: Decimal,
    /// Underlying security symbol of the warrant
    underlying_symbol: String,
}

/// Depth
#[napi_derive::napi]
#[derive(Debug, JsObject, Copy, Clone)]
#[js(remote = "longport::quote::Depth")]
pub struct Depth {
    /// Position
    position: i32,
    /// Price
    #[js(opt)]
    price: Option<Decimal>,
    /// Volume
    volume: i64,
    /// Number of orders
    order_num: i64,
}

/// Security depth
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::SecurityDepth")]
pub struct SecurityDepth {
    /// Ask depth
    #[js(array)]
    asks: Vec<Depth>,
    /// Bid depth
    #[js(array)]
    bids: Vec<Depth>,
}

/// Brokers
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::Brokers")]
pub struct Brokers {
    /// Position
    position: i32,
    /// Broker IDs
    broker_ids: Vec<i32>,
}

/// Security brokers
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::SecurityBrokers")]
pub struct SecurityBrokers {
    /// Ask brokers
    #[js(array)]
    ask_brokers: Vec<Brokers>,
    /// Bid brokers
    #[js(array)]
    bid_brokers: Vec<Brokers>,
}

/// Participant info
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::ParticipantInfo")]
pub struct ParticipantInfo {
    /// Broker IDs
    broker_ids: Vec<i32>,
    /// Participant name (zh-CN)
    name_cn: String,
    /// Participant name (en)
    name_en: String,
    /// Participant name (zh-HK)
    name_hk: String,
}

/// Trade
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::Trade")]
pub struct Trade {
    /// Price
    price: Decimal,
    /// Volume
    volume: i64,
    /// Time of trading
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Trade type
    trade_type: String,
    /// Trade direction
    direction: TradeDirection,
    /// Trade session
    trade_session: TradeSession,
}

/// Intraday line
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::IntradayLine")]
pub struct IntradayLine {
    /// Close price of the minute
    price: Decimal,
    /// Start time of the minute
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Average price
    avg_price: Decimal,
}

/// Candlestick
#[napi_derive::napi]
#[derive(Debug, JsObject, Copy, Clone)]
#[js(remote = "longport::quote::Candlestick")]
pub struct Candlestick {
    /// Close price
    close: Decimal,
    /// Open price
    open: Decimal,
    /// Low price
    low: Decimal,
    /// High price
    high: Decimal,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Timestamp
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Trade session
    trade_session: TradeSession,
}

/// Strike price info
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::StrikePriceInfo")]
pub struct StrikePriceInfo {
    /// Strike price
    price: Decimal,
    /// Security code of call option
    call_symbol: String,
    /// Security code of put option
    put_symbol: String,
    /// Is standard
    standard: bool,
}

/// Issuer info
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::IssuerInfo")]
pub struct IssuerInfo {
    /// Issuer ID
    issuer_id: i32,
    /// Issuer name (zh-CN)
    name_cn: String,
    /// Issuer name (en)
    name_en: String,
    /// Issuer name (zh-HK)
    name_hk: String,
}

/// Sort order type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::SortOrderType")]
pub enum SortOrderType {
    /// Ascending
    Ascending,
    /// Descending
    Descending,
}

/// Warrant sort by
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::WarrantSortBy")]
pub enum WarrantSortBy {
    /// Last done
    LastDone,
    /// Change rate
    ChangeRate,
    /// Change value
    ChangeValue,
    /// Volume
    Volume,
    /// Turnover
    Turnover,
    /// Expiry date
    ExpiryDate,
    /// Strike price
    StrikePrice,
    /// Upper strike price
    UpperStrikePrice,
    /// Lower strike price
    LowerStrikePrice,
    /// Outstanding quantity
    OutstandingQuantity,
    /// Outstanding ratio
    OutstandingRatio,
    /// Premium
    Premium,
    /// In/out of the bound
    ItmOtm,
    /// Implied volatility
    ImpliedVolatility,
    /// Greek value delta
    Delta,
    /// Call price
    CallPrice,
    /// Price interval from the call price
    ToCallPrice,
    /// Effective leverage
    EffectiveLeverage,
    /// Leverage ratio
    LeverageRatio,
    /// Conversion ratio
    ConversionRatio,
    /// Breakeven point
    BalancePoint,
    /// Status
    Status,
}

/// Filter warrant expiry date type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::FilterWarrantExpiryDate")]
#[allow(non_camel_case_types)]
pub enum FilterWarrantExpiryDate {
    /// Less than 3 months
    LT_3,
    /// 3 - 6 months
    Between_3_6,
    /// 6 - 12 months
    Between_6_12,
    /// Greater than 12 months
    GT_12,
}

/// Filter warrant in/out of the bounds type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::FilterWarrantInOutBoundsType")]
pub enum FilterWarrantInOutBoundsType {
    /// In bounds
    In,
    /// Out bounds
    Out,
}

/// Warrant info
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::WarrantInfo")]
pub struct WarrantInfo {
    /// Security code
    symbol: String,
    /// Warrant type
    warrant_type: WarrantType,
    /// Security name
    name: String,
    /// Latest price
    last_done: Decimal,
    /// Quote change rate
    change_rate: Decimal,
    /// Quote change
    change_value: Decimal,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Expiry date
    expiry_date: NaiveDate,
    /// Strike price
    #[js(opt)]
    strike_price: Option<Decimal>,
    /// Upper strike price
    #[js(opt)]
    upper_strike_price: Option<Decimal>,
    /// Lower strike price
    #[js(opt)]
    lower_strike_price: Option<Decimal>,
    /// Outstanding quantity
    outstanding_qty: i64,
    /// Outstanding ratio
    outstanding_ratio: Decimal,
    /// Premium
    premium: Decimal,
    /// In/out of the bound
    #[js(opt)]
    itm_otm: Option<Decimal>,
    /// Implied volatility
    #[js(opt)]
    implied_volatility: Option<Decimal>,
    /// Delta
    #[js(opt)]
    delta: Option<Decimal>,
    /// Call price
    #[js(opt)]
    call_price: Option<Decimal>,
    /// Price interval from the call price
    #[js(opt)]
    to_call_price: Option<Decimal>,
    /// Effective leverage
    #[js(opt)]
    effective_leverage: Option<Decimal>,
    /// Leverage ratio
    leverage_ratio: Decimal,
    /// Conversion ratio
    #[js(opt)]
    conversion_ratio: Option<Decimal>,
    /// Breakeven point
    #[js(opt)]
    balance_point: Option<Decimal>,
    /// Status
    status: WarrantStatus,
}

/// Warrant status
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::WarrantStatus")]
pub enum WarrantStatus {
    /// Suspend
    Suspend,
    /// Prepare List
    PrepareList,
    /// Normal
    Normal,
}

/// The information of trading session
#[napi_derive::napi]
#[derive(Debug, JsObject, Copy, Clone)]
#[js(remote = "longport::quote::TradingSessionInfo")]
pub struct TradingSessionInfo {
    /// Being trading time
    begin_time: Time,
    /// End trading time
    end_time: Time,
    /// Trading session
    trade_session: TradeSession,
}

/// Market trading session
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::MarketTradingSession")]
pub struct MarketTradingSession {
    /// Market
    market: Market,
    /// Trading session
    #[js(array)]
    trade_sessions: Vec<TradingSessionInfo>,
}

/// Real-time quote
#[napi_derive::napi]
#[derive(JsObject, Debug)]
#[js(remote = "longport::quote::RealtimeQuote")]
pub struct RealtimeQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: Decimal,
    /// Open
    open: Decimal,
    /// High
    high: Decimal,
    /// Low
    low: Decimal,
    /// Time of latest price
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Security trading status
    trade_status: TradeStatus,
}

/// Push real-time quote
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::PushQuote")]
pub struct PushQuote {
    /// Latest price
    last_done: Decimal,
    /// Open
    open: Decimal,
    /// High
    high: Decimal,
    /// Low
    low: Decimal,
    /// Time of latest price
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: Decimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Trade session
    trade_session: TradeSession,
    /// Increase volume between pushes
    current_volume: i64,
    /// Increase turnover between pushes
    current_turnover: Decimal,
}

/// Push real-time depth
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::PushDepth")]
pub struct PushDepth {
    /// Ask depth
    #[js(array)]
    asks: Vec<Depth>,
    /// Bid depth
    #[js(array)]
    bids: Vec<Depth>,
}

/// Push real-time brokers
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::PushBrokers")]
pub struct PushBrokers {
    /// Ask brokers
    #[js(array)]
    ask_brokers: Vec<Brokers>,
    /// Bid brokers
    #[js(array)]
    bid_brokers: Vec<Brokers>,
}

/// Push real-time trades
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::PushTrades")]
pub struct PushTrades {
    /// Trades data
    #[js(array)]
    trades: Vec<Trade>,
}

/// Candlestick updated event
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::PushCandlestick")]
pub struct PushCandlestick {
    /// Period type
    period: Period,
    /// Candlestick
    candlestick: Candlestick,
    /// Is confirmed
    is_confirmed: bool,
}

/// Market trading days
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::MarketTradingDays")]
pub struct MarketTradingDays {
    /// Trading days
    #[js(array)]
    trading_days: Vec<NaiveDate>,
    /// Half trading days
    #[js(array)]
    half_trading_days: Vec<NaiveDate>,
}

/// Capital flow line
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::CapitalFlowLine")]
pub struct CapitalFlowLine {
    /// Inflow capital data
    inflow: Decimal,
    /// Time
    #[js(datetime)]
    timestamp: DateTime<Utc>,
}

/// Capital distribution
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::CapitalDistribution")]
pub struct CapitalDistribution {
    /// Large order
    large: Decimal,
    /// Medium order
    medium: Decimal,
    /// Small order
    small: Decimal,
}

/// Capital distribution response
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longport::quote::CapitalDistributionResponse")]
pub struct CapitalDistributionResponse {
    /// Time
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Inflow capital data
    capital_in: CapitalDistribution,
    /// Outflow capital data
    capital_out: CapitalDistribution,
}

/// Watchlist group
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::WatchlistGroup")]
pub struct WatchlistGroup {
    /// Group id
    id: i64,
    /// Group name
    name: String,
    /// Securities
    #[js(array)]
    securities: Vec<WatchlistSecurity>,
}

/// Watchlist security
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::WatchlistSecurity")]
pub struct WatchlistSecurity {
    /// Security symbol
    symbol: String,
    /// Market
    market: Market,
    /// Security name
    name: String,
    /// Watched price
    #[js(opt)]
    watched_price: Option<Decimal>,
    /// Watched time
    #[js(datetime)]
    watched_at: DateTime<Utc>,
}

/// Securities update mode
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::SecuritiesUpdateMode")]
pub enum SecuritiesUpdateMode {
    /// Add securities
    Add,
    /// Remove securities
    Remove,
    /// Replace securities
    Replace,
}

#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::CalcIndex")]
pub enum CalcIndex {
    /// Latest price
    LastDone,
    /// Change value
    ChangeValue,
    /// Change rate
    ChangeRate,
    /// Volume
    Volume,
    /// Turnover
    Turnover,
    /// Year-to-date change ratio
    YtdChangeRate,
    /// Turnover rate
    TurnoverRate,
    /// Total market value
    TotalMarketValue,
    /// Capital flow
    CapitalFlow,
    /// Amplitude
    Amplitude,
    /// Volume ratio
    VolumeRatio,
    /// PE (TTM)
    PeTtmRatio,
    /// PB
    PbRatio,
    /// Dividend ratio (TTM)
    DividendRatioTtm,
    /// Five days change ratio
    FiveDayChangeRate,
    /// Ten days change ratio
    TenDayChangeRate,
    /// Half year change ratio
    HalfYearChangeRate,
    /// Five minutes change ratio
    FiveMinutesChangeRate,
    /// Expiry date
    ExpiryDate,
    /// Strike price
    StrikePrice,
    /// Upper bound price
    UpperStrikePrice,
    /// Lower bound price
    LowerStrikePrice,
    /// Outstanding quantity
    OutstandingQty,
    /// Outstanding ratio
    OutstandingRatio,
    /// Premium
    Premium,
    /// In/out of the bound
    ItmOtm,
    /// Implied volatility
    ImpliedVolatility,
    /// Warrant delta
    WarrantDelta,
    /// Call price
    CallPrice,
    /// Price interval from the call price
    ToCallPrice,
    /// Effective leverage
    EffectiveLeverage,
    /// Leverage ratio
    LeverageRatio,
    /// Conversion ratio
    ConversionRatio,
    /// Breakeven point
    BalancePoint,
    /// Open interest
    OpenInterest,
    /// Delta
    Delta,
    /// Gamma
    Gamma,
    /// Theta
    Theta,
    /// Vega
    Vega,
    /// Rho
    Rho,
}

/// Security calc index response
#[napi_derive::napi]
#[derive(JsObject, Debug, Clone)]
#[js(remote = "longport::quote::SecurityCalcIndex")]
pub struct SecurityCalcIndex {
    /// Security code
    symbol: String,
    /// Latest price
    #[js(opt)]
    last_done: Option<Decimal>,
    /// Change value
    #[js(opt)]
    change_value: Option<Decimal>,
    /// Change ratio
    #[js(opt)]
    change_rate: Option<Decimal>,
    /// Volume
    #[js(opt)]
    volume: Option<i64>,
    /// Turnover
    #[js(opt)]
    turnover: Option<Decimal>,
    /// Year-to-date change ratio
    #[js(opt)]
    ytd_change_rate: Option<Decimal>,
    /// Turnover rate
    #[js(opt)]
    turnover_rate: Option<Decimal>,
    /// Total market value
    #[js(opt)]
    total_market_value: Option<Decimal>,
    /// Capital flow
    #[js(opt)]
    capital_flow: Option<Decimal>,
    /// Amplitude
    #[js(opt)]
    amplitude: Option<Decimal>,
    /// Volume ratio
    #[js(opt)]
    volume_ratio: Option<Decimal>,
    /// PE (TTM)
    #[js(opt)]
    pe_ttm_ratio: Option<Decimal>,
    /// PB
    #[js(opt)]
    pb_ratio: Option<Decimal>,
    /// Dividend ratio (TTM)
    #[js(opt)]
    dividend_ratio_ttm: Option<Decimal>,
    /// Five days change ratio
    #[js(opt)]
    five_day_change_rate: Option<Decimal>,
    /// Ten days change ratio
    #[js(opt)]
    ten_day_change_rate: Option<Decimal>,
    /// Half year change ratio
    #[js(opt)]
    half_year_change_rate: Option<Decimal>,
    /// Five minutes change ratio
    #[js(opt)]
    five_minutes_change_rate: Option<Decimal>,
    /// Expiry date
    #[js(opt)]
    expiry_date: Option<NaiveDate>,
    /// Strike price
    #[js(opt)]
    strike_price: Option<Decimal>,
    /// Upper bound price
    #[js(opt)]
    upper_strike_price: Option<Decimal>,
    /// Lower bound price
    #[js(opt)]
    lower_strike_price: Option<Decimal>,
    /// Outstanding quantity
    #[js(opt)]
    outstanding_qty: Option<i64>,
    /// Outstanding ratio
    #[js(opt)]
    outstanding_ratio: Option<Decimal>,
    /// Premium
    #[js(opt)]
    premium: Option<Decimal>,
    /// In/out of the bound
    #[js(opt)]
    itm_otm: Option<Decimal>,
    /// Implied volatility
    #[js(opt)]
    implied_volatility: Option<Decimal>,
    /// Warrant delta
    #[js(opt)]
    warrant_delta: Option<Decimal>,
    /// Call price
    #[js(opt)]
    call_price: Option<Decimal>,
    /// Price interval from the call price
    #[js(opt)]
    to_call_price: Option<Decimal>,
    /// Effective leverage
    #[js(opt)]
    effective_leverage: Option<Decimal>,
    /// Leverage ratio
    #[js(opt)]
    leverage_ratio: Option<Decimal>,
    /// Conversion ratio
    #[js(opt)]
    conversion_ratio: Option<Decimal>,
    /// Breakeven point
    #[js(opt)]
    balance_point: Option<Decimal>,
    /// Open interest
    #[js(opt)]
    open_interest: Option<i64>,
    /// Delta
    #[js(opt)]
    delta: Option<Decimal>,
    /// Gamma
    #[js(opt)]
    gamma: Option<Decimal>,
    /// Theta
    #[js(opt)]
    theta: Option<Decimal>,
    /// Vega
    #[js(opt)]
    vega: Option<Decimal>,
    /// Rho
    #[js(opt)]
    rho: Option<Decimal>,
}

/// Security list category
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::SecurityListCategory")]
pub enum SecurityListCategory {
    /// Overnight
    Overnight,
}

/// Security
#[napi_derive::napi]
#[derive(JsObject, Debug, Clone)]
#[js(remote = "longport::quote::Security")]
pub struct Security {
    /// Security code
    symbol: String,
    /// Security name (zh-CN)
    name_cn: String,
    /// Security name (en)
    name_en: String,
    /// Security name (zh-HK)
    name_hk: String,
}

#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::QuotePackageDetail")]
pub struct QuotePackageDetail {
    /// Key
    key: String,
    /// Name
    name: String,
    /// Description
    description: String,
    /// Start time
    #[js(datetime)]
    start_at: DateTime<Utc>,
    /// End time
    #[js(datetime)]
    end_at: DateTime<Utc>,
}

/// Trade sessions
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longport::quote::TradeSessions")]
pub enum TradeSessions {
    /// Intraday
    Intraday,
    /// All
    All,
}

/// Market temperature
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::MarketTemperature")]
pub struct MarketTemperature {
    /// Temperature value
    temperature: i32,
    /// Temperature description
    description: String,
    /// Market valuation
    valuation: i32,
    /// Market sentiment
    sentiment: i32,
    /// Time
    #[js(datetime)]
    timestamp: DateTime<Utc>,
}

/// Data granularity
#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longport::quote::Granularity")]
pub enum Granularity {
    /// Unknown
    Unknown,
    /// Daily
    Daily,
    /// Weekly
    Weekly,
    /// Monthly
    Monthly,
}

/// History market temperature response
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::quote::HistoryMarketTemperatureResponse")]
pub struct HistoryMarketTemperatureResponse {
    /// Granularity
    granularity: Granularity,
    /// Records
    #[js(array)]
    records: Vec<MarketTemperature>,
}
