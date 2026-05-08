#![allow(missing_docs)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::utils::counter::deserialize_counter_id_as_symbol;

/// Response for [`crate::SharelistContext::list`] and
/// [`crate::SharelistContext::popular`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharelistList {
    /// User's own and followed sharelists
    pub sharelists: Vec<SharelistInfo>,
    /// Subscribed sharelists (may be absent in popular response)
    #[serde(default)]
    pub subscribed_sharelists: Vec<SharelistInfo>,
    /// Pagination cursor for subscribed list
    #[serde(default)]
    pub tail_mark: String,
}

/// Response for [`crate::SharelistContext::detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharelistDetail {
    /// Sharelist info
    pub sharelist: SharelistInfo,
    /// Subscription scopes
    pub scopes: SharelistScopes,
}

/// Sharelist information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharelistInfo {
    /// Sharelist ID (may be string or integer in API)
    #[serde(deserialize_with = "crate::serde_utils::deserialize_id_as_i64")]
    pub id: i64,
    /// Name
    pub name: String,
    /// Description
    #[serde(default)]
    pub description: String,
    /// Cover image URL
    #[serde(default)]
    pub cover: String,
    /// Number of subscribers (may be null)
    #[serde(default)]
    pub subscribers_count: i64,
    /// Creation time
    #[serde(deserialize_with = "crate::serde_utils::deserialize_timestamp")]
    pub created_at: OffsetDateTime,
    /// Last stock edit time
    #[serde(deserialize_with = "crate::serde_utils::deserialize_timestamp")]
    pub edited_at: OffsetDateTime,
    /// YTD change percentage
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub this_year_chg: Option<Decimal>,
    /// Creator info
    pub creator: serde_json::Value,
    /// Constituent stocks
    pub stocks: Vec<SharelistStock>,
    /// Whether the current user is subscribed
    pub subscribed: bool,
    /// Day change percentage
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg: Option<Decimal>,
    /// Sharelist type: 0=regular, 3=official, 4=industry
    pub sharelist_type: i32,
    /// Industry code (for industry sharelists)
    #[serde(default)]
    pub industry_code: String,
}

/// Stock in a sharelist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharelistStock {
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Security name
    #[serde(default)]
    pub name: String,
    /// Market, e.g. `"HK"`
    #[serde(default)]
    pub market: String,
    /// Ticker code
    #[serde(default)]
    pub code: String,
    /// Brief description
    #[serde(default)]
    pub intro: String,
    /// Unread change log category
    #[serde(default)]
    pub unread_change_log_category: String,
    /// Day change percentage
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub change: Option<Decimal>,
    /// Latest price
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub last_done: Option<Decimal>,
    /// Trade status code
    #[serde(default)]
    pub trade_status: Option<i32>,
    /// Whether delayed quote
    #[serde(default)]
    pub latency: Option<bool>,
}

/// Sharelist subscription scopes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharelistScopes {
    /// Whether the current user is subscribed
    pub subscription: bool,
    /// Whether the current user is the creator
    #[serde(rename = "self")]
    pub is_self: bool,
}
