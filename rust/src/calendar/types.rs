#![allow(missing_docs)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::utils::counter::deserialize_counter_id_as_symbol;

/// Response for [`crate::CalendarContext::finance_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEventsResponse {
    /// Start date of the query window
    pub date: String,
    /// Per-day event groups
    pub list: Vec<CalendarDateGroup>,
}

/// Events for one calendar date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarDateGroup {
    /// Date string, e.g. `"2025-05-02"`
    pub date: String,
    /// Total event count for this date
    pub count: i32,
    /// Event details
    pub infos: Vec<CalendarEventInfo>,
}

/// One financial calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEventInfo {
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Market, e.g. `"HK"`
    pub market: String,
    /// Event content description
    pub content: String,
    /// Security name
    pub counter_name: String,
    /// Date type label, e.g. `"盘前"`
    #[serde(default)]
    pub date_type: String,
    /// Event date string, e.g. `"2025.05.02"`
    pub date: String,
    /// Chart UID (may be empty)
    #[serde(default)]
    pub chart_uid: String,
    /// Structured data key-value pairs
    pub data_kv: Vec<CalendarDataKv>,
    /// Event type code, e.g. `"financial"`
    #[serde(rename = "type")]
    pub event_type: String,
    /// Event datetime (unix timestamp string)
    pub datetime: String,
    /// Icon URL
    #[serde(default)]
    pub icon: String,
    /// Importance star rating (0–3)
    pub star: i32,
    /// Associated live stream (usually null)
    pub live: Option<serde_json::Value>,
    /// Internal event ID
    pub id: String,
    /// Financial market session time string
    #[serde(default)]
    pub financial_market_time: String,
    /// Currency
    #[serde(default)]
    pub currency: String,
    /// Extended data (structure varies by event type)
    pub ext: Option<serde_json::Value>,
    /// Activity type code
    #[serde(default)]
    pub activity_type: String,
}

/// One key-value data pair in a calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarDataKv {
    /// Key (may be empty)
    pub key: String,
    /// Formatted display value
    pub value: String,
    /// Value type code, e.g. `"estimate_eps"`
    #[serde(rename = "type")]
    pub value_type: String,
    /// Raw numeric value (may be empty or non-numeric)
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub value_raw: Option<Decimal>,
}

/// Calendar event category
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CalendarCategory {
    /// Earnings reports
    #[serde(rename = "report")]
    Report,
    /// Dividend events
    #[serde(rename = "dividend")]
    Dividend,
    /// Stock splits
    #[serde(rename = "split")]
    Split,
    /// IPOs
    #[serde(rename = "ipo")]
    Ipo,
    /// Macro-economic data releases
    #[serde(rename = "macrodata")]
    MacroData,
    /// Market closure days
    #[serde(rename = "closed")]
    Closed,
}
