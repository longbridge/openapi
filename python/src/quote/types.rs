use longbridge::quote::SubFlags;
use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::prelude::*;

use crate::{
    decimal::PyDecimal,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper, PyTimeWrapper},
    types::Market,
};

/// Subscription
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::Subscription")]
pub(crate) struct Subscription {
    symbol: String,
    #[py(sub_types)]
    sub_types: Vec<SubType>,
    #[py(array)]
    candlesticks: Vec<Period>,
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
#[py(remote = "longbridge::quote::Period")]
pub(crate) enum Period {
    /// Unknown
    #[py(remote = "UnknownPeriod")]
    Unknown,
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
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::AdjustType")]
pub(crate) enum AdjustType {
    /// Actual
    NoAdjust,
    /// Adjust forward
    ForwardAdjust,
}

/// Derivative type
#[pyclass]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum DerivativeType {
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

/// Security board
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::SecurityBoard")]
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
    #[py(derivative_types)]
    stock_derivatives: Vec<DerivativeType>,
    /// Board
    board: SecurityBoard,
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
    outstanding_quantity: i64,
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
#[derive(Debug, PyObject, Clone)]
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
    /// Trading sessions
    #[py(array)]
    trade_sessions: Vec<TradingSessionInfo>,
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
pub(crate) struct PushTrades {
    /// Trades data
    #[py(array)]
    trades: Vec<Trade>,
}

/// Push candlestick updated event
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::PushCandlestick")]
pub struct PushCandlestick {
    /// Period type
    period: Period,
    /// Candlestick
    candlestick: Candlestick,
}

/// Market trading days
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::MarketTradingDays")]
pub(crate) struct MarketTradingDays {
    /// Trading days
    #[py(array)]
    trading_days: Vec<PyDateWrapper>,
    /// Half trading days
    #[py(array)]
    half_trading_days: Vec<PyDateWrapper>,
}

/// Capital flow line
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longbridge::quote::CapitalFlowLine")]
pub(crate) struct CapitalFlowLine {
    /// Inflow capital data
    inflow: PyDecimal,
    /// Time
    timestamp: PyOffsetDateTimeWrapper,
}

/// Capital distribution
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::quote::CapitalDistribution")]
pub(crate) struct CapitalDistribution {
    /// Large order
    large: PyDecimal,
    /// Medium order
    medium: PyDecimal,
    /// Small order
    small: PyDecimal,
}

/// Capital distribution response
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::quote::CapitalDistributionResponse")]
pub(crate) struct CapitalDistributionResponse {
    /// Time
    timestamp: PyOffsetDateTimeWrapper,
    /// Inflow capital data
    capital_in: CapitalDistribution,
    /// Outflow capital data
    capital_out: CapitalDistribution,
}

/// Watch list group
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::quote::WatchlistGroup")]
pub(crate) struct WatchlistGroup {
    /// Group id
    pub id: i64,
    /// Group name
    pub name: String,
    /// Securities
    #[py(array)]
    securities: Vec<WatchlistSecurity>,
}

/// Watch list security
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::quote::WatchlistSecurity")]
pub(crate) struct WatchlistSecurity {
    /// Security symbol
    symbol: String,
    /// Market
    market: Market,
    /// Security name
    name: String,
    /// Watched price
    #[py(opt)]
    watched_price: Option<PyDecimal>,
    /// Watched time
    watched_at: PyOffsetDateTimeWrapper,
}

/// Securities update mode
#[pyclass]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::quote::SecuritiesUpdateMode")]
pub(crate) enum SecuritiesUpdateMode {
    /// Add securities
    Add,
    /// Remove securities
    Remove,
    /// Replace securities
    Replace,
}
