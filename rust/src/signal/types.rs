use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{Display, EnumString};
use time::OffsetDateTime;

use crate::serde_utils;

/// Options for [`crate::SignalContext::signals`]
///
/// Every field is a filter; leaving one unset removes that filter.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SignalsOptions {
    /// Filter by security symbol, e.g. `AAPL.US` or `700.HK`. If omitted,
    /// returns signals for all symbols.
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
    /// Filter by the name of the factor that triggered the signal, e.g.
    /// `EARNINGS_RELEASED` or `macd_12_26_9` — the `factors[].name` of the
    /// triggering fact, not the display label a signal carries in
    /// [`Signal::key_catalyst`]. If omitted, signals with any catalyst name
    /// are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_name: Option<String>,
    /// Filter by the catalyst type that triggered the signal, e.g. `News`,
    /// `Fundamental`, `Technical`. If omitted, signals with any catalyst type
    /// are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_type: Option<String>,
    /// Only return signals created at or after this time. Sent as RFC3339;
    /// the API also accepts a Unix timestamp. If omitted, no lower bound.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::rfc3339_opt"
    )]
    pub start_time: Option<OffsetDateTime>,
    /// Only return signals created at or before this time. Sent as RFC3339;
    /// the API also accepts a Unix timestamp. If omitted, no upper bound.
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

/// Where a signal is in its lifecycle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SignalStatus {
    /// Generated, not yet published
    Pending = 0,
    /// Published and current — the only status the API serves today
    #[default]
    Active = 1,
    /// Deleted
    Deleted = 2,
    /// The strategy analysis failed to generate
    AiFailed = 3,
    /// Filtered out by a human reviewer
    FilteredByManual = 4,
    /// The strategy analysis failed to submit
    AiSubmitFailed = 5,
    /// A status this SDK does not know yet
    #[serde(other)]
    Unknown = -1,
}

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
    /// Display label of the catalyst that triggered the signal, e.g.
    /// `Q1 Revenue Surge`. This is prose meant for display — filtering with
    /// [`SignalsOptions::catalyst_name`] matches the underlying factor name
    /// instead
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
    /// Where the signal is in its lifecycle
    #[serde(default)]
    pub status: SignalStatus,
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
    pub total: i32,
}

/// Options for [`crate::SignalContext::security_facts`]
#[derive(Debug, Clone, Default, Serialize)]
pub struct SecurityFactsOptions {
    /// The security to query, e.g. `AAPL.US` or `700.HK`. Required.
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

/// Kind of fact, and of the source that produced it.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum FactType {
    /// Unknown
    Unknown,
    /// Derived from news
    News,
    /// Derived from fundamentals
    Fundamental,
    /// Derived from technical indicators
    Technical,
}

impl_default_for_enum_string!(FactType);
impl_serde_for_enum_string!(FactType);

/// Side a fact or factor points to.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum FactDirection {
    /// Unknown
    #[strum(serialize = "")]
    Unknown,
    /// Long
    #[strum(serialize = "long")]
    Long,
    /// Short
    #[strum(serialize = "short")]
    Short,
    /// Neutral — the fact points either way
    #[strum(serialize = "neutral")]
    Neutral,
}

impl_default_for_enum_string!(FactDirection);
impl_serde_for_enum_string!(FactDirection);

/// Where a fact came from.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactDataSource {
    /// Source name, e.g. `Nasdaq`
    #[serde(default)]
    pub source_name: String,
    /// Kind of source
    #[serde(default, rename = "type")]
    pub source_type: FactType,
    /// Link to the source, when it has one
    #[serde(default)]
    pub url: String,
    /// Source icon URL, when it has one
    #[serde(default)]
    pub icon: String,
}

/// Thresholds an anomaly test was scored against.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnomalyThresholds {
    /// Low threshold
    #[serde(default)]
    pub low: String,
    /// Medium threshold
    #[serde(default)]
    pub medium: String,
    /// High threshold
    #[serde(default)]
    pub high: String,
}

/// Outcome of the anomaly test behind a factor. Fields are empty for factors
/// that did not run one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnomalyDetection {
    /// Test outcome
    #[serde(default)]
    pub anomaly_result: String,
    /// Significance level of the test
    #[serde(default)]
    pub significance_level: String,
    /// Method used, e.g. a statistical test name
    #[serde(default)]
    pub test_method: String,
    /// Thresholds the result was scored against
    #[serde(default)]
    pub thresholds: AnomalyThresholds,
}

/// One factor that contributed to a fact.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactFactor {
    /// Factor name, e.g. `rsi_14`
    #[serde(default)]
    pub name: String,
    /// Groups the factor belongs to, e.g. `MOMENTUM`
    #[serde(default)]
    pub factor_groups: Vec<String>,
    /// Side the factor points to
    #[serde(default)]
    pub long_short_direction: FactDirection,
    /// Condition that fired the factor
    #[serde(default)]
    pub trigger_condition: String,
    /// Anomaly test behind the factor
    #[serde(default)]
    pub anomaly_detection: AnomalyDetection,
}

/// A security a fact is about.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactSymbol {
    /// Security symbol, e.g. `AAPL.US`
    #[serde(default)]
    pub symbol: String,
    /// Security name in the caller's language
    #[serde(default)]
    pub security_name: String,
}

/// One `{tag, value}` entry from a natural-language field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NlTag {
    /// What the entry is about, e.g. `RSI`
    #[serde(default)]
    pub tag: String,
    /// The prose
    #[serde(default)]
    pub value: String,
}

/// The natural-language rendering of a fact, in the caller's language.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactNlInfo {
    /// Headline
    #[serde(default)]
    pub title: String,
    /// Sub-headline
    #[serde(default)]
    pub sub_title: String,
    /// What happened, as a JSON array of `{tag, value}` entries carried in a
    /// string. Use [`FactNlInfo::summary_tags`] to read it.
    #[serde(default)]
    pub summary: String,
    /// What it may mean for an investor, in the same JSON-in-a-string shape as
    /// [`FactNlInfo::summary`]. Use [`FactNlInfo::invest_anal_tags`] to read
    /// it.
    #[serde(default)]
    pub invest_anal: String,
    /// A plain-language walk-through of the fact, in the same JSON-in-a-string
    /// shape as [`FactNlInfo::summary`]. Use [`FactNlInfo::eli_explain_tags`]
    /// to read it.
    #[serde(default)]
    pub eli_explain: String,
}

impl FactNlInfo {
    /// Parse [`FactNlInfo::summary`] into its `{tag, value}` entries.
    ///
    /// Returns an empty list when the field is empty or not the expected JSON —
    /// the raw string stays available either way.
    pub fn summary_tags(&self) -> Vec<NlTag> {
        serde_json::from_str(&self.summary).unwrap_or_default()
    }

    /// Parse [`FactNlInfo::invest_anal`] into its `{tag, value}` entries.
    ///
    /// Returns an empty list when the field is empty or not the expected JSON —
    /// the raw string stays available either way.
    pub fn invest_anal_tags(&self) -> Vec<NlTag> {
        serde_json::from_str(&self.invest_anal).unwrap_or_default()
    }

    /// Parse [`FactNlInfo::eli_explain`] into its `{tag, value}` entries.
    ///
    /// Returns an empty list when the field is empty or not the expected JSON —
    /// the raw string stays available either way.
    pub fn eli_explain_tags(&self) -> Vec<NlTag> {
        serde_json::from_str(&self.eli_explain).unwrap_or_default()
    }
}

/// A fact (catalyst) event: something that happened to a security, with the
/// factors, sources and prose behind it.
///
/// Facts are what strategies react to — a signal names the fact that triggered
/// it in [`Signal::key_fact_id`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFact {
    /// Fact ID, e.g. `technical_rsi_14_short_1783674041337603409`
    #[serde(default)]
    pub fact_id: String,
    /// What kind of fact this is
    #[serde(default)]
    pub fact_type: FactType,
    /// Side the fact points to
    #[serde(default)]
    pub direction: FactDirection,
    /// When the fact occurred
    #[serde(with = "time::serde::rfc3339")]
    pub occur_time: OffsetDateTime,
    /// Securities the fact is about
    #[serde(default)]
    pub symbols_info: Vec<FactSymbol>,
    /// Factors that contributed to the fact
    #[serde(default)]
    pub factors: Vec<FactFactor>,
    /// Where the fact came from
    #[serde(default)]
    pub data_source: Vec<FactDataSource>,
    /// Natural-language rendering of the fact
    #[serde(default)]
    pub nl_info: FactNlInfo,
}
