use longbridge_proto::quote::{self, Period, TradeSession, TradeStatus};
use prost::Message;
use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::{
    quote::{cmd_code, Brokers, Candlestick, Depth, Trade},
    Error, Result,
};

/// Quote message
#[derive(Debug, Clone)]
pub struct PushQuote {
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
    /// Trade session
    pub trade_session: TradeSession,
}

impl Default for PushQuote {
    fn default() -> Self {
        Self {
            last_done: Default::default(),
            open: Default::default(),
            high: Default::default(),
            low: Default::default(),
            timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            volume: Default::default(),
            turnover: Default::default(),
            trade_status: Default::default(),
            trade_session: Default::default(),
        }
    }
}

/// Depth message
#[derive(Debug)]
pub struct PushDepth {
    /// Ask depth
    pub asks: Vec<Depth>,
    /// Bid depth
    pub bids: Vec<Depth>,
}

/// Brokers message
#[derive(Debug)]
pub struct PushBrokers {
    /// Ask brokers
    pub ask_brokers: Vec<Brokers>,
    /// Bid brokers
    pub bid_brokers: Vec<Brokers>,
}

/// Trades message
#[derive(Debug)]
pub struct PushTrades {
    /// Trades data
    pub trades: Vec<Trade>,
}

/// Candlestick updated message
#[derive(Debug, Copy, Clone)]
pub struct PushCandlestick {
    /// Period type
    pub period: Period,
    /// Candlestick
    pub candlestick: Candlestick,
}

/// Push event detail
#[derive(Debug)]
pub enum PushEventDetail {
    /// Quote
    Quote(PushQuote),
    /// Depth
    Depth(PushDepth),
    /// Brokers
    Brokers(PushBrokers),
    /// Trade
    Trade(PushTrades),
    /// Candlestick
    Candlestick(PushCandlestick),
}

/// Push event
#[derive(Debug)]
pub struct PushEvent {
    pub(crate) sequence: i64,
    /// Security code
    pub symbol: String,
    /// Event detail
    pub detail: PushEventDetail,
}

impl PushEvent {
    pub(crate) fn parse(command_code: u8, data: &[u8]) -> Result<PushEvent> {
        match command_code {
            cmd_code::PUSH_REALTIME_QUOTE => parse_push_quote(data),
            cmd_code::PUSH_REALTIME_DEPTH => parse_push_depth(data),
            cmd_code::PUSH_REALTIME_BROKERS => parse_push_brokers(data),
            cmd_code::PUSH_REALTIME_TRADES => parse_push_trade(data),
            _ => Err(Error::UnknownCommand(command_code)),
        }
    }
}

fn parse_push_quote(data: &[u8]) -> Result<PushEvent> {
    let push_quote = quote::PushQuote::decode(data)?;
    Ok(PushEvent {
        symbol: push_quote.symbol,
        sequence: push_quote.sequence,
        detail: PushEventDetail::Quote(PushQuote {
            last_done: push_quote.last_done.parse().unwrap_or_default(),
            open: push_quote.open.parse().unwrap_or_default(),
            high: push_quote.high.parse().unwrap_or_default(),
            low: push_quote.low.parse().unwrap_or_default(),
            timestamp: OffsetDateTime::from_unix_timestamp(push_quote.timestamp)
                .map_err(|err| Error::parse_field_error("timestamp", err))?,
            volume: push_quote.volume,
            turnover: push_quote.turnover.parse().unwrap_or_default(),
            trade_status: TradeStatus::from_i32(push_quote.trade_status).unwrap_or_default(),
            trade_session: TradeSession::from_i32(push_quote.trade_session).unwrap_or_default(),
        }),
    })
}

fn parse_push_depth(data: &[u8]) -> Result<PushEvent> {
    let push_depth = quote::PushDepth::decode(data)?;
    Ok(PushEvent {
        symbol: push_depth.symbol,
        sequence: push_depth.sequence,
        detail: PushEventDetail::Depth(PushDepth {
            asks: push_depth
                .ask
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            bids: push_depth
                .bid
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
        }),
    })
}

fn parse_push_brokers(data: &[u8]) -> Result<PushEvent> {
    let push_brokers = quote::PushBrokers::decode(data)?;

    Ok(PushEvent {
        symbol: push_brokers.symbol,
        sequence: push_brokers.sequence,
        detail: PushEventDetail::Brokers(PushBrokers {
            ask_brokers: push_brokers
                .ask_brokers
                .into_iter()
                .map(Into::into)
                .collect(),
            bid_brokers: push_brokers
                .bid_brokers
                .into_iter()
                .map(Into::into)
                .collect(),
        }),
    })
}

fn parse_push_trade(data: &[u8]) -> Result<PushEvent> {
    let push_trades = quote::PushTrade::decode(data)?;
    Ok(PushEvent {
        symbol: push_trades.symbol,
        sequence: push_trades.sequence,
        detail: PushEventDetail::Trade(PushTrades {
            trades: push_trades
                .trade
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
        }),
    })
}
