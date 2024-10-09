use longport_python_macros::PyEnum;
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::Market")]
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
}

#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[py(remote = "longport::Language")]
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
#[py(remote = "longport::PushCandlestickMode")]
pub(crate) enum PushCandlestickMode {
    /// Realtime mode
    Realtime,
    /// Confirmed mode
    Confirmed,
}
