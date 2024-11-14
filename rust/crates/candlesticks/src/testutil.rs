use rust_decimal::{prelude::FromPrimitive, Decimal};
use time::{Date, Month, OffsetDateTime, Time};
use time_tz::PrimitiveDateTimeExt;

use crate::{Candlestick, Days, InputCandlestick, Market, Period, UpdateAction};

pub struct TestCandlestickTime<'a> {
    market: &'a Market,
    period: Period,
}

impl<'a> TestCandlestickTime<'a> {
    #[inline]
    pub fn new(market: &'a Market, period: Period) -> Self {
        Self { market, period }
    }

    #[track_caller]
    pub fn check_time(&self, input: Time, expected: impl Into<Option<Time>>) {
        let date = Date::from_calendar_date(2024, Month::January, 1).unwrap();
        assert_eq!(
            self.market.candlestick_time(
                false,
                self.period,
                date.with_time(input)
                    .assume_timezone(self.market.timezone)
                    .unwrap_first(),
            ),
            expected.into().map(|expected| date
                .with_time(expected)
                .assume_timezone(self.market.timezone)
                .unwrap_first())
        );
    }

    #[track_caller]
    pub fn check_datetime(
        &self,
        input: OffsetDateTime,
        expected: impl Into<Option<OffsetDateTime>>,
    ) {
        assert_eq!(
            self.market.candlestick_time(false, self.period, input),
            expected.into()
        );
    }

    #[track_caller]
    pub fn check_tick(
        &self,
        input: InputCandlestick,
        current: OffsetDateTime,
        expected: UpdateAction,
    ) {
        assert_eq!(
            self.market.tick(true, false, self.period, input, current),
            expected
        );
    }

    #[track_caller]
    pub fn check_tick_with_trading_days<N, H>(
        &self,
        normal_days: N,
        half_days: H,
        input: InputCandlestick,
        current: OffsetDateTime,
        expected: UpdateAction,
    ) where
        N: Days,
        H: Days,
    {
        assert_eq!(
            self.market
                .tick(normal_days, half_days, self.period, input, current),
            expected
        );
    }
}

pub fn normal_candlestick(time: OffsetDateTime) -> Candlestick {
    Candlestick {
        time,
        open: Decimal::from_i32(3).unwrap(),
        high: Decimal::from_i32(5).unwrap(),
        low: Decimal::from_i32(1).unwrap(),
        close: Decimal::from_i32(4).unwrap(),
        volume: 100,
        turnover: Decimal::ONE,
    }
}

pub fn new_candlestick(time: OffsetDateTime) -> Candlestick {
    Candlestick {
        time,
        open: Decimal::from_i32(4).unwrap(),
        high: Decimal::from_i32(4).unwrap(),
        low: Decimal::from_i32(4).unwrap(),
        close: Decimal::from_i32(4).unwrap(),
        volume: 0,
        turnover: Decimal::ZERO,
    }
}
