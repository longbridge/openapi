use strum_macros::{Display, EnumString};

/// Market
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum Market {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// US market
    US,
    /// HK market
    HK,
    /// CN market
    CN,
    /// SG market
    SG,
}

impl_default_for_enum_string!(Market);
impl_serde_for_enum_string!(Market);
