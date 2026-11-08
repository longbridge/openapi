use serde::Serialize;

/// Options for get grid trading trigger history request.
///
/// Note: the required parameter is named `grid_order_id` (not `order_id`);
/// passing the wrong name makes the server return the account-wide trigger
/// history instead of the order-scoped one.
#[derive(Debug, Serialize, Clone)]
pub struct GetGridTriggerHistoryOptions {
    grid_order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl GetGridTriggerHistoryOptions {
    /// Create a new `GetGridTriggerHistoryOptions`
    #[inline]
    pub fn new(grid_order_id: impl Into<String>) -> Self {
        Self {
            grid_order_id: grid_order_id.into(),
            page: None,
            limit: None,
        }
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
}
