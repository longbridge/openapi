use longport_candlesticks::{
    markets::US,
    testutil::{new_candlestick, normal_candlestick, TestCandlestickTime},
    InputCandlestick, Period, UpdateAction,
};
use time::macros::datetime;

#[test]
fn us_tick_min1() {
    let t = TestCandlestickTime::new(&US, Period::Min_1);

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 -5))),
        datetime!(2024-1-1 9:31:00 -5),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 -5))),
        datetime!(2024-1-1 9:31:04 -5),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:30:00 -5))),
        datetime!(2024-1-1 9:31:05 -5),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 9:30:00 -5))),
            new: new_candlestick(datetime!(2024-1-1 9:31:00 -5)),
        },
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 9:31:00 -5))),
        datetime!(2024-1-1 9:31:50 -5),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 9:35:00 -5))),
        datetime!(2024-1-1 9:36:05 -5),
        UpdateAction::AppendNew {
            confirmed: Some(normal_candlestick(datetime!(2024-1-1 9:35:00 -5))),
            new: new_candlestick(datetime!(2024-1-1 9:36:00 -5)),
        },
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
        datetime!(2024-1-1 15:58:00 -5),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
        datetime!(2024-1-1 16:00:00 -5),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
        datetime!(2024-1-1 16:00:5 -5),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
        datetime!(2024-1-1 16:00:5 -5),
        UpdateAction::None,
    );

    t.check_tick(
        InputCandlestick::Normal(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
        datetime!(2024-1-2 00:00:00 -5),
        UpdateAction::Confirm(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
    );

    t.check_tick(
        InputCandlestick::Confirmed(normal_candlestick(datetime!(2024-1-1 15:59:00 -5))),
        datetime!(2024-1-2 00:00:00 -5),
        UpdateAction::None,
    );
}
