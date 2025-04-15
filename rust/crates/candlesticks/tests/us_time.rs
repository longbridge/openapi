use longport_candlesticks::{
    Period, TRADE_SESSION_NORMAL, TRADE_SESSION_OVERNIGHT, TRADE_SESSION_PRE,
    markets::{US, US_OPTION},
    testutil::TestCandlestickTime,
};
use time::macros::time;

#[test]
fn us_min1() {
    let t = TestCandlestickTime::new(&US, Period::Min_1);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:20:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:30:00), time!(09:30:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(09:34:42), time!(09:34:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(15:59:05), time!(15:59:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(16:00:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(16:05:00), None);
}

#[test]
fn us_opt_min1() {
    let t = TestCandlestickTime::new(&US_OPTION, Period::Min_1);
    t.check_time(TRADE_SESSION_NORMAL, time!(09:20:00), None);
    t.check_time(TRADE_SESSION_NORMAL, time!(15:59:05), time!(15:59:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(16:13:30), time!(16:13:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(16:14:30), time!(16:14:00));
    t.check_time(TRADE_SESSION_NORMAL, time!(16:15:00), None);
}

#[test]
fn us_min1_pre() {
    let t = TestCandlestickTime::new(&US, Period::Min_1);
    t.check_time(TRADE_SESSION_PRE, time!(4:00:00), time!(4:00:00));
    t.check_time(TRADE_SESSION_PRE, time!(5:21:05), time!(5:21:00));
    t.check_time(TRADE_SESSION_PRE, time!(9:29:59), time!(9:29:00));
    t.check_time(TRADE_SESSION_PRE, time!(9:30:00), None);
}

#[test]
fn us_min1_overnight() {
    let t = TestCandlestickTime::new(&US, Period::Min_1);
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(20:00:07), time!(20:00:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(23:30:23), time!(23:30:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(23:59:59), time!(23:59:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(0:00:00), time!(0:00:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(0:00:05), time!(0:00:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(3:20:20), time!(3:20:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(3:59:59), time!(3:59:00));
    t.check_time(TRADE_SESSION_OVERNIGHT, time!(4:00:00), None);
}
