use longport_c_macros::CEnum;

/// Language identifer
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::PushCandlestickMode")]
#[allow(clippy::enum_variant_names, non_camel_case_types)]
#[repr(C)]
pub enum CPushCandlestickMode {
    /// Real-time
    #[c(remote = "Realtime")]
    PushCandlestickMode_Realtime,
    /// Confirmed
    #[c(remote = "Confirmed")]
    PushCandlestickMode_Confirmed,
}
