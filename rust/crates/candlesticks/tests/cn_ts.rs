use longport_candlesticks::{TRADE_SESSION_NORMAL, markets::CN};
use time::macros::datetime;

#[test]
fn sh_trade_session() {
    let market = CN;

    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 15:00:00 +8)),
        Some(TRADE_SESSION_NORMAL)
    );
}
