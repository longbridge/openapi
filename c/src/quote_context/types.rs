use std::os::raw::c_char;

use longport::quote::{
    Brokers, Candlestick, CapitalDistribution, CapitalDistributionResponse, CapitalFlowLine, Depth,
    IntradayLine, IssuerInfo, MarketTradingDays, MarketTradingSession, OptionDirection,
    OptionQuote, OptionType, ParticipantInfo, Period, PrePostQuote, PushBrokers, PushCandlestick,
    PushDepth, PushQuote, PushTrades, RealtimeQuote, Security, SecurityBoard, SecurityBrokers,
    SecurityCalcIndex, SecurityDepth, SecurityQuote, SecurityStaticInfo, StrikePriceInfo,
    Subscription, Trade, TradeDirection, TradeSession, TradeStatus, TradingSessionInfo,
    WarrantInfo, WarrantQuote, WarrantType, WatchlistGroup, WatchlistSecurity,
};

use crate::{
    quote_context::enum_types::{
        COptionDirection, COptionType, CPeriod, CSecuritiesUpdateMode, CSecurityBoard,
        CTradeDirection, CTradeSession, CTradeStatus, CWarrantStatus, CWarrantType,
    },
    types::{CDate, CDecimal, CMarket, COption, CString, CTime, CVec, ToFFI},
};

/// Quote message
#[repr(C)]
pub struct CPushQuote {
    /// Security code
    pub symbol: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Open
    pub open: *const CDecimal,
    /// High
    pub high: *const CDecimal,
    /// Low
    pub low: *const CDecimal,
    /// Time of latest price
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Security trading status
    pub trade_status: CTradeStatus,
    /// Trade session
    pub trade_session: CTradeSession,
}

#[derive(Debug)]
pub(crate) struct CPushQuoteOwned {
    symbol: CString,
    last_done: CDecimal,
    open: CDecimal,
    high: CDecimal,
    low: CDecimal,
    timestamp: i64,
    volume: i64,
    turnover: CDecimal,
    trade_status: TradeStatus,
    trade_session: TradeSession,
}

impl From<(String, PushQuote)> for CPushQuoteOwned {
    fn from((symbol, quote): (String, PushQuote)) -> Self {
        let PushQuote {
            last_done,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            trade_session,
        } = quote;
        CPushQuoteOwned {
            symbol: symbol.into(),
            last_done: last_done.into(),
            open: open.into(),
            high: high.into(),
            low: low.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            trade_status,
            trade_session,
        }
    }
}

impl ToFFI for CPushQuoteOwned {
    type FFIType = CPushQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushQuoteOwned {
            symbol,
            last_done,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            trade_session,
        } = self;
        CPushQuote {
            symbol: symbol.to_ffi_type(),
            last_done,
            open,
            high,
            low,
            timestamp: *timestamp,
            volume: *volume,
            turnover,
            trade_status: (*trade_status).into(),
            trade_session: (*trade_session).into(),
        }
    }
}

/// Depth
#[repr(C)]
pub struct CDepth {
    /// Position
    pub position: i32,
    /// Price
    pub price: *const CDecimal,
    /// Volume
    pub volume: i64,
    /// Number of orders
    pub order_num: i64,
}

#[derive(Debug)]
pub(crate) struct CDepthOwned {
    position: i32,
    price: CDecimal,
    volume: i64,
    order_num: i64,
}

impl From<Depth> for CDepthOwned {
    fn from(depth: Depth) -> Self {
        let Depth {
            position,
            price,
            volume,
            order_num,
        } = depth;
        CDepthOwned {
            position,
            price: price.into(),
            volume,
            order_num,
        }
    }
}

impl ToFFI for CDepthOwned {
    type FFIType = CDepth;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CDepthOwned {
            position,
            price,
            volume,
            order_num,
        } = self;
        CDepth {
            position: *position,
            price,
            volume: *volume,
            order_num: *order_num,
        }
    }
}

/// Quote message
#[repr(C)]
pub struct CPushDepth {
    /// Security code
    pub symbol: *const c_char,
    /// Ask depth
    pub asks: *const CDepth,
    /// Number of asks
    pub num_asks: usize,
    /// Bid depth
    pub bids: *const CDepth,
    /// Number of bids
    pub num_bids: usize,
}

#[derive(Debug)]
pub(crate) struct CPushDepthOwned {
    symbol: CString,
    asks: CVec<CDepthOwned>,
    bids: CVec<CDepthOwned>,
}

impl From<(String, PushDepth)> for CPushDepthOwned {
    fn from((symbol, depth): (String, PushDepth)) -> Self {
        let PushDepth { asks, bids } = depth;
        CPushDepthOwned {
            symbol: symbol.into(),
            asks: asks.into(),
            bids: bids.into(),
        }
    }
}

impl ToFFI for CPushDepthOwned {
    type FFIType = CPushDepth;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushDepthOwned { symbol, asks, bids } = self;
        CPushDepth {
            symbol: symbol.to_ffi_type(),
            asks: asks.to_ffi_type(),
            num_asks: asks.len(),
            bids: bids.to_ffi_type(),
            num_bids: bids.len(),
        }
    }
}

/// Brokers
#[derive(Debug)]
#[repr(C)]
pub struct CBrokers {
    /// Position
    pub position: i32,
    /// Broker IDs
    pub broker_ids: *const i32,
    /// Number of broker IDs
    pub num_broker_ids: usize,
}

impl ToFFI for Brokers {
    type FFIType = CBrokers;

    fn to_ffi_type(&self) -> Self::FFIType {
        let Brokers {
            position,
            broker_ids,
        } = self;
        CBrokers {
            position: *position,
            broker_ids: broker_ids.as_ptr(),
            num_broker_ids: broker_ids.len(),
        }
    }
}

/// Brokers message
#[repr(C)]
pub struct CPushBrokers {
    /// Security code
    pub symbol: *const c_char,
    /// Ask depth
    pub ask_brokers: *const CBrokers,
    /// Number of ask brokers
    pub num_ask_brokers: usize,
    /// Bid depth
    pub bid_brokers: *const CBrokers,
    /// Number of bid brokers
    pub num_bid_brokers: usize,
}

/// Brokers message
#[derive(Debug)]
pub(crate) struct CPushBrokersOwned {
    symbol: CString,
    ask_brokers: CVec<Brokers>,
    bid_brokers: CVec<Brokers>,
}

impl From<(String, PushBrokers)> for CPushBrokersOwned {
    fn from((symbol, brokers): (String, PushBrokers)) -> Self {
        let PushBrokers {
            ask_brokers,
            bid_brokers,
        } = brokers;
        CPushBrokersOwned {
            symbol: symbol.into(),
            ask_brokers: ask_brokers.into(),
            bid_brokers: bid_brokers.into(),
        }
    }
}

impl ToFFI for CPushBrokersOwned {
    type FFIType = CPushBrokers;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushBrokersOwned {
            symbol,
            ask_brokers,
            bid_brokers,
        } = self;
        CPushBrokers {
            symbol: symbol.to_ffi_type(),
            ask_brokers: ask_brokers.to_ffi_type(),
            num_ask_brokers: ask_brokers.len(),
            bid_brokers: bid_brokers.to_ffi_type(),
            num_bid_brokers: bid_brokers.len(),
        }
    }
}

/// Trade
#[repr(C)]
pub struct CTrade {
    /// Price
    pub price: *const CDecimal,
    /// Volume
    pub volume: i64,
    /// Time of trading
    pub timestamp: i64,
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
    pub trade_type: *const c_char,
    /// Trade direction
    pub direction: CTradeDirection,
    /// Trade session
    pub trade_session: CTradeSession,
}

pub(crate) struct CTradeOwned {
    price: CDecimal,
    volume: i64,
    timestamp: i64,
    trade_type: CString,
    direction: TradeDirection,
    trade_session: TradeSession,
}

impl From<Trade> for CTradeOwned {
    fn from(trade: Trade) -> Self {
        let Trade {
            price,
            volume,
            timestamp,
            trade_type,
            direction,
            trade_session,
        } = trade;
        CTradeOwned {
            price: price.into(),
            volume,
            timestamp: timestamp.unix_timestamp(),
            trade_type: trade_type.into(),
            direction,
            trade_session,
        }
    }
}

impl ToFFI for CTradeOwned {
    type FFIType = CTrade;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CTradeOwned {
            price,
            volume,
            timestamp,
            trade_type,
            direction,
            trade_session,
        } = self;
        CTrade {
            price,
            volume: *volume,
            timestamp: *timestamp,
            trade_type: trade_type.to_ffi_type(),
            direction: (*direction).into(),
            trade_session: (*trade_session).into(),
        }
    }
}

/// Trades message
#[repr(C)]
pub struct CPushTrades {
    /// Security code
    pub symbol: *const c_char,
    /// Trades data
    pub trades: *const CTrade,
    /// Number of trades
    pub num_trades: usize,
}

pub(crate) struct CPushTradesOwned {
    symbol: CString,
    trades: CVec<CTradeOwned>,
}

impl From<(String, PushTrades)> for CPushTradesOwned {
    fn from((symbol, trades): (String, PushTrades)) -> Self {
        let PushTrades { trades } = trades;
        CPushTradesOwned {
            symbol: symbol.into(),
            trades: trades.into(),
        }
    }
}

impl ToFFI for CPushTradesOwned {
    type FFIType = CPushTrades;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushTradesOwned { symbol, trades } = self;
        CPushTrades {
            symbol: symbol.to_ffi_type(),
            trades: trades.to_ffi_type(),
            num_trades: trades.len(),
        }
    }
}

/// Candlestick
#[repr(C)]
pub struct CCandlestick {
    /// Close price
    pub close: *const CDecimal,
    /// Open price
    pub open: *const CDecimal,
    /// Low price
    pub low: *const CDecimal,
    /// High price
    pub high: *const CDecimal,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Timestamp
    pub timestamp: i64,
}

pub(crate) struct CCandlestickOwned {
    close: CDecimal,
    open: CDecimal,
    low: CDecimal,
    high: CDecimal,
    volume: i64,
    turnover: CDecimal,
    timestamp: i64,
}

impl From<Candlestick> for CCandlestickOwned {
    fn from(candlestick: Candlestick) -> Self {
        let Candlestick {
            close,
            open,
            low,
            high,
            volume,
            turnover,
            timestamp,
        } = candlestick;
        CCandlestickOwned {
            close: close.into(),
            open: open.into(),
            low: low.into(),
            high: high.into(),
            volume,
            turnover: turnover.into(),
            timestamp: timestamp.unix_timestamp(),
        }
    }
}

impl ToFFI for CCandlestickOwned {
    type FFIType = CCandlestick;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCandlestickOwned {
            close,
            open,
            low,
            high,
            volume,
            turnover,
            timestamp,
        } = self;
        CCandlestick {
            close,
            open,
            low,
            high,
            volume: *volume,
            turnover,
            timestamp: *timestamp,
        }
    }
}

/// Candlestick updated message
#[repr(C)]
pub struct CPushCandlestick {
    /// Security code
    pub symbol: *const c_char,
    /// Period type
    pub period: CPeriod,
    /// Candlestick
    pub candlestick: CCandlestick,
}

pub(crate) struct CPushCandlestickOwned {
    symbol: CString,
    period: Period,
    candlestick: CCandlestickOwned,
}

impl From<(String, PushCandlestick)> for CPushCandlestickOwned {
    fn from((symbol, candlestick): (String, PushCandlestick)) -> Self {
        let PushCandlestick {
            period,
            candlestick,
        } = candlestick;
        CPushCandlestickOwned {
            symbol: symbol.into(),
            period,
            candlestick: candlestick.into(),
        }
    }
}

impl ToFFI for CPushCandlestickOwned {
    type FFIType = CPushCandlestick;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushCandlestickOwned {
            symbol,
            period,
            candlestick,
        } = self;
        CPushCandlestick {
            symbol: symbol.to_ffi_type(),
            period: (*period).into(),
            candlestick: candlestick.to_ffi_type(),
        }
    }
}

#[repr(C)]
pub struct CSubscription {
    symbol: *const c_char,
    sub_types: u8,
    candlesticks: *const CPeriod,
    num_candlesticks: usize,
}

#[derive(Debug)]
pub(crate) struct CSubscriptionOwned {
    symbol: CString,
    sub_types: u8,
    candlesticks: Vec<CPeriod>,
}

impl From<Subscription> for CSubscriptionOwned {
    fn from(subscription: Subscription) -> Self {
        let Subscription {
            symbol,
            sub_types,
            candlesticks,
        } = subscription;
        CSubscriptionOwned {
            symbol: symbol.into(),
            sub_types: sub_types.bits(),
            candlesticks: candlesticks.into_iter().map(Into::into).collect(),
        }
    }
}

impl ToFFI for CSubscriptionOwned {
    type FFIType = CSubscription;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSubscriptionOwned {
            symbol,
            sub_types,
            candlesticks,
        } = self;
        CSubscription {
            symbol: symbol.to_ffi_type(),
            sub_types: *sub_types,
            candlesticks: candlesticks.as_ptr(),
            num_candlesticks: candlesticks.len(),
        }
    }
}

/// The basic information of securities
#[repr(C)]
pub struct CSecurityStaticInfo {
    /// Security code
    pub symbol: *const c_char,
    /// Security name (zh-CN)
    pub name_cn: *const c_char,
    /// Security name (en)
    pub name_en: *const c_char,
    /// Security name (zh-HK)
    pub name_hk: *const c_char,
    /// Exchange which the security belongs to
    pub exchange: *const c_char,
    /// Trading currency
    pub currency: *const c_char,
    /// Lot size
    pub lot_size: i32,
    /// Total shares
    pub total_shares: i64,
    /// Circulating shares
    pub circulating_shares: i64,
    /// HK shares (only HK stocks)
    pub hk_shares: i64,
    /// Earnings per share
    pub eps: *const CDecimal,
    /// Earnings per share (TTM)
    pub eps_ttm: *const CDecimal,
    /// Net assets per share
    pub bps: *const CDecimal,
    /// Dividend yield
    pub dividend_yield: *const CDecimal,
    /// Types of supported derivatives
    pub stock_derivatives: u8,
    /// Board
    pub board: CSecurityBoard,
}

#[derive(Debug)]
pub(crate) struct CSecurityStaticInfoOwned {
    pub symbol: CString,
    pub name_cn: CString,
    pub name_en: CString,
    pub name_hk: CString,
    pub exchange: CString,
    pub currency: CString,
    pub lot_size: i32,
    pub total_shares: i64,
    pub circulating_shares: i64,
    pub hk_shares: i64,
    pub eps: CDecimal,
    pub eps_ttm: CDecimal,
    pub bps: CDecimal,
    pub dividend_yield: CDecimal,
    pub stock_derivatives: u8,
    pub board: SecurityBoard,
}

impl From<SecurityStaticInfo> for CSecurityStaticInfoOwned {
    fn from(info: SecurityStaticInfo) -> Self {
        let SecurityStaticInfo {
            symbol,
            name_cn,
            name_en,
            name_hk,
            exchange,
            currency,
            lot_size,
            total_shares,
            circulating_shares,
            hk_shares,
            eps,
            eps_ttm,
            bps,
            dividend_yield,
            stock_derivatives,
            board,
        } = info;
        CSecurityStaticInfoOwned {
            symbol: symbol.into(),
            name_cn: name_cn.into(),
            name_en: name_en.into(),
            name_hk: name_hk.into(),
            exchange: exchange.into(),
            currency: currency.into(),
            lot_size,
            total_shares,
            circulating_shares,
            hk_shares,
            eps: eps.into(),
            eps_ttm: eps_ttm.into(),
            bps: bps.into(),
            dividend_yield: dividend_yield.into(),
            stock_derivatives: stock_derivatives.bits(),
            board,
        }
    }
}

impl ToFFI for CSecurityStaticInfoOwned {
    type FFIType = CSecurityStaticInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSecurityStaticInfoOwned {
            symbol,
            name_cn,
            name_en,
            name_hk,
            exchange,
            currency,
            lot_size,
            total_shares,
            circulating_shares,
            hk_shares,
            eps,
            eps_ttm,
            bps,
            dividend_yield,
            stock_derivatives,
            board,
        } = self;
        CSecurityStaticInfo {
            symbol: symbol.to_ffi_type(),
            name_cn: name_cn.to_ffi_type(),
            name_en: name_en.to_ffi_type(),
            name_hk: name_hk.to_ffi_type(),
            exchange: exchange.to_ffi_type(),
            currency: currency.to_ffi_type(),
            lot_size: *lot_size,
            total_shares: *total_shares,
            circulating_shares: *circulating_shares,
            hk_shares: *hk_shares,
            eps: eps.to_ffi_type(),
            eps_ttm: eps_ttm.to_ffi_type(),
            bps: bps.to_ffi_type(),
            dividend_yield: dividend_yield.to_ffi_type(),
            stock_derivatives: *stock_derivatives,
            board: (*board).into(),
        }
    }
}

/// Quote of US pre/post market
#[repr(C)]
pub struct CPrePostQuote {
    /// Latest price
    pub last_done: *const CDecimal,
    /// Time of latest price
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// High
    pub high: *const CDecimal,
    /// Low
    pub low: *const CDecimal,
    /// Close of the last trade session
    pub prev_close: *const CDecimal,
}

/// Quote of US pre/post market
pub(crate) struct CPrePostQuoteOwned {
    last_done: CDecimal,
    timestamp: i64,
    volume: i64,
    turnover: CDecimal,
    high: CDecimal,
    low: CDecimal,
    prev_close: CDecimal,
}

impl From<PrePostQuote> for CPrePostQuoteOwned {
    fn from(quote: PrePostQuote) -> Self {
        let PrePostQuote {
            last_done,
            timestamp,
            volume,
            turnover,
            high,
            low,
            prev_close,
        } = quote;
        Self {
            last_done: last_done.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            high: high.into(),
            low: low.into(),
            prev_close: prev_close.into(),
        }
    }
}

impl ToFFI for CPrePostQuoteOwned {
    type FFIType = CPrePostQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPrePostQuoteOwned {
            last_done,
            timestamp,
            volume,
            turnover,
            high,
            low,
            prev_close,
        } = self;
        CPrePostQuote {
            last_done,
            timestamp: *timestamp,
            volume: *volume,
            turnover,
            high,
            low,
            prev_close,
        }
    }
}

/// Quote of securitity
#[repr(C)]
pub struct CSecurityQuote {
    /// Security code
    pub symbol: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Yesterday's close
    pub prev_close: *const CDecimal,
    /// Open
    pub open: *const CDecimal,
    /// High
    pub high: *const CDecimal,
    /// Low
    pub low: *const CDecimal,
    /// Time of latest price
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Security trading status
    pub trade_status: CTradeStatus,
    /// Quote of US pre market
    pub pre_market_quote: *const CPrePostQuote,
    /// Quote of US post market
    pub post_market_quote: *const CPrePostQuote,
    /// Quote of US overnight market
    pub overnight_quote: *const CPrePostQuote,
}

pub(crate) struct CSecurityQuoteOwned {
    symbol: CString,
    last_done: CDecimal,
    prev_close: CDecimal,
    open: CDecimal,
    high: CDecimal,
    low: CDecimal,
    timestamp: i64,
    volume: i64,
    turnover: CDecimal,
    trade_status: TradeStatus,
    pre_market_quote: COption<CPrePostQuoteOwned>,
    post_market_quote: COption<CPrePostQuoteOwned>,
    overnight_quote: COption<CPrePostQuoteOwned>,
}

impl From<SecurityQuote> for CSecurityQuoteOwned {
    fn from(quote: SecurityQuote) -> Self {
        let SecurityQuote {
            symbol,
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            pre_market_quote,
            post_market_quote,
            overnight_quote,
        } = quote;
        Self {
            symbol: symbol.into(),
            last_done: last_done.into(),
            prev_close: prev_close.into(),
            open: open.into(),
            high: high.into(),
            low: low.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            trade_status,
            pre_market_quote: pre_market_quote.into(),
            post_market_quote: post_market_quote.into(),
            overnight_quote: overnight_quote.into(),
        }
    }
}

impl ToFFI for CSecurityQuoteOwned {
    type FFIType = CSecurityQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSecurityQuoteOwned {
            symbol,
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            pre_market_quote,
            post_market_quote,
            overnight_quote,
        } = self;
        CSecurityQuote {
            symbol: symbol.to_ffi_type(),
            last_done: last_done.to_ffi_type(),
            prev_close: prev_close.to_ffi_type(),
            open: open.to_ffi_type(),
            high: high.to_ffi_type(),
            low: low.to_ffi_type(),
            timestamp: *timestamp,
            volume: *volume,
            turnover: turnover.to_ffi_type(),
            trade_status: (*trade_status).into(),
            pre_market_quote: pre_market_quote.to_ffi_type(),
            post_market_quote: post_market_quote.to_ffi_type(),
            overnight_quote: overnight_quote.to_ffi_type(),
        }
    }
}

/// Quote of option
#[repr(C)]
pub struct COptionQuote {
    /// Security code
    pub symbol: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Yesterday's close
    pub prev_close: *const CDecimal,
    /// Open
    pub open: *const CDecimal,
    /// High
    pub high: *const CDecimal,
    /// Low
    pub low: *const CDecimal,
    /// Time of latest price
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Security trading status
    pub trade_status: CTradeStatus,
    /// Implied volatility
    pub implied_volatility: *const CDecimal,
    /// Number of open positions
    pub open_interest: i64,
    /// Exprity date
    pub expiry_date: CDate,
    /// Strike price
    pub strike_price: *const CDecimal,
    /// Contract multiplier
    pub contract_multiplier: *const CDecimal,
    /// Option type
    pub contract_type: COptionType,
    /// Contract size
    pub contract_size: *const CDecimal,
    /// Option direction
    pub direction: COptionDirection,
    /// Underlying security historical volatility of the option
    pub historical_volatility: *const CDecimal,
    /// Underlying security symbol of the option
    pub underlying_symbol: *const c_char,
}

pub(crate) struct COptionQuoteOwned {
    symbol: CString,
    last_done: CDecimal,
    prev_close: CDecimal,
    open: CDecimal,
    high: CDecimal,
    low: CDecimal,
    timestamp: i64,
    volume: i64,
    turnover: CDecimal,
    trade_status: TradeStatus,
    implied_volatility: CDecimal,
    open_interest: i64,
    expiry_date: CDate,
    strike_price: CDecimal,
    contract_multiplier: CDecimal,
    contract_type: OptionType,
    contract_size: CDecimal,
    direction: OptionDirection,
    historical_volatility: CDecimal,
    underlying_symbol: CString,
}

impl From<OptionQuote> for COptionQuoteOwned {
    fn from(quote: OptionQuote) -> Self {
        let OptionQuote {
            symbol,
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            implied_volatility,
            open_interest,
            expiry_date,
            strike_price,
            contract_multiplier,
            contract_type,
            contract_size,
            direction,
            historical_volatility,
            underlying_symbol,
        } = quote;
        Self {
            symbol: symbol.into(),
            last_done: last_done.into(),
            prev_close: prev_close.into(),
            open: open.into(),
            high: high.into(),
            low: low.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            trade_status,
            implied_volatility: implied_volatility.into(),
            open_interest,
            expiry_date: expiry_date.into(),
            strike_price: strike_price.into(),
            contract_multiplier: contract_multiplier.into(),
            contract_type,
            contract_size: contract_size.into(),
            direction,
            historical_volatility: historical_volatility.into(),
            underlying_symbol: underlying_symbol.into(),
        }
    }
}

impl ToFFI for COptionQuoteOwned {
    type FFIType = COptionQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COptionQuoteOwned {
            symbol,
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            implied_volatility,
            open_interest,
            expiry_date,
            strike_price,
            contract_multiplier,
            contract_type,
            contract_size,
            direction,
            historical_volatility,
            underlying_symbol,
        } = self;
        COptionQuote {
            symbol: symbol.to_ffi_type(),
            last_done: last_done.to_ffi_type(),
            prev_close: prev_close.to_ffi_type(),
            open: open.to_ffi_type(),
            high: high.to_ffi_type(),
            low: low.to_ffi_type(),
            timestamp: *timestamp,
            volume: *volume,
            turnover: turnover.to_ffi_type(),
            trade_status: (*trade_status).into(),
            implied_volatility: implied_volatility.to_ffi_type(),
            open_interest: *open_interest,
            expiry_date: *expiry_date,
            strike_price: strike_price.to_ffi_type(),
            contract_multiplier: contract_multiplier.to_ffi_type(),
            contract_type: (*contract_type).into(),
            contract_size: contract_size.to_ffi_type(),
            direction: (*direction).into(),
            historical_volatility: historical_volatility.to_ffi_type(),
            underlying_symbol: underlying_symbol.to_ffi_type(),
        }
    }
}

/// Quote of warrant
#[repr(C)]
pub struct CWarrantQuote {
    /// Security code
    pub symbol: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Yesterday's close
    pub prev_close: *const CDecimal,
    /// Open
    pub open: *const CDecimal,
    /// High
    pub high: *const CDecimal,
    /// Low
    pub low: *const CDecimal,
    /// Time of latest price
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Security trading status
    pub trade_status: CTradeStatus,
    /// Implied volatility
    pub implied_volatility: *const CDecimal,
    /// Exprity date
    pub expiry_date: CDate,
    /// Last tradalbe date
    pub last_trade_date: CDate,
    /// Outstanding ratio
    pub outstanding_ratio: *const CDecimal,
    /// Outstanding quantity
    pub outstanding_quantity: i64,
    /// Conversion ratio
    pub conversion_ratio: *const CDecimal,
    /// Warrant type
    pub category: CWarrantType,
    /// Strike price
    pub strike_price: *const CDecimal,
    /// Upper bound price
    pub upper_strike_price: *const CDecimal,
    /// Lower bound price
    pub lower_strike_price: *const CDecimal,
    /// Call price
    pub call_price: *const CDecimal,
    /// Underlying security symbol of the warrant
    pub underlying_symbol: *const c_char,
}

pub(crate) struct CWarrantQuoteOwned {
    symbol: CString,
    last_done: CDecimal,
    prev_close: CDecimal,
    open: CDecimal,
    high: CDecimal,
    low: CDecimal,
    timestamp: i64,
    volume: i64,
    turnover: CDecimal,
    trade_status: TradeStatus,
    implied_volatility: CDecimal,
    expiry_date: CDate,
    last_trade_date: CDate,
    outstanding_ratio: CDecimal,
    outstanding_quantity: i64,
    conversion_ratio: CDecimal,
    category: WarrantType,
    strike_price: CDecimal,
    upper_strike_price: CDecimal,
    lower_strike_price: CDecimal,
    call_price: CDecimal,
    underlying_symbol: CString,
}

impl From<WarrantQuote> for CWarrantQuoteOwned {
    fn from(quote: WarrantQuote) -> Self {
        let WarrantQuote {
            symbol,
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            implied_volatility,
            expiry_date,
            last_trade_date,
            outstanding_ratio,
            outstanding_quantity,
            conversion_ratio,
            category,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            call_price,
            underlying_symbol,
        } = quote;
        Self {
            symbol: symbol.into(),
            last_done: last_done.into(),
            prev_close: prev_close.into(),
            open: open.into(),
            high: high.into(),
            low: low.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            trade_status,
            implied_volatility: implied_volatility.into(),
            expiry_date: expiry_date.into(),
            last_trade_date: last_trade_date.into(),
            outstanding_ratio: outstanding_ratio.into(),
            outstanding_quantity,
            conversion_ratio: conversion_ratio.into(),
            category,
            strike_price: strike_price.into(),
            upper_strike_price: upper_strike_price.into(),
            lower_strike_price: lower_strike_price.into(),
            call_price: call_price.into(),
            underlying_symbol: underlying_symbol.into(),
        }
    }
}

impl ToFFI for CWarrantQuoteOwned {
    type FFIType = CWarrantQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWarrantQuoteOwned {
            symbol,
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
            implied_volatility,
            expiry_date,
            last_trade_date,
            outstanding_ratio,
            outstanding_quantity,
            conversion_ratio,
            category,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            call_price,
            underlying_symbol,
        } = self;
        CWarrantQuote {
            symbol: symbol.to_ffi_type(),
            last_done: last_done.to_ffi_type(),
            prev_close: prev_close.to_ffi_type(),
            open: open.to_ffi_type(),
            high: high.to_ffi_type(),
            low: low.to_ffi_type(),
            timestamp: *timestamp,
            volume: *volume,
            turnover: turnover.to_ffi_type(),
            trade_status: (*trade_status).into(),
            implied_volatility: implied_volatility.to_ffi_type(),
            expiry_date: *expiry_date,
            last_trade_date: *last_trade_date,
            outstanding_ratio: outstanding_ratio.to_ffi_type(),
            outstanding_quantity: *outstanding_quantity,
            conversion_ratio: conversion_ratio.to_ffi_type(),
            category: (*category).into(),
            strike_price: strike_price.to_ffi_type(),
            upper_strike_price: upper_strike_price.to_ffi_type(),
            lower_strike_price: lower_strike_price.to_ffi_type(),
            call_price: call_price.to_ffi_type(),
            underlying_symbol: underlying_symbol.to_ffi_type(),
        }
    }
}

/// Quote message
#[repr(C)]
pub struct CSecurityDepth {
    /// Ask depth
    pub asks: *const CDepth,
    /// Number of asks
    pub num_asks: usize,
    /// Bid depth
    pub bids: *const CDepth,
    /// Number of bids
    pub num_bids: usize,
}

#[derive(Debug)]
pub(crate) struct CSecurityDepthOwned {
    asks: CVec<CDepthOwned>,
    bids: CVec<CDepthOwned>,
}

impl From<SecurityDepth> for CSecurityDepthOwned {
    fn from(depth: SecurityDepth) -> Self {
        let SecurityDepth { asks, bids } = depth;
        CSecurityDepthOwned {
            asks: asks.into(),
            bids: bids.into(),
        }
    }
}

impl ToFFI for CSecurityDepthOwned {
    type FFIType = CSecurityDepth;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSecurityDepthOwned { asks, bids } = self;
        CSecurityDepth {
            asks: asks.to_ffi_type(),
            num_asks: asks.len(),
            bids: bids.to_ffi_type(),
            num_bids: bids.len(),
        }
    }
}

/// Security brokers
#[repr(C)]
pub struct CSecurityBrokers {
    /// Ask brokers
    pub ask_brokers: *const CBrokers,
    /// Number of ask brokers
    pub num_ask_brokers: usize,
    /// Bid brokers
    pub bid_brokers: *const CBrokers,
    /// Number of bid brokers
    pub num_bid_brokers: usize,
}

#[derive(Debug)]
pub(crate) struct CSecurityBrokersOwned {
    ask_brokers: CVec<Brokers>,
    bid_brokers: CVec<Brokers>,
}

impl From<SecurityBrokers> for CSecurityBrokersOwned {
    fn from(brokers: SecurityBrokers) -> Self {
        let SecurityBrokers {
            ask_brokers,
            bid_brokers,
        } = brokers;
        CSecurityBrokersOwned {
            ask_brokers: ask_brokers.into(),
            bid_brokers: bid_brokers.into(),
        }
    }
}

impl ToFFI for CSecurityBrokersOwned {
    type FFIType = CSecurityBrokers;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSecurityBrokersOwned {
            ask_brokers,
            bid_brokers,
        } = self;
        CSecurityBrokers {
            ask_brokers: ask_brokers.to_ffi_type(),
            num_ask_brokers: ask_brokers.len(),
            bid_brokers: bid_brokers.to_ffi_type(),
            num_bid_brokers: bid_brokers.len(),
        }
    }
}

/// Participant info
#[repr(C)]
pub struct CParticipantInfo {
    /// Broker IDs
    pub broker_ids: *const i32,
    /// Number of broker IDs
    pub num_broker_ids: usize,
    /// Participant name (zh-CN)
    pub name_cn: *const c_char,
    /// Participant name (en)
    pub name_en: *const c_char,
    /// Participant name (zh-HK)
    pub name_hk: *const c_char,
}

#[derive(Debug)]
pub struct CParticipantInfoOwned {
    broker_ids: Vec<i32>,
    name_cn: CString,
    name_en: CString,
    name_hk: CString,
}

impl From<ParticipantInfo> for CParticipantInfoOwned {
    fn from(participant: ParticipantInfo) -> Self {
        let ParticipantInfo {
            broker_ids,
            name_cn,
            name_en,
            name_hk,
        } = participant;
        CParticipantInfoOwned {
            broker_ids,
            name_cn: name_cn.into(),
            name_en: name_en.into(),
            name_hk: name_hk.into(),
        }
    }
}

impl ToFFI for CParticipantInfoOwned {
    type FFIType = CParticipantInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CParticipantInfoOwned {
            broker_ids,
            name_cn,
            name_en,
            name_hk,
        } = self;
        CParticipantInfo {
            broker_ids: broker_ids.as_ptr(),
            num_broker_ids: broker_ids.len(),
            name_cn: name_cn.to_ffi_type(),
            name_en: name_en.to_ffi_type(),
            name_hk: name_hk.to_ffi_type(),
        }
    }
}

/// Intraday line
#[repr(C)]
pub struct CIntradayLine {
    /// Close price of the minute
    pub price: *const CDecimal,
    /// Start time of the minute
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Average price
    pub avg_price: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CIntradayLineOwned {
    pub price: CDecimal,
    pub timestamp: i64,
    pub volume: i64,
    pub turnover: CDecimal,
    pub avg_price: CDecimal,
}

impl From<IntradayLine> for CIntradayLineOwned {
    fn from(line: IntradayLine) -> Self {
        let IntradayLine {
            price,
            timestamp,
            volume,
            turnover,
            avg_price,
        } = line;
        CIntradayLineOwned {
            price: price.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            avg_price: avg_price.into(),
        }
    }
}

impl ToFFI for CIntradayLineOwned {
    type FFIType = CIntradayLine;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CIntradayLineOwned {
            price,
            timestamp,
            volume,
            turnover,
            avg_price,
        } = self;
        CIntradayLine {
            price: price.to_ffi_type(),
            timestamp: *timestamp,
            volume: *volume,
            turnover: turnover.to_ffi_type(),
            avg_price: avg_price.to_ffi_type(),
        }
    }
}

/// Strike price info
#[repr(C)]
pub struct CStrikePriceInfo {
    /// Strike price
    pub price: *const CDecimal,
    /// Security code of call option
    pub call_symbol: *const c_char,
    /// Security code of put option
    pub put_symbol: *const c_char,
    /// Is standard
    pub standard: bool,
}

#[derive(Debug)]
pub(crate) struct CStrikePriceInfoOwned {
    price: CDecimal,
    call_symbol: CString,
    put_symbol: CString,
    standard: bool,
}

impl From<StrikePriceInfo> for CStrikePriceInfoOwned {
    fn from(info: StrikePriceInfo) -> Self {
        let StrikePriceInfo {
            price,
            call_symbol,
            put_symbol,
            standard,
        } = info;
        CStrikePriceInfoOwned {
            price: price.into(),
            call_symbol: call_symbol.into(),
            put_symbol: put_symbol.into(),
            standard,
        }
    }
}

impl ToFFI for CStrikePriceInfoOwned {
    type FFIType = CStrikePriceInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStrikePriceInfoOwned {
            price,
            call_symbol,
            put_symbol,
            standard,
        } = self;
        CStrikePriceInfo {
            price,
            call_symbol: call_symbol.to_ffi_type(),
            put_symbol: put_symbol.to_ffi_type(),
            standard: *standard,
        }
    }
}

/// Issuer info
#[repr(C)]
pub struct CIssuerInfo {
    /// Issuer ID
    pub issuer_id: i32,
    /// Issuer name (zh-CN)
    pub name_cn: *const c_char,
    /// Issuer name (en)
    pub name_en: *const c_char,
    /// Issuer name (zh-HK)
    pub name_hk: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CIssuerInfoOwned {
    issuer_id: i32,
    name_cn: CString,
    name_en: CString,
    name_hk: CString,
}

impl From<IssuerInfo> for CIssuerInfoOwned {
    fn from(info: IssuerInfo) -> Self {
        let IssuerInfo {
            issuer_id,
            name_cn,
            name_en,
            name_hk,
        } = info;
        CIssuerInfoOwned {
            issuer_id,
            name_cn: name_cn.into(),
            name_en: name_en.into(),
            name_hk: name_hk.into(),
        }
    }
}

impl ToFFI for CIssuerInfoOwned {
    type FFIType = CIssuerInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CIssuerInfoOwned {
            issuer_id,
            name_cn,
            name_en,
            name_hk,
        } = self;
        CIssuerInfo {
            issuer_id: *issuer_id,
            name_cn: name_cn.to_ffi_type(),
            name_en: name_en.to_ffi_type(),
            name_hk: name_hk.to_ffi_type(),
        }
    }
}

/// The information of trading session
#[repr(C)]
pub struct CTradingSessionInfo {
    /// Being trading time
    pub begin_time: CTime,
    /// End trading time
    pub end_time: CTime,
    /// Trading session
    pub trade_session: CTradeSession,
}

impl ToFFI for TradingSessionInfo {
    type FFIType = CTradingSessionInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let TradingSessionInfo {
            begin_time,
            end_time,
            trade_session,
        } = self;
        CTradingSessionInfo {
            begin_time: (*begin_time).into(),
            end_time: (*end_time).into(),
            trade_session: (*trade_session).into(),
        }
    }
}

/// Market trading session
#[repr(C)]
pub struct CMarketTradingSession {
    /// Market
    pub market: CMarket,
    /// Trading sessions
    pub trade_sessions: *const CTradingSessionInfo,
    /// Number trading sessions
    pub num_trade_sessions: usize,
}

#[derive(Debug)]
pub(crate) struct CMarketTradingSessionOwned {
    market: CMarket,
    trade_sessions: CVec<TradingSessionInfo>,
}

impl From<MarketTradingSession> for CMarketTradingSessionOwned {
    fn from(info: MarketTradingSession) -> Self {
        let MarketTradingSession {
            market,
            trade_sessions,
        } = info;
        CMarketTradingSessionOwned {
            market: market.into(),
            trade_sessions: trade_sessions.into(),
        }
    }
}

impl ToFFI for CMarketTradingSessionOwned {
    type FFIType = CMarketTradingSession;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CMarketTradingSessionOwned {
            market,
            trade_sessions,
        } = self;
        CMarketTradingSession {
            market: *market,
            trade_sessions: trade_sessions.to_ffi_type(),
            num_trade_sessions: trade_sessions.len(),
        }
    }
}

/// Market trading days
#[repr(C)]
pub struct CMarketTradingDays {
    /// Trading days
    pub trading_days: *const CDate,
    /// Number of trading days
    pub num_trading_days: usize,
    /// Half trading days
    pub half_trading_days: *const CDate,
    /// Number of half trading days
    pub num_half_trading_days: usize,
}

#[derive(Debug)]
pub(crate) struct CMarketTradingDaysOwned {
    /// Trading days
    trading_days: CVec<CDate>,
    /// Half trading days
    half_trading_days: CVec<CDate>,
}

impl From<MarketTradingDays> for CMarketTradingDaysOwned {
    fn from(trading_days: MarketTradingDays) -> Self {
        let MarketTradingDays {
            trading_days,
            half_trading_days,
        } = trading_days;
        CMarketTradingDaysOwned {
            trading_days: trading_days.into(),
            half_trading_days: half_trading_days.into(),
        }
    }
}

impl ToFFI for CMarketTradingDaysOwned {
    type FFIType = CMarketTradingDays;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CMarketTradingDaysOwned {
            trading_days,
            half_trading_days,
        } = self;
        CMarketTradingDays {
            trading_days: trading_days.to_ffi_type(),
            num_trading_days: trading_days.len(),
            half_trading_days: half_trading_days.to_ffi_type(),
            num_half_trading_days: half_trading_days.len(),
        }
    }
}

/// Market trading days
#[repr(C)]
pub struct CCapitalFlowLine {
    /// Inflow capital data
    pub inflow: *const CDecimal,
    /// Time
    pub timestamp: i64,
}

#[derive(Debug)]
pub struct CCapitalFlowLineOwned {
    pub inflow: CDecimal,
    pub timestamp: i64,
}

impl From<CapitalFlowLine> for CCapitalFlowLineOwned {
    fn from(line: CapitalFlowLine) -> Self {
        let CapitalFlowLine { inflow, timestamp } = line;
        CCapitalFlowLineOwned {
            inflow: inflow.into(),
            timestamp: timestamp.unix_timestamp(),
        }
    }
}

impl ToFFI for CCapitalFlowLineOwned {
    type FFIType = CCapitalFlowLine;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCapitalFlowLineOwned { inflow, timestamp } = self;
        CCapitalFlowLine {
            inflow: inflow.to_ffi_type(),
            timestamp: *timestamp,
        }
    }
}

/// Capital distribution
#[repr(C)]
pub struct CCapitalDistribution {
    /// Large order
    pub large: *const CDecimal,
    /// Medium order
    pub medium: *const CDecimal,
    /// Small order
    pub small: *const CDecimal,
}

pub(crate) struct CCapitalDistributionOwned {
    large: CDecimal,
    medium: CDecimal,
    small: CDecimal,
}

impl From<CapitalDistribution> for CCapitalDistributionOwned {
    fn from(cd: CapitalDistribution) -> Self {
        let CapitalDistribution {
            large,
            medium,
            small,
        } = cd;
        CCapitalDistributionOwned {
            large: large.into(),
            medium: medium.into(),
            small: small.into(),
        }
    }
}

impl ToFFI for CCapitalDistributionOwned {
    type FFIType = CCapitalDistribution;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCapitalDistributionOwned {
            large,
            medium,
            small,
        } = self;
        CCapitalDistribution {
            large,
            medium,
            small,
        }
    }
}

/// Capital distribution response
#[repr(C)]
pub struct CCapitalDistributionResponse {
    /// Time
    pub timestamp: i64,
    /// Inflow capital data
    pub capital_in: CCapitalDistribution,
    /// Outflow capital data
    pub capital_out: CCapitalDistribution,
}

pub(crate) struct CCapitalDistributionResponseOwned {
    timestamp: i64,
    capital_in: CCapitalDistributionOwned,
    capital_out: CCapitalDistributionOwned,
}

impl From<CapitalDistributionResponse> for CCapitalDistributionResponseOwned {
    fn from(resp: CapitalDistributionResponse) -> Self {
        let CapitalDistributionResponse {
            timestamp,
            capital_in,
            capital_out,
        } = resp;
        CCapitalDistributionResponseOwned {
            timestamp: timestamp.unix_timestamp(),
            capital_in: capital_in.into(),
            capital_out: capital_out.into(),
        }
    }
}

impl ToFFI for CCapitalDistributionResponseOwned {
    type FFIType = CCapitalDistributionResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCapitalDistributionResponseOwned {
            timestamp,
            capital_in,
            capital_out,
        } = self;
        CCapitalDistributionResponse {
            timestamp: *timestamp,
            capital_in: capital_in.to_ffi_type(),
            capital_out: capital_out.to_ffi_type(),
        }
    }
}

/// Real-time quote
#[repr(C)]
pub struct CRealtimeQuote {
    /// Security code
    pub symbol: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Open
    pub open: *const CDecimal,
    /// High
    pub high: *const CDecimal,
    /// Low
    pub low: *const CDecimal,
    /// Time of latest price
    pub timestamp: i64,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Security trading status
    pub trade_status: CTradeStatus,
}

/// Real-time quote
#[derive(Debug)]
pub(crate) struct CRealtimeQuoteOwned {
    symbol: CString,
    last_done: CDecimal,
    open: CDecimal,
    high: CDecimal,
    low: CDecimal,
    timestamp: i64,
    volume: i64,
    turnover: CDecimal,
    trade_status: TradeStatus,
}

impl From<RealtimeQuote> for CRealtimeQuoteOwned {
    fn from(resp: RealtimeQuote) -> Self {
        let RealtimeQuote {
            symbol,
            last_done,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
        } = resp;
        CRealtimeQuoteOwned {
            symbol: symbol.into(),
            last_done: last_done.into(),
            open: open.into(),
            high: high.into(),
            low: low.into(),
            timestamp: timestamp.unix_timestamp(),
            volume,
            turnover: turnover.into(),
            trade_status,
        }
    }
}

impl ToFFI for CRealtimeQuoteOwned {
    type FFIType = CRealtimeQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CRealtimeQuoteOwned {
            symbol,
            last_done,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            trade_status,
        } = self;
        CRealtimeQuote {
            symbol: symbol.to_ffi_type(),
            last_done: last_done.to_ffi_type(),
            open: open.to_ffi_type(),
            high: high.to_ffi_type(),
            low: low.to_ffi_type(),
            timestamp: *timestamp,
            volume: *volume,
            turnover: turnover.to_ffi_type(),
            trade_status: (*trade_status).into(),
        }
    }
}

/// Watchlist security
#[repr(C)]
pub struct CWatchlistSecurity {
    /// Security symbol
    pub symbol: *const c_char,
    /// Market
    pub market: CMarket,
    /// Security name
    pub name: *const c_char,
    /// Watched price (maybe null)
    pub watched_price: *const CDecimal,
    /// Watched time
    pub watched_at: i64,
}

#[derive(Debug)]
pub(crate) struct CWatchlistSecurityOwned {
    symbol: CString,
    market: CMarket,
    name: CString,
    watched_price: Option<CDecimal>,
    watched_at: i64,
}

impl From<WatchlistSecurity> for CWatchlistSecurityOwned {
    fn from(resp: WatchlistSecurity) -> Self {
        let WatchlistSecurity {
            symbol,
            market,
            name,
            watched_price,
            watched_at,
        } = resp;
        CWatchlistSecurityOwned {
            symbol: symbol.into(),
            market: market.into(),
            name: name.into(),
            watched_price: watched_price.map(Into::into),
            watched_at: watched_at.unix_timestamp(),
        }
    }
}

impl ToFFI for CWatchlistSecurityOwned {
    type FFIType = CWatchlistSecurity;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWatchlistSecurityOwned {
            symbol,
            market,
            name,
            watched_price,
            watched_at,
        } = self;
        CWatchlistSecurity {
            symbol: symbol.to_ffi_type(),
            market: *market,
            name: name.to_ffi_type(),
            watched_price: watched_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            watched_at: *watched_at,
        }
    }
}

/// Watchlist group
#[repr(C)]
pub struct CWatchlistGroup {
    /// Group id
    pub id: i64,
    /// Group name
    pub name: *const c_char,
    /// Securities
    pub securities: *const CWatchlistSecurity,
    /// Number of securities
    pub num_securities: usize,
}

#[derive(Debug)]
pub(crate) struct CWatchlistGroupOwned {
    id: i64,
    name: CString,
    securities: CVec<CWatchlistSecurityOwned>,
}

impl From<WatchlistGroup> for CWatchlistGroupOwned {
    fn from(resp: WatchlistGroup) -> Self {
        let WatchlistGroup {
            id,
            name,
            securities,
        } = resp;
        CWatchlistGroupOwned {
            id,
            name: name.into(),
            securities: securities.into(),
        }
    }
}

impl ToFFI for CWatchlistGroupOwned {
    type FFIType = CWatchlistGroup;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWatchlistGroupOwned {
            id,
            name,
            securities,
        } = self;
        CWatchlistGroup {
            id: *id,
            name: name.to_ffi_type(),
            securities: securities.to_ffi_type(),
            num_securities: securities.len(),
        }
    }
}

/// An request to create a watchlist group
#[derive(Debug)]
#[repr(C)]
pub struct CCreateWatchlistGroup {
    /// Group name
    pub name: *const c_char,
    /// Securities
    pub securities: *const *const c_char,
    /// Number of securities
    pub num_securities: usize,
}

/// Update name of watchlist group
pub const LB_WATCHLIST_GROUP_NAME: u32 = 0x1;

/// Update securities of watchlist group
pub const LB_WATCHLIST_GROUP_SECURITIES: u32 = 0x2;

/// An request to update a watchlist group
#[derive(Debug)]
#[repr(C)]
pub struct CUpdateWatchlistGroup {
    /// Flags
    pub flags: u32,
    /// Group id
    pub id: i64,
    /// Group name
    pub name: *const c_char,
    /// Securities
    pub securities: *const *const c_char,
    /// Number of securities
    pub num_securities: usize,
    /// Securities update mode
    pub mode: CSecuritiesUpdateMode,
}

/// Security calc index response
#[repr(C)]
pub struct CSecurityCalcIndex {
    /// Security code
    pub symbol: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Change value
    pub change_value: *const CDecimal,
    /// Change ratio
    pub change_rate: *const CDecimal,
    /// Volume
    pub volume: *const i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Year-to-date change ratio
    pub ytd_change_rate: *const CDecimal,
    /// Turnover rate
    pub turnover_rate: *const CDecimal,
    /// Total market value
    pub total_market_value: *const CDecimal,
    /// Capital flow
    pub capital_flow: *const CDecimal,
    /// Amplitude
    pub amplitude: *const CDecimal,
    /// Volume ratio
    pub volume_ratio: *const CDecimal,
    /// PE (TTM)
    pub pe_ttm_ratio: *const CDecimal,
    /// PB
    pub pb_ratio: *const CDecimal,
    /// Dividend ratio (TTM)
    pub dividend_ratio_ttm: *const CDecimal,
    /// Five days change ratio
    pub five_day_change_rate: *const CDecimal,
    /// Ten days change ratio
    pub ten_day_change_rate: *const CDecimal,
    /// Half year change ratio
    pub half_year_change_rate: *const CDecimal,
    /// Five minutes change ratio
    pub five_minutes_change_rate: *const CDecimal,
    /// Expiry date
    pub expiry_date: *const CDate,
    /// Strike price
    pub strike_price: *const CDecimal,
    /// Upper bound price
    pub upper_strike_price: *const CDecimal,
    /// Lower bound price
    pub lower_strike_price: *const CDecimal,
    /// Outstanding quantity
    pub outstanding_qty: *const i64,
    /// Outstanding ratio
    pub outstanding_ratio: *const CDecimal,
    /// Premium
    pub premium: *const CDecimal,
    /// In/out of the bound
    pub itm_otm: *const CDecimal,
    /// Implied volatility
    pub implied_volatility: *const CDecimal,
    /// Warrant delta
    pub warrant_delta: *const CDecimal,
    /// Call price
    pub call_price: *const CDecimal,
    /// Price interval from the call price
    pub to_call_price: *const CDecimal,
    /// Effective leverage
    pub effective_leverage: *const CDecimal,
    /// Leverage ratio
    pub leverage_ratio: *const CDecimal,
    /// Conversion ratio
    pub conversion_ratio: *const CDecimal,
    /// Breakeven point
    pub balance_point: *const CDecimal,
    /// Open interest
    pub open_interest: *const i64,
    /// Delta
    pub delta: *const CDecimal,
    /// Gamma
    pub gamma: *const CDecimal,
    /// Theta
    pub theta: *const CDecimal,
    /// Vega
    pub vega: *const CDecimal,
    /// Rho
    pub rho: *const CDecimal,
}

pub(crate) struct CSecurityCalcIndexOwned {
    /// Security code
    symbol: CString,
    /// Latest price
    last_done: COption<CDecimal>,
    /// Change value
    change_value: COption<CDecimal>,
    /// Change ratio
    change_rate: COption<CDecimal>,
    /// Volume
    volume: COption<i64>,
    /// Turnover
    turnover: COption<CDecimal>,
    /// Year-to-date change ratio
    ytd_change_rate: COption<CDecimal>,
    /// Turnover rate
    turnover_rate: COption<CDecimal>,
    /// Total market value
    total_market_value: COption<CDecimal>,
    /// Capital flow
    capital_flow: COption<CDecimal>,
    /// Amplitude
    amplitude: COption<CDecimal>,
    /// Volume ratio
    volume_ratio: COption<CDecimal>,
    /// PE (TTM)
    pe_ttm_ratio: COption<CDecimal>,
    /// PB
    pb_ratio: COption<CDecimal>,
    /// Dividend ratio (TTM)
    dividend_ratio_ttm: COption<CDecimal>,
    /// Five days change ratio
    five_day_change_rate: COption<CDecimal>,
    /// Ten days change ratio
    ten_day_change_rate: COption<CDecimal>,
    /// Half year change ratio
    half_year_change_rate: COption<CDecimal>,
    /// Five minutes change ratio
    five_minutes_change_rate: COption<CDecimal>,
    /// Expiry date
    expiry_date: COption<CDate>,
    /// Strike price
    strike_price: COption<CDecimal>,
    /// Upper bound price
    upper_strike_price: COption<CDecimal>,
    /// Lower bound price
    lower_strike_price: COption<CDecimal>,
    /// Outstanding quantity
    outstanding_qty: COption<i64>,
    /// Outstanding ratio
    outstanding_ratio: COption<CDecimal>,
    /// Premium
    premium: COption<CDecimal>,
    /// In/out of the bound
    itm_otm: COption<CDecimal>,
    /// Implied volatility
    implied_volatility: COption<CDecimal>,
    /// Warrant delta
    warrant_delta: COption<CDecimal>,
    /// Call price
    call_price: COption<CDecimal>,
    /// Price interval from the call price
    to_call_price: COption<CDecimal>,
    /// Effective leverage
    effective_leverage: COption<CDecimal>,
    /// Leverage ratio
    leverage_ratio: COption<CDecimal>,
    /// Conversion ratio
    conversion_ratio: COption<CDecimal>,
    /// Breakeven point
    balance_point: COption<CDecimal>,
    /// Open interest
    open_interest: COption<i64>,
    /// Delta
    delta: COption<CDecimal>,
    /// Gamma
    gamma: COption<CDecimal>,
    /// Theta
    theta: COption<CDecimal>,
    /// Vega
    vega: COption<CDecimal>,
    /// Rho
    rho: COption<CDecimal>,
}

impl From<SecurityCalcIndex> for CSecurityCalcIndexOwned {
    fn from(resp: SecurityCalcIndex) -> Self {
        let SecurityCalcIndex {
            symbol,
            last_done,
            change_value,
            change_rate,
            volume,
            turnover,
            ytd_change_rate,
            turnover_rate,
            total_market_value,
            capital_flow,
            amplitude,
            volume_ratio,
            pe_ttm_ratio,
            pb_ratio,
            dividend_ratio_ttm,
            five_day_change_rate,
            ten_day_change_rate,
            half_year_change_rate,
            five_minutes_change_rate,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty,
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            warrant_delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            open_interest,
            delta,
            gamma,
            theta,
            vega,
            rho,
        } = resp;
        CSecurityCalcIndexOwned {
            symbol: symbol.into(),
            last_done: last_done.into(),
            change_value: change_value.into(),
            change_rate: change_rate.into(),
            volume: volume.into(),
            turnover: turnover.into(),
            ytd_change_rate: ytd_change_rate.into(),
            turnover_rate: turnover_rate.into(),
            total_market_value: total_market_value.into(),
            capital_flow: capital_flow.into(),
            amplitude: amplitude.into(),
            volume_ratio: volume_ratio.into(),
            pe_ttm_ratio: pe_ttm_ratio.into(),
            pb_ratio: pb_ratio.into(),
            dividend_ratio_ttm: dividend_ratio_ttm.into(),
            five_day_change_rate: five_day_change_rate.into(),
            ten_day_change_rate: ten_day_change_rate.into(),
            half_year_change_rate: half_year_change_rate.into(),
            five_minutes_change_rate: five_minutes_change_rate.into(),
            expiry_date: expiry_date.into(),
            strike_price: strike_price.into(),
            upper_strike_price: upper_strike_price.into(),
            lower_strike_price: lower_strike_price.into(),
            outstanding_qty: outstanding_qty.into(),
            outstanding_ratio: outstanding_ratio.into(),
            premium: premium.into(),
            itm_otm: itm_otm.into(),
            implied_volatility: implied_volatility.into(),
            warrant_delta: warrant_delta.into(),
            call_price: call_price.into(),
            to_call_price: to_call_price.into(),
            effective_leverage: effective_leverage.into(),
            leverage_ratio: leverage_ratio.into(),
            conversion_ratio: conversion_ratio.into(),
            balance_point: balance_point.into(),
            open_interest: open_interest.into(),
            delta: delta.into(),
            gamma: gamma.into(),
            theta: theta.into(),
            vega: vega.into(),
            rho: rho.into(),
        }
    }
}

impl ToFFI for CSecurityCalcIndexOwned {
    type FFIType = CSecurityCalcIndex;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSecurityCalcIndexOwned {
            symbol,
            last_done,
            change_value,
            change_rate,
            volume,
            turnover,
            ytd_change_rate,
            turnover_rate,
            total_market_value,
            capital_flow,
            amplitude,
            volume_ratio,
            pe_ttm_ratio,
            pb_ratio,
            dividend_ratio_ttm,
            five_day_change_rate,
            ten_day_change_rate,
            half_year_change_rate,
            five_minutes_change_rate,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty,
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            warrant_delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            open_interest,
            delta,
            gamma,
            theta,
            vega,
            rho,
        } = self;
        CSecurityCalcIndex {
            symbol: symbol.to_ffi_type(),
            last_done: last_done.to_ffi_type().to_ffi_type(),
            change_value: change_value.to_ffi_type().to_ffi_type(),
            change_rate: change_rate.to_ffi_type().to_ffi_type(),
            volume: volume.to_ffi_type(),
            turnover: turnover.to_ffi_type().to_ffi_type(),
            ytd_change_rate: ytd_change_rate.to_ffi_type().to_ffi_type(),
            turnover_rate: turnover_rate.to_ffi_type().to_ffi_type(),
            total_market_value: total_market_value.to_ffi_type().to_ffi_type(),
            capital_flow: capital_flow.to_ffi_type().to_ffi_type(),
            amplitude: amplitude.to_ffi_type().to_ffi_type(),
            volume_ratio: volume_ratio.to_ffi_type().to_ffi_type(),
            pe_ttm_ratio: pe_ttm_ratio.to_ffi_type().to_ffi_type(),
            pb_ratio: pb_ratio.to_ffi_type().to_ffi_type(),
            dividend_ratio_ttm: dividend_ratio_ttm.to_ffi_type().to_ffi_type(),
            five_day_change_rate: five_day_change_rate.to_ffi_type().to_ffi_type(),
            ten_day_change_rate: ten_day_change_rate.to_ffi_type().to_ffi_type(),
            half_year_change_rate: half_year_change_rate.to_ffi_type().to_ffi_type(),
            five_minutes_change_rate: five_minutes_change_rate.to_ffi_type().to_ffi_type(),
            expiry_date: expiry_date.to_ffi_type(),
            strike_price: strike_price.to_ffi_type().to_ffi_type(),
            upper_strike_price: upper_strike_price.to_ffi_type().to_ffi_type(),
            lower_strike_price: lower_strike_price.to_ffi_type().to_ffi_type(),
            outstanding_qty: outstanding_qty.to_ffi_type(),
            outstanding_ratio: outstanding_ratio.to_ffi_type().to_ffi_type(),
            premium: premium.to_ffi_type().to_ffi_type(),
            itm_otm: itm_otm.to_ffi_type().to_ffi_type(),
            implied_volatility: implied_volatility.to_ffi_type().to_ffi_type(),
            warrant_delta: warrant_delta.to_ffi_type().to_ffi_type(),
            call_price: call_price.to_ffi_type().to_ffi_type(),
            to_call_price: to_call_price.to_ffi_type().to_ffi_type(),
            effective_leverage: effective_leverage.to_ffi_type().to_ffi_type(),
            leverage_ratio: leverage_ratio.to_ffi_type().to_ffi_type(),
            conversion_ratio: conversion_ratio.to_ffi_type().to_ffi_type(),
            balance_point: balance_point.to_ffi_type().to_ffi_type(),
            open_interest: open_interest.to_ffi_type(),
            delta: delta.to_ffi_type().to_ffi_type(),
            gamma: gamma.to_ffi_type().to_ffi_type(),
            theta: theta.to_ffi_type().to_ffi_type(),
            vega: vega.to_ffi_type().to_ffi_type(),
            rho: rho.to_ffi_type().to_ffi_type(),
        }
    }
}

/// Warrant info
#[repr(C)]
pub struct CWarrantInfo {
    /// Security code
    pub symbol: *const c_char,
    /// Warrant type
    pub warrant_type: CWarrantType,
    /// Security name
    pub name: *const c_char,
    /// Latest price
    pub last_done: *const CDecimal,
    /// Quote change rate
    pub change_rate: *const CDecimal,
    /// Quote change
    pub change_value: *const CDecimal,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: *const CDecimal,
    /// Expiry date
    pub expiry_date: CDate,
    /// Strike price
    pub strike_price: *const CDecimal,
    /// Upper strike price
    pub upper_strike_price: *const CDecimal,
    /// Lower strike price
    pub lower_strike_price: *const CDecimal,
    /// Outstanding quantity
    pub outstanding_qty: i64,
    /// Outstanding ratio
    pub outstanding_ratio: *const CDecimal,
    /// Premium
    pub premium: *const CDecimal,
    /// In/out of the bound
    pub itm_otm: *const CDecimal,
    /// Implied volatility
    pub implied_volatility: *const CDecimal,
    /// Delta
    pub delta: *const CDecimal,
    /// Call price
    pub call_price: *const CDecimal,
    /// Price interval from the call price
    pub to_call_price: *const CDecimal,
    /// Effective leverage
    pub effective_leverage: *const CDecimal,
    /// Leverage ratio
    pub leverage_ratio: *const CDecimal,
    /// Conversion ratio
    pub conversion_ratio: *const CDecimal,
    /// Breakeven point
    pub balance_point: *const CDecimal,
    /// Status
    pub status: CWarrantStatus,
}

pub(crate) struct CWarrantInfoOwned {
    /// Security code
    pub symbol: CString,
    /// Warrant type
    pub warrant_type: CWarrantType,
    /// Security name
    pub name: CString,
    /// Latest price
    pub last_done: CDecimal,
    /// Quote change rate
    pub change_rate: CDecimal,
    /// Quote change
    pub change_value: CDecimal,
    /// Volume
    pub volume: i64,
    /// Turnover
    pub turnover: CDecimal,
    /// Expiry date
    pub expiry_date: CDate,
    /// Strike price
    pub strike_price: Option<CDecimal>,
    /// Upper strike price
    pub upper_strike_price: Option<CDecimal>,
    /// Lower strike price
    pub lower_strike_price: Option<CDecimal>,
    /// Outstanding quantity
    pub outstanding_qty: i64,
    /// Outstanding ratio
    pub outstanding_ratio: CDecimal,
    /// Premium
    pub premium: CDecimal,
    /// In/out of the bound
    pub itm_otm: Option<CDecimal>,
    /// Implied volatility
    pub implied_volatility: Option<CDecimal>,
    /// Delta
    pub delta: Option<CDecimal>,
    /// Call price
    pub call_price: Option<CDecimal>,
    /// Price interval from the call price
    pub to_call_price: Option<CDecimal>,
    /// Effective leverage
    pub effective_leverage: Option<CDecimal>,
    /// Leverage ratio
    pub leverage_ratio: CDecimal,
    /// Conversion ratio
    pub conversion_ratio: Option<CDecimal>,
    /// Breakeven point
    pub balance_point: Option<CDecimal>,
    /// Status
    pub status: CWarrantStatus,
}

impl ToFFI for CWarrantInfoOwned {
    type FFIType = CWarrantInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWarrantInfoOwned {
            symbol,
            warrant_type,
            name,
            last_done,
            change_rate,
            change_value,
            volume,
            turnover,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty,
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            status,
        } = self;
        CWarrantInfo {
            symbol: symbol.to_ffi_type(),
            warrant_type: *warrant_type,
            name: name.to_ffi_type(),
            last_done: last_done.to_ffi_type(),
            change_rate: change_rate.to_ffi_type(),
            change_value: change_value.to_ffi_type(),
            volume: *volume,
            turnover: turnover.to_ffi_type(),
            expiry_date: expiry_date.to_ffi_type(),
            strike_price: strike_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            upper_strike_price: upper_strike_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            lower_strike_price: lower_strike_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            outstanding_qty: *outstanding_qty,
            outstanding_ratio: outstanding_ratio.to_ffi_type(),
            premium: premium.to_ffi_type(),
            itm_otm: itm_otm
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            implied_volatility: implied_volatility
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            delta: delta
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            call_price: call_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            to_call_price: to_call_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            effective_leverage: effective_leverage
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            leverage_ratio: leverage_ratio.to_ffi_type(),
            conversion_ratio: conversion_ratio
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            balance_point: balance_point
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            status: *status,
        }
    }
}

impl From<WarrantInfo> for CWarrantInfoOwned {
    fn from(resp: WarrantInfo) -> Self {
        let WarrantInfo {
            symbol,
            warrant_type,
            name,
            last_done,
            change_rate,
            change_value,
            volume,
            turnover,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty,
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            status,
        } = resp;
        CWarrantInfoOwned {
            symbol: symbol.into(),
            warrant_type: warrant_type.into(),
            name: name.into(),
            last_done: last_done.into(),
            change_rate: change_rate.into(),
            change_value: change_value.into(),
            volume,
            turnover: turnover.into(),
            expiry_date: expiry_date.into(),
            strike_price: strike_price.map(Into::into),
            upper_strike_price: upper_strike_price.map(Into::into),
            lower_strike_price: lower_strike_price.map(Into::into),
            outstanding_qty,
            outstanding_ratio: outstanding_ratio.into(),
            premium: premium.into(),
            itm_otm: itm_otm.map(Into::into),
            implied_volatility: implied_volatility.map(Into::into),
            delta: delta.map(Into::into),
            call_price: call_price.map(Into::into),
            to_call_price: to_call_price.map(Into::into),
            effective_leverage: effective_leverage.map(Into::into),
            leverage_ratio: leverage_ratio.into(),
            conversion_ratio: conversion_ratio.map(Into::into),
            balance_point: balance_point.map(Into::into),
            status: status.into(),
        }
    }
}

/// Security
#[repr(C)]
pub struct CSecurity {
    /// Security code
    pub symbol: *const c_char,
    /// Security name (zh-CN)
    pub name_cn: *const c_char,
    /// Security name (en)
    pub name_en: *const c_char,
    /// Security name (zh-HK)
    pub name_hk: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CSecurityOwned {
    pub symbol: CString,
    pub name_cn: CString,
    pub name_en: CString,
    pub name_hk: CString,
}

impl From<Security> for CSecurityOwned {
    fn from(info: Security) -> Self {
        let Security {
            symbol,
            name_cn,
            name_en,
            name_hk,
        } = info;
        CSecurityOwned {
            symbol: symbol.into(),
            name_cn: name_cn.into(),
            name_en: name_en.into(),
            name_hk: name_hk.into(),
        }
    }
}

impl ToFFI for CSecurityOwned {
    type FFIType = CSecurity;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSecurityOwned {
            symbol,
            name_cn,
            name_en,
            name_hk,
        } = self;
        CSecurity {
            symbol: symbol.to_ffi_type(),
            name_cn: name_cn.to_ffi_type(),
            name_en: name_en.to_ffi_type(),
            name_hk: name_hk.to_ffi_type(),
        }
    }
}
