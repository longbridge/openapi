use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::UpdateFields;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Period {
    Min_1,
    Min_5,
    Min_15,
    Min_30,
    Min_60,
    Day,
    Week,
    Month,
    Year,
}

impl Period {
    #[inline]
    pub(crate) fn minutes(&self) -> u8 {
        match self {
            Period::Min_5 => 5,
            Period::Min_15 => 15,
            Period::Min_30 => 30,
            Period::Min_60 => 60,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Trade {
    pub time: OffsetDateTime,
    pub price: Decimal,
    pub volume: i64,
    pub update_fields: UpdateFields,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Quote {
    pub time: OffsetDateTime,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub last_done: Decimal,
    pub volume: i64,
    pub turnover: Decimal,
}
