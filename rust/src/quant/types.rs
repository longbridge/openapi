//! Quant script types.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Options for running a quant script.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunQuantScriptOptions {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    pub script: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
}
