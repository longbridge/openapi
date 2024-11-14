use rust_decimal::Decimal;
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Candlestick {
    pub time: OffsetDateTime,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: i64,
    pub turnover: Decimal,
}
