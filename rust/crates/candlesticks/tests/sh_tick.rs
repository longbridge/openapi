use std::collections::HashSet;

use longport_candlesticks::{
    markets::CN,
    testutil::{new_candlestick, normal_candlestick, TestCandlestickTime},
    InputCandlestick, Period, UpdateAction,
};
use time::{
    macros::{date, datetime},
    Date,
};

#[test]
fn sh_tick_min1() {
    let t = TestCandlestickTime::new(&CN, Period::Min_1);

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
        datetime!(2024-1-1 9:31:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
        datetime!(2024-1-1 9:31:04 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
        datetime!(2024-1-1 9:31:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 9:31:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:35:00 +8))),
        datetime!(2024-1-1 9:36:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 9:35:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 9:36:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 9:35:00 +8))),
        datetime!(2024-1-1 9:36:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-1 9:36:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 11:30:04 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 11:30:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 11:30:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 13:00:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 13:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 13:00:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-1 13:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 14:59:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 14:59:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 15:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 14:59:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:02:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:10:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:10:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-2 9:30:02 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
            new: new_candlestick(datetime!(2024-1-2 9:30:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-2 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
            new: new_candlestick(datetime!(2024-1-2 9:30:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-2 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-2 9:30:00 +8)),
        },
    );
}

#[test]
fn sh_tick_min5() {
    let t = TestCandlestickTime::new(&CN, Period::Min_5);

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
        datetime!(2024-1-1 9:35:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
        datetime!(2024-1-1 9:35:04 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
        datetime!(2024-1-1 9:35:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 9:30:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 9:35:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:35:00 +8))),
        datetime!(2024-1-1 9:40:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 9:35:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 9:40:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 9:35:00 +8))),
        datetime!(2024-1-1 9:40:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-1 9:40:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 11:30:04 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 11:30:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 11:30:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 13:00:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 13:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 11:30:00 +8))),
        datetime!(2024-1-1 13:00:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-1 13:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 14:55:00 +8))),
        datetime!(2024-1-1 15:00:10 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 14:55:00 +8))),
            new: new_candlestick(datetime!(2024-1-1 15:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:10:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:10:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:00:00 +8))),
        datetime!(2024-1-1 15:06:05 +8),
        UpdateAction::None,
    );
}

#[test]
fn sh_tick_day() {
    let t = TestCandlestickTime::new(&CN, Period::Day);

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 9:35:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 11:32:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 15:00:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-2 9:30:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-2 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
            new: new_candlestick(datetime!(2024-1-2 00:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-2 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-2 00:00:00 +8)),
        },
    );
}

#[test]
fn sh_tick_day_with_trading_days() {
    let t = TestCandlestickTime::new(&CN, Period::Day);

    let normal_days: HashSet<Date> = [
        date!(2024 - 1 - 1),
        date!(2024 - 1 - 2),
        date!(2024 - 1 - 3),
        date!(2024 - 1 - 8),
        date!(2024 - 1 - 9),
        date!(2024 - 1 - 10),
    ]
    .into_iter()
    .collect();

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
    );

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-3 00:00:00 +8))),
        datetime!(2024-1-3 15:00:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-3 00:00:00 +8))),
    );

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-3 00:00:00 +8))),
        datetime!(2024-1-4 9:30:05 +8),
        UpdateAction::None,
    );

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-3 00:00:00 +8))),
        datetime!(2024-1-8 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-3 00:00:00 +8))),
            new: new_candlestick(datetime!(2024-1-8 00:00:00 +8)),
        },
    );

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-3 00:00:00 +8))),
        datetime!(2024-1-8 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-8 00:00:00 +8)),
        },
    );
}

#[test]
fn sh_tick_week() {
    let t = TestCandlestickTime::new(&CN, Period::Week);

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 9:35:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 15:00:00 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-1 15:00:05 +8),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-7 15:00:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-8 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
            new: new_candlestick(datetime!(2024-1-8 00:00:00 +8)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-8 00:00:00 +8))),
        datetime!(2024-1-10 9:30:05 +8),
        UpdateAction::None,
    );
}

#[test]
fn sh_tick_week_with_trading_days() {
    let t = TestCandlestickTime::new(&CN, Period::Week);

    let normal_days: HashSet<Date> = [
        date!(2024 - 1 - 1),
        date!(2024 - 1 - 2),
        date!(2024 - 1 - 3),
        date!(2024 - 1 - 8),
        date!(2024 - 1 - 9),
        date!(2024 - 1 - 10),
    ]
    .into_iter()
    .collect();

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-3 15:00:05 +8),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
    );

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-8 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
            new: new_candlestick(datetime!(2024-1-8 00:00:00 +8)),
        },
    );

    t.check_tick_with_trading_days(
        &normal_days,
        false,
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 00:00:00 +8))),
        datetime!(2024-1-8 9:30:05 +8),
        UpdateAction::AppendNew {
            confirmed: None,
            new: new_candlestick(datetime!(2024-1-8 00:00:00 +8)),
        },
    );
}
