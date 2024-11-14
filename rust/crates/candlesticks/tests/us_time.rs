use longport_candlesticks::{
    markets::{US, US_OPTION},
    testutil::TestCandlestickTime,
    Period,
};
use time::macros::time;

#[test]
fn us_min1() {
    let t = TestCandlestickTime::new(&US, Period::Min_1);
    t.check_time(time!(09:20:00), None);
    t.check_time(time!(09:30:00), time!(09:30:00));
    t.check_time(time!(09:34:42), time!(09:34:00));
    t.check_time(time!(15:59:05), time!(15:59:00));
    t.check_time(time!(16:00:00), None);
    t.check_time(time!(16:05:00), None);
}

#[test]
fn us_opt_min1() {
    let t = TestCandlestickTime::new(&US_OPTION, Period::Min_1);
    t.check_time(time!(09:20:00), None);
    t.check_time(time!(15:59:05), time!(15:59:00));
    t.check_time(time!(16:13:30), time!(16:13:00));
    t.check_time(time!(16:14:30), time!(16:14:00));
    t.check_time(time!(16:15:00), None);
}
