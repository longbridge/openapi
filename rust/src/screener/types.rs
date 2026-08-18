use serde::{Deserialize, Serialize};

// ── screener_recommend_strategies ─────────────────────────────────

/// Response for [`crate::ScreenerContext::screener_recommend_strategies`]
///
/// The raw data contains a list of recommended built-in screener
/// strategies.  The exact structure varies so the payload is
/// preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerRecommendStrategiesResponse {
    /// Raw recommended strategies data
    pub data: serde_json::Value,
}

// ── screener_user_strategies ──────────────────────────────────────

/// Response for [`crate::ScreenerContext::screener_user_strategies`]
///
/// The raw data contains the current user's saved screener strategies.
/// The exact structure varies so the payload is preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerUserStrategiesResponse {
    /// Raw user strategies data
    pub data: serde_json::Value,
}

// ── screener_strategy ─────────────────────────────────────────────

/// Response for [`crate::ScreenerContext::screener_strategy`]
///
/// The raw data contains detail for one screener strategy.
/// The exact structure varies so the payload is preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerStrategyResponse {
    /// Raw strategy detail data
    pub data: serde_json::Value,
}

// ── screener_condition ───────────────────────────────────────────

/// A filter condition for [`crate::ScreenerContext::screener_search`] Mode B.
///
/// `key` is the indicator key (without the `filter_` prefix, e.g. `"pettm"`).
/// `min` / `max` bound the range; leave empty for an open bound.
/// `tech_values` is used for technical indicators (e.g. MACD/RSI); pass an
/// empty map `{}` for fundamental indicators.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreenerCondition {
    /// Indicator key without `filter_` prefix, e.g. `"pettm"`, `"roe"`,
    /// `"macd_day"`
    pub key: String,
    /// Lower bound (empty string = no lower bound)
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub min: String,
    /// Upper bound (empty string = no upper bound)
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub max: String,
    /// Technical indicator parameters (empty map for fundamental indicators).
    /// Example: `{"category": "goldenfork", "period": "day"}`
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub tech_values: serde_json::Value,
}

// ── screener_search ───────────────────────────────────────────────

/// Response for [`crate::ScreenerContext::screener_search`]
///
/// The raw data contains a page of screened security results.
/// The exact structure varies so the payload is preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerSearchResponse {
    /// Raw screener search results
    pub data: serde_json::Value,
}

// ── screener_indicators ───────────────────────────────────────────

/// Response for [`crate::ScreenerContext::screener_indicators`]
///
/// The raw data contains all available screener indicator definitions.
/// The exact structure varies so the payload is preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerIndicatorsResponse {
    /// Raw indicator definitions
    pub data: serde_json::Value,
}
