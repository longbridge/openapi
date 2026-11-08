use serde::Serialize;

/// Options for get grid trading order detail request
#[derive(Debug, Serialize, Clone)]
pub struct GetGridOrderDetailOptions {
    order_id: String,
    /// History cursor for paging through the trigger history
    #[serde(skip_serializing_if = "Option::is_none")]
    history_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl GetGridOrderDetailOptions {
    /// Create a new `GetGridOrderDetailOptions`
    #[inline]
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            history_id: None,
            limit: None,
        }
    }

    /// Set the history cursor
    #[inline]
    #[must_use]
    pub fn history_id(self, history_id: impl Into<String>) -> Self {
        Self {
            history_id: Some(history_id.into()),
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
