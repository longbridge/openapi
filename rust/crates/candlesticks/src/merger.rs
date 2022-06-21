use rust_decimal::{prelude::FromPrimitive, Decimal};
use time::{macros::time, Date, Duration, Month, OffsetDateTime, Time, Weekday};
use time_tz::OffsetDateTimeExt;

use crate::{market::UpdateFields, Market, Period, Type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Candlestick {
    pub time: OffsetDateTime,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: i64,
    pub turnover: Decimal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Trade<'a> {
    pub time: OffsetDateTime,
    pub price: Decimal,
    pub volume: i64,
    pub trade_type: &'a str,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpdateAction {
    UpdateLast(Candlestick),
    AppendNew(Candlestick),
    None,
}

pub trait IsHalfTradeDay: Copy {
    fn is_half(&self, date: Date) -> bool;
}

impl IsHalfTradeDay for bool {
    #[inline]
    fn is_half(&self, _date: Date) -> bool {
        *self
    }
}

pub struct Merger<T> {
    market: Market,
    period: Period,
    is_half_trade_day: T,
}

impl<T> Merger<T>
where
    T: IsHalfTradeDay,
{
    #[inline]
    pub fn new(market: Market, period: Period, is_half_trade_day: T) -> Self {
        Self {
            market,
            period,
            is_half_trade_day,
        }
    }

    fn round_time(
        &self,
        mut time: OffsetDateTime,
        trade_sessions: &[(Time, Time)],
    ) -> OffsetDateTime {
        for (idx, (start, end)) in trade_sessions.iter().enumerate() {
            if time.time() < *start {
                time = if idx == 0 {
                    time.replace_time(*start)
                } else {
                    time.replace_time(trade_sessions[idx - 1].1)
                };
                break;
            } else if time.time() < *end {
                break;
            } else if idx == trade_sessions.len() - 1 {
                time = time.replace_time(*end);
                break;
            }
        }

        time
    }

    pub fn candlestick_time(&self, ty: Type, time: OffsetDateTime) -> OffsetDateTime {
        let Merger {
            market,
            period,
            is_half_trade_day,
        } = self;
        let trade_sessions = if !is_half_trade_day.is_half(time.date()) {
            market.trade_sessions(ty)
        } else {
            market.half_trade_sessions(ty)
        };
        match period {
            Period::Min_1 => self
                .round_time(time, trade_sessions)
                .replace_second(0)
                .unwrap(),
            Period::Min_5 | Period::Min_15 | Period::Min_30 => {
                let time = self.round_time(time, trade_sessions);
                let n = period.minutes() as i64;
                let minutes = time.hour() as i64 * 60 + time.minute() as i64 - 1;
                let minutes = (minutes / n + 1) * n;
                let mut time = time.replace_time(
                    Time::from_hms((minutes / 60) as u8, (minutes % 60) as u8, 0).unwrap(),
                );
                for (start, end) in trade_sessions {
                    let s = time.replace_time(*start);
                    if time < s + Duration::minutes(n as i64) {
                        time = s + Duration::minutes(n as i64);
                        break;
                    } else if time <= time.replace_time(*end) {
                        break;
                    }
                }
                time
            }
            Period::Min_60 => {
                let time = self.round_time(time, trade_sessions);
                let (start, end) = trade_sessions
                    .iter()
                    .find(|ts| time.time() >= ts.0 && time.time() <= ts.1)
                    .unwrap();
                let start_minutes = start.hour() as i64 * 60 + start.minute() as i64;
                let curr_minutes = time.hour() as i64 * 60 + time.minute() as i64 - 1;
                let offset_minutes = ((curr_minutes - start_minutes) / 60 + 1) * 60;
                time.replace_time((*start + Duration::minutes(offset_minutes)).min(*end))
            }
            Period::Day => time.replace_time(time!(00:00:00)),
            Period::Week => {
                let week = time.iso_week();
                Date::from_iso_week_date(time.year(), week, Weekday::Monday)
                    .and_then(|date| date.with_hms(0, 0, 0))
                    .unwrap()
                    .assume_utc()
            }
            Period::Month => time
                .replace_day(1)
                .map(|time| time.replace_time(time!(00:00:00)))
                .unwrap(),
            Period::Year => time
                .replace_month(Month::January)
                .and_then(|time| time.replace_day(1))
                .map(|time| time.replace_time(time!(00:00:00)))
                .and_then(|time| time.replace_day(1))
                .unwrap(),
        }
    }

    #[must_use]
    pub fn merge(&self, ty: Type, prev: Option<&Candlestick>, trade: Trade<'_>) -> UpdateAction {
        let Merger { market, .. } = self;
        let tz = market.timezone();
        let time = self.candlestick_time(ty, trade.time.to_timezone(tz));
        let update_fields = market.update_fields(trade.trade_type);

        match prev {
            Some(prev) if time == prev.time => {
                let mut candlestick = *prev;

                if update_fields.contains(UpdateFields::PRICE) {
                    candlestick.high = candlestick.high.max(trade.price);
                    candlestick.low = candlestick.low.min(trade.price);
                    candlestick.close = trade.price;
                }

                if update_fields.contains(UpdateFields::VOLUME) {
                    candlestick.volume += trade.volume;
                    if let Some(volume) = Decimal::from_i64(trade.volume) {
                        candlestick.turnover += trade.price * volume;
                    }
                }

                UpdateAction::UpdateLast(candlestick)
            }
            Some(prev) if time < prev.time => UpdateAction::None,
            _ => {
                if update_fields.contains(UpdateFields::PRICE) {
                    Decimal::from_i64(trade.volume)
                        .map(|volume| {
                            let new_candlestick = Candlestick {
                                time: time.to_timezone(time_tz::timezones::db::UTC),
                                open: trade.price,
                                high: trade.price,
                                low: trade.price,
                                close: trade.price,
                                volume: trade.volume,
                                turnover: trade.price * volume,
                            };
                            UpdateAction::AppendNew(new_candlestick)
                        })
                        .unwrap_or(UpdateAction::None)
                } else {
                    UpdateAction::None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test_round_time() {
        let trade_sessions = Market::HK.trade_sessions(Type::Normal);
        let merger = Merger::new(Market::HK, Period::Day, false);

        assert_eq!(
            merger.round_time(datetime!(2022-1-1 9:28:0 UTC), trade_sessions),
            datetime!(2022-1-1 9:30:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 9:31:0 UTC), trade_sessions),
            datetime!(2022-1-1 9:31:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 12:0:0 UTC), trade_sessions),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 12:5:0 UTC), trade_sessions),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 13:0:0 UTC), trade_sessions),
            datetime!(2022-1-1 13:0:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 14:0:0 UTC), trade_sessions),
            datetime!(2022-1-1 14:0:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 16:0:0 UTC), trade_sessions),
            datetime!(2022-1-1 16:0:0 UTC)
        );
        assert_eq!(
            merger.round_time(datetime!(2022-1-1 16:2:0 UTC), trade_sessions),
            datetime!(2022-1-1 16:0:0 UTC)
        );
    }

    #[test]
    fn test_time_min1() {
        let merger = Merger::new(Market::HK, Period::Min_1, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 9:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:30:25 UTC)),
            datetime!(2022-1-1 9:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:31:0 UTC)),
            datetime!(2022-1-1 9:31:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 12:05:0 UTC)),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 13:0:0 UTC)),
            datetime!(2022-1-1 13:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:0:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:2:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
    }

    #[test]
    fn test_time_min5() {
        let merger = Merger::new(Market::HK, Period::Min_5, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 9:35:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:30:25 UTC)),
            datetime!(2022-1-1 9:35:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:35:59 UTC)),
            datetime!(2022-1-1 9:35:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:36:0 UTC)),
            datetime!(2022-1-1 9:40:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 12:05:0 UTC)),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 13:0:0 UTC)),
            datetime!(2022-1-1 13:5:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:0:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:2:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
    }

    #[test]
    fn test_time_min15() {
        let merger = Merger::new(Market::HK, Period::Min_15, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 9:45:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:30:25 UTC)),
            datetime!(2022-1-1 9:45:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:35:59 UTC)),
            datetime!(2022-1-1 9:45:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:36:0 UTC)),
            datetime!(2022-1-1 9:45:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 12:05:0 UTC)),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 13:0:0 UTC)),
            datetime!(2022-1-1 13:15:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:0:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:2:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
    }

    #[test]
    fn test_time_min30() {
        let merger = Merger::new(Market::HK, Period::Min_30, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 10:00:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:30:25 UTC)),
            datetime!(2022-1-1 10:00:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:35:59 UTC)),
            datetime!(2022-1-1 10:00:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:36:0 UTC)),
            datetime!(2022-1-1 10:00:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 12:05:0 UTC)),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 13:0:0 UTC)),
            datetime!(2022-1-1 13:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:0:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:2:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
    }

    #[test]
    fn test_time_min60() {
        let merger = Merger::new(Market::HK, Period::Min_60, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:30:25 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:35:59 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:36:0 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 10:30:59 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 10:31:0 UTC)),
            datetime!(2022-1-1 11:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 12:05:0 UTC)),
            datetime!(2022-1-1 12:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 13:0:0 UTC)),
            datetime!(2022-1-1 14:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 14:2:0 UTC)),
            datetime!(2022-1-1 15:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:0:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 16:2:0 UTC)),
            datetime!(2022-1-1 16:0:0 UTC)
        );
    }

    #[test]
    fn test_time_min60_usoq() {
        let merger = Merger::new(Market::US, Period::Min_60, false);

        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 9:30:25 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 9:35:59 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 9:36:0 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 10:30:59 UTC)),
            datetime!(2022-1-1 10:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 10:31:0 UTC)),
            datetime!(2022-1-1 11:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 12:05:0 UTC)),
            datetime!(2022-1-1 12:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 13:0:0 UTC)),
            datetime!(2022-1-1 13:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 14:2:0 UTC)),
            datetime!(2022-1-1 14:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 15:30:59 UTC)),
            datetime!(2022-1-1 15:30:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 15:31:0 UTC)),
            datetime!(2022-1-1 16:15:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::USOQ, datetime!(2022-1-1 16:2:0 UTC)),
            datetime!(2022-1-1 16:15:0 UTC)
        );
    }

    #[test]
    fn test_time_day() {
        let merger = Merger::new(Market::HK, Period::Day, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 9:28:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-1 10:0:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-3 10:0:0 UTC)),
            datetime!(2022-1-3 0:0:0 UTC)
        );
    }

    #[test]
    fn test_time_week() {
        let merger = Merger::new(Market::HK, Period::Week, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-6 9:28:0 UTC)),
            datetime!(2022-1-3 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-10 9:28:0 UTC)),
            datetime!(2022-1-10 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-6-8 9:28:0 UTC)),
            datetime!(2022-6-6 0:0:0 UTC)
        );
    }

    #[test]
    fn test_time_month() {
        let merger = Merger::new(Market::HK, Period::Month, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-6 9:28:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-10 9:28:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-6-8 9:28:0 UTC)),
            datetime!(2022-6-1 0:0:0 UTC)
        );
    }

    #[test]
    fn test_time_year() {
        let merger = Merger::new(Market::HK, Period::Year, false);

        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-1-6 9:28:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-3-10 9:28:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
        assert_eq!(
            merger.candlestick_time(Type::Normal, datetime!(2022-6-8 9:28:0 UTC)),
            datetime!(2022-1-1 0:0:0 UTC)
        );
    }
}
