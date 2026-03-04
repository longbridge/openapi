use longbridge_python_macros::PyEnum;
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::Market")]
pub(crate) enum Market {
    /// Unknown
    Unknown,
    /// US market
    US,
    /// HK market
    HK,
    /// CN market
    CN,
    /// SG market
    SG,
    /// Crypto market
    Crypto,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[py(remote = "longbridge::Language")]
pub(crate) enum Language {
    /// zh-CN
    ZH_CN,
    /// zh-HK
    ZH_HK,
    /// en
    EN,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[py(remote = "longbridge::PushCandlestickMode")]
pub(crate) enum PushCandlestickMode {
    /// Realtime mode
    Realtime,
    /// Confirmed mode
    Confirmed,
}
