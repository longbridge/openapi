//! DCA (Dollar-Cost Averaging) types.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Options for creating a DCA plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDcaPlanOptions {
    pub symbol: String,
    pub amount: String,
    pub frequency: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Options for updating a DCA plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDcaPlanOptions {
    pub plan_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Options for checking DCA support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDcaSupportOptions {
    pub symbols: Vec<String>,
}

/// Optional query parameters for listing DCA history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DcaHistoryOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
