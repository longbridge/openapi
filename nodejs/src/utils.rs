use chrono::{DateTime, TimeZone, Utc};
use napi::threadsafe_function::ThreadsafeFunction;
use serde_json::Value;
use time::OffsetDateTime;

pub(crate) type JsCallback<T> = ThreadsafeFunction<T, ()>;

pub(crate) fn to_datetime(time: OffsetDateTime) -> DateTime<Utc> {
    DateTime::from_timestamp(time.unix_timestamp(), 0).unwrap()
}

pub(crate) fn from_datetime(time: DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(time.timestamp()).expect("invalid timestamp")
}

pub(crate) trait ToJSON {
    fn to_json(&self) -> Value;
}

impl ToJSON for String {
    fn to_json(&self) -> Value {
        Value::String(self.clone())
    }
}

impl ToJSON for i32 {
    fn to_json(&self) -> Value {
        Value::Number((*self).into())
    }
}

impl ToJSON for i64 {
    fn to_json(&self) -> Value {
        Value::Number((*self).into())
    }
}

impl ToJSON for bool {
    fn to_json(&self) -> Value {
        Value::Bool(*self)
    }
}

impl<Tz> ToJSON for DateTime<Tz>
where
    Tz: TimeZone,
{
    fn to_json(&self) -> Value {
        Value::String(self.to_rfc3339())
    }
}

impl<T> ToJSON for Option<T>
where
    T: ToJSON,
{
    fn to_json(&self) -> Value {
        match self {
            Some(value) => value.to_json(),
            None => Value::Null,
        }
    }
}

impl<T> ToJSON for Vec<T>
where
    T: ToJSON,
{
    fn to_json(&self) -> Value {
        Value::Array(self.iter().map(ToJSON::to_json).collect())
    }
}
