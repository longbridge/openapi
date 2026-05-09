use longbridge::dca::types as lb;

/// One DCA (dollar-cost averaging) investment plan
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaPlan {
    /// Plan ID
    pub plan_id: String,
    /// Plan status
    pub status: DCAStatus,
    /// Security symbol
    pub symbol: String,
    /// Member ID
    pub member_id: String,
    /// Account ID
    pub aaid: String,
    /// Account channel
    pub account_channel: String,
    /// Display account
    pub display_account: String,
    /// Market
    pub market: crate::types::Market,
    /// Investment amount per period
    pub per_invest_amount: String,
    /// Investment frequency
    pub invest_frequency: DCAFrequency,
    /// Day of week for weekly plans (e.g. `"Mon"`)
    pub invest_day_of_week: String,
    /// Day of month for monthly plans
    pub invest_day_of_month: String,
    /// Whether margin finance is allowed
    pub allow_margin_finance: bool,
    /// Reminder time
    pub alter_hours: String,
    /// Creation time
    pub created_at: String,
    /// Last updated time
    pub updated_at: String,
    /// Next investment date
    pub next_trd_date: String,
    /// Security name
    pub stock_name: String,
    /// Cumulative invested amount
    pub cum_amount: Option<String>,
    /// Number of completed investment periods
    pub issue_number: i64,
    /// Average cost
    pub average_cost: Option<String>,
    /// Cumulative profit/loss
    pub cum_profit: Option<String>,
}
impl From<lb::DcaPlan> for DcaPlan {
    fn from(v: lb::DcaPlan) -> Self {
        Self {
            plan_id: v.plan_id,
            status: v.status.into(),
            symbol: v.symbol,
            member_id: v.member_id,
            aaid: v.aaid,
            account_channel: v.account_channel,
            display_account: v.display_account,
            market: v.market.into(),
            per_invest_amount: v.per_invest_amount.to_string(),
            invest_frequency: v.invest_frequency.into(),
            invest_day_of_week: v.invest_day_of_week,
            invest_day_of_month: v.invest_day_of_month,
            allow_margin_finance: v.allow_margin_finance,
            alter_hours: v.alter_hours,
            created_at: v.created_at,
            updated_at: v.updated_at,
            next_trd_date: v.next_trd_date,
            stock_name: v.stock_name,
            cum_amount: v.cum_amount.map(|d| d.to_string()),
            issue_number: v.issue_number,
            average_cost: v.average_cost.map(|d| d.to_string()),
            cum_profit: v.cum_profit.map(|d| d.to_string()),
        }
    }
}

/// Response for DCA list and write operations
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaList {
    /// DCA plans
    pub plans: Vec<DcaPlan>,
}
impl From<lb::DcaList> for DcaList {
    fn from(v: lb::DcaList) -> Self {
        Self {
            plans: v.plans.into_iter().map(Into::into).collect(),
        }
    }
}

/// DCA statistics response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaStats {
    /// Number of active plans
    pub active_count: String,
    /// Number of finished plans
    pub finished_count: String,
    /// Number of suspended plans
    pub suspended_count: String,
    /// Nearest upcoming plans
    pub nearest_plans: Vec<DcaPlan>,
    /// Days until next investment
    pub rest_days: String,
    /// Total invested amount
    pub total_amount: Option<String>,
    /// Total profit/loss
    pub total_profit: Option<String>,
}
impl From<lb::DcaStats> for DcaStats {
    fn from(v: lb::DcaStats) -> Self {
        Self {
            active_count: v.active_count,
            finished_count: v.finished_count,
            suspended_count: v.suspended_count,
            nearest_plans: v.nearest_plans.into_iter().map(Into::into).collect(),
            rest_days: v.rest_days,
            total_amount: v.total_amount.map(|d| d.to_string()),
            total_profit: v.total_profit.map(|d| d.to_string()),
        }
    }
}

/// DCA support info for one security
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaSupportInfo {
    /// Security symbol
    pub symbol: String,
    /// Whether DCA is supported for this security
    pub support_regular_saving: bool,
}
impl From<lb::DcaSupportInfo> for DcaSupportInfo {
    fn from(v: lb::DcaSupportInfo) -> Self {
        Self {
            symbol: v.symbol,
            support_regular_saving: v.support_regular_saving,
        }
    }
}

/// Response for DCA support check
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaSupportList {
    /// Support info per security
    pub infos: Vec<DcaSupportInfo>,
}
impl From<lb::DcaSupportList> for DcaSupportList {
    fn from(v: lb::DcaSupportList) -> Self {
        Self {
            infos: v.infos.into_iter().map(Into::into).collect(),
        }
    }
}

/// One DCA execution record
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
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
    pub executed_qty: Option<String>,
    /// Executed price
    pub executed_price: Option<String>,
    /// Executed amount
    pub executed_amount: Option<String>,
    /// Rejection reason (if any)
    pub rejected_reason: String,
    /// Security symbol
    pub symbol: String,
}
impl From<lb::DcaHistoryRecord> for DcaHistoryRecord {
    fn from(v: lb::DcaHistoryRecord) -> Self {
        Self {
            created_at: v.created_at,
            order_id: v.order_id,
            status: v.status,
            action: v.action,
            order_type: v.order_type,
            executed_qty: v.executed_qty.map(|d| d.to_string()),
            executed_price: v.executed_price.map(|d| d.to_string()),
            executed_amount: v.executed_amount.map(|d| d.to_string()),
            rejected_reason: v.rejected_reason,
            symbol: v.symbol,
        }
    }
}

/// DCA execution history response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaHistoryResponse {
    /// Execution history records
    pub records: Vec<DcaHistoryRecord>,
    /// Whether more records exist
    pub has_more: bool,
}
impl From<lb::DcaHistoryResponse> for DcaHistoryResponse {
    fn from(v: lb::DcaHistoryResponse) -> Self {
        Self {
            records: v.records.into_iter().map(Into::into).collect(),
            has_more: v.has_more,
        }
    }
}

/// DCA investment frequency
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum DCAFrequency {
    /// Daily investment
    Daily,
    /// Weekly investment
    Weekly,
    /// Fortnightly (every two weeks) investment
    Fortnightly,
    /// Monthly investment
    Monthly,
}
impl From<DCAFrequency> for lb::DCAFrequency {
    fn from(v: DCAFrequency) -> Self {
        match v {
            DCAFrequency::Daily => lb::DCAFrequency::Daily,
            DCAFrequency::Weekly => lb::DCAFrequency::Weekly,
            DCAFrequency::Fortnightly => lb::DCAFrequency::Fortnightly,
            DCAFrequency::Monthly => lb::DCAFrequency::Monthly,
        }
    }
}
impl From<lb::DCAFrequency> for DCAFrequency {
    fn from(v: lb::DCAFrequency) -> Self {
        match v {
            lb::DCAFrequency::Daily => DCAFrequency::Daily,
            lb::DCAFrequency::Weekly => DCAFrequency::Weekly,
            lb::DCAFrequency::Fortnightly => DCAFrequency::Fortnightly,
            lb::DCAFrequency::Monthly => DCAFrequency::Monthly,
        }
    }
}

/// Result of a DCA date calculation
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaCalcDateResult {
    /// Next projected trade date (unix timestamp string)
    pub trade_date: String,
}
impl From<lb::DcaCalcDateResult> for DcaCalcDateResult {
    fn from(v: lb::DcaCalcDateResult) -> Self {
        Self {
            trade_date: v.trade_date,
        }
    }
}

/// DCA plan status
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum DCAStatus {
    /// Active plan
    Active,
    /// Suspended plan
    Suspended,
    /// Finished plan
    Finished,
}
impl From<DCAStatus> for lb::DCAStatus {
    fn from(v: DCAStatus) -> Self {
        match v {
            DCAStatus::Active => lb::DCAStatus::Active,
            DCAStatus::Suspended => lb::DCAStatus::Suspended,
            DCAStatus::Finished => lb::DCAStatus::Finished,
        }
    }
}
impl From<lb::DCAStatus> for DCAStatus {
    fn from(v: lb::DCAStatus) -> Self {
        match v {
            lb::DCAStatus::Active => DCAStatus::Active,
            lb::DCAStatus::Suspended => DCAStatus::Suspended,
            lb::DCAStatus::Finished => DCAStatus::Finished,
        }
    }
}

/// Result of creating or updating a DCA plan
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DcaCreateResult {
    /// The plan ID
    pub plan_id: String,
}

impl From<lb::DcaCreateResult> for DcaCreateResult {
    fn from(v: lb::DcaCreateResult) -> Self {
        Self { plan_id: v.plan_id }
    }
}
