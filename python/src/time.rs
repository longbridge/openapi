use std::fmt::{self, Debug};

use pyo3::{
    prelude::*,
    types::{PyDate, PyDateAccess, PyDateTime, PyTime, PyTimeAccess},
};
use time::{
    format_description::well_known::Rfc3339, Date, Month, OffsetDateTime, PrimitiveDateTime, Time,
};

#[derive(Copy, Clone)]
pub(crate) struct PyOffsetDateTimeWrapper(pub(crate) OffsetDateTime);

impl Debug for PyOffsetDateTimeWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("\"{}\"", self.0.format(&Rfc3339).unwrap()))
    }
}

impl<'py> FromPyObject<'py> for PyOffsetDateTimeWrapper {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let date: Bound<PyDateTime> = ob.extract()?;
        let year = date.get_year();
        let month = date.get_month();
        let day = date.get_day();
        let hour = date.get_hour();
        let minute = date.get_minute();
        let second = date.get_second();
        Ok(Self(
            PrimitiveDateTime::new(
                Date::from_calendar_date(year, Month::try_from(month).expect("valid month"), day)
                    .expect("valid date"),
                Time::from_hms(hour, minute, second).expect("valid time"),
            )
            .assume_utc(),
        ))
    }
}

impl From<OffsetDateTime> for PyOffsetDateTimeWrapper {
    #[inline]
    fn from(value: OffsetDateTime) -> Self {
        PyOffsetDateTimeWrapper(value)
    }
}

impl IntoPy<PyObject> for PyOffsetDateTimeWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyDateTime::from_timestamp_bound(py, self.0.unix_timestamp() as f64, None)
            .unwrap()
            .into_py(py)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct PyDateWrapper(pub(crate) Date);

impl Debug for PyDateWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "\"{}-{}-{}\"",
            self.0.year(),
            self.0.month(),
            self.0.day()
        ))
    }
}

impl From<Date> for PyDateWrapper {
    #[inline]
    fn from(value: Date) -> Self {
        PyDateWrapper(value)
    }
}

impl From<PyDateWrapper> for Date {
    #[inline]
    fn from(value: PyDateWrapper) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyDateWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyDate::new_bound(py, self.0.year(), self.0.month() as u8, self.0.day())
            .unwrap()
            .into_py(py)
    }
}

impl<'py> FromPyObject<'py> for PyDateWrapper {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let date: Bound<PyDate> = ob.extract()?;
        let year = date.get_year();
        let month = date.get_month();
        let day = date.get_day();
        Ok(Self(
            Date::from_calendar_date(year, Month::try_from(month).expect("valid month"), day)
                .expect("valid date"),
        ))
    }
}

#[derive(Copy, Clone)]
pub(crate) struct PyTimeWrapper(pub(crate) Time);

impl Debug for PyTimeWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "\"{}:{}:{}\"",
            self.0.hour(),
            self.0.minute(),
            self.0.second()
        ))
    }
}

impl From<Time> for PyTimeWrapper {
    #[inline]
    fn from(value: Time) -> Self {
        PyTimeWrapper(value)
    }
}

impl IntoPy<PyObject> for PyTimeWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyTime::new_bound(py, self.0.hour(), self.0.minute(), self.0.second(), 0, None)
            .expect("valid time")
            .into_py(py)
    }
}
