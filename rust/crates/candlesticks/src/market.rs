use std::collections::HashSet;

use rust_decimal::{Decimal, prelude::FromPrimitive};
use time::{Date, Duration, OffsetDateTime, Time, Weekday, macros::time};
use time_tz::{OffsetDateTimeExt, PrimitiveDateTimeExt, Tz};

use crate::{
    Period, Quote, Trade, UpdateFields,
    candlestick::Candlestick,
    find_session::{FindSession, FindSessionResult},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TradeSession {
    pub start: Time,
    pub end: Time,
    pub inclusive: bool,
    pub timeout: Duration,
}

impl TradeSession {
    #[inline]
    pub const fn new(start: Time, end: Time) -> Self {
        Self {
            start,
            end,
            inclusive: false,
            timeout: Duration::ZERO,
        }
    }

    #[inline]
    pub const fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    #[inline]
    pub const fn with_inclusive(self) -> Self {
        Self {
            inclusive: true,
            ..self
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TradeSessionType(usize);

impl std::fmt::Debug for TradeSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "normal"),
            1 => write!(f, "pre"),
            2 => write!(f, "post"),
            3 => write!(f, "overnight"),
            _ => unreachable!(),
        }
    }
}

impl TradeSessionType {
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TRADE_SESSION_NORMAL
    }
}

pub const TRADE_SESSION_NORMAL: TradeSessionType = TradeSessionType(0);
pub const TRADE_SESSION_PRE: TradeSessionType = TradeSessionType(1);
pub const TRADE_SESSION_POST: TradeSessionType = TradeSessionType(2);
pub const TRADE_SESSION_OVERNIGHT: TradeSessionType = TradeSessionType(3);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Market {
    pub timezone: &'static Tz,
    pub trade_sessions: &'static [&'static [TradeSession]],
    pub half_trade_sessions: &'static [&'static [TradeSession]],
    pub lot_size: i64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpdateAction {
    UpdateLast(Candlestick),
    AppendNew {
        confirmed: Option<Candlestick>,
        new: Candlestick,
    },
    None,
}

pub trait Days: std::fmt::Debug + Copy {
    fn contains(&self, date: Date) -> bool;
}

impl Days for bool {
    #[inline]
    fn contains(&self, _date: Date) -> bool {
        *self
    }
}

impl Days for &HashSet<Date> {
    #[inline]
    fn contains(&self, date: Date) -> bool {
        HashSet::contains(self, &date)
    }
}

impl Market {
    pub fn candlestick_time<H>(
        &self,
        ts: TradeSessionType,
        half_days: H,
        period: Period,
        t: OffsetDateTime,
    ) -> Option<OffsetDateTime>
    where
        H: Days,
    {
        use Period::*;

        assert!(ts.is_normal() || (!ts.is_normal() && period.is_minute()));

        let t = t.to_timezone(self.timezone);
        let time = t.time();
        let trade_sessions = if !half_days.contains(t.date()) {
            self.trade_sessions.get(ts.0)?
        } else {
            self.half_trade_sessions.get(ts.0)?
        };
        let res = trade_sessions.find_session(time);
        let (time, n) = match res {
            FindSessionResult::BeforeFirst => return None,
            FindSessionResult::Between(n) => Some((time, n)),
            FindSessionResult::After(n) => {
                if time >= trade_sessions[n].end + trade_sessions[n].timeout {
                    return None;
                } else {
                    Some((trade_sessions[n].end, n))
                }
            }
        }?;

        Some(match period {
            Min_1 => t.replace_time(Time::from_hms(time.hour(), time.minute(), 0).ok()?),
            Min_2 | Min_3 | Min_5 | Min_10 | Min_15 | Min_20 | Min_30 | Min_45 | Min_60
            | Min_120 | Min_180 | Min_240 => {
                let minutes = period.minutes() as i64;
                let TradeSession { start, .. } = &trade_sessions[n];
                let start_minutes = start.hour() as i64 * 60 + start.minute() as i64;
                let current_minutes = time.hour() as i64 * 60 + time.minute() as i64;
                let offset_minutes = ((current_minutes - start_minutes) / minutes) * minutes;
                t.replace_time(*start + Duration::minutes(offset_minutes))
            }
            Day => t.replace_time(time!(00:00:00)),
            Week => {
                let week = t.iso_week();
                Date::from_iso_week_date(t.year(), week, Weekday::Monday)
                    .ok()?
                    .with_hms(0, 0, 0)
                    .ok()?
                    .assume_timezone(self.timezone)
                    .take_first()?
            }
            Month => t.replace_day(1).ok()?.replace_time(time!(00:00:00)),
            Quarter => {
                let month = t.month();
                let quarter = (month as u8 - 1) / 3;
                t.replace_month(time::Month::try_from(quarter * 3 + 1).ok()?)
                    .ok()?
                    .replace_day(1)
                    .ok()?
                    .replace_time(time!(00:00:00))
            }
            Year => t
                .replace_month(time::Month::January)
                .ok()?
                .replace_day(1)
                .ok()?
                .replace_time(time!(00:00:00)),
        })
    }

    #[must_use]
    pub fn merge_trade<H>(
        &self,
        ts: TradeSessionType,
        half_days: H,
        period: Period,
        input: Option<Candlestick>,
        trade: Trade,
    ) -> UpdateAction
    where
        H: Days,
    {
        debug_assert!(period != Period::Day);

        let Some(time) =
            self.candlestick_time(ts, half_days, period, trade.time.to_timezone(self.timezone))
        else {
            return UpdateAction::None;
        };

        match input {
            Some(prev) if time == prev.time => {
                let mut candlestick = prev;

                if trade.update_fields.contains(UpdateFields::PRICE) {
                    candlestick.high = candlestick.high.max(trade.price);
                    candlestick.low = candlestick.low.min(trade.price);
                    candlestick.close = trade.price;
                }

                if trade.update_fields.contains(UpdateFields::VOLUME) {
                    candlestick.volume += trade.volume;
                    candlestick.turnover += trade.price
                        * Decimal::from_i64(trade.volume * self.lot_size).unwrap_or_default();
                }

                UpdateAction::UpdateLast(candlestick)
            }
            None => {
                if trade.update_fields.contains(UpdateFields::PRICE) {
                    let new_candlestick = Candlestick {
                        time: time.to_timezone(time_tz::timezones::db::UTC),
                        open: trade.price,
                        high: trade.price,
                        low: trade.price,
                        close: trade.price,
                        volume: trade.volume,
                        turnover: trade.price
                            * Decimal::from_i64(trade.volume * self.lot_size).unwrap_or_default(),
                    };
                    UpdateAction::AppendNew {
                        confirmed: None,
                        new: new_candlestick,
                    }
                } else {
                    UpdateAction::None
                }
            }
            Some(prev) if time > prev.time => {
                let mut new_candlestick = Candlestick {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: prev.close,
                    high: prev.close,
                    low: prev.close,
                    close: prev.close,
                    volume: 0,
                    turnover: Decimal::ZERO,
                };

                if trade.update_fields.contains(UpdateFields::PRICE) {
                    new_candlestick.open = trade.price;
                    new_candlestick.high = trade.price;
                    new_candlestick.low = trade.price;
                    new_candlestick.close = trade.price;
                }

                if trade.update_fields.contains(UpdateFields::VOLUME) {
                    new_candlestick.volume = trade.volume;
                    new_candlestick.turnover = trade.price
                        * Decimal::from_i64(trade.volume * self.lot_size).unwrap_or_default();
                }

                UpdateAction::AppendNew {
                    confirmed: Some(prev),
                    new: new_candlestick,
                }
            }
            _ => UpdateAction::None,
        }
    }

    #[must_use]
    pub fn merge_quote<H>(
        &self,
        ts: TradeSessionType,
        half_days: H,
        period: Period,
        input: Option<Candlestick>,
        quote: Quote,
    ) -> UpdateAction
    where
        H: Days,
    {
        debug_assert!(period == Period::Day);

        let tz = self.timezone;
        let Some(time) = self.candlestick_time(ts, half_days, period, quote.time.to_timezone(tz))
        else {
            return UpdateAction::None;
        };

        match input {
            Some(prev) if time == prev.time => UpdateAction::UpdateLast(Candlestick {
                time: time.to_timezone(time_tz::timezones::db::UTC),
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.last_done,
                volume: quote.volume,
                turnover: quote.turnover,
            }),
            None => UpdateAction::AppendNew {
                confirmed: None,
                new: Candlestick {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: quote.open,
                    high: quote.high,
                    low: quote.low,
                    close: quote.last_done,
                    volume: quote.volume,
                    turnover: quote.turnover,
                },
            },
            Some(prev) if time > prev.time => UpdateAction::AppendNew {
                confirmed: Some(prev),
                new: Candlestick {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: quote.open,
                    high: quote.high,
                    low: quote.low,
                    close: quote.last_done,
                    volume: quote.volume,
                    turnover: quote.turnover,
                },
            },
            _ => UpdateAction::None,
        }
    }

    pub fn candlestick_trade_session(
        &self,
        candlestick_time: OffsetDateTime,
    ) -> Option<TradeSessionType> {
        let candlestick_time = candlestick_time.to_timezone(self.timezone);
        for (idx, trade_sessions) in self.trade_sessions.iter().enumerate() {
            for TradeSession {
                start,
                end,
                inclusive,
                timeout,
                ..
            } in trade_sessions.iter()
            {
                let time = candlestick_time.time();
                if !*inclusive && timeout.is_zero() {
                    if time >= *start && time < *end {
                        return Some(TradeSessionType(idx));
                    }
                } else if time >= *start && time <= *end {
                    return Some(TradeSessionType(idx));
                }
            }
        }
        None
    }
}
