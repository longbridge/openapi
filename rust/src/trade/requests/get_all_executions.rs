use serde::Serialize;
use time::OffsetDateTime;

use crate::serde_utils;

/// Options for get all executions request
#[derive(Debug, Serialize, Default, Clone)]
pub struct GetAllExecutionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u64>,
}

impl GetAllExecutionsOptions {
    /// Create a new `GetAllExecutionsOptions`
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

    /// Set the order id
    #[inline]
    #[must_use]
    pub fn order_id(self, order_id: impl Into<String>) -> Self {
        Self {
            order_id: Some(order_id.into()),
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

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: u64) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }
}
