use longbridge::screener::types as lb;
use pyo3::prelude::*;

// ── ScreenerRecommendStrategiesResponse ───────────────────────────

/// Recommended screener strategies response.
///
/// `data` is the raw JSON returned by the API preserved as a Python
/// object (dict / list / etc.).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ScreenerRecommendStrategiesResponse {
    /// Raw recommended strategies data (JSON object)
    pub data: crate::fundamental::types::JsonValue,
}

impl From<lb::ScreenerRecommendStrategiesResponse> for ScreenerRecommendStrategiesResponse {
    fn from(v: lb::ScreenerRecommendStrategiesResponse) -> Self {
        Self {
            data: crate::fundamental::types::JsonValue(v.data),
        }
    }
}

// ── ScreenerUserStrategiesResponse ────────────────────────────────

/// User screener strategies response.
///
/// `data` is the raw JSON returned by the API preserved as a Python
/// object (dict / list / etc.).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ScreenerUserStrategiesResponse {
    /// Raw user strategies data (JSON object)
    pub data: crate::fundamental::types::JsonValue,
}

impl From<lb::ScreenerUserStrategiesResponse> for ScreenerUserStrategiesResponse {
    fn from(v: lb::ScreenerUserStrategiesResponse) -> Self {
        Self {
            data: crate::fundamental::types::JsonValue(v.data),
        }
    }
}

// ── ScreenerStrategyResponse ──────────────────────────────────────

/// Single screener strategy response.
///
/// `data` is the raw JSON returned by the API preserved as a Python
/// object (dict / list / etc.).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ScreenerStrategyResponse {
    /// Raw strategy detail data (JSON object)
    pub data: crate::fundamental::types::JsonValue,
}

impl From<lb::ScreenerStrategyResponse> for ScreenerStrategyResponse {
    fn from(v: lb::ScreenerStrategyResponse) -> Self {
        Self {
            data: crate::fundamental::types::JsonValue(v.data),
        }
    }
}

// ── ScreenerSearchResponse ────────────────────────────────────────

/// Screener search results response.
///
/// `data` is the raw JSON returned by the API preserved as a Python
/// object (dict / list / etc.).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ScreenerSearchResponse {
    /// Raw search results data (JSON object)
    pub data: crate::fundamental::types::JsonValue,
}

impl From<lb::ScreenerSearchResponse> for ScreenerSearchResponse {
    fn from(v: lb::ScreenerSearchResponse) -> Self {
        Self {
            data: crate::fundamental::types::JsonValue(v.data),
        }
    }
}

// ── ScreenerIndicatorsResponse ────────────────────────────────────

/// Screener indicator definitions response.
///
/// `data` is the raw JSON returned by the API preserved as a Python
/// object (dict / list / etc.).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ScreenerIndicatorsResponse {
    /// Raw indicator definitions data (JSON object)
    pub data: crate::fundamental::types::JsonValue,
}

impl From<lb::ScreenerIndicatorsResponse> for ScreenerIndicatorsResponse {
    fn from(v: lb::ScreenerIndicatorsResponse) -> Self {
        Self {
            data: crate::fundamental::types::JsonValue(v.data),
        }
    }
}
