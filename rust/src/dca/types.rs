#![allow(missing_docs)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{types::Market, utils::counter::deserialize_counter_id_as_symbol};

/// Response for [`crate::DCAContext::list`] and write operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaList {
    /// DCA plans
    pub plans: Vec<DcaPlan>,
}

/// One DCA (dollar-cost averaging) investment plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaPlan {
    /// Plan ID
    pub plan_id: String,
    /// Status
    #[serde(default)]
    pub status: DCAStatus,
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Member ID
    #[serde(default)]
    pub member_id: String,
    /// Account ID
    #[serde(default)]
    pub aaid: String,
    /// Account channel
    #[serde(default)]
    pub account_channel: String,
    /// Display account
    #[serde(default)]
    pub display_account: String,
    /// Market
    #[serde(default)]
    pub market: Market,
    /// Investment amount per period
    #[serde(default, with = "crate::serde_utils::decimal_empty_is_0")]
    pub per_invest_amount: Decimal,
    /// Investment frequency
    #[serde(default)]
    pub invest_frequency: DCAFrequency,
    /// Day of week for weekly plans (e.g. `"Mon"`)
    #[serde(default)]
    pub invest_day_of_week: String,
    /// Day of month for monthly plans
    #[serde(default)]
    pub invest_day_of_month: String,
    /// Whether margin finance is allowed
    #[serde(default)]
    pub allow_margin_finance: bool,
    /// Reminder notification hours before execution (API may return integer or
    /// string)
    #[serde(
        default,
        deserialize_with = "crate::serde_utils::deserialize_string_or_int_as_string"
    )]
    pub alter_hours: String,
    /// Creation time
    #[serde(default)]
    pub created_at: String,
    /// Last updated time
    #[serde(default)]
    pub updated_at: String,
    /// Next investment date
    #[serde(default)]
    pub next_trd_date: String,
    /// Security name
    #[serde(default)]
    pub stock_name: String,
    /// Cumulative invested amount
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub cum_amount: Option<Decimal>,
    /// Number of completed investment periods
    #[serde(default)]
    pub issue_number: i64,
    /// Average cost
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub average_cost: Option<Decimal>,
    /// Cumulative profit/loss
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub cum_profit: Option<Decimal>,
}

/// Response for [`crate::DCAContext::stats`]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DcaStats {
    /// Number of active plans
    #[serde(default)]
    pub active_count: String,
    /// Number of finished plans
    #[serde(default)]
    pub finished_count: String,
    /// Number of suspended plans
    #[serde(default)]
    pub suspended_count: String,
    /// Nearest upcoming plans
    #[serde(default)]
    pub nearest_plans: Vec<DcaPlan>,
    /// Days until next investment
    #[serde(default)]
    pub rest_days: String,
    /// Total invested amount
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub total_amount: Option<Decimal>,
    /// Total profit/loss
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub total_profit: Option<Decimal>,
}

/// Response for [`crate::DCAContext::check_support`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaSupportList {
    /// Support info per security
    pub infos: Vec<DcaSupportInfo>,
}

/// DCA support info for one security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaSupportInfo {
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Whether DCA is supported for this security
    pub support_regular_saving: bool,
}

/// Response for [`crate::DCAContext::history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaHistoryResponse {
    /// Execution history records
    pub records: Vec<DcaHistoryRecord>,
    /// Whether more records exist
    #[serde(default)]
    pub has_more: bool,
}

/// One DCA execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaHistoryRecord {
    /// Execution time
    pub created_at: String,
    /// Associated order ID
    pub order_id: String,
    /// Status
    pub status: String,
    /// Action type
    pub action: String,
    /// Order type
    pub order_type: String,
    /// Executed quantity
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub executed_qty: Option<Decimal>,
    /// Executed price
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub executed_price: Option<Decimal>,
    /// Executed amount
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub executed_amount: Option<Decimal>,
    /// Rejection reason (if any)
    pub rejected_reason: String,
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
}

/// DCA investment frequency
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum DCAFrequency {
    /// Daily investment
    #[serde(rename = "Daily")]
    Daily,
    /// Weekly investment
    #[serde(rename = "Weekly")]
    Weekly,
    /// Fortnightly (every two weeks) investment
    #[serde(rename = "Fortnightly")]
    Fortnightly,
    /// Monthly investment
    #[default]
    #[serde(rename = "Monthly")]
    Monthly,
}

/// Response for [`crate::DCAContext::create`] and [`crate::DCAContext::update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaCreateResult {
    /// The plan ID of the created or updated plan
    pub plan_id: String,
}

/// Response for [`crate::DCAContext::calc_date`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcaCalcDateResult {
    /// Next projected trade date (unix timestamp string)
    pub trade_date: String,
}

/// DCA plan status
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum DCAStatus {
    /// Active plan
    #[default]
    #[serde(rename = "Active")]
    Active,
    /// Suspended plan
    #[serde(rename = "Suspended")]
    Suspended,
    /// Finished plan
    #[serde(rename = "Finished")]
    Finished,
}
