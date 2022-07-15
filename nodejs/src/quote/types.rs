use chrono::{DateTime, Utc};
use longbridge::quote::SubFlags;
use longbridge_nodejs_macros::{JsEnum, JsObject};
use napi::bindgen_prelude::*;

use crate::{
    decimal::Decimal,
    time::{NaiveDate, Time},
    types::Market,
};

/// Subscription
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::quote::Subscription")]
pub struct Subscription {
    symbol: String,
    #[js(sub_types)]
    sub_types: Vec<SubType>,
    #[js(array)]
    candlesticks: Vec<Period>,
}

/// Derivative type
#[napi_derive::napi]
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum DerivativeType {
    /// US stock options
    Option,
    /// HK warrants
    Warrant,
}

struct DerivativeTypes(Vec<DerivativeType>);

impl From<longbridge::quote::DerivativeType> for DerivativeTypes {
    fn from(ty: longbridge::quote::DerivativeType) -> Self {
        let mut res = Vec::new();
        if ty.contains(longbridge::quote::DerivativeType::OPTION) {
            res.push(DerivativeType::Option);
        }
        if ty.contains(longbridge::quote::DerivativeType::WARRANT) {
            res.push(DerivativeType::Warrant);
        }
        DerivativeTypes(res)
    }
}

#[napi_derive::napi]
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::TradeStatus")]
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
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::TradeSession")]
pub enum TradeSession {
    /// Trading
    #[js(remote = "NormalTrade")]
    Normal,
    /// Pre-Trading
    #[js(remote = "PreTrade")]
    Pre,
    /// Post-Trading
    #[js(remote = "PostTrade")]
    Post,
}

/// Quote type of subscription
#[napi_derive::napi]
#[derive(Debug, Hash, Eq, PartialEq)]
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
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::TradeDirection")]
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
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::OptionType")]
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
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::OptionDirection")]
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
#[derive(JsEnum, Debug, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::WarrantType")]
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
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::Period")]
pub enum Period {
    /// One Minute
    #[js(remote = "UnknownPeriod")]
    Unknown,
    /// One Minute
    #[js(remote = "OneMinute")]
    Min_1,
    /// Five Minutes
    #[js(remote = "FiveMinute")]
    Min_5,
    /// Fifteen Minutes
    #[js(remote = "FifteenMinute")]
    Min_15,
    /// Thirty Minutes
    #[js(remote = "ThirtyMinute")]
    Min_30,
    /// Sixty Minutes
    #[js(remote = "SixtyMinute")]
    Min_60,
    /// One Day
    Day,
    /// One Week
    Week,
    /// One Month
    Month,
    /// One Year
    Year,
}

/// Candlestick adjustment type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::quote::AdjustType", from = false)]
pub enum AdjustType {
    /// Actual
    NoAdjust,
    /// Adjust forward
    ForwardAdjust,
}

/// The basic information of securities
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::quote::SecurityStaticInfo")]
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
}

/// Quote of US pre/post market
#[napi_derive::napi]
#[derive(Debug, JsObject, Copy, Clone)]
#[js(remote = "longbridge::quote::PrePostQuote")]
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
#[js(remote = "longbridge::quote::SecurityQuote")]
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
}

/// Quote of option
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::quote::OptionQuote")]
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
#[js(remote = "longbridge::quote::WarrantQuote")]
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
#[js(remote = "longbridge::quote::Depth")]
pub struct Depth {
    /// Position
    position: i32,
    /// Price
    price: Decimal,
    /// Volume
    volume: i64,
    /// Number of orders
    order_num: i64,
}

/// Security depth
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::quote::SecurityDepth")]
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
#[js(remote = "longbridge::quote::Brokers")]
pub struct Brokers {
    /// Position
    position: i32,
    /// Broker IDs
    broker_ids: Vec<i32>,
}

/// Security brokers
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::quote::SecurityBrokers")]
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
#[js(remote = "longbridge::quote::ParticipantInfo")]
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
#[js(remote = "longbridge::quote::Trade")]
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
#[js(remote = "longbridge::quote::IntradayLine")]
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
#[js(remote = "longbridge::quote::Candlestick")]
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
}

/// Strike price info
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::quote::StrikePriceInfo")]
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
#[js(remote = "longbridge::quote::IssuerInfo")]
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

/// The information of trading session
#[napi_derive::napi]
#[derive(Debug, JsObject, Copy, Clone)]
#[js(remote = "longbridge::quote::TradingSessionInfo")]
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
#[js(remote = "longbridge::quote::MarketTradingSession")]
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
#[js(remote = "longbridge::quote::RealtimeQuote")]
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
#[js(remote = "longbridge::quote::PushQuote")]
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
}

/// Push real-time depth
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::quote::PushDepth")]
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
#[js(remote = "longbridge::quote::PushBrokers")]
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
#[js(remote = "longbridge::quote::PushTrades")]
pub struct PushTrades {
    /// Trades data
    #[js(array)]
    trades: Vec<Trade>,
}

/// Candlestick updated event
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::quote::PushCandlestick")]
pub struct PushCandlestick {
    /// Period type
    period: Period,
    /// Candlestick
    candlestick: Candlestick,
}

/// Market trading days
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longbridge::quote::MarketTradingDays")]
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
#[js(remote = "longbridge::quote::CapitalFlowLine")]
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
#[js(remote = "longbridge::quote::CapitalDistribution")]
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
#[js(remote = "longbridge::quote::CapitalDistributionResponse")]
pub struct CapitalDistributionResponse {
    /// Time
    #[js(datetime)]
    timestamp: DateTime<Utc>,
    /// Inflow capital data
    capital_in: CapitalDistribution,
    /// Outflow capital data
    capital_out: CapitalDistribution,
}

/// Watch list group
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::quote::WatchListGroup")]
pub struct WatchListGroup {
    /// Group id
    pub id: i64,
    /// Group name
    pub name: String,
    /// Securities
    #[js(array)]
    securities: Vec<WatchListSecurity>,
}

/// Watch list security
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::quote::WatchListSecurity")]
pub struct WatchListSecurity {
    /// Security symbol
    symbol: String,
    /// Market
    market: Market,
    /// Security name
    name: String,
    /// Latest price
    price: Decimal,
    /// Watched time
    #[js(datetime)]
    watched_at: DateTime<Utc>,
}
