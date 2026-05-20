use longbridge_c_macros::CEnum;

/// Flow direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::portfolio::types::FlowDirection")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CFlowDirection {
    /// Unknown direction
    #[c(remote = "Unknown")]
    FlowDirectionUnknown,
    /// Buy
    #[c(remote = "Buy")]
    FlowDirectionBuy,
    /// Sell
    #[c(remote = "Sell")]
    FlowDirectionSell,
}

/// Asset type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::portfolio::types::AssetType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAssetType {
    /// Unknown type
    #[c(remote = "Unknown")]
    AssetTypeUnknown,
    /// Stock
    #[c(remote = "Stock")]
    AssetTypeStock,
    /// Fund
    #[c(remote = "Fund")]
    AssetTypeFund,
    /// Crypto
    #[c(remote = "Crypto")]
    AssetTypeCrypto,
}
