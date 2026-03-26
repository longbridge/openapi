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
pub struct GetStatementDataListOptions {
    aaid: i64,
    statement_type: i32,
    page: i32,
    page_size: i32,
}

impl GetStatementDataListOptions {
    /// Create a new `GetStatementDataListOptions`
    #[inline]
    pub fn new(aaid: i64, statement_type: StatementType) -> Self {
        Self {
            aaid,
            statement_type: statement_type.into(),
            page: 1,
            page_size: 20,
        }
    }

    /// Set the page number
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self { page, ..self }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn page_size(self, page_size: i32) -> Self {
        Self { page_size, ..self }
    }
}
