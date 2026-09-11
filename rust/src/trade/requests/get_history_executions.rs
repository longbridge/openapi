use serde::Serialize;
use time::OffsetDateTime;

use crate::serde_utils;

/// Options for get history executions request
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
    // Pagination cursor (1-based), set internally by `history_executions` while
    // walking pages. Not a public builder — callers always get every page.
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
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

    /// Internal 1-based pagination cursor used by `history_executions` to walk
    /// all pages. Not a public builder — the SDK sets this while paginating.
    #[inline]
    pub(crate) fn with_page(self, page: u32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }
}
