/// Statement type enum
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum StatementType {
    /// Daily statement
    Daily = 1,
    /// Monthly statement
    Monthly = 2,
}

impl From<StatementType> for longbridge::asset::StatementType {
    fn from(value: StatementType) -> Self {
        match value {
            StatementType::Daily => longbridge::asset::StatementType::Daily,
            StatementType::Monthly => longbridge::asset::StatementType::Monthly,
        }
    }
}

/// Options for listing statements
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetStatementListRequest {
    /// Statement type: Daily (1) or Monthly (2)
    pub statement_type: Option<StatementType>,
    /// Start date for pagination
    pub start_date: Option<i32>,
    /// Number of results (default 20)
    pub limit: Option<i32>,
}

/// Options for getting a statement download URL
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct GetStatementDownloadUrlRequest {
    /// File key obtained from the list statements endpoint
    pub file_key: String,
}
