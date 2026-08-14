use serde::{Deserialize, Serialize};

/// Response for get statement data list request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatementListResponse {
    /// Available statement entries
    pub list: Vec<StatementItem>,
}

/// Statement data info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatementItem {
    /// Statement date as `YYYYMM` (e.g. `202401`)
    pub dt: i32,
    /// Opaque key identifying the statement file (pass to the download
    /// endpoint)
    pub file_key: String,
}

/// Response for get statement data download url request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatementResponse {
    /// Temporary download URL for the statement file
    pub url: String,
}
