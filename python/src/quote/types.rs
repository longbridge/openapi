use longport::quote::SubFlags;
use longport_python_macros::{PyEnum, PyObject};
use pyo3::prelude::*;

use crate::{
    decimal::PyDecimal,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper, PyTimeWrapper},
    types::Market,
};

/// Subscription
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::Subscription")]
pub(crate) struct Subscription {
    symbol: String,
    #[py(sub_types)]
    sub_types: Vec<SubType>,
    #[py(array)]
    candlesticks: Vec<Period>,
}

#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::TradeStatus")]
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
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::TradeSession")]
pub(crate) enum TradeSession {
    /// Trading
    Intraday,
    /// Pre-Market
    Pre,
    /// Post-Market
    Post,
    /// Overnight
    Overnight,
}

/// Quote type of subscription
#[pyclass(eq, eq_int)]
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
            res.push(SubType::Depth);
        }
        if flags.contains(SubFlags::BROKER) {
            res.push(SubType::Brokers);
        }
        if flags.contains(SubFlags::TRADE) {
            res.push(SubType::Trade);
        }
        SubTypes(res)
    }
}

/// Trade direction
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::TradeDirection")]
pub(crate) enum TradeDirection {
    /// Neutral
    Neutral,
    /// Down
    Down,
    /// Up
    Up,
}

/// Option type
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::OptionType")]
pub(crate) enum OptionType {
    /// Unknown
    Unknown,
    /// American
    American,
    /// Europe
    Europe,
}

/// Option direction
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::OptionDirection")]
pub(crate) enum OptionDirection {
    /// Unknown
    Unknown,
    /// Put
    Put,
    /// Call
    Call,
}

/// Warrant type
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::WarrantType")]
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
#[pyclass(eq, eq_int)]
#[allow(non_camel_case_types)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::Period")]
pub(crate) enum Period {
    /// Unknown
    #[py(remote = "UnknownPeriod")]
    Unknown,
    /// One Minute
    #[py(remote = "OneMinute")]
    Min_1,
    /// Two Minutes
    #[py(remote = "TwoMinute")]
    Min_2,
    /// Three Minutes
    #[py(remote = "ThreeMinute")]
    Min_3,
    /// Five Minutes
    #[py(remote = "FiveMinute")]
    Min_5,
    /// Ten Minutes
    #[py(remote = "TenMinute")]
    Min_10,
    /// Fifteen Minutes
    #[py(remote = "FifteenMinute")]
    Min_15,
    /// Twenty Minutes
    #[py(remote = "TwentyMinute")]
    Min_20,
    /// Thirty Minutes
    #[py(remote = "ThirtyMinute")]
    Min_30,
    /// Forty-Five Minutes
    #[py(remote = "FortyFiveMinute")]
    Min_45,
    /// One Hour
    #[py(remote = "SixtyMinute")]
    Min_60,
    /// Two Hours
    #[py(remote = "TwoHour")]
    Min_120,
    /// Three Hours
    #[py(remote = "ThreeHour")]
    Min_180,
    /// Four Hours
    #[py(remote = "FourHour")]
    Min_240,
    /// Daily
    Day,
    /// Weekly
    Week,
    /// Monthly
    Month,
    /// Quarterly
    #[py(remote = "Quarter")]
    Quarter,
    /// Yearly
    Year,
}

/// Candlestick adjustment type
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::AdjustType")]
pub(crate) enum AdjustType {
    /// Actual
    NoAdjust,
    /// Adjust forward
    ForwardAdjust,
}

/// Derivative type
#[pyclass(eq, eq_int)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum DerivativeType {
    /// US stock options
    Option,
    /// HK warrants
    Warrant,
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

/// Security board
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::SecurityBoard")]
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
#[py(remote = "longport::quote::SecurityStaticInfo")]
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
#[py(remote = "longport::quote::PrePostQuote")]
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
#[py(remote = "longport::quote::SecurityQuote")]
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
    /// Quote of US overnight market
    #[py(opt)]
    overnight_quote: Option<PrePostQuote>,
}

/// Quote of option
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::OptionQuote")]
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
#[py(remote = "longport::quote::WarrantQuote")]
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
#[py(remote = "longport::quote::Depth")]
pub(crate) struct Depth {
    /// Position
    position: i32,
    /// Price
    #[py(opt)]
    price: Option<PyDecimal>,
    /// Volume
    volume: i64,
    /// Number of orders
    order_num: i64,
}

/// Security depth
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::SecurityDepth")]
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
#[py(remote = "longport::quote::Brokers")]
pub(crate) struct Brokers {
    /// Position
    position: i32,
    /// Broker IDs
    broker_ids: Vec<i32>,
}

/// Security brokers
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::SecurityBrokers")]
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
#[py(remote = "longport::quote::ParticipantInfo")]
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
#[py(remote = "longport::quote::Trade")]
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
#[py(remote = "longport::quote::IntradayLine")]
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
#[py(remote = "longport::quote::Candlestick")]
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
    /// Trade session
    trade_session: TradeSession,
}

/// Strike price info
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::StrikePriceInfo")]
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
#[py(remote = "longport::quote::IssuerInfo")]
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

/// Sort order type
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::SortOrderType")]
pub enum SortOrderType {
    /// Ascending
    Ascending,
    /// Descending
    Descending,
}

/// Warrant sort by
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::WarrantSortBy")]
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
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::FilterWarrantExpiryDate")]
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
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::FilterWarrantInOutBoundsType")]
pub enum FilterWarrantInOutBoundsType {
    /// In bounds
    In,
    /// Out bounds
    Out,
}

/// Warrant info
#[pyclass]
#[derive(Debug, Clone, PyObject)]
#[py(remote = "longport::quote::WarrantInfo")]
pub(crate) struct WarrantInfo {
    /// Security code
    symbol: String,
    /// Warrant type
    warrant_type: WarrantType,
    /// Security name
    name: String,
    /// Latest price
    last_done: PyDecimal,
    /// Quote change rate
    change_rate: PyDecimal,
    /// Quote change
    change_value: PyDecimal,
    /// Volume
    volume: i64,
    /// Turnover
    turnover: PyDecimal,
    /// Expiry date
    expiry_date: PyDateWrapper,
    /// Strike price
    #[py(opt)]
    strike_price: Option<PyDecimal>,
    /// Upper strike price
    #[py(opt)]
    upper_strike_price: Option<PyDecimal>,
    /// Lower strike price
    #[py(opt)]
    lower_strike_price: Option<PyDecimal>,
    /// Outstanding quantity
    outstanding_qty: i64,
    /// Outstanding ratio
    outstanding_ratio: PyDecimal,
    /// Premium
    premium: PyDecimal,
    /// In/out of the bound
    #[py(opt)]
    itm_otm: Option<PyDecimal>,
    /// Implied volatility
    #[py(opt)]
    implied_volatility: Option<PyDecimal>,
    /// Delta
    #[py(opt)]
    delta: Option<PyDecimal>,
    /// Call price
    #[py(opt)]
    call_price: Option<PyDecimal>,
    /// Price interval from the call price
    #[py(opt)]
    to_call_price: Option<PyDecimal>,
    /// Effective leverage
    #[py(opt)]
    effective_leverage: Option<PyDecimal>,
    /// Leverage ratio
    leverage_ratio: PyDecimal,
    /// Conversion ratio
    #[py(opt)]
    conversion_ratio: Option<PyDecimal>,
    /// Breakeven point
    #[py(opt)]
    balance_point: Option<PyDecimal>,
    /// Status
    status: WarrantStatus,
}

/// Warrant status
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::WarrantStatus")]
pub enum WarrantStatus {
    /// Suspend
    Suspend,
    /// Prepare List
    PrepareList,
    /// Normal
    Normal,
}

/// The information of trading session
#[pyclass]
#[derive(Debug, PyObject, Copy, Clone)]
#[py(remote = "longport::quote::TradingSessionInfo")]
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
#[py(remote = "longport::quote::MarketTradingSession")]
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
#[py(remote = "longport::quote::RealtimeQuote")]
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
#[py(remote = "longport::quote::PushQuote")]
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
    /// Increase volume between pushes
    current_volume: i64,
    /// Increase turnover between pushes
    current_turnover: PyDecimal,
}

/// Push real-time depth
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::PushDepth")]
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
#[py(remote = "longport::quote::PushBrokers")]
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
#[py(remote = "longport::quote::PushTrades")]
pub(crate) struct PushTrades {
    /// Trades data
    #[py(array)]
    trades: Vec<Trade>,
}

/// Push candlestick updated event
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::PushCandlestick")]
pub struct PushCandlestick {
    /// Period type
    period: Period,
    /// Candlestick
    candlestick: Candlestick,
    /// Is confirmed
    is_confirmed: bool,
}

/// Market trading days
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::MarketTradingDays")]
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
#[py(remote = "longport::quote::CapitalFlowLine")]
pub(crate) struct CapitalFlowLine {
    /// Inflow capital data
    inflow: PyDecimal,
    /// Time
    timestamp: PyOffsetDateTimeWrapper,
}

/// Capital distribution
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longport::quote::CapitalDistribution")]
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
#[py(remote = "longport::quote::CapitalDistributionResponse")]
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
#[py(remote = "longport::quote::WatchlistGroup")]
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
#[py(remote = "longport::quote::WatchlistSecurity")]
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
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::SecuritiesUpdateMode")]
pub(crate) enum SecuritiesUpdateMode {
    /// Add securities
    Add,
    /// Remove securities
    Remove,
    /// Replace securities
    Replace,
}

/// Calc index
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::CalcIndex")]
pub(crate) enum CalcIndex {
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
#[pyclass]
#[derive(PyObject, Debug, Clone)]
#[py(remote = "longport::quote::SecurityCalcIndex")]
pub(crate) struct SecurityCalcIndex {
    /// Security code
    symbol: String,
    /// Latest price
    #[py(opt)]
    last_done: Option<PyDecimal>,
    /// Change value
    #[py(opt)]
    change_value: Option<PyDecimal>,
    /// Change ratio
    #[py(opt)]
    change_rate: Option<PyDecimal>,
    /// Volume
    #[py(opt)]
    volume: Option<i64>,
    /// Turnover
    #[py(opt)]
    turnover: Option<PyDecimal>,
    /// Year-to-date change ratio
    #[py(opt)]
    ytd_change_rate: Option<PyDecimal>,
    /// Turnover rate
    #[py(opt)]
    turnover_rate: Option<PyDecimal>,
    /// Total market value
    #[py(opt)]
    total_market_value: Option<PyDecimal>,
    /// Capital flow
    #[py(opt)]
    capital_flow: Option<PyDecimal>,
    /// Amplitude
    #[py(opt)]
    amplitude: Option<PyDecimal>,
    /// Volume ratio
    #[py(opt)]
    volume_ratio: Option<PyDecimal>,
    /// PE (TTM)
    #[py(opt)]
    pe_ttm_ratio: Option<PyDecimal>,
    /// PB
    #[py(opt)]
    pb_ratio: Option<PyDecimal>,
    /// Dividend ratio (TTM)
    #[py(opt)]
    dividend_ratio_ttm: Option<PyDecimal>,
    /// Five days change ratio
    #[py(opt)]
    five_day_change_rate: Option<PyDecimal>,
    /// Ten days change ratio
    #[py(opt)]
    ten_day_change_rate: Option<PyDecimal>,
    /// Half year change ratio
    #[py(opt)]
    half_year_change_rate: Option<PyDecimal>,
    /// Five minutes change ratio
    #[py(opt)]
    five_minutes_change_rate: Option<PyDecimal>,
    /// Expiry date
    #[py(opt)]
    expiry_date: Option<PyDateWrapper>,
    /// Strike price
    #[py(opt)]
    strike_price: Option<PyDecimal>,
    /// Upper bound price
    #[py(opt)]
    upper_strike_price: Option<PyDecimal>,
    /// Lower bound price
    #[py(opt)]
    lower_strike_price: Option<PyDecimal>,
    /// Outstanding quantity
    #[py(opt)]
    outstanding_qty: Option<i64>,
    /// Outstanding ratio
    #[py(opt)]
    outstanding_ratio: Option<PyDecimal>,
    /// Premium
    #[py(opt)]
    premium: Option<PyDecimal>,
    /// In/out of the bound
    #[py(opt)]
    itm_otm: Option<PyDecimal>,
    /// Implied volatility
    #[py(opt)]
    implied_volatility: Option<PyDecimal>,
    /// Warrant delta
    #[py(opt)]
    warrant_delta: Option<PyDecimal>,
    /// Call price
    #[py(opt)]
    call_price: Option<PyDecimal>,
    /// Price interval from the call price
    #[py(opt)]
    to_call_price: Option<PyDecimal>,
    /// Effective leverage
    #[py(opt)]
    effective_leverage: Option<PyDecimal>,
    /// Leverage ratio
    #[py(opt)]
    leverage_ratio: Option<PyDecimal>,
    /// Conversion ratio
    #[py(opt)]
    conversion_ratio: Option<PyDecimal>,
    /// Breakeven point
    #[py(opt)]
    balance_point: Option<PyDecimal>,
    /// Open interest
    #[py(opt)]
    open_interest: Option<i64>,
    /// Delta
    #[py(opt)]
    delta: Option<PyDecimal>,
    /// Gamma
    #[py(opt)]
    gamma: Option<PyDecimal>,
    /// Theta
    #[py(opt)]
    theta: Option<PyDecimal>,
    /// Vega
    #[py(opt)]
    vega: Option<PyDecimal>,
    /// Rho
    #[py(opt)]
    rho: Option<PyDecimal>,
}

/// Security list category
#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::SecurityListCategory")]
pub(crate) enum SecurityListCategory {
    /// Overnight
    Overnight,
}

/// Security
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::Security")]
pub(crate) struct Security {
    /// Security code
    symbol: String,
    /// Security name (zh-CN)
    name_cn: String,
    /// Security name (en)
    name_en: String,
    /// Security name (zh-HK)
    name_hk: String,
}

/// Quote package detail
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::QuotePackageDetail")]
pub(crate) struct QuotePackageDetail {
    /// Key
    pub key: String,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Start time
    pub start_at: PyOffsetDateTimeWrapper,
    /// End time
    pub end_at: PyOffsetDateTimeWrapper,
}

#[pyclass(eq, eq_int)]
#[derive(PyEnum, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::TradeSessions")]
pub(crate) enum TradeSessions {
    Intraday,
    All,
}

/// Market temperature
#[pyclass]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longport::quote::MarketTemperature")]
pub(crate) struct MarketTemperature {
    /// Temperature value
    temperature: i32,
    /// Temperature description
    description: String,
    /// Market valuation
    valuation: i32,
    /// Market sentiment
    sentiment: i32,
    /// Time
    timestamp: PyOffsetDateTimeWrapper,
}

/// Data granularity
#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::quote::Granularity")]
pub(crate) enum Granularity {
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
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longport::quote::HistoryMarketTemperatureResponse")]
pub(crate) struct HistoryMarketTemperatureResponse {
    /// Granularity
    granularity: Granularity,
    /// Records
    #[py(array)]
    records: Vec<MarketTemperature>,
}
