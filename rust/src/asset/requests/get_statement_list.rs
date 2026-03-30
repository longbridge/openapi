use serde::Serialize;

/// Statement type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StatementType {
    /// Daily statement
    Daily = 1,
    /// Monthly statement
    Monthly = 2,
}

impl From<StatementType> for i32 {
    #[inline]
    fn from(value: StatementType) -> Self {
        value as i32
    }
}

/// Options for get statement data list request
#[derive(Debug, Serialize, Clone)]
pub struct GetStatementListOptions {
    statement_type: i32,
    start_date: i32,
    limit: i32,
}

impl GetStatementListOptions {
    /// Create a new `GetStatementDataListOptions`
    #[inline]
    pub fn new(statement_type: StatementType) -> Self {
        Self {
            statement_type: statement_type.into(),
            start_date: 1,
            limit: 20,
        }
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            start_date: page,
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn page_size(self, page_size: i32) -> Self {
        Self {
            limit: page_size,
            ..self
        }
    }
}
