use std::os::raw::c_char;

use longbridge::calendar::types::*;

use crate::types::{CString, CVec, ToFFI};

/// A key-value pair carrying calendar data fields.
#[repr(C)]
pub struct CCalendarDataKv {
    /// Field key name.
    pub key: *const c_char,
    /// Display value.
    pub value: *const c_char,
    /// Type of the value (e.g. "string", "number").
    pub value_type: *const c_char,
    /// Raw, unformatted value.
    pub value_raw: *const c_char,
}
pub(crate) struct CCalendarDataKvOwned {
    key: CString,
    value: CString,
    value_type: CString,
    value_raw: CString,
}
impl From<CalendarDataKv> for CCalendarDataKvOwned {
    fn from(v: CalendarDataKv) -> Self {
        Self {
            key: v.key.into(),
            value: v.value.into(),
            value_type: v.value_type.into(),
            value_raw: v
                .value_raw
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}
impl ToFFI for CCalendarDataKvOwned {
    type FFIType = CCalendarDataKv;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCalendarDataKv {
            key: self.key.to_ffi_type(),
            value: self.value.to_ffi_type(),
            value_type: self.value_type.to_ffi_type(),
            value_raw: self.value_raw.to_ffi_type(),
        }
    }
}

/// Detailed information for a single calendar event.
#[repr(C)]
pub struct CCalendarEventInfo {
    /// Associated ticker symbol (may be empty).
    pub symbol: *const c_char,
    /// Market the symbol belongs to (e.g. "US", "HK").
    pub market: *const c_char,
    /// Human-readable event description / content.
    pub content: *const c_char,
    /// Display name of the issuer or counter party.
    pub counter_name: *const c_char,
    /// Classification of the date field (e.g. "announce", "ex-dividend").
    pub date_type: *const c_char,
    /// Event date string (e.g. "2025-03-15").
    pub date: *const c_char,
    /// Unique identifier used to retrieve the associated chart.
    pub chart_uid: *const c_char,
    /// Pointer to an array of extra key-value data pairs for this event.
    pub data_kv: *const CCalendarDataKv,
    /// Number of elements in the `data_kv` array.
    pub num_data_kv: usize,
    /// Event type identifier string.
    pub event_type: *const c_char,
    /// Full datetime string for events with a specific time component.
    pub datetime: *const c_char,
    /// URL of the icon image representing this event.
    pub icon: *const c_char,
    /// Star / importance rating for the event (higher is more important).
    pub star: i32,
    /// Unique event ID.
    pub id: *const c_char,
    /// Financial-market local time string for this event.
    pub financial_market_time: *const c_char,
    /// Currency code relevant to the event (e.g. "USD").
    pub currency: *const c_char,
    /// Activity type classification string.
    pub activity_type: *const c_char,
}
pub(crate) struct CCalendarEventInfoOwned {
    symbol: CString,
    market: CString,
    content: CString,
    counter_name: CString,
    date_type: CString,
    date: CString,
    chart_uid: CString,
    data_kv: CVec<CCalendarDataKvOwned>,
    event_type: CString,
    datetime: CString,
    icon: CString,
    star: i32,
    id: CString,
    financial_market_time: CString,
    currency: CString,
    activity_type: CString,
}
impl From<CalendarEventInfo> for CCalendarEventInfoOwned {
    fn from(v: CalendarEventInfo) -> Self {
        Self {
            symbol: v.symbol.into(),
            market: v.market.into(),
            content: v.content.into(),
            counter_name: v.counter_name.into(),
            date_type: v.date_type.into(),
            date: v.date.into(),
            chart_uid: v.chart_uid.into(),
            data_kv: v.data_kv.into(),
            event_type: v.event_type.into(),
            datetime: v.datetime.into(),
            icon: v.icon.into(),
            star: v.star,
            id: v.id.into(),
            financial_market_time: v.financial_market_time.into(),
            currency: v.currency.into(),
            activity_type: v.activity_type.into(),
        }
    }
}
impl ToFFI for CCalendarEventInfoOwned {
    type FFIType = CCalendarEventInfo;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCalendarEventInfo {
            symbol: self.symbol.to_ffi_type(),
            market: self.market.to_ffi_type(),
            content: self.content.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            date_type: self.date_type.to_ffi_type(),
            date: self.date.to_ffi_type(),
            chart_uid: self.chart_uid.to_ffi_type(),
            data_kv: self.data_kv.to_ffi_type(),
            num_data_kv: self.data_kv.len(),
            event_type: self.event_type.to_ffi_type(),
            datetime: self.datetime.to_ffi_type(),
            icon: self.icon.to_ffi_type(),
            star: self.star,
            id: self.id.to_ffi_type(),
            financial_market_time: self.financial_market_time.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            activity_type: self.activity_type.to_ffi_type(),
        }
    }
}

/// A group of calendar events that share the same date.
#[repr(C)]
pub struct CCalendarDateGroup {
    /// Date string for this group (e.g. "2025-03-15").
    pub date: *const c_char,
    /// Total number of events on this date.
    pub count: i32,
    /// Pointer to an array of event info items.
    pub infos: *const CCalendarEventInfo,
    /// Number of elements in the `infos` array.
    pub num_infos: usize,
}
pub(crate) struct CCalendarDateGroupOwned {
    date: CString,
    count: i32,
    infos: CVec<CCalendarEventInfoOwned>,
}
impl From<CalendarDateGroup> for CCalendarDateGroupOwned {
    fn from(v: CalendarDateGroup) -> Self {
        Self {
            date: v.date.into(),
            count: v.count,
            infos: v.infos.into(),
        }
    }
}
impl ToFFI for CCalendarDateGroupOwned {
    type FFIType = CCalendarDateGroup;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCalendarDateGroup {
            date: self.date.to_ffi_type(),
            count: self.count,
            infos: self.infos.to_ffi_type(),
            num_infos: self.infos.len(),
        }
    }
}

/// Response containing calendar events grouped by date.
#[repr(C)]
pub struct CCalendarEventsResponse {
    /// Reference date string used for the query (e.g. "2025-03-15").
    pub date: *const c_char,
    /// Pointer to an array of date-grouped event lists.
    pub list: *const CCalendarDateGroup,
    /// Number of elements in the `list` array.
    pub num_list: usize,
}
pub(crate) struct CCalendarEventsResponseOwned {
    date: CString,
    list: CVec<CCalendarDateGroupOwned>,
}
impl From<CalendarEventsResponse> for CCalendarEventsResponseOwned {
    fn from(v: CalendarEventsResponse) -> Self {
        Self {
            date: v.date.into(),
            list: v.list.into(),
        }
    }
}
impl ToFFI for CCalendarEventsResponseOwned {
    type FFIType = CCalendarEventsResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCalendarEventsResponse {
            date: self.date.to_ffi_type(),
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
        }
    }
}
