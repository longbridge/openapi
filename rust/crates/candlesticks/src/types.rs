use time::OffsetDateTime;

use crate::TradeSessionType;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Period {
    Min_1 = 0,
    Min_2 = 1,
    Min_3 = 2,
    Min_5 = 3,
    Min_10 = 4,
    Min_15 = 5,
    Min_20 = 6,
    Min_30 = 7,
    Min_45 = 8,
    Min_60 = 9,
    Min_120 = 10,
    Min_180 = 11,
    Min_240 = 12,
    Day = 100,
    Week = 101,
    Month = 102,
    Quarter = 103,
    Year = 104,
}

impl Period {
    #[inline]
    pub(crate) fn minutes(&self) -> u8 {
        match self {
            Period::Min_1 => 1,
            Period::Min_2 => 2,
            Period::Min_3 => 3,
            Period::Min_5 => 5,
            Period::Min_10 => 10,
            Period::Min_15 => 15,
            Period::Min_20 => 20,
            Period::Min_30 => 30,
            Period::Min_45 => 45,
            Period::Min_60 => 60,
            Period::Min_120 => 120,
            Period::Min_180 => 180,
            Period::Min_240 => 240,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn is_minute(&self) -> bool {
        (*self as u8) < 100
    }
}

pub trait QuoteType {
    type PriceType;
    type VolumeType;
    type TurnoverType;
    type TradeSessionType;

    fn time(&self) -> OffsetDateTime;
    fn open(&self) -> Self::PriceType;
    fn high(&self) -> Self::PriceType;
    fn low(&self) -> Self::PriceType;
    fn last_done(&self) -> Self::PriceType;
    fn volume(&self) -> Self::VolumeType;
    fn turnover(&self) -> Self::TurnoverType;
    fn trade_session(&self) -> Self::TradeSessionType;
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct UpdateFields: u32 {
        const PRICE = 0x1;
        const VOLUME = 0x2;
    }
}

pub trait TradeType {
    type PriceType;
    type VolumeType;
    type TurnoverType;
    type TradeSessionType: TradeSessionType;

    fn time(&self) -> OffsetDateTime;
    fn price(&self) -> Self::PriceType;
    fn volume(&self) -> Self::VolumeType;
    fn turnover(&self, lot_size: i32) -> Self::TurnoverType;
    fn trade_session(&self) -> Self::TradeSessionType;
}
