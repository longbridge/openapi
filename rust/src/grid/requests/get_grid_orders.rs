use serde::Serialize;

use crate::Market;

/// Options for get grid trading orders (list) request
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetGridOrdersOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<Market>,
    /// Comma-joined status filter (e.g. `Performing,Suspended`)
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
}

impl GetGridOrdersOptions {
    /// Create a new `GetGridOrdersOptions`
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn limit(self, limit: i32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    /// Set the market filter
    #[inline]
    #[must_use]
    pub fn market(self, market: Market) -> Self {
        Self {
            market: Some(market),
            ..self
        }
    }

    /// Set the status filter (comma-joined, e.g. `Performing,Suspended`)
    #[inline]
    #[must_use]
    pub fn status(self, status: impl Into<String>) -> Self {
        Self {
            status: Some(status.into()),
            ..self
        }
    }

    /// Set the security symbol filter (e.g. `700.HK`)
    #[inline]
    #[must_use]
    pub fn symbol(self, symbol: impl Into<String>) -> Self {
        Self {
            symbol: Some(symbol.into()),
            ..self
        }
    }

    /// Set the sort field
    #[inline]
    #[must_use]
    pub fn sort_by(self, sort_by: impl Into<String>) -> Self {
        Self {
            sort_by: Some(sort_by.into()),
            ..self
        }
    }

    /// Set the sort order
    #[inline]
    #[must_use]
    pub fn sort_order(self, sort_order: impl Into<String>) -> Self {
        Self {
            sort_order: Some(sort_order.into()),
            ..self
        }
    }
}
