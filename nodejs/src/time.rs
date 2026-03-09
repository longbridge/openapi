use napi::{Error, Result};
use serde_json::Value;

use crate::utils::ToJSON;

/// Naive date type
#[derive(Debug, Copy, Clone)]
#[napi_derive::napi]
pub struct NaiveDate(pub(crate) time::Date);

impl From<time::Date> for NaiveDate {
    #[inline]
    fn from(date: time::Date) -> Self {
        Self(date)
    }
}

#[napi_derive::napi]
impl NaiveDate {
    #[napi(constructor)]
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self> {
        let month =
            time::Month::try_from(month).map_err(|err| Error::from_reason(err.to_string()))?;
        Ok(Self(
            time::Date::from_calendar_date(year, month, day)
                .map_err(|err| Error::from_reason(err.to_string()))?,
        ))
    }

    #[napi(getter)]
    #[inline]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    #[napi(getter)]
    #[inline]
    pub fn month(&self) -> u8 {
        self.0.month() as u8
    }

    #[napi(getter)]
    #[inline]
    pub fn day(&self) -> u8 {
        self.0.day()
    }

    #[napi]
    #[inline]
    #[allow(clippy::wrong_self_convention, clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0
            .format(time::macros::format_description!("[year]-[month]-[day]"))
            .unwrap()
    }

    #[napi(js_name = "toJSON")]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_json(&self) -> Value {
        <Self as ToJSON>::to_json(self)
    }
}

impl ToJSON for NaiveDate {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(format!(
            "{}-{:02}-{:02}",
            self.0.year(),
            self.0.month() as u8,
            self.0.day()
        ))
    }
}

/// Time type
#[derive(Debug, Copy, Clone)]
#[napi_derive::napi]
pub struct Time(pub(crate) time::Time);

impl From<time::Time> for Time {
    #[inline]
    fn from(time: time::Time) -> Self {
        Self(time)
    }
}

#[napi_derive::napi]
impl Time {
    #[napi(constructor)]
    pub fn new(hour: u8, minute: u8, second: u8) -> Result<Self> {
        Ok(Self(
            time::Time::from_hms(hour, minute, second)
                .map_err(|err| Error::from_reason(err.to_string()))?,
        ))
    }

    #[napi(getter)]
    #[inline]
    pub fn hour(&self) -> u8 {
        self.0.hour()
    }

    #[napi(getter)]
    #[inline]
    pub fn minute(&self) -> u8 {
        self.0.minute()
    }

    #[napi(getter)]
    #[inline]
    #[allow(clippy::wrong_self_convention, clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0
            .format(time::macros::format_description!(
                "[hour]:[minute]:[second]"
            ))
            .unwrap()
    }

    #[napi(js_name = "toJSON")]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_json(&self) -> Value {
        <Self as ToJSON>::to_json(self)
    }
}

impl ToJSON for Time {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(format!(
            "{:02}:{:02}:{:02}",
            self.0.hour(),
            self.0.minute(),
            self.0.second()
        ))
    }
}

/// Naive datetime type
#[derive(Debug, Copy, Clone)]
#[napi_derive::napi]
pub struct NaiveDatetime(pub(crate) time::PrimitiveDateTime);

impl From<time::PrimitiveDateTime> for NaiveDatetime {
    #[inline]
    fn from(datetime: time::PrimitiveDateTime) -> Self {
        Self(datetime)
    }
}

#[napi_derive::napi]
impl NaiveDatetime {
    #[napi(constructor)]
    pub fn new(date: &NaiveDate, time: &Time) -> Self {
        Self(time::PrimitiveDateTime::new(date.0, time.0))
    }

    #[napi(getter)]
    #[inline]
    pub fn date(&self) -> NaiveDate {
        self.0.date().into()
    }

    #[napi(getter)]
    #[inline]
    pub fn time(&self) -> Time {
        self.0.time().into()
    }

    #[napi]
    #[inline]
    #[allow(clippy::wrong_self_convention, clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0
            .format(time::macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second]"
            ))
            .unwrap()
    }

    #[napi(js_name = "toJSON")]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_json(&self) -> Value {
        <Self as ToJSON>::to_json(self)
    }
}

impl ToJSON for NaiveDatetime {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(format!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.0.year(),
            self.0.month() as u8,
            self.0.day(),
            self.0.hour(),
            self.0.minute(),
            self.0.second()
        ))
    }
}
