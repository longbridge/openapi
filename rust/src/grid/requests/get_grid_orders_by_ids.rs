use serde::Serialize;

/// Options for query grid trading orders by IDs request
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetGridOrdersByIdsOptions {
    order_ids: Vec<String>,
}

impl GetGridOrdersByIdsOptions {
    /// Create a new `GetGridOrdersByIdsOptions`
    #[inline]
    pub fn new<I, T>(order_ids: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            order_ids: order_ids.into_iter().map(Into::into).collect(),
        }
    }
}
