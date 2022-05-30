use serde::Serialize;

/// Options for get today executions request
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetTodayExecutionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
}

impl GetTodayExecutionsOptions {
    /// Create a new `GetTodayExecutionsOptions`
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
}
