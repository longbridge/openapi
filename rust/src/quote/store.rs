use std::collections::HashMap;

use longport_candlesticks::{Days, UpdateAction, UpdateFields};
use longport_proto::quote::Period;

use crate::{
    Market,
    quote::{
        Brokers, Candlestick, Depth, PushBrokers, PushDepth, PushEvent, PushTrades, SecurityBoard,
        Trade, TradeDirection, TradeSession, TradeSessions,
        push_types::{PushEventDetail, PushQuote},
    },
};

const MAX_TRADES: usize = 500;
const MAX_CANDLESTICKS: usize = 500;

macro_rules! merge_decimal {
    ($prev:expr, $new:expr, $field:ident) => {
        if !$new.$field.is_zero() {
            $new.$field
        } else {
            $prev.$field
        }
    };
}

macro_rules! merge_i64 {
    ($prev:expr, $new:expr, $field:ident) => {
        if $new.$field != 0 {
            $new.$field
        } else {
            $prev.$field
        }
    };
}

#[derive(Debug)]
pub(crate) struct TailCandlestick {
    pub(crate) index: usize,
    pub(crate) candlestick: Candlestick,
}

#[derive(Debug)]
pub(crate) struct Candlesticks {
    pub(crate) trade_sessions: TradeSessions,
    pub(crate) candlesticks: Vec<Candlestick>,
    pub(crate) tails: HashMap<TradeSession, TailCandlestick>,
}

impl Candlesticks {
    #[inline]
    fn merge_input(&self, ts: TradeSession) -> Option<Candlestick> {
        self.tails.get(&ts).map(|tail| tail.candlestick)
    }

    pub(crate) fn insert_candlestick_by_time(&mut self, candlestick: Candlestick) -> usize {
        let timestamp = candlestick.timestamp;
        match self
            .candlesticks
            .binary_search_by_key(&timestamp, |c| c.timestamp)
        {
            Ok(index) => {
                self.candlesticks[index] = candlestick;
                index
            }
            Err(index) => {
                self.candlesticks.insert(index, candlestick);
                index
            }
        }
    }

    pub(crate) fn merge_trade<H>(
        &mut self,
        market_type: Market,
        half_days: H,
        board: SecurityBoard,
        period: Period,
        trade: &Trade,
    ) -> UpdateAction<Candlestick>
    where
        H: Days,
    {
        let Some(market) = get_market(market_type, board) else {
            return UpdateAction::None;
        };
        let ts = trade.trade_session;
        let period = convert_period(period);

        let trade_type = trade.trade_type.as_str();
        let update_fields = match market_type {
            Market::Unknown => unreachable!(),
            Market::HK => match trade_type {
                "" => UpdateFields::all(),
                "D" => UpdateFields::VOLUME,
                "M" => UpdateFields::VOLUME,
                "P" => UpdateFields::VOLUME,
                "U" => UpdateFields::all(),
                "X" => UpdateFields::VOLUME,
                "Y" => UpdateFields::VOLUME,
                _ => UpdateFields::empty(),
            },
            Market::US => match trade_type {
                "" => UpdateFields::all(),
                "A" => UpdateFields::all(),
                "B" => UpdateFields::all(),
                "C" => UpdateFields::VOLUME,
                "D" => UpdateFields::all(),
                "E" => UpdateFields::all(),
                "F" => UpdateFields::all(),
                "G" => UpdateFields::VOLUME,
                "H" => UpdateFields::VOLUME,
                "I" if board == SecurityBoard::USOption || board == SecurityBoard::USOptionS => {
                    UpdateFields::all()
                }
                "I" if ts == TradeSession::Pre || ts == TradeSession::Post => UpdateFields::all(),
                "I" => UpdateFields::VOLUME,
                "K" => UpdateFields::all(),
                "M" => UpdateFields::empty(),
                "P" => UpdateFields::empty(),
                "S" => UpdateFields::all(),
                "V" => UpdateFields::VOLUME,
                "W" => UpdateFields::VOLUME,
                "X" => UpdateFields::all(),
                "1" => UpdateFields::all(),
                _ => UpdateFields::empty(),
            },
            Market::CN | Market::SG | Market::Crypto => UpdateFields::all(),
        };

        market.merge_trade(
            half_days,
            period,
            self.merge_input(ts),
            trade,
            update_fields,
        )
    }

    pub(crate) fn merge_quote_day<H>(
        &mut self,
        market_type: Market,
        half_days: H,
        board: SecurityBoard,
        period: Period,
        push_quote: &PushQuote,
    ) -> UpdateAction<Candlestick>
    where
        H: Days,
    {
        debug_assert!(period == Period::Day);

        let Some(market) = get_market(market_type, board) else {
            return UpdateAction::None;
        };
        let ts = push_quote.trade_session;
        let period = convert_period(period);
        market.merge_quote_day(half_days, period, self.merge_input(ts), push_quote)
    }

    pub(crate) fn merge_quote<H>(
        &mut self,
        market_type: Market,
        half_days: H,
        board: SecurityBoard,
        period: Period,
        push_quote: &PushQuote,
    ) -> UpdateAction<Candlestick>
    where
        H: Days,
    {
        let Some(market) = get_market(market_type, board) else {
            return UpdateAction::None;
        };
        let ts = push_quote.trade_session;
        let period = convert_period(period);
        market.merge_trade(
            half_days,
            period,
            self.merge_input(ts),
            &Trade {
                price: push_quote.last_done,
                volume: push_quote.current_volume,
                timestamp: push_quote.timestamp,
                trade_type: String::new(),
                direction: TradeDirection::Neutral,
                trade_session: push_quote.trade_session,
            },
            UpdateFields::all(),
        )
    }

    pub(crate) fn check_and_remove(&mut self) {
        if self.candlesticks.len() <= MAX_CANDLESTICKS * 2 {
            return;
        }

        let remove_count = self.candlesticks.len() - MAX_CANDLESTICKS;
        let mut remove_tails = vec![];

        for (ts, tail) in &mut self.tails {
            if tail.index >= remove_count {
                tail.index -= remove_count;
            } else {
                remove_tails.push(*ts);
            }
        }

        for ts in remove_tails {
            self.tails.remove(&ts);
        }

        self.candlesticks.drain(..remove_count);
    }
}

#[derive(Debug, Default)]
pub(crate) struct SecuritiesData {
    pub(crate) quote: PushQuote,

    pub(crate) asks: Vec<Depth>,
    pub(crate) bids: Vec<Depth>,

    pub(crate) ask_brokers: Vec<Brokers>,
    pub(crate) bid_brokers: Vec<Brokers>,

    pub(crate) trades: Vec<Trade>,

    pub(crate) board: SecurityBoard,
    pub(crate) candlesticks: HashMap<Period, Candlesticks>,
}

#[derive(Debug, Default)]
pub(crate) struct Store {
    pub(crate) securities: HashMap<String, SecuritiesData>,
}

impl Store {
    pub(crate) fn handle_push(&mut self, event: &mut PushEvent) {
        let data = self.securities.entry(event.symbol.clone()).or_default();

        match &mut event.detail {
            PushEventDetail::Quote(quote) => merge_quote(data, quote),
            PushEventDetail::Depth(depth) => merge_depth(data, depth),
            PushEventDetail::Brokers(brokers) => merge_brokers(data, brokers),
            PushEventDetail::Trade(trade) => merge_trades(data, trade),
            PushEventDetail::Candlestick(_) => unreachable!(),
        }
    }
}

fn merge_quote(data: &mut SecuritiesData, quote: &mut PushQuote) {
    let prev_quote = &data.quote;
    let new_quote = PushQuote {
        last_done: merge_decimal!(prev_quote, quote, last_done),
        open: merge_decimal!(prev_quote, quote, open),
        high: merge_decimal!(prev_quote, quote, high),
        low: merge_decimal!(prev_quote, quote, low),
        timestamp: quote.timestamp,
        volume: merge_i64!(prev_quote, quote, volume),
        turnover: merge_decimal!(prev_quote, quote, turnover),
        trade_status: quote.trade_status,
        trade_session: quote.trade_session,
        current_volume: quote.current_volume,
        current_turnover: quote.current_turnover,
    };
    data.quote = new_quote.clone();
    *quote = new_quote;
}

fn merge_depth(data: &mut SecuritiesData, depth: &PushDepth) {
    replace(&mut data.asks, depth.asks.clone(), |v| v.position);
    replace(&mut data.bids, depth.bids.clone(), |v| v.position);
}

fn merge_brokers(data: &mut SecuritiesData, brokers: &PushBrokers) {
    replace(&mut data.ask_brokers, brokers.ask_brokers.clone(), |v| {
        v.position
    });
    replace(&mut data.bid_brokers, brokers.bid_brokers.clone(), |v| {
        v.position
    });
}

fn merge_trades(data: &mut SecuritiesData, trades: &PushTrades) {
    data.trades.extend(trades.trades.clone());
    if data.trades.len() > MAX_TRADES * 2 {
        data.trades.drain(..MAX_TRADES);
    }
}

fn replace<T, B, F>(elements: &mut Vec<T>, others: Vec<T>, f: F)
where
    F: Fn(&T) -> B,
    B: Ord,
{
    for v in others {
        match elements.binary_search_by_key(&f(&v), &f) {
            Ok(index) => elements[index] = v,
            Err(index) => elements.insert(index, v),
        }
    }
}

pub(crate) fn get_market(
    market: Market,
    board: SecurityBoard,
) -> Option<&'static longport_candlesticks::Market> {
    use longport_candlesticks::markets::*;

    Some(match market {
        Market::US if board == SecurityBoard::USOptionS => &US_OPTION,
        Market::US => &US,
        Market::HK => &HK,
        Market::SG => &SG,
        Market::CN => &CN,
        Market::Unknown | Market::Crypto => return None,
    })
}

fn convert_period(period: Period) -> longport_candlesticks::Period {
    use longport_candlesticks::Period::*;

    match period {
        Period::UnknownPeriod => unreachable!(),
        Period::OneMinute => Min_1,
        Period::TwoMinute => Min_2,
        Period::ThreeMinute => Min_3,
        Period::FiveMinute => Min_5,
        Period::TenMinute => Min_10,
        Period::FifteenMinute => Min_15,
        Period::TwentyMinute => Min_20,
        Period::ThirtyMinute => Min_30,
        Period::FortyFiveMinute => Min_45,
        Period::SixtyMinute => Min_60,
        Period::TwoHour => Min_120,
        Period::ThreeHour => Min_180,
        Period::FourHour => Min_240,
        Period::Day => Day,
        Period::Week => Week,
        Period::Month => Month,
        Period::Quarter => Quarter,
        Period::Year => Year,
    }
}
