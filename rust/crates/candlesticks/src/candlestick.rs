use time::OffsetDateTime;

use crate::TradeSessionType;

#[derive(Debug)]
pub struct CandlestickComponents<PriceType, VolumeType, TurnoverType, TradeSessionType> {
    pub time: OffsetDateTime,
    pub open: PriceType,
    pub high: PriceType,
    pub low: PriceType,
    pub close: PriceType,
    pub volume: VolumeType,
    pub turnover: TurnoverType,
    pub trade_session: TradeSessionType,
}

pub trait CandlestickType {
    type PriceType;
    type VolumeType;
    type TurnoverType;
    type TradeSessionType: TradeSessionType;

    fn new(
        components: CandlestickComponents<
            Self::PriceType,
            Self::VolumeType,
            Self::TurnoverType,
            Self::TradeSessionType,
        >,
    ) -> Self;

    fn time(&self) -> OffsetDateTime;
    fn set_time(&mut self, time: OffsetDateTime);

    fn open(&self) -> Self::PriceType;
    fn set_open(&mut self, open: Self::PriceType);

    fn high(&self) -> Self::PriceType;
    fn set_high(&mut self, high: Self::PriceType);

    fn low(&self) -> Self::PriceType;
    fn set_low(&mut self, low: Self::PriceType);

    fn close(&self) -> Self::PriceType;
    fn set_close(&mut self, close: Self::PriceType);

    fn volume(&self) -> Self::VolumeType;
    fn set_volume(&mut self, volume: Self::VolumeType);

    fn turnover(&self) -> Self::TurnoverType;
    fn set_turnover(&mut self, turnover: Self::TurnoverType);

    fn trade_session(&self) -> Self::TradeSessionType;
}
