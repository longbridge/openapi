use longbridge_c_macros::CEnum;

/// DCA investment frequency
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::dca::types::DCAFrequency")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CDCAFrequency {
    /// Invest every trading day
    #[c(remote = "Daily")]
    DcaFrequencyDaily,
    /// Invest once per week
    #[c(remote = "Weekly")]
    DcaFrequencyWeekly,
    /// Invest every two weeks
    #[c(remote = "Fortnightly")]
    DcaFrequencyFortnightly,
    /// Invest once per month
    #[c(remote = "Monthly")]
    DcaFrequencyMonthly,
}

/// DCA plan status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::dca::types::DCAStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CDCAStatus {
    /// Plan is active
    #[c(remote = "Active")]
    DcaStatusActive,
    /// Plan has been paused
    #[c(remote = "Suspended")]
    DcaStatusSuspended,
    /// Plan has finished
    #[c(remote = "Finished")]
    DcaStatusFinished,
}
