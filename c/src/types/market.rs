use longport_c_macros::CEnum;

/// Market type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::Market")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CMarket {
    /// Unknown
    #[c(remote = "Unknown")]
    MarketUnknown,
    /// US market
    #[c(remote = "US")]
    MarketUS,
    /// HK market
    #[c(remote = "HK")]
    MarketHK,
    /// CN market
    #[c(remote = "CN")]
    MarketCN,
    /// SG market
    #[c(remote = "SG")]
    MarketSG,
}
