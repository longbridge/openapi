use longbridge::dca::types as lb;
use pyo3::prelude::*;

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaPlan {
    pub plan_id: String,
    pub status: DCAStatus,
    pub symbol: String,
    pub member_id: String,
    pub aaid: String,
    pub account_channel: String,
    pub display_account: String,
    pub market: crate::types::Market,
    pub per_invest_amount: String,
    pub invest_frequency: DCAFrequency,
    pub invest_day_of_week: String,
    pub invest_day_of_month: String,
    pub allow_margin_finance: bool,
    pub alter_hours: String,
    pub created_at: String,
    pub updated_at: String,
    pub next_trd_date: String,
    pub stock_name: String,
    pub cum_amount: Option<String>,
    pub issue_number: i64,
    pub average_cost: Option<String>,
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaList {
    pub plans: Vec<DcaPlan>,
}
impl From<lb::DcaList> for DcaList {
    fn from(v: lb::DcaList) -> Self {
        Self {
            plans: v.plans.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaStats {
    pub active_count: String,
    pub finished_count: String,
    pub suspended_count: String,
    pub nearest_plans: Vec<DcaPlan>,
    pub rest_days: String,
    pub total_amount: Option<String>,
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaSupportInfo {
    pub symbol: String,
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaSupportList {
    pub infos: Vec<DcaSupportInfo>,
}
impl From<lb::DcaSupportList> for DcaSupportList {
    fn from(v: lb::DcaSupportList) -> Self {
        Self {
            infos: v.infos.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaHistoryRecord {
    pub created_at: String,
    pub order_id: String,
    pub status: String,
    pub action: String,
    pub order_type: String,
    pub executed_qty: Option<String>,
    pub executed_price: Option<String>,
    pub executed_amount: Option<String>,
    pub rejected_reason: String,
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaHistoryResponse {
    pub records: Vec<DcaHistoryRecord>,
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

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum DCAFrequency {
    Daily = 0,
    Weekly = 1,
    Fortnightly = 2,
    Monthly = 3,
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaCalcDateResult {
    pub trade_date: String,
}
impl From<lb::DcaCalcDateResult> for DcaCalcDateResult {
    fn from(v: lb::DcaCalcDateResult) -> Self {
        Self {
            trade_date: v.trade_date,
        }
    }
}

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum DCAStatus {
    Active = 0,
    Suspended = 1,
    Finished = 2,
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DcaCreateResult {
    /// The created or updated plan ID
    pub plan_id: String,
}

impl From<lb::DcaCreateResult> for DcaCreateResult {
    fn from(v: lb::DcaCreateResult) -> Self {
        Self { plan_id: v.plan_id }
    }
}
