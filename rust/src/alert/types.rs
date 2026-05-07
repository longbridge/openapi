//! Alert (price reminder) types.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Direction for a price alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertDirection {
    Up,
    Down,
}

/// Options for creating a new price alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAlertOptions {
    pub symbol: String,
    pub price: String,
    pub direction: AlertDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}
