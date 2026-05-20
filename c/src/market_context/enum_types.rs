use longbridge_c_macros::CEnum;

/// Broker holding lookback period
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::market::types::BrokerHoldingPeriod")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBrokerHoldingPeriod {
    /// 1 recent trading day
    #[c(remote = "Rct1")]
    BrokerHoldingPeriodRct1,
    /// 5 recent trading days
    #[c(remote = "Rct5")]
    BrokerHoldingPeriodRct5,
    /// 20 recent trading days
    #[c(remote = "Rct20")]
    BrokerHoldingPeriodRct20,
    /// 60 recent trading days
    #[c(remote = "Rct60")]
    BrokerHoldingPeriodRct60,
}

/// A/H premium K-line period
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::market::types::AhPremiumPeriod")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAhPremiumPeriod {
    /// 1-minute
    #[c(remote = "Min1")]
    AhPremiumPeriodMin1,
    /// 5-minute
    #[c(remote = "Min5")]
    AhPremiumPeriodMin5,
    /// 15-minute
    #[c(remote = "Min15")]
    AhPremiumPeriodMin15,
    /// 30-minute
    #[c(remote = "Min30")]
    AhPremiumPeriodMin30,
    /// 60-minute
    #[c(remote = "Min60")]
    AhPremiumPeriodMin60,
    /// Daily
    #[c(remote = "Day")]
    AhPremiumPeriodDay,
    /// Weekly
    #[c(remote = "Week")]
    AhPremiumPeriodWeek,
    /// Monthly
    #[c(remote = "Month")]
    AhPremiumPeriodMonth,
    /// Yearly
    #[c(remote = "Year")]
    AhPremiumPeriodYear,
}
