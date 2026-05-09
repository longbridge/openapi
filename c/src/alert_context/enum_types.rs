use longbridge_c_macros::CEnum;

/// Alert trigger condition
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::alert::types::AlertCondition")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAlertCondition {
    /// Price rises above threshold
    #[c(remote = "PriceRise")]
    AlertConditionPriceRise,
    /// Price falls below threshold
    #[c(remote = "PriceFall")]
    AlertConditionPriceFall,
    /// Percentage rises above threshold
    #[c(remote = "PercentRise")]
    AlertConditionPercentRise,
    /// Percentage falls below threshold
    #[c(remote = "PercentFall")]
    AlertConditionPercentFall,
}

/// Alert notification frequency
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::alert::types::AlertFrequency")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAlertFrequency {
    /// Trigger at most once per day
    #[c(remote = "Daily")]
    AlertFrequencyDaily,
    /// Trigger every time the condition is met
    #[c(remote = "EveryTime")]
    AlertFrequencyEveryTime,
    /// Trigger only the first time
    #[c(remote = "Once")]
    AlertFrequencyOnce,
}
