use serde::Serialize;

/// Options for get fund positions request
#[derive(Debug, Serialize, Default)]
pub struct GetFundPositionsOptions {
    #[serde(skip_serializing_if = "<[_]>::is_empty", rename = "symbol")]
    symbols: Vec<String>,
}

impl GetFundPositionsOptions {
    /// Create a new `GetFundPositionsOptions`
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the fund symbols
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
