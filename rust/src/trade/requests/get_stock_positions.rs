use serde::Serialize;

/// Options for get stock positions request
#[derive(Debug, Serialize, Default)]
pub struct GetStockPositionsOptions {
    #[serde(skip_serializing_if = "<[_]>::is_empty", rename = "symbol")]
    symbols: Vec<String>,
}

impl GetStockPositionsOptions {
    /// Create a new `GetStockPositionsOptions`
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the stock symbols
    #[inline]
    #[must_use]
    pub fn symbols<I, T>(self, symbols: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            symbols: symbols.into_iter().map(Into::into).collect(),
        }
    }
}
