use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use time::OffsetDateTime;

use crate::serde_utils;

/// Options for [`crate::SignalContext::signals`]
///
/// Every field is a filter; leaving one unset removes that filter.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SignalsOptions {
    /// Filter by stock symbol in `ticker.region` format, e.g. `AAPL.US` or
    /// `700.HK`. If omitted, returns signals for all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    /// Filter by strategy id, e.g. `buffett-value`. Preferred over the
    /// deprecated `strategy_name`; takes precedence when both are provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    /// Filter by strategy name. If omitted, returns signals from all
    /// strategies.
    ///
    /// Deprecated in favour of [`SignalsOptions::strategy_id`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_name: Option<String>,
    /// Filter by the catalyst name that triggered the signal. If omitted,
    /// signals with any catalyst name are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_name: Option<String>,
    /// Filter by the catalyst type that triggered the signal, e.g. `News`,
    /// `Fundamental`, `Technical`. If omitted, signals with any catalyst type
    /// are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_type: Option<String>,
    /// Only return signals created at or after this time. If omitted, no
    /// lower bound.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::rfc3339_opt"
    )]
    pub start_time: Option<OffsetDateTime>,
    /// Only return signals created at or before this time. If omitted, no
    /// upper bound.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::rfc3339_opt"
    )]
    pub end_time: Option<OffsetDateTime>,
    /// Maximum number of results to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Number of results to skip for pagination. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Direction a strategy expects the security to take.
///
/// Wire values are the five labels the API returns, matching the
/// `core_conclusion.outlook_enum` scale 1..=5 inside [`Signal::json_data`].
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum Outlook {
    /// Unknown
    Unknown,
    /// Strong bullish (`outlook_enum` 1)
    #[strum(serialize = "Strong bullish")]
    StrongBullish,
    /// Bullish (`outlook_enum` 2)
    Bullish,
    /// Neutral (`outlook_enum` 3)
    Neutral,
    /// Bearish (`outlook_enum` 4)
    Bearish,
    /// Strong bearish (`outlook_enum` 5)
    #[strum(serialize = "Strong bearish")]
    StrongBearish,
}

impl_default_for_enum_string!(Outlook);
impl_serde_for_enum_string!(Outlook);

/// One signal: a strategy's take on a security, triggered by a catalyst.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// Signal ID, e.g. `sign_992_1a00c9425c3_48ab`. Pass it to
    /// [`crate::SignalContext::signal`] for the full record.
    pub id: String,
    /// Security symbol, e.g. `992.HK`
    #[serde(default)]
    pub symbol: String,
    /// Company name
    #[serde(default)]
    pub company_name: String,
    /// Market the security trades in, e.g. `HK`
    #[serde(default)]
    pub market: String,
    /// Signal headline
    #[serde(default)]
    pub title: String,
    /// Natural-language summary of the signal, in Markdown
    #[serde(default)]
    pub summary: String,
    /// Strategy ID that produced the signal
    #[serde(default)]
    pub strategy_id: String,
    /// Strategy name that produced the signal
    #[serde(default)]
    pub strategy_name: String,
    /// Who recommended the signal; empty for strategy-generated signals
    #[serde(default)]
    pub recommend_by: String,
    /// Strategy expression, e.g. `992.HK:GROWTH:long`
    #[serde(default)]
    pub expression: String,
    /// ID of the fact that triggered the signal
    #[serde(default)]
    pub key_fact_id: String,
    /// Display name of the catalyst that triggered the signal
    #[serde(default)]
    pub key_catalyst: String,
    /// Price the analysis was based on
    #[serde(default)]
    pub analysis_price: f64,
    /// Conservative-scenario target price
    #[serde(default)]
    pub conservative_price: f64,
    /// Benchmark-scenario target price
    #[serde(default)]
    pub benchmark_price: f64,
    /// Optimistic-scenario target price
    #[serde(default)]
    pub optimistic_price: f64,
    /// Outlook the strategy takes on the security
    #[serde(default)]
    pub outlook: Outlook,
    /// Outlook label in the caller's language — the localized rendering of
    /// [`Signal::outlook`]
    #[serde(default)]
    pub outlook_desc: String,
    /// Risk level, e.g. `R4`
    #[serde(default)]
    pub risk_level: String,
    /// Signal status
    #[serde(default)]
    pub status: i32,
    /// Display control flag
    #[serde(default)]
    pub display_control: i32,
    /// Full analysis behind the signal, as a JSON document: strategy fit
    /// scores, valuation scenarios, evidence sources and related fact IDs.
    /// Carried verbatim because its shape is strategy-specific.
    #[serde(default)]
    pub json_data: String,
    /// Creation time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp_ms::deserialize"
    )]
    pub created_at: OffsetDateTime,
    /// Last update time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp_ms::deserialize"
    )]
    pub updated_at: OffsetDateTime,
}

/// A page of signals returned by [`crate::SignalContext::signals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalsResponse {
    /// Signals on this page
    #[serde(default)]
    pub signals: Vec<Signal>,
    /// Total number of signals matching the filters, for paging with
    /// [`SignalsOptions::offset`]
    #[serde(default)]
    pub total: i64,
}

/// Options for [`crate::SignalContext::security_facts`]
#[derive(Debug, Clone, Default, Serialize)]
pub struct SecurityFactsOptions {
    /// The security to query, in `ticker.region` format, e.g. `AAPL.US` or
    /// `700.HK`. Required.
    pub symbol: String,
    /// Start of the query window. If omitted, the query includes the earliest
    /// available data.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::rfc3339_opt"
    )]
    pub begin_time: Option<OffsetDateTime>,
    /// End of the query window. If omitted, the query returns the latest data.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::rfc3339_opt"
    )]
    pub end_time: Option<OffsetDateTime>,
    /// Maximum number of facts to return. When more facts fall inside the time
    /// range, only the latest `limit` are returned. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl SecurityFactsOptions {
    /// Create a [`SecurityFactsOptions`] for one security
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            ..Default::default()
        }
    }
}
