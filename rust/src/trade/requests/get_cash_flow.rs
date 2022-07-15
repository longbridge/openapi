use serde::Serialize;
use time::OffsetDateTime;

use crate::{serde_utils, trade::BalanceType};

/// Options for cash flow request
#[derive(Debug, Serialize, Clone)]
pub struct GetCashFlowOptions {
    #[serde(rename = "start_time", with = "serde_utils::timestamp")]
    start_at: OffsetDateTime,
    #[serde(rename = "end_time", with = "serde_utils::timestamp")]
    end_at: OffsetDateTime,
    business_type: Option<BalanceType>,
    symbol: Option<String>,
    page: Option<usize>,
    size: Option<usize>,
}

impl GetCashFlowOptions {
    /// Create a new `GetCashFlowOptions`
    #[inline]
    pub fn new(start_at: OffsetDateTime, end_at: OffsetDateTime) -> Self {
        Self {
            start_at,
            end_at,
            business_type: None,
            symbol: None,
            page: None,
            size: None,
        }
    }

    /// Set the business type
    #[inline]
    #[must_use]
    pub fn business_type(self, business_type: BalanceType) -> Self {
        Self {
            business_type: Some(business_type),
            ..self
        }
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

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: usize) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: usize) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}
