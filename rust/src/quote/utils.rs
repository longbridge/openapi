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

#[inline]
pub(crate) fn get_market_from_symbol(symbol: &str) -> Option<&str> {
    symbol.find('.').map(|idx| &symbol[idx + 1..])
}
