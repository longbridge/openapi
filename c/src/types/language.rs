use longbridge_c_macros::CEnum;

/// Language identifer
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::Language")]
#[allow(clippy::enum_variant_names, non_camel_case_types)]
#[repr(C)]
pub enum CLanguage {
    /// zh-CN
    #[c(remote = "ZH_CN")]
    Language_ZH_CN,
    /// zh-HK
    #[c(remote = "ZH_HK")]
    Language_ZH_HK,
    /// en
    #[c(remote = "EN")]
    Language_EN,
}
