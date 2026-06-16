use serde::Serialize;

/// Options for get order detail request
#[derive(Debug, Serialize, Clone)]
pub struct GetOrderDetailOptions {
    order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_attached: Option<bool>,
}

impl GetOrderDetailOptions {
    /// Create new options with order ID
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            is_attached: None,
        }
    }
    /// Query by attached order
    pub fn is_attached(self) -> Self {
        Self {
            is_attached: Some(true),
            ..self
        }
    }
}

impl From<String> for GetOrderDetailOptions {
    fn from(order_id: String) -> Self {
        Self::new(order_id)
    }
}

impl<'a> From<&'a str> for GetOrderDetailOptions {
    fn from(order_id: &'a str) -> Self {
        Self::new(order_id)
    }
}
