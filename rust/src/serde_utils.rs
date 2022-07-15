#![allow(dead_code)]

use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serializer};
use time::{Date, OffsetDateTime};

pub(crate) mod timestamp {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value
            .parse::<i64>()
            .map_err(|_| Error::custom("invalid timestamp"))?;
        OffsetDateTime::from_unix_timestamp(value).map_err(|_| Error::custom("invalid timestamp"))
    }

    pub(crate) fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&datetime.unix_timestamp().to_string())
    }
}

pub(crate) mod timestamp_opt {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value
            .parse::<i64>()
            .map_err(|_| Error::custom("invalid timestamp"))?;
        if value != 0 {
            let datetime = OffsetDateTime::from_unix_timestamp(value)
                .map_err(|_| Error::custom("invalid timestamp"))?;
            Ok(Some(datetime))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn serialize<S>(
        datetime: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(datetime) => serializer.serialize_str(&datetime.unix_timestamp().to_string()),
            None => serializer.serialize_none(),
        }
    }
}

pub(crate) mod date_opt {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            let datetime = Date::parse(
                &value,
                time::macros::format_description!("[year]-[month]-[day]"),
            )
            .map_err(|_| Error::custom("invalid timestamp"))?;
            Ok(Some(datetime))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn serialize<S>(datetime: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(date) => serializer.serialize_str(
                &date
                    .format(time::macros::format_description!("[year]-[month]-[day]"))
                    .unwrap(),
            ),
            None => serializer.serialize_none(),
        }
    }
}

pub(crate) mod risk_level {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            Ok(value
                .parse::<i32>()
                .map_err(|err| D::Error::custom(err.to_string()))?)
        } else {
            Ok(0)
        }
    }
}

pub(crate) mod decimal_empty_is_0 {
    use rust_decimal::Decimal;

    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            let n = value
                .parse::<Decimal>()
                .map_err(|err| Error::custom(err.to_string()))?;
            Ok(n)
        } else {
            Ok(Decimal::ZERO)
        }
    }
}

pub(crate) mod decimal_opt_empty_is_none {
    use rust_decimal::Decimal;

    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            let n = value
                .parse::<Decimal>()
                .map_err(|err| Error::custom(err.to_string()))?;
            Ok(Some(n))
        } else {
            Ok(None)
        }
    }
}

pub(crate) mod decimal_opt_0_is_none {
    use rust_decimal::Decimal;

    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let n = value
            .parse::<Decimal>()
            .map_err(|err| Error::custom(err.to_string()))?;
        if !n.is_zero() {
            Ok(Some(n))
        } else {
            Ok(None)
        }
    }
}

pub(crate) mod trigger_status {
    use super::*;
    use crate::trade::TriggerStatus;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<TriggerStatus>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NOT_USED" => Ok(None),
            _ => Ok(Some(
                TriggerStatus::from_str(value.as_str()).unwrap_or_default(),
            )),
        }
    }
}

pub(crate) mod outside_rth {
    use super::*;
    use crate::trade::OutsideRTH;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<OutsideRTH>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "UnknownOutsideRth" => Ok(None),
            _ => Ok(Some(
                OutsideRTH::from_str(value.as_str()).unwrap_or_default(),
            )),
        }
    }
}

pub(crate) mod symbol_opt {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <Option<String>>::deserialize(deserializer)? {
            Some(value) if value.is_empty() => Ok(None),
            Some(value) => Ok(Some(value)),
            _ => Ok(None),
        }
    }
}

pub(crate) mod int64_str {
    use super::*;

    pub(crate) fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(value.parse::<i64>().unwrap_or_default())
    }
}

pub(crate) mod int64_str_empty_is_none {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            Ok(Some(
                value
                    .parse::<i64>()
                    .map_err(|err| D::Error::custom(err.to_string()))?,
            ))
        } else {
            Ok(None)
        }
    }
}
