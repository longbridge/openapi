use serde::Serialize;
use time::OffsetDateTime;

use crate::serde_utils;

/// Options for get histroy executions request
#[derive(Debug, Serialize, Default, Clone)]
pub struct GetHistoryExecutionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::timestamp_opt"
    )]
    start_at: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::timestamp_opt"
    )]
    end_at: Option<OffsetDateTime>,
}

impl GetHistoryExecutionsOptions {
    /// Create a new `GetHistoryExecutionsOptions`
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the security symbol
    #[inline]
    #[must_use]
    pub fn symbol(self, symbol: impl Into<String>) -> Self {
        Self {
            symbol: Some(symbol.into()),
            ..self
        }
    }

    /// Set the start time
    #[inline]
    #[must_use]
    pub fn start_at(self, start_at: OffsetDateTime) -> Self {
        Self {
            start_at: Some(start_at),
            ..self
        }
    }

    /// Set the end time
    #[inline]
    #[must_use]
    pub fn end_at(self, end_at: OffsetDateTime) -> Self {
        Self {
            end_at: Some(end_at),
            ..self
        }
    }
}
