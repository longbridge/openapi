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

/// Convert a symbol like "700.HK" into the counter-id format "ST/HK/700".
pub(crate) fn symbol_to_counter_id(symbol: &str) -> String {
    if let Some((code, market)) = symbol.rsplit_once('.') {
        format!("ST/{}/{}", market.to_uppercase(), code)
    } else {
        symbol.to_string()
    }
}
