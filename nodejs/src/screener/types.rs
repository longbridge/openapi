use longbridge::screener::types as lb;

// ── ScreenerRecommendStrategiesResponse ───────────────────────────

/// Recommended screener strategies response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ScreenerRecommendStrategiesResponse {
    /// Raw recommended strategies data (JSON string)
    pub data: String,
}

impl From<lb::ScreenerRecommendStrategiesResponse> for ScreenerRecommendStrategiesResponse {
    fn from(v: lb::ScreenerRecommendStrategiesResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── ScreenerUserStrategiesResponse ────────────────────────────────

/// User screener strategies response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ScreenerUserStrategiesResponse {
    /// Raw user strategies data (JSON string)
    pub data: String,
}

impl From<lb::ScreenerUserStrategiesResponse> for ScreenerUserStrategiesResponse {
    fn from(v: lb::ScreenerUserStrategiesResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── ScreenerStrategyResponse ──────────────────────────────────────

/// Single screener strategy response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ScreenerStrategyResponse {
    /// Raw strategy detail data (JSON string)
    pub data: String,
}

impl From<lb::ScreenerStrategyResponse> for ScreenerStrategyResponse {
    fn from(v: lb::ScreenerStrategyResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── ScreenerSearchResponse ────────────────────────────────────────

/// Screener search results response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ScreenerSearchResponse {
    /// Raw search results data (JSON string)
    pub data: String,
}

impl From<lb::ScreenerSearchResponse> for ScreenerSearchResponse {
    fn from(v: lb::ScreenerSearchResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── ScreenerIndicatorsResponse ────────────────────────────────────

/// Screener indicator definitions response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ScreenerIndicatorsResponse {
    /// Raw indicator definitions data (JSON string)
    pub data: String,
}

impl From<lb::ScreenerIndicatorsResponse> for ScreenerIndicatorsResponse {
    fn from(v: lb::ScreenerIndicatorsResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}
