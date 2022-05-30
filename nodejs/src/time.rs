use std::time::Duration;

use napi::{Error, Result};

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
}

/// Naive date type
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
    pub fn monute(&self) -> u8 {
        self.0.minute() as u8
    }

    #[napi(getter)]
    #[inline]
    pub fn second(&self) -> u8 {
        self.0.second()
    }
}

#[napi_derive::napi]
async fn sleep(milliseconds: i64) {
    tokio::time::sleep(Duration::from_millis(milliseconds as u64)).await
}
