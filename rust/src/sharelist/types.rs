//! Sharelist types.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Options for creating a sharelist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSharelistOptions {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Options for adding/removing/sorting items in a sharelist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharelistItemsOptions {
    pub symbols: Vec<String>,
}
