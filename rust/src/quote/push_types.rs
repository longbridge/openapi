use longport_proto::quote::{self, Period, PushQuoteTag, TradeStatus};
use prost::Message;
use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::{
    Error, Result,
    quote::{Brokers, Candlestick, Depth, Trade, TradeSession, cmd_code},
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
    /// Increase volume between pushes
    pub current_volume: i64,
    /// Increase turnover between pushes
    pub current_turnover: Decimal,
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
            current_volume: Default::default(),
            current_turnover: Default::default(),
        }
    }
}

impl longport_candlesticks::QuoteType for PushQuote {
    type PriceType = Decimal;
    type VolumeType = i64;
    type TurnoverType = Decimal;
    type TradeSessionType = TradeSession;

    #[inline]
    fn time(&self) -> OffsetDateTime {
        self.timestamp
    }

    #[inline]
    fn open(&self) -> Self::PriceType {
        self.open
    }

    #[inline]
    fn high(&self) -> Self::PriceType {
        self.high
    }

    #[inline]
    fn low(&self) -> Self::PriceType {
        self.low
    }

    #[inline]
    fn last_done(&self) -> Self::PriceType {
        self.last_done
    }

    #[inline]
    fn volume(&self) -> Self::VolumeType {
        self.volume
    }

    #[inline]
    fn turnover(&self) -> Self::TurnoverType {
        self.turnover
    }

    #[inline]
    fn trade_session(&self) -> Self::TradeSessionType {
        self.trade_session
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
    /// Is confirmed
    pub is_confirmed: bool,
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
    #[allow(dead_code)]
    pub(crate) sequence: i64,
    /// Security code
    pub symbol: String,
    /// Event detail
    pub detail: PushEventDetail,
}

impl PushEvent {
    pub(crate) fn parse(
        command_code: u8,
        data: &[u8],
    ) -> Result<(PushEvent, Option<PushQuoteTag>)> {
        match command_code {
            cmd_code::PUSH_REALTIME_QUOTE => {
                parse_push_quote(data).map(|(event, tag)| (event, Some(tag)))
            }
            cmd_code::PUSH_REALTIME_DEPTH => parse_push_depth(data).map(|event| (event, None)),
            cmd_code::PUSH_REALTIME_BROKERS => parse_push_brokers(data).map(|event| (event, None)),
            cmd_code::PUSH_REALTIME_TRADES => parse_push_trade(data).map(|event| (event, None)),
            _ => Err(Error::UnknownCommand(command_code)),
        }
    }
}

fn parse_push_quote(data: &[u8]) -> Result<(PushEvent, PushQuoteTag)> {
    let push_quote = quote::PushQuote::decode(data)?;
    Ok((
        PushEvent {
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
                trade_status: TradeStatus::try_from(push_quote.trade_status).unwrap_or_default(),
                trade_session: longport_proto::quote::TradeSession::try_from(
                    push_quote.trade_session,
                )
                .unwrap_or_default()
                .into(),
                current_volume: push_quote.current_volume,
                current_turnover: push_quote.current_turnover.parse().unwrap_or_default(),
            }),
        },
        PushQuoteTag::try_from(push_quote.tag).unwrap_or_default(),
    ))
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
