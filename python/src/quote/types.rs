use longbridge::quote::SubFlags;
use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::prelude::*;

use crate::{
    decimal::PyDecimal,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper, PyTimeWrapper},
    types::Market,
};

/// Derivative type
#[pyclass]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::DerivativeType")]
pub(crate) enum DerivativeType {
    /// US stock options
    Option,
    /// HK warrants
    Warrant,
}

#[pyclass]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::TradeStatus")]
pub(crate) enum TradeStatus {
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
    #[py(remote = "SuspendTrade")]
    Suspend,
}

/// Trade session
#[pyclass]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::TradeSession")]
pub(crate) enum TradeSession {
    /// Trading
    #[py(remote = "NormalTrade")]
    Normal,
    /// Pre-Trading
    #[py(remote = "PreTrade")]
    Pre,
    /// Post-Trading
    #[py(remote = "PostTrade")]
    Post,
}

/// Quote type of subscription
#[pyclass]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum SubType {
    /// Quote
    Quote,
    /// Depth
    Depth,
    /// Brokers
    Brokers,
    /// Trade
    Trade,
}

pub(crate) struct SubTypes(pub(crate) Vec<SubType>);

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

/// Trade direction
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::TradeDirection")]
pub(crate) enum TradeDirection {
    /// Neutral
    Neutral,
    /// Down
    Down,
    /// Up
    Up,
}

/// Option type
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::OptionType")]
pub(crate) enum OptionType {
    /// Unknown
    Unknown,
    /// American
    American,
    /// Europe
    Europe,
}

/// Option direction
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::OptionDirection")]
pub(crate) enum OptionDirection {
    /// Unknown
    Unknown,
    /// Put
    Put,
    /// Call
    Call,
}

/// Warrant type
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::WarrantType")]
pub(crate) enum WarrantType {
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
#[pyclass]
#[allow(non_camel_case_types)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::Period", from = false)]
pub(crate) enum Period {
    /// One Minute
    #[py(remote = "OneMinute")]
    Min_1,
    /// Five Minutes
    #[py(remote = "FiveMinute")]
    Min_5,
    /// Fifteen Minutes
    #[py(remote = "FifteenMinute")]
    Min_15,
    /// Thirty Minutes
    #[py(remote = "ThirtyMinute")]
    Min_30,
    /// Sixty Minutes
    #[py(remote = "SixtyMinute")]
    Min_60,
    /// One Days
    Day,
    /// One Week
    Week,
    /// One Month
    Month,
    /// One Year
    Year,
}

/// Candlestick adjustment type
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::AdjustType")]
pub(crate) enum AdjustType {
    /// Actual
    NoAdjust,
    /// Adjust forward
    ForwardAdjust,
}

/// The basic information of securities
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::SecurityStaticInfo")]
pub(crate) struct SecurityStaticInfo {
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
    eps: PyDecimal,
    /// Earnings per share (TTM)
    eps_ttm: PyDecimal,
    /// Net assets per share
    bps: PyDecimal,
    /// Dividend yield
    dividend_yield: PyDecimal,
    /// Types of supported derivatives
    #[py(array)]
    stock_derivatives: Vec<DerivativeType>,
}

/// Quote of US pre/post market
#[pyclass]
#[derive(Debug, PyObject, Copy, Clone)]
#[py(remote = "longbridge::quote::PrePostQuote")]
pub(crate) struct PrePostQuote {
    /// Latest price
    last_done: PyDecimal,
    /// Time of latest price
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// High
    high: PyDecimal,
    /// Low
    low: PyDecimal,
    /// Close of the last trade session
    prev_close: PyDecimal,
}

/// Quote of securitity
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::SecurityQuote")]
pub(crate) struct SecurityQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: PyDecimal,
    /// Yesterday's close
    prev_close: PyDecimal,
    /// Open
    open: PyDecimal,
    /// High
    high: PyDecimal,
    /// Low
    low: PyDecimal,
    /// Time of latest price
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Quote of US pre market
    #[py(opt)]
    pre_market_quote: Option<PrePostQuote>,
    /// Quote of US post market
    #[py(opt)]
    post_market_quote: Option<PrePostQuote>,
}

/// Quote of option
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::OptionQuote")]
pub(crate) struct OptionQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: PyDecimal,
    /// Yesterday's close
    prev_close: PyDecimal,
    /// Open
    open: PyDecimal,
    /// High
    high: PyDecimal,
    /// Low
    low: PyDecimal,
    /// Time of latest price
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Implied volatility
    implied_volatility: PyDecimal,
    /// Number of open positions
    open_interest: i64,
    /// Exprity date
    expiry_date: PyDateWrapper,
    /// Strike price
    strike_price: PyDecimal,
    /// Contract multiplier
    contract_multiplier: PyDecimal,
    /// Option type
    contract_type: OptionType,
    /// Contract size
    contract_size: PyDecimal,
    /// Option direction
    direction: OptionDirection,
    /// Underlying security historical volatility of the option
    historical_volatility: PyDecimal,
    /// Underlying security symbol of the option
    underlying_symbol: String,
}

/// Quote of warrant
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::WarrantQuote")]
pub(crate) struct WarrantQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: PyDecimal,
    /// Yesterday's close
    prev_close: PyDecimal,
    /// Open
    open: PyDecimal,
    /// High
    high: PyDecimal,
    /// Low
    low: PyDecimal,
    /// Time of latest price
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Implied volatility
    implied_volatility: PyDecimal,
    /// Exprity date
    expiry_date: PyDateWrapper,
    /// Last tradalbe date
    last_trade_date: PyDateWrapper,
    /// Outstanding ratio
    outstanding_ratio: PyDecimal,
    /// Outstanding quantity
    outstanding_qty: i64,
    /// Conversion ratio
    conversion_ratio: PyDecimal,
    /// Warrant type
    category: WarrantType,
    /// Strike price
    strike_price: PyDecimal,
    /// Upper bound price
    upper_strike_price: PyDecimal,
    /// Lower bound price
    lower_strike_price: PyDecimal,
    /// Call price
    call_price: PyDecimal,
    /// Underlying security symbol of the warrant
    underlying_symbol: String,
}

/// Depth
#[pyclass]
#[derive(Debug, PyObject, Copy, Clone)]
#[py(remote = "longbridge::quote::Depth")]
pub(crate) struct Depth {
    /// Position
    position: i32,
    /// Price
    price: PyDecimal,
    /// Volume
    volume: i64,
    /// Number of orders
    order_num: i64,
}

/// Security depth
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::SecurityDepth")]
pub(crate) struct SecurityDepth {
    /// Ask depth
    #[py(array)]
    asks: Vec<Depth>,
    /// Bid depth
    #[py(array)]
    bids: Vec<Depth>,
}

/// Brokers
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::quote::Brokers")]
pub(crate) struct Brokers {
    /// Position
    position: i32,
    /// Broker IDs
    broker_ids: Vec<i32>,
}

/// Security brokers
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::SecurityBrokers")]
pub(crate) struct SecurityBrokers {
    /// Ask brokers
    #[py(array)]
    ask_brokers: Vec<Brokers>,
    /// Bid brokers
    #[py(array)]
    bid_brokers: Vec<Brokers>,
}

/// Participant info
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::ParticipantInfo")]
pub(crate) struct ParticipantInfo {
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
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::quote::Trade")]
pub(crate) struct Trade {
    /// Price
    price: PyDecimal,
    /// Volume
    volume: i64,
    /// Time of trading
    timestamp: PyOffsetDateTimeWrapper,
    /// Trade type
    trade_type: String,
    /// Trade direction
    direction: TradeDirection,
    /// Trade session
    trade_session: TradeSession,
}

/// Intraday line
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::IntradayLine")]
pub(crate) struct IntradayLine {
    /// Close price of the minute
    price: PyDecimal,
    /// Start time of the minute
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Average price
    avg_price: PyDecimal,
}

/// Candlestick
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::Candlestick")]
pub(crate) struct Candlestick {
    /// Close price
    close: PyDecimal,
    /// Open price
    open: PyDecimal,
    /// Low price
    low: PyDecimal,
    /// High price
    high: PyDecimal,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Timestamp
    timestamp: PyOffsetDateTimeWrapper,
}

/// Strike price info
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::StrikePriceInfo")]
pub(crate) struct StrikePriceInfo {
    /// Strike price
    price: PyDecimal,
    /// Security code of call option
    call_symbol: String,
    /// Security code of put option
    put_symbol: String,
    /// Is standard
    standard: bool,
}

/// Issuer info
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::IssuerInfo")]
pub(crate) struct IssuerInfo {
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
#[pyclass]
#[derive(Debug, PyObject, Copy, Clone)]
#[py(remote = "longbridge::quote::TradingSessionInfo")]
pub(crate) struct TradingSessionInfo {
    /// Being trading time
    begin_time: PyTimeWrapper,
    /// End trading time
    end_time: PyTimeWrapper,
    /// Trading session
    trade_session: TradeSession,
}

/// Market trading session
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::MarketTradingSession")]
pub(crate) struct MarketTradingSession {
    /// Market
    market: Market,
    /// Trading session
    #[py(array)]
    trade_session: Vec<TradingSessionInfo>,
}

/// Real-time quote
#[pyclass]
#[derive(PyObject, Debug, Clone)]
#[py(remote = "longbridge::quote::RealtimeQuote")]
pub struct RealtimeQuote {
    /// Security code
    symbol: String,
    /// Latest price
    last_done: PyDecimal,
    /// Open
    open: PyDecimal,
    /// High
    high: PyDecimal,
    /// Low
    low: PyDecimal,
    /// Time of latest price
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Security trading status
    trade_status: TradeStatus,
}

/// Push real-time quote
#[pyclass]
#[derive(PyObject)]
#[py(remote = "longbridge::quote::PushQuote")]
#[derive(Debug, Clone)]
pub struct PushQuote {
    /// Latest price
    last_done: PyDecimal,
    /// Open
    open: PyDecimal,
    /// High
    high: PyDecimal,
    /// Low
    low: PyDecimal,
    /// Time of latest price
    timestamp: PyOffsetDateTimeWrapper,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Security trading status
    trade_status: TradeStatus,
    /// Trade session,
    trade_session: TradeSession,
}

/// Push real-time depth
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::PushDepth")]
pub(crate) struct PushDepth {
    /// Ask depth
    #[py(array)]
    asks: Vec<Depth>,
    /// Bid depth
    #[py(array)]
    bids: Vec<Depth>,
}

/// Push real-time brokers
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::PushBrokers")]
pub(crate) struct PushBrokers {
    /// Ask brokers
    #[py(array)]
    ask_brokers: Vec<Brokers>,
    /// Bid brokers
    #[py(array)]
    bid_brokers: Vec<Brokers>,
}

/// Push real-time trades
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::PushTrades")]
pub struct PushTrades {
    /// Trades data
    #[py(array)]
    trades: Vec<Trade>,
}

/// Market trading days
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::MarketTradingDays")]
pub struct MarketTradingDays {
    /// Trading days
    #[py(array)]
    trading_days: Vec<PyDateWrapper>,
    /// Half trading days
    #[py(array)]
    half_trading_days: Vec<PyDateWrapper>,
}
