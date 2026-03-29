#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Response for get statement data list request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatementListResponse {
    pub list: Vec<StatementItem>,
}

/// Statement data info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatementItem {
    pub dt: i32,
    pub file_key: String,
}

/// Response for get statement data download url request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatementResponse {
    pub url: String,
}
