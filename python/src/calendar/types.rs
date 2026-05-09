use longbridge::calendar::types as lb;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

#[derive(Debug, Clone)]
pub(crate) struct JsonValue(pub(crate) serde_json::Value);

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
impl<'py> IntoPyObject<'py> for &JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

/// One key-value pair in a calendar event
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct CalendarDataKv {
    /// Key (may be empty)
    pub key: String,
    /// Formatted display value
    pub value: String,
    /// Value type code
    pub value_type: String,
    /// Raw numeric value
    pub value_raw: Option<String>,
}

impl From<lb::CalendarDataKv> for CalendarDataKv {
    fn from(v: lb::CalendarDataKv) -> Self {
        Self {
            key: v.key,
            value: v.value,
            value_type: v.value_type,
            value_raw: v.value_raw.map(|d| d.to_string()),
        }
    }
}

/// One financial calendar event
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct CalendarEventInfo {
    /// Security symbol
    pub symbol: String,
    /// Market
    pub market: String,
    /// Event description
    pub content: String,
    /// Security name
    pub counter_name: String,
    /// Date type label
    pub date_type: String,
    /// Event date string
    pub date: String,
    /// Chart UID
    pub chart_uid: String,
    /// Structured key-value pairs
    pub data_kv: Vec<CalendarDataKv>,
    /// Event type code
    pub event_type: String,
    /// Event datetime (unix timestamp string)
    pub datetime: String,
    /// Icon URL
    pub icon: String,
    /// Importance star rating
    pub star: i32,
    /// Internal event ID
    pub id: String,
    /// Financial market session time
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
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct CalendarDateGroup {
    /// Date string
    pub date: String,
    /// Total event count
    pub count: i32,
    /// Events
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
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct CalendarEventsResponse {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum CalendarCategory {
    /// Earnings reports
    Report = 0,
    /// Dividends
    Dividend = 1,
    /// Stock splits
    Split = 2,
    /// IPOs
    Ipo = 3,
    /// Macro-economic data
    MacroData = 4,
    /// Market closure days
    Closed = 5,
    /// Shareholder / analyst meetings
    Meeting = 6,
    /// Stock consolidations / mergers
    Merge = 7,
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
