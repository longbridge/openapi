use longbridge::calendar::types as lb;

/// One key-value data pair in a calendar event
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CalendarDataKv {
    /// Key (may be empty)
    pub key: String,
    /// Formatted display value
    pub value: String,
    /// Value type code, e.g. `"estimate_eps"`
    pub value_type: String,
    /// Raw numeric value
    pub value_raw: String,
}
impl From<lb::CalendarDataKv> for CalendarDataKv {
    fn from(v: lb::CalendarDataKv) -> Self {
        Self {
            key: v.key,
            value: v.value,
            value_type: v.value_type,
            value_raw: v.value_raw.map(|d| d.to_string()).unwrap_or_default(),
        }
    }
}

/// One financial calendar event
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CalendarEventInfo {
    /// Security symbol
    pub symbol: String,
    /// Market, e.g. `"HK"`
    pub market: String,
    /// Event content description
    pub content: String,
    /// Security name
    pub counter_name: String,
    /// Date type label, e.g. `"盘前"`
    pub date_type: String,
    /// Event date string, e.g. `"2025.05.02"`
    pub date: String,
    /// Chart UID (may be empty)
    pub chart_uid: String,
    /// Structured data key-value pairs
    pub data_kv: Vec<CalendarDataKv>,
    /// Event type code, e.g. `"financial"`
    pub event_type: String,
    /// Event datetime (unix timestamp string)
    pub datetime: String,
    /// Icon URL
    pub icon: String,
    /// Importance star rating (0–3)
    pub star: i32,
    /// Internal event ID
    pub id: String,
    /// Financial market session time string
    pub financial_market_time: String,
    /// Currency
    pub currency: String,
    /// Activity type code
    pub activity_type: String,
}
impl From<lb::CalendarEventInfo> for CalendarEventInfo {
    fn from(v: lb::CalendarEventInfo) -> Self {
        Self {
            symbol: v.symbol,
            market: v.market,
            content: v.content,
            counter_name: v.counter_name,
            date_type: v.date_type,
            date: v.date,
            chart_uid: v.chart_uid,
            data_kv: v.data_kv.into_iter().map(Into::into).collect(),
            event_type: v.event_type,
            datetime: v.datetime,
            icon: v.icon,
            star: v.star,
            id: v.id,
            financial_market_time: v.financial_market_time,
            currency: v.currency,
            activity_type: v.activity_type,
        }
    }
}

/// Events for one calendar date
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CalendarDateGroup {
    /// Date string, e.g. `"2025-05-02"`
    pub date: String,
    /// Total event count for this date
    pub count: i32,
    /// Event details
    pub infos: Vec<CalendarEventInfo>,
}
impl From<lb::CalendarDateGroup> for CalendarDateGroup {
    fn from(v: lb::CalendarDateGroup) -> Self {
        Self {
            date: v.date,
            count: v.count,
            infos: v.infos.into_iter().map(Into::into).collect(),
        }
    }
}

/// Finance calendar response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CalendarEventsResponse {
    /// Start date of the query window
    pub date: String,
    /// Per-day event groups
    pub list: Vec<CalendarDateGroup>,
}
impl From<lb::CalendarEventsResponse> for CalendarEventsResponse {
    fn from(v: lb::CalendarEventsResponse) -> Self {
        Self {
            date: v.date,
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Calendar event category
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum CalendarCategory {
    /// Earnings reports
    Report,
    /// Dividend events
    Dividend,
    /// Stock splits
    Split,
    /// IPOs
    Ipo,
    /// Macro-economic data releases
    MacroData,
    /// Market closure days
    Closed,
    /// Shareholder / analyst meetings
    Meeting,
    /// Stock consolidations / mergers
    Merge,
}
impl From<CalendarCategory> for lb::CalendarCategory {
    fn from(v: CalendarCategory) -> Self {
        match v {
            CalendarCategory::Report => lb::CalendarCategory::Report,
            CalendarCategory::Dividend => lb::CalendarCategory::Dividend,
            CalendarCategory::Split => lb::CalendarCategory::Split,
            CalendarCategory::Ipo => lb::CalendarCategory::Ipo,
            CalendarCategory::MacroData => lb::CalendarCategory::MacroData,
            CalendarCategory::Closed => lb::CalendarCategory::Closed,
            CalendarCategory::Meeting => lb::CalendarCategory::Meeting,
            CalendarCategory::Merge => lb::CalendarCategory::Merge,
        }
    }
}
