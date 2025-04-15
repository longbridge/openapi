use longport_proto::quote::{self, Period, TradeSession, TradeStatus};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use time::{Date, OffsetDateTime, Time};

use crate::{
    Error, Market, Result,
    quote::{SubFlags, utils::parse_date},
    serde_utils,
};

/// Subscription
#[derive(Debug, Clone)]
pub struct Subscription {
    /// Security code
    pub symbol: String,
    /// Subscription flags
    pub sub_types: SubFlags,
    /// Candlesticks
    pub candlesticks: Vec<Period>,
}

/// Depth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Depth {
    /// Position
    pub position: i32,
    /// Price
    pub price: Option<Decimal>,
    /// Volume
    pub volume: i64,
    /// Number of orders
    pub order_num: i64,
}

impl TryFrom<quote::Depth> for Depth {
    type Error = Error;

    fn try_from(depth: quote::Depth) -> Result<Self> {
        Ok(Self {
            position: depth.position,
            price: depth.price.parse().ok(),
            volume: depth.volume,
            order_num: depth.order_num,
        })
    }
}

/// Brokers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brokers {
    /// Position
    pub position: i32,
    /// Broker IDs
    pub broker_ids: Vec<i32>,
}

impl From<quote::Brokers> for Brokers {
    fn from(brokers: quote::Brokers) -> Self {
        Self {
            position: brokers.position,
            broker_ids: brokers.broker_ids,
        }
    }
}

/// Trade direction
#[derive(Debug, FromPrimitive, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[repr(i32)]
pub enum TradeDirection {
    /// Neutral
    #[num_enum(default)]
    Neutral = 0,
    /// Down
    Down = 1,
    /// Up
    Up = 2,
}

/// Trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Price
    pub price: Decimal,
    /// Volume
    pub volume: i64,
    /// Time of trading
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Trade type
    ///
    /// HK
    ///
    /// - `*` - Overseas trade
    /// - `D` - Odd-lot trade
    /// - `M` - Non-direct off-exchange trade
    /// - `P` - Late trade (Off-exchange previous day)
    /// - `U` - Auction trade
    /// - `X` - Direct off-exchange trade
    /// - `Y` - Automatch internalized
    /// - `<empty string>` -  Automatch normal
    ///
    /// US
    ///
    /// - `<empty string>` - Regular sale
    /// - `A` - Acquisition
    /// - `B` - Bunched trade
    /// - `D` - Distribution
    /// - `F` - Intermarket sweep
    /// - `G` - Bunched sold trades
    /// - `H` - Price variation trade
    /// - `I` - Odd lot trade
    /// - `K` - Rule 155 trde(NYSE MKT)
    /// - `M` - Market center close price
    /// - `P` - Prior reference price
    /// - `Q` - Market center open price
    /// - `S` - Split trade
    /// - `V` - Contingent trade
    /// - `W` - Average price trade
    /// - `X` - Cross trade
    /// - `1` - Stopped stock(Regular trade)
    pub trade_type: String,
    /// Trade direction
    pub direction: TradeDirection,
    /// Trade session
    pub trade_session: TradeSession,
}

impl TryFrom<quote::Trade> for Trade {
    type Error = Error;

    fn try_from(trade: quote::Trade) -> Result<Self> {
        Ok(Self {
            price: trade.price.parse().unwrap_or_default(),
            volume: trade.volume,
            timestamp: OffsetDateTime::from_unix_timestamp(trade.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            trade_type: trade.trade_type,
            direction: trade.direction.into(),
            trade_session: TradeSession::try_from(trade.trade_session).unwrap_or_default(),
        })
    }
}

bitflags::bitflags! {
    /// Derivative type
    #[derive(Debug, Copy, Clone, Serialize,Deserialize)]
    pub struct DerivativeType: u8 {
        /// US stock options
        const OPTION = 0x1;

        /// HK warrants
        const WARRANT = 0x2;
    }
}

/// Security board
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityStaticInfo {
    /// Security code
    pub symbol: String,
    /// Security name (zh-CN)
    pub name_cn: String,
    /// Security name (en)
    pub name_en: String,
    /// Security name (zh-HK)
    pub name_hk: String,
    /// Exchange which the security belongs to
    pub exchange: String,
    /// Trading currency
    pub currency: String,
    /// Lot size
    pub lot_size: i32,
    /// Total shares
    pub total_shares: i64,
    /// Circulating shares
    pub circulating_shares: i64,
    /// HK shares (only HK stocks)
    pub hk_shares: i64,
    /// Earnings per share
    pub eps: Decimal,
    /// Earnings per share (TTM)
    pub eps_ttm: Decimal,
    /// Net assets per share
    pub bps: Decimal,
    /// Dividend yield
    pub dividend_yield: Decimal,
    /// Types of supported derivatives
    pub stock_derivatives: DerivativeType,
    /// Board
    pub board: SecurityBoard,
}

impl TryFrom<quote::StaticInfo> for SecurityStaticInfo {
    type Error = Error;

    fn try_from(resp: quote::StaticInfo) -> Result<Self> {
        Ok(SecurityStaticInfo {
            symbol: resp.symbol,
            name_cn: resp.name_cn,
            name_en: resp.name_en,
            name_hk: resp.name_hk,
            exchange: resp.exchange,
            currency: resp.currency,
            lot_size: resp.lot_size,
            total_shares: resp.total_shares,
            circulating_shares: resp.circulating_shares,
            hk_shares: resp.hk_shares,
            eps: resp.eps.parse().unwrap_or_default(),
            eps_ttm: resp.eps_ttm.parse().unwrap_or_default(),
            bps: resp.bps.parse().unwrap_or_default(),
            dividend_yield: resp.dividend_yield.parse().unwrap_or_default(),
            stock_derivatives: resp.stock_derivatives.into_iter().fold(
                DerivativeType::empty(),
                |acc, value| match value {
                    1 => acc | DerivativeType::OPTION,
                    2 => acc | DerivativeType::WARRANT,
                    _ => acc,
                },
            ),
            board: resp.board.parse().unwrap_or(SecurityBoard::Unknown),
        })
    }
}

/// Real-time quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeQuote {
    /// Security code
    pub symbol: String,
    /// Latest price
    pub last_done: Decimal,
    /// Open
    pub open: Decimal,
    /// High
    pub high: Decimal,
    /// Low
    pub low: Decimal,
    /// Time of latest price
    pub timestamp: OffsetDateTime,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Security trading status
    pub trade_status: TradeStatus,
}

/// Quote of US pre/post market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrePostQuote {
    /// Latest price
    pub last_done: Decimal,
    /// Time of latest price
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// High
    pub high: Decimal,
    /// Low
    pub low: Decimal,
    /// Close of the last trade session
    pub prev_close: Decimal,
}

impl TryFrom<quote::PrePostQuote> for PrePostQuote {
    type Error = Error;

    fn try_from(quote: quote::PrePostQuote) -> Result<Self> {
        Ok(Self {
            last_done: quote.last_done.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(quote.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            volume: quote.volume,
            turnover: quote.turnover.parse().unwrap_or_default(),
            high: quote.high.parse().unwrap_or_default(),
            low: quote.low.parse().unwrap_or_default(),
            prev_close: quote.prev_close.parse().unwrap_or_default(),
        })
    }
}

/// Quote of securitity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuote {
    /// Security code
    pub symbol: String,
    /// Latest price
    pub last_done: Decimal,
    /// Yesterday's close
    pub prev_close: Decimal,
    /// Open
    pub open: Decimal,
    /// High
    pub high: Decimal,
    /// Low
    pub low: Decimal,
    /// Time of latest price
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Security trading status
    pub trade_status: TradeStatus,
    /// Quote of US pre market
    pub pre_market_quote: Option<PrePostQuote>,
    /// Quote of US post market
    pub post_market_quote: Option<PrePostQuote>,
    /// Quote of US overnight market
    pub overnight_quote: Option<PrePostQuote>,
}

impl TryFrom<quote::SecurityQuote> for SecurityQuote {
    type Error = Error;

    fn try_from(quote: quote::SecurityQuote) -> Result<Self> {
        Ok(Self {
            symbol: quote.symbol,
            last_done: quote.last_done.parse().unwrap_or_default(),
            prev_close: quote.prev_close.parse().unwrap_or_default(),
            open: quote.open.parse().unwrap_or_default(),
            high: quote.high.parse().unwrap_or_default(),
            low: quote.low.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(quote.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            volume: quote.volume,
            turnover: quote.turnover.parse().unwrap_or_default(),
            trade_status: TradeStatus::try_from(quote.trade_status).unwrap_or_default(),
            pre_market_quote: quote.pre_market_quote.map(TryInto::try_into).transpose()?,
            post_market_quote: quote.post_market_quote.map(TryInto::try_into).transpose()?,
            overnight_quote: quote.over_night_quote.map(TryInto::try_into).transpose()?,
        })
    }
}

/// Option type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Serialize, Deserialize)]
pub enum OptionType {
    /// Unknown
    Unknown,
    /// American
    #[strum(serialize = "A")]
    American,
    /// Europe
    #[strum(serialize = "U")]
    Europe,
}

/// Option direction
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Serialize, Deserialize)]
pub enum OptionDirection {
    /// Unknown
    Unknown,
    /// Put
    #[strum(serialize = "P")]
    Put,
    /// Call
    #[strum(serialize = "C")]
    Call,
}

/// Quote of option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionQuote {
    /// Security code
    pub symbol: String,
    /// Latest price
    pub last_done: Decimal,
    /// Yesterday's close
    pub prev_close: Decimal,
    /// Open
    pub open: Decimal,
    /// High
    pub high: Decimal,
    /// Low
    pub low: Decimal,
    /// Time of latest price
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Security trading status
    pub trade_status: TradeStatus,
    /// Implied volatility
    pub implied_volatility: Decimal,
    /// Number of open positions
    pub open_interest: i64,
    /// Exprity date
    pub expiry_date: Date,
    /// Strike price
    pub strike_price: Decimal,
    /// Contract multiplier
    pub contract_multiplier: Decimal,
    /// Option type
    pub contract_type: OptionType,
    /// Contract size
    pub contract_size: Decimal,
    /// Option direction
    pub direction: OptionDirection,
    /// Underlying security historical volatility of the option
    pub historical_volatility: Decimal,
    /// Underlying security symbol of the option
    pub underlying_symbol: String,
}

impl TryFrom<quote::OptionQuote> for OptionQuote {
    type Error = Error;

    fn try_from(quote: quote::OptionQuote) -> Result<Self> {
        let option_extend = quote.option_extend.unwrap_or_default();

        Ok(Self {
            symbol: quote.symbol,
            last_done: quote.last_done.parse().unwrap_or_default(),
            prev_close: quote.prev_close.parse().unwrap_or_default(),
            open: quote.open.parse().unwrap_or_default(),
            high: quote.high.parse().unwrap_or_default(),
            low: quote.low.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(quote.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            volume: quote.volume,
            turnover: quote.turnover.parse().unwrap_or_default(),
            trade_status: TradeStatus::try_from(quote.trade_status).unwrap_or_default(),
            implied_volatility: option_extend.implied_volatility.parse().unwrap_or_default(),
            open_interest: option_extend.open_interest,
            expiry_date: parse_date(&option_extend.expiry_date)
                .map_err(|err| Error::parse_field_error("expiry_date", err))?,
            strike_price: option_extend.strike_price.parse().unwrap_or_default(),
            contract_multiplier: option_extend
                .contract_multiplier
                .parse()
                .unwrap_or_default(),
            contract_type: option_extend.contract_type.parse().unwrap_or_default(),
            contract_size: option_extend.contract_size.parse().unwrap_or_default(),
            direction: option_extend.direction.parse().unwrap_or_default(),
            historical_volatility: option_extend
                .historical_volatility
                .parse()
                .unwrap_or_default(),
            underlying_symbol: option_extend.underlying_symbol,
        })
    }
}

/// Warrant type
#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    EnumString,
    IntoPrimitive,
    TryFromPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(i32)]
pub enum WarrantType {
    /// Unknown
    Unknown = -1,
    /// Call
    Call = 0,
    /// Put
    Put = 1,
    /// Bull
    Bull = 2,
    /// Bear
    Bear = 3,
    /// Inline
    Inline = 4,
}

/// Quote of warrant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantQuote {
    /// Security code
    pub symbol: String,
    /// Latest price
    pub last_done: Decimal,
    /// Yesterday's close
    pub prev_close: Decimal,
    /// Open
    pub open: Decimal,
    /// High
    pub high: Decimal,
    /// Low
    pub low: Decimal,
    /// Time of latest price
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Security trading status
    pub trade_status: TradeStatus,
    /// Implied volatility
    pub implied_volatility: Decimal,
    /// Exprity date
    pub expiry_date: Date,
    /// Last tradalbe date
    pub last_trade_date: Date,
    /// Outstanding ratio
    pub outstanding_ratio: Decimal,
    /// Outstanding quantity
    pub outstanding_quantity: i64,
    /// Conversion ratio
    pub conversion_ratio: Decimal,
    /// Warrant type
    pub category: WarrantType,
    /// Strike price
    pub strike_price: Decimal,
    /// Upper bound price
    pub upper_strike_price: Decimal,
    /// Lower bound price
    pub lower_strike_price: Decimal,
    /// Call price
    pub call_price: Decimal,
    /// Underlying security symbol of the warrant
    pub underlying_symbol: String,
}

impl TryFrom<quote::WarrantQuote> for WarrantQuote {
    type Error = Error;

    fn try_from(quote: quote::WarrantQuote) -> Result<Self> {
        let warrant_extend = quote.warrant_extend.unwrap_or_default();

        Ok(Self {
            symbol: quote.symbol,
            last_done: quote.last_done.parse().unwrap_or_default(),
            prev_close: quote.prev_close.parse().unwrap_or_default(),
            open: quote.open.parse().unwrap_or_default(),
            high: quote.high.parse().unwrap_or_default(),
            low: quote.low.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(quote.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            volume: quote.volume,
            turnover: quote.turnover.parse().unwrap_or_default(),
            trade_status: TradeStatus::try_from(quote.trade_status).unwrap_or_default(),
            implied_volatility: warrant_extend
                .implied_volatility
                .parse()
                .unwrap_or_default(),
            expiry_date: parse_date(&warrant_extend.expiry_date)
                .map_err(|err| Error::parse_field_error("expiry_date", err))?,
            last_trade_date: parse_date(&warrant_extend.last_trade_date)
                .map_err(|err| Error::parse_field_error("last_trade_date", err))?,
            outstanding_ratio: warrant_extend.outstanding_ratio.parse().unwrap_or_default(),
            outstanding_quantity: warrant_extend.outstanding_qty,
            conversion_ratio: warrant_extend.conversion_ratio.parse().unwrap_or_default(),
            category: warrant_extend.category.parse().unwrap_or_default(),
            strike_price: warrant_extend.strike_price.parse().unwrap_or_default(),
            upper_strike_price: warrant_extend
                .upper_strike_price
                .parse()
                .unwrap_or_default(),
            lower_strike_price: warrant_extend
                .lower_strike_price
                .parse()
                .unwrap_or_default(),
            call_price: warrant_extend.call_price.parse().unwrap_or_default(),
            underlying_symbol: warrant_extend.underlying_symbol,
        })
    }
}

/// Security depth
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityDepth {
    /// Ask depth
    pub asks: Vec<Depth>,
    /// Bid depth
    pub bids: Vec<Depth>,
}

/// Security brokers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityBrokers {
    /// Ask brokers
    pub ask_brokers: Vec<Brokers>,
    /// Bid brokers
    pub bid_brokers: Vec<Brokers>,
}

/// Participant info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantInfo {
    /// Broker IDs
    pub broker_ids: Vec<i32>,
    /// Participant name (zh-CN)
    pub name_cn: String,
    /// Participant name (en)
    pub name_en: String,
    /// Participant name (zh-HK)
    pub name_hk: String,
}

impl From<quote::ParticipantInfo> for ParticipantInfo {
    fn from(info: quote::ParticipantInfo) -> Self {
        Self {
            broker_ids: info.broker_ids,
            name_cn: info.participant_name_cn,
            name_en: info.participant_name_en,
            name_hk: info.participant_name_hk,
        }
    }
}

/// Intraday line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntradayLine {
    /// Close price of the minute
    pub price: Decimal,
    /// Start time of the minute
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Average price
    pub avg_price: Decimal,
}

impl TryFrom<quote::Line> for IntradayLine {
    type Error = Error;

    fn try_from(value: quote::Line) -> Result<Self> {
        Ok(Self {
            price: value.price.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            volume: value.volume,
            turnover: value.turnover.parse().unwrap_or_default(),
            avg_price: value.avg_price.parse().unwrap_or_default(),
        })
    }
}

/// Candlestick
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    /// Close price
    pub close: Decimal,
    /// Open price
    pub open: Decimal,
    /// Low price
    pub low: Decimal,
    /// High price
    pub high: Decimal,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Timestamp
    #[serde(with = "serde_utils::timestamp")]
    pub timestamp: OffsetDateTime,
    /// Trade session
    pub trade_session: TradeSession,
}

impl TryFrom<quote::Candlestick> for Candlestick {
    type Error = Error;

    fn try_from(value: quote::Candlestick) -> Result<Self> {
        Ok(Self {
            close: value.close.parse().unwrap_or_default(),
            open: value.open.parse().unwrap_or_default(),
            low: value.low.parse().unwrap_or_default(),
            high: value.high.parse().unwrap_or_default(),
            volume: value.volume,
            turnover: value.turnover.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            trade_session: TradeSession::try_from(value.trade_session)
                .map_err(|err| Error::parse_field_error("trade_session", err))?,
        })
    }
}

impl From<(longport_candlesticks::Candlestick, TradeSession)> for Candlestick {
    #[inline]
    fn from(
        (candlestick, trade_session): (longport_candlesticks::Candlestick, TradeSession),
    ) -> Self {
        Self {
            close: candlestick.close,
            open: candlestick.open,
            low: candlestick.low,
            high: candlestick.high,
            volume: candlestick.volume,
            turnover: candlestick.turnover,
            timestamp: candlestick.time,
            trade_session,
        }
    }
}

impl From<Candlestick> for longport_candlesticks::Candlestick {
    #[inline]
    fn from(candlestick: Candlestick) -> Self {
        Self {
            time: candlestick.timestamp,
            open: candlestick.open,
            high: candlestick.high,
            low: candlestick.low,
            close: candlestick.close,
            volume: candlestick.volume,
            turnover: candlestick.turnover,
        }
    }
}

/// Strike price info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrikePriceInfo {
    /// Strike price
    pub price: Decimal,
    /// Security code of call option
    pub call_symbol: String,
    /// Security code of put option
    pub put_symbol: String,
    /// Is standard
    pub standard: bool,
}

impl TryFrom<quote::StrikePriceInfo> for StrikePriceInfo {
    type Error = Error;

    fn try_from(value: quote::StrikePriceInfo) -> Result<Self> {
        Ok(Self {
            price: value.price.parse().unwrap_or_default(),
            call_symbol: value.call_symbol,
            put_symbol: value.put_symbol,
            standard: value.standard,
        })
    }
}

/// Issuer info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuerInfo {
    /// Issuer ID
    pub issuer_id: i32,
    /// Issuer name (zh-CN)
    pub name_cn: String,
    /// Issuer name (en)
    pub name_en: String,
    /// Issuer name (zh-HK)
    pub name_hk: String,
}

impl From<quote::IssuerInfo> for IssuerInfo {
    fn from(info: quote::IssuerInfo) -> Self {
        Self {
            issuer_id: info.id,
            name_cn: info.name_cn,
            name_en: info.name_en,
            name_hk: info.name_hk,
        }
    }
}

/// Sort order type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
pub enum SortOrderType {
    /// Ascending
    Ascending = 0,
    /// Descending
    Descending = 1,
}

/// Warrant sort by
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
pub enum WarrantSortBy {
    /// Last done
    LastDone = 0,
    /// Change rate
    ChangeRate = 1,
    /// Change value
    ChangeValue = 2,
    /// Volume
    Volume = 3,
    /// Turnover
    Turnover = 4,
    /// Expiry date
    ExpiryDate = 5,
    /// Strike price
    StrikePrice = 6,
    /// Upper strike price
    UpperStrikePrice = 7,
    /// Lower strike price
    LowerStrikePrice = 8,
    /// Outstanding quantity
    OutstandingQuantity = 9,
    /// Outstanding ratio
    OutstandingRatio = 10,
    /// Premium
    Premium = 11,
    /// In/out of the bound
    ItmOtm = 12,
    /// Implied volatility
    ImpliedVolatility = 13,
    /// Greek value Delta
    Delta = 14,
    /// Call price
    CallPrice = 15,
    /// Price interval from the call price
    ToCallPrice = 16,
    /// Effective leverage
    EffectiveLeverage = 17,
    /// Leverage ratio
    LeverageRatio = 18,
    /// Conversion ratio
    ConversionRatio = 19,
    /// Breakeven point
    BalancePoint = 20,
    /// Status
    Status = 21,
}

/// Filter warrant expiry date type
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
pub enum FilterWarrantExpiryDate {
    /// Less than 3 months
    LT_3 = 1,
    /// 3 - 6 months
    Between_3_6 = 2,
    /// 6 - 12 months
    Between_6_12 = 3,
    /// Greater than 12 months
    GT_12 = 4,
}

/// Filter warrant in/out of the bounds type
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
pub enum FilterWarrantInOutBoundsType {
    /// In bounds
    In = 1,
    /// Out bounds
    Out = 2,
}

/// Warrant status
#[derive(
    Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum WarrantStatus {
    /// Suspend
    Suspend = 2,
    /// Prepare List
    PrepareList = 3,
    /// Normal
    Normal = 4,
}

/// Warrant info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantInfo {
    /// Security code
    pub symbol: String,
    /// Warrant type
    pub warrant_type: WarrantType,
    /// Security name
    pub name: String,
    /// Latest price
    pub last_done: Decimal,
    /// Quote change rate
    pub change_rate: Decimal,
    /// Quote change
    pub change_value: Decimal,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: Decimal,
    /// Expiry date
    pub expiry_date: Date,
    /// Strike price
    pub strike_price: Option<Decimal>,
    /// Upper strike price
    pub upper_strike_price: Option<Decimal>,
    /// Lower strike price
    pub lower_strike_price: Option<Decimal>,
    /// Outstanding quantity
    pub outstanding_qty: i64,
    /// Outstanding ratio
    pub outstanding_ratio: Decimal,
    /// Premium
    pub premium: Decimal,
    /// In/out of the bound
    pub itm_otm: Option<Decimal>,
    /// Implied volatility
    pub implied_volatility: Option<Decimal>,
    /// Delta
    pub delta: Option<Decimal>,
    /// Call price
    pub call_price: Option<Decimal>,
    /// Price interval from the call price
    pub to_call_price: Option<Decimal>,
    /// Effective leverage
    pub effective_leverage: Option<Decimal>,
    /// Leverage ratio
    pub leverage_ratio: Decimal,
    /// Conversion ratio
    pub conversion_ratio: Option<Decimal>,
    /// Breakeven point
    pub balance_point: Option<Decimal>,
    /// Status
    pub status: WarrantStatus,
}

impl TryFrom<quote::FilterWarrant> for WarrantInfo {
    type Error = Error;

    fn try_from(info: quote::FilterWarrant) -> Result<Self> {
        let r#type = WarrantType::try_from(info.r#type)
            .map_err(|err| Error::parse_field_error("type", err))?;

        match r#type {
            WarrantType::Unknown => unreachable!(),
            WarrantType::Call | WarrantType::Put => Ok(Self {
                symbol: info.symbol,
                warrant_type: r#type,
                name: info.name,
                last_done: info.last_done.parse().unwrap_or_default(),
                change_rate: info.change_rate.parse().unwrap_or_default(),
                change_value: info.change_val.parse().unwrap_or_default(),
                volume: info.volume,
                turnover: info.turnover.parse().unwrap_or_default(),
                expiry_date: parse_date(&info.expiry_date)
                    .map_err(|err| Error::parse_field_error("expiry_date", err))?,
                strike_price: Some(info.last_done.parse().unwrap_or_default()),
                upper_strike_price: None,
                lower_strike_price: None,
                outstanding_qty: info.outstanding_qty.parse().unwrap_or_default(),
                outstanding_ratio: info.outstanding_ratio.parse().unwrap_or_default(),
                premium: info.premium.parse().unwrap_or_default(),
                itm_otm: Some(info.last_done.parse().unwrap_or_default()),
                implied_volatility: Some(info.last_done.parse().unwrap_or_default()),
                delta: Some(info.last_done.parse().unwrap_or_default()),
                call_price: None,
                to_call_price: None,
                effective_leverage: Some(info.last_done.parse().unwrap_or_default()),
                leverage_ratio: info.leverage_ratio.parse().unwrap_or_default(),
                conversion_ratio: Some(info.last_done.parse().unwrap_or_default()),
                balance_point: Some(info.last_done.parse().unwrap_or_default()),
                status: WarrantStatus::try_from(info.status)
                    .map_err(|err| Error::parse_field_error("state", err))?,
            }),
            WarrantType::Bull | WarrantType::Bear => Ok(Self {
                symbol: info.symbol,
                warrant_type: r#type,
                name: info.name,
                last_done: info.last_done.parse().unwrap_or_default(),
                change_rate: info.change_rate.parse().unwrap_or_default(),
                change_value: info.change_val.parse().unwrap_or_default(),
                volume: info.volume,
                turnover: info.turnover.parse().unwrap_or_default(),
                expiry_date: parse_date(&info.expiry_date)
                    .map_err(|err| Error::parse_field_error("expiry_date", err))?,
                strike_price: Some(info.last_done.parse().unwrap_or_default()),
                upper_strike_price: None,
                lower_strike_price: None,
                outstanding_qty: info.outstanding_qty.parse().unwrap_or_default(),
                outstanding_ratio: info.outstanding_ratio.parse().unwrap_or_default(),
                premium: info.premium.parse().unwrap_or_default(),
                itm_otm: Some(info.last_done.parse().unwrap_or_default()),
                implied_volatility: None,
                delta: None,
                call_price: Some(info.call_price.parse().unwrap_or_default()),
                to_call_price: Some(info.to_call_price.parse().unwrap_or_default()),
                effective_leverage: None,
                leverage_ratio: info.leverage_ratio.parse().unwrap_or_default(),
                conversion_ratio: Some(info.last_done.parse().unwrap_or_default()),
                balance_point: Some(info.last_done.parse().unwrap_or_default()),
                status: WarrantStatus::try_from(info.status)
                    .map_err(|err| Error::parse_field_error("state", err))?,
            }),
            WarrantType::Inline => Ok(Self {
                symbol: info.symbol,
                warrant_type: r#type,
                name: info.name,
                last_done: info.last_done.parse().unwrap_or_default(),
                change_rate: info.change_rate.parse().unwrap_or_default(),
                change_value: info.change_val.parse().unwrap_or_default(),
                volume: info.volume,
                turnover: info.turnover.parse().unwrap_or_default(),
                expiry_date: parse_date(&info.expiry_date)
                    .map_err(|err| Error::parse_field_error("expiry_date", err))?,
                strike_price: None,
                upper_strike_price: Some(info.upper_strike_price.parse().unwrap_or_default()),
                lower_strike_price: Some(info.lower_strike_price.parse().unwrap_or_default()),
                outstanding_qty: info.outstanding_qty.parse().unwrap_or_default(),
                outstanding_ratio: info.outstanding_ratio.parse().unwrap_or_default(),
                premium: info.premium.parse().unwrap_or_default(),
                itm_otm: None,
                implied_volatility: None,
                delta: None,
                call_price: None,
                to_call_price: None,
                effective_leverage: None,
                leverage_ratio: info.leverage_ratio.parse().unwrap_or_default(),
                conversion_ratio: None,
                balance_point: None,
                status: WarrantStatus::try_from(info.status)
                    .map_err(|err| Error::parse_field_error("state", err))?,
            }),
        }
    }
}

/// The information of trading session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSessionInfo {
    /// Being trading time
    pub begin_time: Time,
    /// End trading time
    pub end_time: Time,
    /// Trading session
    pub trade_session: TradeSession,
}

impl TryFrom<quote::TradePeriod> for TradingSessionInfo {
    type Error = Error;

    fn try_from(value: quote::TradePeriod) -> Result<Self> {
        #[inline]
        fn parse_time(value: i32) -> ::std::result::Result<Time, time::error::ComponentRange> {
            Time::from_hms(((value / 100) % 100) as u8, (value % 100) as u8, 0)
        }

        Ok(Self {
            begin_time: parse_time(value.beg_time)
                .map_err(|err| Error::parse_field_error("beg_time", err))?,
            end_time: parse_time(value.end_time)
                .map_err(|err| Error::parse_field_error("end_time", err))?,
            trade_session: TradeSession::try_from(value.trade_session).unwrap_or_default(),
        })
    }
}

/// Market trading session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTradingSession {
    /// Market
    pub market: Market,
    /// Trading session
    pub trade_sessions: Vec<TradingSessionInfo>,
}

impl TryFrom<quote::MarketTradePeriod> for MarketTradingSession {
    type Error = Error;

    fn try_from(value: quote::MarketTradePeriod) -> Result<Self> {
        Ok(Self {
            market: value.market.parse().unwrap_or_default(),
            trade_sessions: value
                .trade_session
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

/// Market trading days
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTradingDays {
    /// Trading days
    pub trading_days: Vec<Date>,
    /// Half trading days
    pub half_trading_days: Vec<Date>,
}

/// Capital flow line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalFlowLine {
    /// Inflow capital data
    pub inflow: Decimal,
    /// Time
    pub timestamp: OffsetDateTime,
}

impl TryFrom<quote::capital_flow_intraday_response::CapitalFlowLine> for CapitalFlowLine {
    type Error = Error;

    fn try_from(value: quote::capital_flow_intraday_response::CapitalFlowLine) -> Result<Self> {
        Ok(Self {
            inflow: value.inflow.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
        })
    }
}

/// Capital distribution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapitalDistribution {
    /// Large order
    pub large: Decimal,
    /// Medium order
    pub medium: Decimal,
    /// Small order
    pub small: Decimal,
}

impl TryFrom<quote::capital_distribution_response::CapitalDistribution> for CapitalDistribution {
    type Error = Error;

    fn try_from(value: quote::capital_distribution_response::CapitalDistribution) -> Result<Self> {
        Ok(Self {
            large: value.large.parse().unwrap_or_default(),
            medium: value.medium.parse().unwrap_or_default(),
            small: value.small.parse().unwrap_or_default(),
        })
    }
}

/// Capital distribution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalDistributionResponse {
    /// Time
    pub timestamp: OffsetDateTime,
    /// Inflow capital data
    pub capital_in: CapitalDistribution,
    /// Outflow capital data
    pub capital_out: CapitalDistribution,
}

impl TryFrom<quote::CapitalDistributionResponse> for CapitalDistributionResponse {
    type Error = Error;

    fn try_from(value: quote::CapitalDistributionResponse) -> Result<Self> {
        Ok(Self {
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            capital_in: value
                .capital_in
                .map(TryInto::try_into)
                .transpose()?
                .unwrap_or_default(),
            capital_out: value
                .capital_out
                .map(TryInto::try_into)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}

/// Watchlist security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistSecurity {
    /// Security symbol
    pub symbol: String,
    /// Market
    pub market: Market,
    /// Security name
    pub name: String,
    /// Watched price
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub watched_price: Option<Decimal>,
    /// Watched time
    #[serde(with = "serde_utils::timestamp")]
    pub watched_at: OffsetDateTime,
}

/// Watchlist group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistGroup {
    /// Group id
    #[serde(with = "serde_utils::int64_str")]
    pub id: i64,
    /// Group name
    pub name: String,
    /// Securities
    pub securities: Vec<WatchlistSecurity>,
}

impl_default_for_enum_string!(OptionType, OptionDirection, WarrantType, SecurityBoard);

/// An request for create watchlist group
#[derive(Debug, Clone)]
pub struct RequestCreateWatchlistGroup {
    /// Group name
    pub name: String,
    /// Securities
    pub securities: Option<Vec<String>>,
}

impl RequestCreateWatchlistGroup {
    /// Create a new request for create watchlist group
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            securities: None,
        }
    }

    /// Set securities to the request
    pub fn securities<I, T>(self, securities: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            securities: Some(securities.into_iter().map(Into::into).collect()),
            ..self
        }
    }
}

/// Securities update mode
#[derive(Debug, Copy, Clone, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SecuritiesUpdateMode {
    /// Add securities
    Add,
    /// Remove securities
    Remove,
    /// Replace securities
    #[default]
    Replace,
}

/// An request for update watchlist group
#[derive(Debug, Clone)]
pub struct RequestUpdateWatchlistGroup {
    /// Group id
    pub id: i64,
    /// Group name
    pub name: Option<String>,
    /// Securities
    pub securities: Option<Vec<String>>,
    /// Securities Update mode
    pub mode: SecuritiesUpdateMode,
}

impl RequestUpdateWatchlistGroup {
    /// Create a new request for update watchlist group
    #[inline]
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: None,
            securities: None,
            mode: SecuritiesUpdateMode::default(),
        }
    }

    /// Set group name to the request
    pub fn name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }

    /// Set securities to the request
    pub fn securities<I, T>(self, securities: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            securities: Some(securities.into_iter().map(Into::into).collect()),
            ..self
        }
    }

    /// Set securities update mode to the request
    pub fn mode(self, mode: SecuritiesUpdateMode) -> Self {
        Self { mode, ..self }
    }
}

/// Calc index
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl From<CalcIndex> for longport_proto::quote::CalcIndex {
    fn from(value: CalcIndex) -> Self {
        use longport_proto::quote::CalcIndex::*;

        match value {
            CalcIndex::LastDone => CalcindexLastDone,
            CalcIndex::ChangeValue => CalcindexChangeVal,
            CalcIndex::ChangeRate => CalcindexChangeRate,
            CalcIndex::Volume => CalcindexVolume,
            CalcIndex::Turnover => CalcindexTurnover,
            CalcIndex::YtdChangeRate => CalcindexYtdChangeRate,
            CalcIndex::TurnoverRate => CalcindexTurnoverRate,
            CalcIndex::TotalMarketValue => CalcindexTotalMarketValue,
            CalcIndex::CapitalFlow => CalcindexCapitalFlow,
            CalcIndex::Amplitude => CalcindexAmplitude,
            CalcIndex::VolumeRatio => CalcindexVolumeRatio,
            CalcIndex::PeTtmRatio => CalcindexPeTtmRatio,
            CalcIndex::PbRatio => CalcindexPbRatio,
            CalcIndex::DividendRatioTtm => CalcindexDividendRatioTtm,
            CalcIndex::FiveDayChangeRate => CalcindexFiveDayChangeRate,
            CalcIndex::TenDayChangeRate => CalcindexTenDayChangeRate,
            CalcIndex::HalfYearChangeRate => CalcindexHalfYearChangeRate,
            CalcIndex::FiveMinutesChangeRate => CalcindexFiveMinutesChangeRate,
            CalcIndex::ExpiryDate => CalcindexExpiryDate,
            CalcIndex::StrikePrice => CalcindexStrikePrice,
            CalcIndex::UpperStrikePrice => CalcindexUpperStrikePrice,
            CalcIndex::LowerStrikePrice => CalcindexLowerStrikePrice,
            CalcIndex::OutstandingQty => CalcindexOutstandingQty,
            CalcIndex::OutstandingRatio => CalcindexOutstandingRatio,
            CalcIndex::Premium => CalcindexPremium,
            CalcIndex::ItmOtm => CalcindexItmOtm,
            CalcIndex::ImpliedVolatility => CalcindexImpliedVolatility,
            CalcIndex::WarrantDelta => CalcindexWarrantDelta,
            CalcIndex::CallPrice => CalcindexCallPrice,
            CalcIndex::ToCallPrice => CalcindexToCallPrice,
            CalcIndex::EffectiveLeverage => CalcindexEffectiveLeverage,
            CalcIndex::LeverageRatio => CalcindexLeverageRatio,
            CalcIndex::ConversionRatio => CalcindexConversionRatio,
            CalcIndex::BalancePoint => CalcindexBalancePoint,
            CalcIndex::OpenInterest => CalcindexOpenInterest,
            CalcIndex::Delta => CalcindexDelta,
            CalcIndex::Gamma => CalcindexGamma,
            CalcIndex::Theta => CalcindexTheta,
            CalcIndex::Vega => CalcindexVega,
            CalcIndex::Rho => CalcindexRho,
        }
    }
}

/// Security calc index response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCalcIndex {
    /// Security code
    pub symbol: String,
    /// Latest price
    pub last_done: Option<Decimal>,
    /// Change value
    pub change_value: Option<Decimal>,
    /// Change ratio
    pub change_rate: Option<Decimal>,
    /// Volume
    pub volume: Option<i64>,
    /// Turnover
    pub turnover: Option<Decimal>,
    /// Year-to-date change ratio
    pub ytd_change_rate: Option<Decimal>,
    /// Turnover rate
    pub turnover_rate: Option<Decimal>,
    /// Total market value
    pub total_market_value: Option<Decimal>,
    /// Capital flow
    pub capital_flow: Option<Decimal>,
    /// Amplitude
    pub amplitude: Option<Decimal>,
    /// Volume ratio
    pub volume_ratio: Option<Decimal>,
    /// PE (TTM)
    pub pe_ttm_ratio: Option<Decimal>,
    /// PB
    pub pb_ratio: Option<Decimal>,
    /// Dividend ratio (TTM)
    pub dividend_ratio_ttm: Option<Decimal>,
    /// Five days change ratio
    pub five_day_change_rate: Option<Decimal>,
    /// Ten days change ratio
    pub ten_day_change_rate: Option<Decimal>,
    /// Half year change ratio
    pub half_year_change_rate: Option<Decimal>,
    /// Five minutes change ratio
    pub five_minutes_change_rate: Option<Decimal>,
    /// Expiry date
    pub expiry_date: Option<Date>,
    /// Strike price
    pub strike_price: Option<Decimal>,
    /// Upper bound price
    pub upper_strike_price: Option<Decimal>,
    /// Lower bound price
    pub lower_strike_price: Option<Decimal>,
    /// Outstanding quantity
    pub outstanding_qty: Option<i64>,
    /// Outstanding ratio
    pub outstanding_ratio: Option<Decimal>,
    /// Premium
    pub premium: Option<Decimal>,
    /// In/out of the bound
    pub itm_otm: Option<Decimal>,
    /// Implied volatility
    pub implied_volatility: Option<Decimal>,
    /// Warrant delta
    pub warrant_delta: Option<Decimal>,
    /// Call price
    pub call_price: Option<Decimal>,
    /// Price interval from the call price
    pub to_call_price: Option<Decimal>,
    /// Effective leverage
    pub effective_leverage: Option<Decimal>,
    /// Leverage ratio
    pub leverage_ratio: Option<Decimal>,
    /// Conversion ratio
    pub conversion_ratio: Option<Decimal>,
    /// Breakeven point
    pub balance_point: Option<Decimal>,
    /// Open interest
    pub open_interest: Option<i64>,
    /// Delta
    pub delta: Option<Decimal>,
    /// Gamma
    pub gamma: Option<Decimal>,
    /// Theta
    pub theta: Option<Decimal>,
    /// Vega
    pub vega: Option<Decimal>,
    /// Rho
    pub rho: Option<Decimal>,
}

impl SecurityCalcIndex {
    pub(crate) fn from_proto(
        resp: longport_proto::quote::SecurityCalcIndex,
        indexes: &[CalcIndex],
    ) -> Self {
        let mut output = SecurityCalcIndex {
            symbol: resp.symbol,
            last_done: None,
            change_value: None,
            change_rate: None,
            volume: None,
            turnover: None,
            ytd_change_rate: None,
            turnover_rate: None,
            total_market_value: None,
            capital_flow: None,
            amplitude: None,
            volume_ratio: None,
            pe_ttm_ratio: None,
            pb_ratio: None,
            dividend_ratio_ttm: None,
            five_day_change_rate: None,
            ten_day_change_rate: None,
            half_year_change_rate: None,
            five_minutes_change_rate: None,
            expiry_date: None,
            strike_price: None,
            upper_strike_price: None,
            lower_strike_price: None,
            outstanding_qty: None,
            outstanding_ratio: None,
            premium: None,
            itm_otm: None,
            implied_volatility: None,
            warrant_delta: None,
            call_price: None,
            to_call_price: None,
            effective_leverage: None,
            leverage_ratio: None,
            conversion_ratio: None,
            balance_point: None,
            open_interest: None,
            delta: None,
            gamma: None,
            theta: None,
            vega: None,
            rho: None,
        };

        for index in indexes {
            match index {
                CalcIndex::LastDone => output.last_done = resp.last_done.parse().ok(),
                CalcIndex::ChangeValue => output.change_value = resp.change_val.parse().ok(),
                CalcIndex::ChangeRate => output.change_rate = resp.change_rate.parse().ok(),
                CalcIndex::Volume => output.volume = Some(resp.volume),
                CalcIndex::Turnover => output.turnover = resp.turnover.parse().ok(),
                CalcIndex::YtdChangeRate => {
                    output.ytd_change_rate = resp.ytd_change_rate.parse().ok()
                }
                CalcIndex::TurnoverRate => output.turnover_rate = resp.turnover_rate.parse().ok(),
                CalcIndex::TotalMarketValue => {
                    output.total_market_value = resp.total_market_value.parse().ok()
                }
                CalcIndex::CapitalFlow => output.capital_flow = resp.capital_flow.parse().ok(),
                CalcIndex::Amplitude => output.amplitude = resp.amplitude.parse().ok(),
                CalcIndex::VolumeRatio => output.volume_ratio = resp.volume_ratio.parse().ok(),
                CalcIndex::PeTtmRatio => output.pe_ttm_ratio = resp.pe_ttm_ratio.parse().ok(),
                CalcIndex::PbRatio => output.pb_ratio = resp.pb_ratio.parse().ok(),
                CalcIndex::DividendRatioTtm => {
                    output.dividend_ratio_ttm = resp.dividend_ratio_ttm.parse().ok()
                }
                CalcIndex::FiveDayChangeRate => {
                    output.five_day_change_rate = resp.five_day_change_rate.parse().ok()
                }
                CalcIndex::TenDayChangeRate => {
                    output.ten_day_change_rate = resp.ten_day_change_rate.parse().ok()
                }
                CalcIndex::HalfYearChangeRate => {
                    output.half_year_change_rate = resp.half_year_change_rate.parse().ok()
                }
                CalcIndex::FiveMinutesChangeRate => {
                    output.five_minutes_change_rate = resp.five_minutes_change_rate.parse().ok()
                }
                CalcIndex::ExpiryDate => output.expiry_date = parse_date(&resp.expiry_date).ok(),
                CalcIndex::StrikePrice => output.strike_price = resp.strike_price.parse().ok(),
                CalcIndex::UpperStrikePrice => {
                    output.upper_strike_price = resp.upper_strike_price.parse().ok()
                }
                CalcIndex::LowerStrikePrice => {
                    output.lower_strike_price = resp.lower_strike_price.parse().ok()
                }
                CalcIndex::OutstandingQty => output.outstanding_qty = Some(resp.outstanding_qty),
                CalcIndex::OutstandingRatio => {
                    output.outstanding_ratio = resp.outstanding_ratio.parse().ok()
                }
                CalcIndex::Premium => output.premium = resp.premium.parse().ok(),
                CalcIndex::ItmOtm => output.itm_otm = resp.itm_otm.parse().ok(),
                CalcIndex::ImpliedVolatility => {
                    output.implied_volatility = resp.implied_volatility.parse().ok()
                }
                CalcIndex::WarrantDelta => output.warrant_delta = resp.warrant_delta.parse().ok(),
                CalcIndex::CallPrice => output.call_price = resp.call_price.parse().ok(),
                CalcIndex::ToCallPrice => output.to_call_price = resp.to_call_price.parse().ok(),
                CalcIndex::EffectiveLeverage => {
                    output.effective_leverage = resp.effective_leverage.parse().ok()
                }
                CalcIndex::LeverageRatio => {
                    output.leverage_ratio = resp.leverage_ratio.parse().ok()
                }
                CalcIndex::ConversionRatio => {
                    output.conversion_ratio = resp.conversion_ratio.parse().ok()
                }
                CalcIndex::BalancePoint => output.balance_point = resp.balance_point.parse().ok(),
                CalcIndex::OpenInterest => output.open_interest = Some(resp.open_interest),
                CalcIndex::Delta => output.delta = resp.delta.parse().ok(),
                CalcIndex::Gamma => output.gamma = resp.gamma.parse().ok(),
                CalcIndex::Theta => output.theta = resp.theta.parse().ok(),
                CalcIndex::Vega => output.vega = resp.vega.parse().ok(),
                CalcIndex::Rho => output.rho = resp.rho.parse().ok(),
            }
        }

        output
    }
}

/// Security list category
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum SecurityListCategory {
    /// Overnight
    Overnight,
}

impl_serialize_for_enum_string!(SecurityListCategory);

/// The basic information of securities
#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    /// Security code
    pub symbol: String,
    /// Security name (zh-CN)
    pub name_cn: String,
    /// Security name (en)
    pub name_en: String,
    /// Security name (zh-HK)
    pub name_hk: String,
}

/// Quote package detail
#[derive(Debug, Clone)]
pub struct QuotePackageDetail {
    /// Key
    pub key: String,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Start time
    pub start_at: OffsetDateTime,
    /// End time
    pub end_at: OffsetDateTime,
}

impl TryFrom<quote::user_quote_level_detail::PackageDetail> for QuotePackageDetail {
    type Error = Error;

    fn try_from(quote: quote::user_quote_level_detail::PackageDetail) -> Result<Self> {
        Ok(Self {
            key: quote.key,
            name: quote.name,
            description: quote.description,
            start_at: OffsetDateTime::from_unix_timestamp(quote.start)
                .map_err(|err| Error::parse_field_error("start_at", err))?,
            end_at: OffsetDateTime::from_unix_timestamp(quote.end)
                .map_err(|err| Error::parse_field_error("end_at", err))?,
        })
    }
}

/// Trade sessions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TradeSessions {
    /// Normal trade session
    Normal = 0,
    /// All trade sessions
    All = 100,
}

impl TradeSessions {
    #[inline]
    pub(crate) fn contains(&self, session: TradeSession) -> bool {
        match self {
            TradeSessions::Normal => session == TradeSession::NormalTrade,
            TradeSessions::All => true,
        }
    }
}
