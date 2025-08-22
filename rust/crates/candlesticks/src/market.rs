use std::{collections::HashSet, ops::Add};

use num_traits::Zero;
use time::{Date, Duration, OffsetDateTime, Time, Weekday, macros::time};
use time_tz::{OffsetDateTimeExt, PrimitiveDateTimeExt, Tz};

use crate::{
    CandlestickComponents, CandlestickType, Period, QuoteType, TradeType, UpdateFields,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TradeSessionKind(usize);

pub trait TradeSessionType: Copy {
    fn kind(&self) -> TradeSessionKind;

    #[inline]
    fn is_intraday(&self) -> bool {
        self.kind() == TRADE_SESSION_INTRADAY
    }

    #[inline]
    fn as_str(&self) -> &'static str {
        let kind = self.kind();
        if kind == TRADE_SESSION_INTRADAY {
            "intraday"
        } else if kind == TRADE_SESSION_PRE {
            "pre"
        } else if kind == TRADE_SESSION_POST {
            "post"
        } else if kind == TRADE_SESSION_OVERNIGHT {
            "overnight"
        } else {
            unreachable!()
        }
    }
}

impl TradeSessionType for TradeSessionKind {
    #[inline]
    fn kind(&self) -> TradeSessionKind {
        *self
    }
}

pub const TRADE_SESSION_INTRADAY: TradeSessionKind = TradeSessionKind(0);
pub const TRADE_SESSION_PRE: TradeSessionKind = TradeSessionKind(1);
pub const TRADE_SESSION_POST: TradeSessionKind = TradeSessionKind(2);
pub const TRADE_SESSION_OVERNIGHT: TradeSessionKind = TradeSessionKind(3);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Market {
    pub timezone: &'static Tz,
    pub trade_sessions: &'static [&'static [TradeSession]],
    pub half_trade_sessions: &'static [&'static [TradeSession]],
    pub lot_size: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpdateAction<T> {
    UpdateLast(T),
    AppendNew { confirmed: Option<T>, new: T },
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
    pub fn candlestick_time<H, TS>(
        &self,
        ts: TS,
        half_days: H,
        period: Period,
        t: OffsetDateTime,
    ) -> Option<OffsetDateTime>
    where
        H: Days,
        TS: TradeSessionType,
    {
        use Period::*;

        if !ts.is_intraday() && !period.is_minute() {
            return None;
        }

        let ts = ts.kind();

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
    pub fn merge_trade<H, TS, C, T, P, V, R>(
        &self,
        half_days: H,
        period: Period,
        input: Option<C>,
        trade: &T,
        update_fields: UpdateFields,
    ) -> UpdateAction<C>
    where
        H: Days,
        TS: TradeSessionType + Eq,
        C: CandlestickType<PriceType = P, VolumeType = V, TurnoverType = R, TradeSessionType = TS>,
        T: TradeType<PriceType = P, VolumeType = V, TurnoverType = R, TradeSessionType = TS>,
        P: PartialOrd + Add<Output = P>,
        V: Add<Output = V> + Zero,
        R: Add<Output = R> + Zero,
    {
        let trade_session = trade.trade_session();

        if let Some(input_trade_session) = input.as_ref().map(|c| c.trade_session()) {
            debug_assert!(input_trade_session == trade_session);
        }

        let Some(time) = self.candlestick_time(
            trade_session,
            half_days,
            period,
            trade.time().to_timezone(self.timezone),
        ) else {
            return UpdateAction::None;
        };

        match input {
            Some(prev) if time == prev.time() => {
                let mut candlestick = prev;

                if update_fields.contains(UpdateFields::PRICE) {
                    candlestick.set_high(if trade.price() > candlestick.high() {
                        trade.price()
                    } else {
                        candlestick.high()
                    });

                    candlestick.set_low(if trade.price() < candlestick.low() {
                        trade.price()
                    } else {
                        candlestick.low()
                    });

                    candlestick.set_close(trade.price());
                }

                if update_fields.contains(UpdateFields::VOLUME) {
                    candlestick.set_volume(candlestick.volume() + trade.volume());
                    candlestick
                        .set_turnover(candlestick.turnover() + trade.turnover(self.lot_size));
                }

                UpdateAction::UpdateLast(candlestick)
            }
            None => {
                if update_fields.contains(UpdateFields::PRICE) {
                    let new_candlestick = C::new(CandlestickComponents {
                        time: time.to_timezone(time_tz::timezones::db::UTC),
                        open: trade.price(),
                        high: trade.price(),
                        low: trade.price(),
                        close: trade.price(),
                        volume: trade.volume(),
                        turnover: trade.turnover(self.lot_size),
                        trade_session,
                    });
                    UpdateAction::AppendNew {
                        confirmed: None,
                        new: new_candlestick,
                    }
                } else {
                    UpdateAction::None
                }
            }
            Some(prev) if time > prev.time() => {
                let mut new_candlestick = C::new(CandlestickComponents {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: prev.close(),
                    high: prev.close(),
                    low: prev.close(),
                    close: prev.close(),
                    volume: V::zero(),
                    turnover: R::zero(),
                    trade_session,
                });

                if update_fields.contains(UpdateFields::PRICE) {
                    new_candlestick.set_open(trade.price());
                    new_candlestick.set_high(trade.price());
                    new_candlestick.set_low(trade.price());
                    new_candlestick.set_close(trade.price());
                }

                if update_fields.contains(UpdateFields::VOLUME) {
                    new_candlestick.set_volume(trade.volume());
                    new_candlestick.set_turnover(trade.turnover(self.lot_size));
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
    pub fn merge_quote_day<TS, C, Q, P, V, R>(&self, input: Option<C>, quote: &Q) -> UpdateAction<C>
    where
        TS: TradeSessionType + Eq,
        C: CandlestickType<PriceType = P, VolumeType = V, TurnoverType = R, TradeSessionType = TS>,
        Q: QuoteType<PriceType = P, VolumeType = V, TurnoverType = R, TradeSessionType = TS>,
    {
        let trade_session = quote.trade_session();

        if !trade_session.is_intraday() {
            return UpdateAction::None;
        }

        if let Some(input_trade_session) = input.as_ref().map(|c| c.trade_session()) {
            debug_assert!(input_trade_session == trade_session);
        }

        let tz = self.timezone;
        let time = quote.time().to_timezone(tz).replace_time(Time::MIDNIGHT);

        match input {
            Some(prev) if time == prev.time() => {
                UpdateAction::UpdateLast(C::new(CandlestickComponents {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: quote.open(),
                    high: quote.high(),
                    low: quote.low(),
                    close: quote.last_done(),
                    volume: quote.volume(),
                    turnover: quote.turnover(),
                    trade_session,
                }))
            }
            None => UpdateAction::AppendNew {
                confirmed: None,
                new: C::new(CandlestickComponents {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: quote.open(),
                    high: quote.high(),
                    low: quote.low(),
                    close: quote.last_done(),
                    volume: quote.volume(),
                    turnover: quote.turnover(),
                    trade_session,
                }),
            },
            Some(prev) if time > prev.time() => UpdateAction::AppendNew {
                confirmed: Some(prev),
                new: C::new(CandlestickComponents {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: quote.open(),
                    high: quote.high(),
                    low: quote.low(),
                    close: quote.last_done(),
                    volume: quote.volume(),
                    turnover: quote.turnover(),
                    trade_session,
                }),
            },
            _ => UpdateAction::None,
        }
    }

    pub fn trade_session(&self, candlestick_time: OffsetDateTime) -> Option<TradeSessionKind> {
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
                        return Some(TradeSessionKind(idx));
                    }
                } else if time >= *start && time <= *end {
                    return Some(TradeSessionKind(idx));
                }
            }
        }
        None
    }
}
