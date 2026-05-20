use std::os::raw::c_char;

use longbridge::screener::types::{
    ScreenerIndicatorsResponse, ScreenerRecommendStrategiesResponse, ScreenerSearchResponse,
    ScreenerStrategyResponse, ScreenerUserStrategiesResponse,
};

use crate::types::{CString, ToFFI};

// ── ScreenerRecommendStrategiesResponse ───────────────────────────

/// Recommended screener strategies response. `data` is a JSON string.
#[repr(C)]
pub struct CScreenerRecommendStrategiesResponse {
    pub data: *const c_char,
}

pub(crate) struct CScreenerRecommendStrategiesResponseOwned {
    data: CString,
}

impl From<ScreenerRecommendStrategiesResponse> for CScreenerRecommendStrategiesResponseOwned {
    fn from(v: ScreenerRecommendStrategiesResponse) -> Self {
        Self {
            data: serde_json::to_string(&v.data).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CScreenerRecommendStrategiesResponseOwned {
    type FFIType = CScreenerRecommendStrategiesResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CScreenerRecommendStrategiesResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── ScreenerUserStrategiesResponse ────────────────────────────────

/// User screener strategies response. `data` is a JSON string.
#[repr(C)]
pub struct CScreenerUserStrategiesResponse {
    pub data: *const c_char,
}

pub(crate) struct CScreenerUserStrategiesResponseOwned {
    data: CString,
}

impl From<ScreenerUserStrategiesResponse> for CScreenerUserStrategiesResponseOwned {
    fn from(v: ScreenerUserStrategiesResponse) -> Self {
        Self {
            data: serde_json::to_string(&v.data).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CScreenerUserStrategiesResponseOwned {
    type FFIType = CScreenerUserStrategiesResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CScreenerUserStrategiesResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── ScreenerStrategyResponse ──────────────────────────────────────

/// Single screener strategy response. `data` is a JSON string.
#[repr(C)]
pub struct CScreenerStrategyResponse {
    pub data: *const c_char,
}

pub(crate) struct CScreenerStrategyResponseOwned {
    data: CString,
}

impl From<ScreenerStrategyResponse> for CScreenerStrategyResponseOwned {
    fn from(v: ScreenerStrategyResponse) -> Self {
        Self {
            data: serde_json::to_string(&v.data).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CScreenerStrategyResponseOwned {
    type FFIType = CScreenerStrategyResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CScreenerStrategyResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── ScreenerSearchResponse ────────────────────────────────────────

/// Screener search results response. `data` is a JSON string.
#[repr(C)]
pub struct CScreenerSearchResponse {
    pub data: *const c_char,
}

pub(crate) struct CScreenerSearchResponseOwned {
    data: CString,
}

impl From<ScreenerSearchResponse> for CScreenerSearchResponseOwned {
    fn from(v: ScreenerSearchResponse) -> Self {
        Self {
            data: serde_json::to_string(&v.data).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CScreenerSearchResponseOwned {
    type FFIType = CScreenerSearchResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CScreenerSearchResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── ScreenerIndicatorsResponse ────────────────────────────────────

/// Screener indicator definitions response. `data` is a JSON string.
#[repr(C)]
pub struct CScreenerIndicatorsResponse {
    pub data: *const c_char,
}

pub(crate) struct CScreenerIndicatorsResponseOwned {
    data: CString,
}

impl From<ScreenerIndicatorsResponse> for CScreenerIndicatorsResponseOwned {
    fn from(v: ScreenerIndicatorsResponse) -> Self {
        Self {
            data: serde_json::to_string(&v.data).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CScreenerIndicatorsResponseOwned {
    type FFIType = CScreenerIndicatorsResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CScreenerIndicatorsResponse {
            data: self.data.to_ffi_type(),
        }
    }
}
