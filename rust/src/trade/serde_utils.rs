#![allow(dead_code)]

use std::{fmt::Display, str::FromStr};

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

pub(crate) mod timestamp_int {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        OffsetDateTime::from_unix_timestamp(value).map_err(|_| Error::custom("invalid timestamp"))
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

pub(crate) mod number_str_opt {
    use num_traits::Num;

    use super::*;

    pub(crate) fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Num + Display,
    {
        match value {
            Some(value) => serializer.collect_str(value),
            None => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Num,
        T::FromStrRadixErr: Display,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            let n = <T as Num>::from_str_radix(&value, 10)
                .map_err(|err| D::Error::custom(err.to_string()))?;
            Ok(Some(n))
        } else {
            Ok(None)
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

pub(crate) mod cash_flow_symbol {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "." => Ok(None),
            _ => Ok(Some(value)),
        }
    }
}
