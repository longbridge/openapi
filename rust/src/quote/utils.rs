use longport_candlesticks::{
    TRADE_SESSION_NORMAL, TRADE_SESSION_OVERNIGHT, TRADE_SESSION_POST, TRADE_SESSION_PRE,
    TradeSessionType,
};
use time::Date;

#[inline]
pub(crate) fn parse_date(date: &str) -> Result<Date, time::error::Parse> {
    Date::parse(
        date,
        time::macros::format_description!("[year][month][day]"),
    )
}

pub(crate) fn format_date(date: Date) -> String {
    date.format(time::macros::format_description!("[year][month][day]"))
        .unwrap()
}

pub(crate) fn convert_trade_session(ts: longport_proto::quote::TradeSession) -> TradeSessionType {
    match ts {
        longport_proto::quote::TradeSession::PreTrade => TRADE_SESSION_PRE,
        longport_proto::quote::TradeSession::NormalTrade => TRADE_SESSION_NORMAL,
        longport_proto::quote::TradeSession::PostTrade => TRADE_SESSION_POST,
        longport_proto::quote::TradeSession::OvernightTrade => TRADE_SESSION_OVERNIGHT,
    }
}
