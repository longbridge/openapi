use std::os::raw::c_char;

use longbridge::dca::{
    DcaHistoryRecord, DcaHistoryResponse, DcaList, DcaPlan, DcaStats, DcaSupportInfo,
    DcaSupportList,
};

use crate::{
    dca_context::enum_types::{CDCAFrequency, CDCAStatus},
    types::{CMarket, CString, CVec, ToFFI},
};

/// DCA (dollar-cost averaging) plan details.
#[repr(C)]
pub struct CDcaPlan {
    /// Unique plan identifier.
    pub plan_id: *const c_char,
    /// Current status of the plan.
    pub status: CDCAStatus,
    /// Stock symbol (e.g. "AAPL.US").
    pub symbol: *const c_char,
    /// Member ID that owns this plan.
    pub member_id: *const c_char,
    /// Account identifier (AAID).
    pub aaid: *const c_char,
    /// Account channel identifier.
    pub account_channel: *const c_char,
    /// Display-friendly account name.
    pub display_account: *const c_char,
    /// Market code.
    pub market: CMarket,
    /// Investment amount per period (decimal string).
    pub per_invest_amount: *const c_char,
    /// Investment frequency.
    pub invest_frequency: CDCAFrequency,
    /// Day of the week on which investment is executed (if weekly frequency).
    pub invest_day_of_week: *const c_char,
    /// Day of the month on which investment is executed (if monthly frequency).
    pub invest_day_of_month: *const c_char,
    /// Whether margin financing is allowed for this plan.
    pub allow_margin_finance: bool,
    /// After-hours trading setting.
    pub alter_hours: *const c_char,
    /// Plan creation timestamp (ISO 8601 string).
    pub created_at: *const c_char,
    /// Plan last-updated timestamp (ISO 8601 string).
    pub updated_at: *const c_char,
    /// Next scheduled trading date (ISO 8601 date string).
    pub next_trd_date: *const c_char,
    /// Stock display name.
    pub stock_name: *const c_char,
    /// Cumulative invested amount (decimal string).
    pub cum_amount: *const c_char,
    /// Total number of investment executions to date.
    pub issue_number: i64,
    /// Average cost per share across all executions (decimal string).
    pub average_cost: *const c_char,
    /// Cumulative profit/loss (decimal string).
    pub cum_profit: *const c_char,
}

pub(crate) struct CDcaPlanOwned {
    plan_id: CString,
    status: CDCAStatus,
    symbol: CString,
    member_id: CString,
    aaid: CString,
    account_channel: CString,
    display_account: CString,
    market: CMarket,
    per_invest_amount: CString,
    invest_frequency: CDCAFrequency,
    invest_day_of_week: CString,
    invest_day_of_month: CString,
    allow_margin_finance: bool,
    alter_hours: CString,
    created_at: CString,
    updated_at: CString,
    next_trd_date: CString,
    stock_name: CString,
    cum_amount: CString,
    issue_number: i64,
    average_cost: CString,
    cum_profit: CString,
}

impl From<DcaPlan> for CDcaPlanOwned {
    fn from(v: DcaPlan) -> Self {
        Self {
            plan_id: v.plan_id.into(),
            status: v.status.into(),
            symbol: v.symbol.into(),
            member_id: v.member_id.into(),
            aaid: v.aaid.into(),
            account_channel: v.account_channel.into(),
            display_account: v.display_account.into(),
            market: v.market.into(),
            per_invest_amount: v.per_invest_amount.to_string().into(),
            invest_frequency: v.invest_frequency.into(),
            invest_day_of_week: v.invest_day_of_week.into(),
            invest_day_of_month: v.invest_day_of_month.into(),
            allow_margin_finance: v.allow_margin_finance,
            alter_hours: v.alter_hours.into(),
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
            next_trd_date: v.next_trd_date.into(),
            stock_name: v.stock_name.into(),
            cum_amount: v
                .cum_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            issue_number: v.issue_number,
            average_cost: v
                .average_cost
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            cum_profit: v
                .cum_profit
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CDcaPlanOwned {
    type FFIType = CDcaPlan;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaPlan {
            plan_id: self.plan_id.to_ffi_type(),
            status: self.status,
            symbol: self.symbol.to_ffi_type(),
            member_id: self.member_id.to_ffi_type(),
            aaid: self.aaid.to_ffi_type(),
            account_channel: self.account_channel.to_ffi_type(),
            display_account: self.display_account.to_ffi_type(),
            market: self.market,
            per_invest_amount: self.per_invest_amount.to_ffi_type(),
            invest_frequency: self.invest_frequency,
            invest_day_of_week: self.invest_day_of_week.to_ffi_type(),
            invest_day_of_month: self.invest_day_of_month.to_ffi_type(),
            allow_margin_finance: self.allow_margin_finance,
            alter_hours: self.alter_hours.to_ffi_type(),
            created_at: self.created_at.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
            next_trd_date: self.next_trd_date.to_ffi_type(),
            stock_name: self.stock_name.to_ffi_type(),
            cum_amount: self.cum_amount.to_ffi_type(),
            issue_number: self.issue_number,
            average_cost: self.average_cost.to_ffi_type(),
            cum_profit: self.cum_profit.to_ffi_type(),
        }
    }
}

/// List of DCA plans.
#[repr(C)]
pub struct CDcaList {
    /// Pointer to the array of DCA plans.
    pub plans: *const CDcaPlan,
    /// Number of plans in the array.
    pub num_plans: usize,
}

pub(crate) struct CDcaListOwned {
    plans: CVec<CDcaPlanOwned>,
}

impl From<DcaList> for CDcaListOwned {
    fn from(v: DcaList) -> Self {
        Self {
            plans: v.plans.into(),
        }
    }
}

impl ToFFI for CDcaListOwned {
    type FFIType = CDcaList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaList {
            plans: self.plans.to_ffi_type(),
            num_plans: self.plans.len(),
        }
    }
}

/// Aggregate statistics across all DCA plans for a user.
#[repr(C)]
pub struct CDcaStats {
    /// Number of currently active plans (decimal string).
    pub active_count: *const c_char,
    /// Number of finished plans (decimal string).
    pub finished_count: *const c_char,
    /// Number of suspended plans (decimal string).
    pub suspended_count: *const c_char,
    /// Pointer to the array of nearest upcoming plans.
    pub nearest_plans: *const CDcaPlan,
    /// Number of plans in the nearest_plans array.
    pub num_nearest_plans: usize,
    /// Days remaining until the next scheduled investment (decimal string).
    pub rest_days: *const c_char,
    /// Total invested amount across all plans (decimal string).
    pub total_amount: *const c_char,
    /// Total profit/loss across all plans (decimal string).
    pub total_profit: *const c_char,
}

pub(crate) struct CDcaStatsOwned {
    active_count: CString,
    finished_count: CString,
    suspended_count: CString,
    nearest_plans: CVec<CDcaPlanOwned>,
    rest_days: CString,
    total_amount: CString,
    total_profit: CString,
}

impl From<DcaStats> for CDcaStatsOwned {
    fn from(v: DcaStats) -> Self {
        Self {
            active_count: v.active_count.into(),
            finished_count: v.finished_count.into(),
            suspended_count: v.suspended_count.into(),
            nearest_plans: v.nearest_plans.into(),
            rest_days: v.rest_days.into(),
            total_amount: v
                .total_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            total_profit: v
                .total_profit
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CDcaStatsOwned {
    type FFIType = CDcaStats;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaStats {
            active_count: self.active_count.to_ffi_type(),
            finished_count: self.finished_count.to_ffi_type(),
            suspended_count: self.suspended_count.to_ffi_type(),
            nearest_plans: self.nearest_plans.to_ffi_type(),
            num_nearest_plans: self.nearest_plans.len(),
            rest_days: self.rest_days.to_ffi_type(),
            total_amount: self.total_amount.to_ffi_type(),
            total_profit: self.total_profit.to_ffi_type(),
        }
    }
}

/// DCA support information for a single security.
#[repr(C)]
pub struct CDcaSupportInfo {
    /// Stock symbol (e.g. "AAPL.US").
    pub symbol: *const c_char,
    /// Whether regular (recurring) saving/investment is supported for this
    /// symbol.
    pub support_regular_saving: bool,
}

pub(crate) struct CDcaSupportInfoOwned {
    symbol: CString,
    support_regular_saving: bool,
}

impl From<DcaSupportInfo> for CDcaSupportInfoOwned {
    fn from(v: DcaSupportInfo) -> Self {
        Self {
            symbol: v.symbol.into(),
            support_regular_saving: v.support_regular_saving,
        }
    }
}

impl ToFFI for CDcaSupportInfoOwned {
    type FFIType = CDcaSupportInfo;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaSupportInfo {
            symbol: self.symbol.to_ffi_type(),
            support_regular_saving: self.support_regular_saving,
        }
    }
}

/// List of DCA support information entries.
#[repr(C)]
pub struct CDcaSupportList {
    /// Pointer to the array of support info entries.
    pub infos: *const CDcaSupportInfo,
    /// Number of entries in the array.
    pub num_infos: usize,
}

pub(crate) struct CDcaSupportListOwned {
    infos: CVec<CDcaSupportInfoOwned>,
}

impl From<DcaSupportList> for CDcaSupportListOwned {
    fn from(v: DcaSupportList) -> Self {
        Self {
            infos: v.infos.into(),
        }
    }
}

impl ToFFI for CDcaSupportListOwned {
    type FFIType = CDcaSupportList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaSupportList {
            infos: self.infos.to_ffi_type(),
            num_infos: self.infos.len(),
        }
    }
}

/// Result returned by DCA create and update operations.
#[repr(C)]
pub struct CDcaCreateResult {
    /// The plan ID of the created or updated DCA plan.
    pub plan_id: *const c_char,
}

pub(crate) struct CDcaCreateResultOwned {
    plan_id: CString,
}

impl From<longbridge::dca::DcaCreateResult> for CDcaCreateResultOwned {
    fn from(v: longbridge::dca::DcaCreateResult) -> Self {
        Self {
            plan_id: v.plan_id.into(),
        }
    }
}

impl ToFFI for CDcaCreateResultOwned {
    type FFIType = CDcaCreateResult;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaCreateResult {
            plan_id: self.plan_id.to_ffi_type(),
        }
    }
}

/// One DCA execution history record.
#[repr(C)]
pub struct CDcaHistoryRecord {
    /// Execution timestamp (ISO 8601 string).
    pub created_at: *const c_char,
    /// Associated order ID.
    pub order_id: *const c_char,
    /// Execution status string.
    pub status: *const c_char,
    /// Action type string.
    pub action: *const c_char,
    /// Order type string.
    pub order_type: *const c_char,
    /// Executed quantity (decimal string, may be empty).
    pub executed_qty: *const c_char,
    /// Executed price (decimal string, may be empty).
    pub executed_price: *const c_char,
    /// Executed amount (decimal string, may be empty).
    pub executed_amount: *const c_char,
    /// Rejection reason (empty string if not rejected).
    pub rejected_reason: *const c_char,
    /// Security symbol.
    pub symbol: *const c_char,
}

pub(crate) struct CDcaHistoryRecordOwned {
    created_at: CString,
    order_id: CString,
    status: CString,
    action: CString,
    order_type: CString,
    executed_qty: CString,
    executed_price: CString,
    executed_amount: CString,
    rejected_reason: CString,
    symbol: CString,
}

impl From<DcaHistoryRecord> for CDcaHistoryRecordOwned {
    fn from(v: DcaHistoryRecord) -> Self {
        Self {
            created_at: v.created_at.into(),
            order_id: v.order_id.into(),
            status: v.status.into(),
            action: v.action.into(),
            order_type: v.order_type.into(),
            executed_qty: v
                .executed_qty
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            executed_price: v
                .executed_price
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            executed_amount: v
                .executed_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            rejected_reason: v.rejected_reason.into(),
            symbol: v.symbol.into(),
        }
    }
}

impl ToFFI for CDcaHistoryRecordOwned {
    type FFIType = CDcaHistoryRecord;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaHistoryRecord {
            created_at: self.created_at.to_ffi_type(),
            order_id: self.order_id.to_ffi_type(),
            status: self.status.to_ffi_type(),
            action: self.action.to_ffi_type(),
            order_type: self.order_type.to_ffi_type(),
            executed_qty: self.executed_qty.to_ffi_type(),
            executed_price: self.executed_price.to_ffi_type(),
            executed_amount: self.executed_amount.to_ffi_type(),
            rejected_reason: self.rejected_reason.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
        }
    }
}

/// Paginated DCA execution history response.
#[repr(C)]
pub struct CDcaHistoryResponse {
    /// Pointer to the array of history records.
    pub records: *const CDcaHistoryRecord,
    /// Number of records in the array.
    pub num_records: usize,
    /// Whether more records exist.
    pub has_more: bool,
}

pub(crate) struct CDcaHistoryResponseOwned {
    records: CVec<CDcaHistoryRecordOwned>,
    has_more: bool,
}

impl From<DcaHistoryResponse> for CDcaHistoryResponseOwned {
    fn from(v: DcaHistoryResponse) -> Self {
        Self {
            records: v.records.into(),
            has_more: v.has_more,
        }
    }
}

impl ToFFI for CDcaHistoryResponseOwned {
    type FFIType = CDcaHistoryResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaHistoryResponse {
            records: self.records.to_ffi_type(),
            num_records: self.records.len(),
            has_more: self.has_more,
        }
    }
}

/// Result returned by DCA calc_date operation.
#[repr(C)]
pub struct CDcaCalcDateResult {
    /// Next projected trade date (unix timestamp string).
    pub trade_date: *const c_char,
}

pub(crate) struct CDcaCalcDateResultOwned {
    trade_date: CString,
}

impl From<longbridge::dca::DcaCalcDateResult> for CDcaCalcDateResultOwned {
    fn from(v: longbridge::dca::DcaCalcDateResult) -> Self {
        Self {
            trade_date: v.trade_date.into(),
        }
    }
}

impl ToFFI for CDcaCalcDateResultOwned {
    type FFIType = CDcaCalcDateResult;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDcaCalcDateResult {
            trade_date: self.trade_date.to_ffi_type(),
        }
    }
}
