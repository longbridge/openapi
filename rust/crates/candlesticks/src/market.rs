use std::collections::HashSet;

use rust_decimal::{prelude::FromPrimitive, Decimal};
use time::{macros::time, Date, Duration, Month, OffsetDateTime, Time, Weekday};
use time_tz::{OffsetDateTimeExt, PrimitiveDateTimeExt, Tz};

use crate::{
    candlestick::Candlestick,
    find_session::{FindSession, FindSessionResult},
    Period, Quote, Trade,
};

const TICK_TIMEOUT: Duration = Duration::seconds(5);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Market {
    pub timezone: &'static Tz,
    pub trade_sessions: &'static [(Time, Time, Duration)],
    pub half_trade_sessions: &'static [(Time, Time, Duration)],
    pub lot_size: i64,
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct UpdateFields: u32 {
        const PRICE = 0x1;
        const VOLUME = 0x2;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpdateAction {
    UpdateLast(Candlestick),
    AppendNew {
        confirmed: Option<Candlestick>,
        new: Candlestick,
    },
    Confirm(Candlestick),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum InputCandlestick {
    Normal(Candlestick),
    Confirmed(Candlestick),
    None,
}

pub trait Days: Copy {
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
        half_days: H,
        period: Period,
        t: OffsetDateTime,
    ) -> Option<OffsetDateTime>
    where
        H: Days,
    {
        let t = t.to_timezone(self.timezone);
        let time = t.time();
        let trade_sessions = if !half_days.contains(t.date()) {
            self.trade_sessions
        } else {
            self.half_trade_sessions
        };
        let res = trade_sessions.find_session(time);
        let (time, n) = match res {
            FindSessionResult::BeforeFirst => return None,
            FindSessionResult::Between(n) => Some((time, n)),
            FindSessionResult::After(n) => {
                if time >= trade_sessions[n].1 + trade_sessions[n].2 {
                    return None;
                } else {
                    Some((trade_sessions[n].1, n))
                }
            }
        }?;

        Some(match period {
            Period::Min_1 => t.replace_time(Time::from_hms(time.hour(), time.minute(), 0).ok()?),
            Period::Min_5 | Period::Min_15 | Period::Min_30 => {
                let n = period.minutes() as i64;
                let minutes = time.hour() as i64 * 60 + time.minute() as i64;
                let minutes = (minutes / n) * n;
                t.replace_time(Time::from_hms((minutes / 60) as u8, (minutes % 60) as u8, 0).ok()?)
            }
            Period::Min_60 => {
                let (start, _, _) = &trade_sessions[n];
                let start_minutes = start.hour() as i64 * 60 + start.minute() as i64;
                let current_minutes = time.hour() as i64 * 60 + time.minute() as i64;
                let offset_minutes = ((current_minutes - start_minutes) / 60) * 60;
                t.replace_time(*start + Duration::minutes(offset_minutes))
            }
            Period::Day => t.replace_time(time!(00:00:00)),
            Period::Week => {
                let week = t.iso_week();
                Date::from_iso_week_date(t.year(), week, Weekday::Monday)
                    .ok()?
                    .with_hms(0, 0, 0)
                    .ok()?
                    .assume_timezone(self.timezone)
                    .take_first()?
            }
            Period::Month => t.replace_day(1).ok()?.replace_time(time!(00:00:00)),
            Period::Year => t
                .replace_month(Month::January)
                .ok()?
                .replace_day(1)
                .ok()?
                .replace_time(time!(00:00:00)),
        })
    }

    #[must_use]
    pub fn merge_trade<H>(
        &self,
        half_days: H,
        period: Period,
        input: InputCandlestick,
        trade: Trade,
    ) -> UpdateAction
    where
        H: Days,
    {
        let Some(time) =
            self.candlestick_time(half_days, period, trade.time.to_timezone(self.timezone))
        else {
            return UpdateAction::None;
        };

        match input {
            InputCandlestick::Normal(prev) if time == prev.time => {
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
            InputCandlestick::None => {
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
            InputCandlestick::Normal(prev) | InputCandlestick::Confirmed(prev)
                if time > prev.time =>
            {
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
                        confirmed: if matches!(input, InputCandlestick::Normal(_)) {
                            Some(prev)
                        } else {
                            None
                        },
                        new: new_candlestick,
                    }
                } else {
                    UpdateAction::None
                }
            }
            _ => UpdateAction::None,
        }
    }

    #[must_use]
    pub fn merge_quote<H>(
        &self,
        half_days: H,
        period: Period,
        input: InputCandlestick,
        quote: Quote,
    ) -> UpdateAction
    where
        H: Days,
    {
        assert_eq!(period, Period::Day);
        let tz = self.timezone;
        let Some(time) = self.candlestick_time(half_days, period, quote.time.to_timezone(tz))
        else {
            return UpdateAction::None;
        };
        match input {
            InputCandlestick::Normal(prev) if time == prev.time => {
                UpdateAction::UpdateLast(Candlestick {
                    time: time.to_timezone(time_tz::timezones::db::UTC),
                    open: quote.open,
                    high: quote.high,
                    low: quote.low,
                    close: quote.last_done,
                    volume: quote.volume,
                    turnover: quote.turnover,
                })
            }
            InputCandlestick::None => UpdateAction::AppendNew {
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
            InputCandlestick::Normal(prev) | InputCandlestick::Confirmed(prev)
                if time > prev.time =>
            {
                UpdateAction::AppendNew {
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
                }
            }
            _ => UpdateAction::None,
        }
    }

    #[must_use]
    pub fn tick<N, H>(
        &self,
        normal_days: N,
        half_days: H,
        period: Period,
        input: InputCandlestick,
        current_time: OffsetDateTime,
    ) -> UpdateAction
    where
        H: Days,
        N: Days,
    {
        let current_time = current_time.to_timezone(self.timezone) - TICK_TIMEOUT;
        if !normal_days.contains(current_time.date()) && !half_days.contains(current_time.date()) {
            return UpdateAction::None;
        }
        let trade_sessions = if !half_days.contains(current_time.date()) {
            self.trade_sessions
        } else {
            self.half_trade_sessions
        };

        let res = trade_sessions.find_session(current_time.time());
        match period {
            Period::Min_1 | Period::Min_5 | Period::Min_15 | Period::Min_30 | Period::Min_60 => {
                match (res, input) {
                    (
                        FindSessionResult::Between(_),
                        InputCandlestick::Normal(candlestick)
                        | InputCandlestick::Confirmed(candlestick),
                    ) => {
                        let current_candlestick_time = self
                            .candlestick_time(half_days, period, current_time)
                            .unwrap();
                        if current_candlestick_time > candlestick.time {
                            make_append_new(current_candlestick_time, input)
                        } else {
                            UpdateAction::None
                        }
                    }
                    (FindSessionResult::After(_), InputCandlestick::Normal(candlestick)) => {
                        let Some(current_candlestick_time) =
                            self.candlestick_time(half_days, period, current_time)
                        else {
                            return UpdateAction::Confirm(candlestick);
                        };
                        if current_candlestick_time > candlestick.time {
                            make_append_new(current_candlestick_time, input)
                        } else {
                            UpdateAction::None
                        }
                    }
                    (
                        _,
                        InputCandlestick::Normal(candlestick)
                        | InputCandlestick::Confirmed(candlestick),
                    ) => {
                        if current_time.date() > candlestick.time.date() {
                            make_append_new(
                                self.candlestick_time(
                                    half_days,
                                    period,
                                    current_time.replace_time(trade_sessions[0].0),
                                )
                                .unwrap(),
                                input,
                            )
                        } else {
                            UpdateAction::None
                        }
                    }
                    _ => UpdateAction::None,
                }
            }
            Period::Day | Period::Week | Period::Month | Period::Year => match (res, input) {
                (FindSessionResult::After(n), InputCandlestick::Normal(candlestick))
                    if n == trade_sessions.len() - 1 =>
                {
                    let mut next_trading_day = 1;
                    for i in 1..30 {
                        let next_date = (current_time + Duration::days(i)).date();
                        if normal_days.contains(next_date) || half_days.contains(next_date) {
                            next_trading_day = i;
                            break;
                        }
                    }
                    let Some(next_day_time) = self.candlestick_time(
                        half_days,
                        period,
                        (current_time + Duration::days(next_trading_day))
                            .replace_time(trade_sessions[0].0),
                    ) else {
                        return UpdateAction::None;
                    };
                    if next_day_time > candlestick.time {
                        UpdateAction::Confirm(candlestick)
                    } else {
                        UpdateAction::None
                    }
                }
                (
                    FindSessionResult::Between(_),
                    InputCandlestick::Normal(candlestick)
                    | InputCandlestick::Confirmed(candlestick),
                ) => {
                    let current_candlestick_time = self
                        .candlestick_time(half_days, period, current_time)
                        .unwrap();
                    if current_candlestick_time > candlestick.time {
                        make_append_new(current_candlestick_time, input)
                    } else {
                        UpdateAction::None
                    }
                }
                _ => UpdateAction::None,
            },
        }
    }
}

fn make_append_new(new_time: OffsetDateTime, input: InputCandlestick) -> UpdateAction {
    match input {
        InputCandlestick::Normal(candlestick) | InputCandlestick::Confirmed(candlestick) => {
            UpdateAction::AppendNew {
                confirmed: if matches!(input, InputCandlestick::Normal(_)) {
                    Some(candlestick)
                } else {
                    None
                },
                new: Candlestick {
                    time: new_time.to_timezone(time_tz::timezones::db::UTC),
                    open: candlestick.close,
                    high: candlestick.close,
                    low: candlestick.close,
                    close: candlestick.close,
                    volume: 0,
                    turnover: Decimal::ZERO,
                },
            }
        }
        _ => unreachable!(),
    }
}
