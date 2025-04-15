use longport_candlesticks::{
    Period, TRADE_SESSION_NORMAL, markets::CN, testutil::TestCandlestickTime,
};
use time::macros::{datetime, time};

#[test]
fn sh_min1() {
    let t = TestCandlestickTime::new(&CN, Period::Min_1);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:20:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:02), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:31:05), time!(09:31:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:29:59), time!(11:29:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:30:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(11:31:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:05), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:01:05), time!(13:01:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:59:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:01:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:09:59), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:10:00), None);
}

#[test]
fn sh_min2() {
    let t = TestCandlestickTime::new(&CN, Period::Min_2);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:20:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:01), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:31:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:31:01), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:32:00), time!(09:32:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:32:01), time!(09:32:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:29:59), time!(11:28:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:30:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(11:31:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:01), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:01:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:58:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:01:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:09:59), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:10:00), None);
}

#[test]
fn sh_min3() {
    let t = TestCandlestickTime::new(&CN, Period::Min_3);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:20:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:01), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:31:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:31:01), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:32:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:32:01), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:33:00), time!(09:33:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:33:01), time!(09:33:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:29:59), time!(11:27:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:30:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(11:31:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:01), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:01:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:57:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:01:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:09:59), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:10:00), None);
}

#[test]
fn sh_min5() {
    let t = TestCandlestickTime::new(&CN, Period::Min_5);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:34:59), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:35:00), time!(09:35:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:35:01), time!(09:35:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:32:32), time!(10:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:29:59), time!(11:25:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:30:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(11:31:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:05), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:04:59), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:23:59), time!(14:20:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:55:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:01:00), time!(15:00:00));
}

#[test]
fn sh_min10() {
    let t = TestCandlestickTime::new(&CN, Period::Min_10);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:39:59), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:40:00), time!(09:40:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:40:01), time!(09:40:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:29:59), time!(10:20:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:30:00), time!(10:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:31:00), time!(10:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:37:00), time!(10:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:05), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:09:59), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:50:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:01:00), time!(15:00:00));
}

#[test]
fn sh_min45() {
    let t = TestCandlestickTime::new(&CN, Period::Min_45);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:59:59), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:00:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:00:01), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:15:0), time!(10:15:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:30:00), time!(10:15:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:31:00), time!(10:15:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:37:00), time!(10:15:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:05), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:09:59), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(14:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:01:00), time!(14:30:00));
}

#[test]
fn sh_min60() {
    let t = TestCandlestickTime::new(&CN, Period::Min_60);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:10:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:29:59), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:30:00), time!(10:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:59:59), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:00:00), time!(14:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(14:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:05:00), time!(15:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:10:00), None);
}

#[test]
fn sh_min240() {
    let t = TestCandlestickTime::new(&CN, Period::Min_240);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:10:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:29:59), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:49:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(10:49:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(11:26:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(13:59:59), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(14:59:59), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:00:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:05:00), time!(13:00:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:10:00), None);
}

#[test]
fn sh_day() {
    let t = TestCandlestickTime::new(&CN, Period::Day);
    t.check_datetime(TRADE_SESSION_NORMAL, datetime!(2024-1-1 09:20:01 +8), None);
    t.check_datetime(TRADE_SESSION_NORMAL, datetime!(2024-1-1 09:29:59 +8), None);
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2024-1-1 09:30:00 +8),
        datetime!(2024-1-1 00:00:00 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2024-1-1 13:30:00 +8),
        datetime!(2024-1-1 00:00:00 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2024-1-1 15:00:00 +8),
        datetime!(2024-1-1 00:00:00 +8),
    );
}

#[test]
fn sh_week() {
    let t = TestCandlestickTime::new(&CN, Period::Week);
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-1-6 9:30:0 +8),
        datetime!(2022-1-3 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-1-10 9:30:0 +8),
        datetime!(2022-1-10 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-6-8 9:30:0 +8),
        datetime!(2022-6-6 0:0:0 +8),
    );
}

#[test]
fn sh_month() {
    let t = TestCandlestickTime::new(&CN, Period::Month);
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-1-6 9:30:0 +8),
        datetime!(2022-1-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-1-10 9:30:0 +8),
        datetime!(2022-1-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-5-10 9:30:0 +8),
        datetime!(2022-5-1 0:0:0 +8),
    );
}

#[test]
fn sh_quarter() {
    let t = TestCandlestickTime::new(&CN, Period::Quarter);
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-1-6 9:30:0 +8),
        datetime!(2022-1-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-4-6 9:30:0 +8),
        datetime!(2022-4-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-6-6 9:30:0 +8),
        datetime!(2022-4-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-7-6 9:30:0 +8),
        datetime!(2022-7-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-9-6 9:30:0 +8),
        datetime!(2022-7-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-10-6 9:30:0 +8),
        datetime!(2022-10-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-12-6 9:30:0 +8),
        datetime!(2022-10-1 0:0:0 +8),
    );
}

#[test]
fn sh_year() {
    let t = TestCandlestickTime::new(&CN, Period::Year);
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2022-1-6 9:30:0 +8),
        datetime!(2022-1-1 0:0:0 +8),
    );
    t.check_datetime(
        TRADE_SESSION_NORMAL,
        datetime!(2023-6-6 9:30:0 +8),
        datetime!(2023-1-1 0:0:0 +8),
    );
}
