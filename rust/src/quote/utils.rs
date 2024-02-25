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
