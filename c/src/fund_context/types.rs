use std::os::raw::c_char;

use longbridge::fund::{
    FundAnalysis, FundAnalysisDetail, FundAnnualReturn, FundAssetAllocation,
    FundAssetAllocationItem, FundBrief, FundDatedValue, FundDetail, FundDividend, FundDividends,
    FundFilters, FundHolding, FundHoldings, FundNamedContrast, FundNavValue, FundOrder,
    FundOrderDetail, FundOrderInfo, FundOrderKeyword, FundOrderStage, FundOrderSubmitResponse,
    FundOrderValidation, FundPerformance, FundPerformanceComparison, FundPerformancePoint,
    FundPosition, FundPositionDetail, FundPositionDetailValues, FundPositionNav,
    FundPositionPerformance, FundPositionProfits, FundPositions, FundQuarterlyReturn,
    FundStockHolding, FundTransaction, FundTrend, FundTrendContrast, FundUnitValue, HotFund,
};
use serde_json::Value;

use crate::types::{CString, CVec, ToFFI};

// Helpers for `serde_json::Value` ("any") fields ------------------------------

/// Serialize a single JSON value to a JSON-encoded C string.
fn value_to_cstring(value: &Value) -> CString {
    serde_json::to_string(value).unwrap_or_default().into()
}

/// Serialize a list of JSON values to a `CVec` of JSON-encoded C strings.
fn values_to_cvec(values: Vec<Value>) -> CVec<CString> {
    values
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap_or_default())
        .collect::<Vec<String>>()
        .into()
}

// ── FundNavValue ────────────────────────────────────────────────────────────

/// A fund net-asset-value data point (latest / historical).
#[repr(C)]
pub struct CFundNavValue {
    /// Net value change
    pub change: *const c_char,
    /// Net value change percent
    pub change_percent: *const c_char,
    /// Formatted change percent
    pub change_percent_format: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Fund name
    pub counter_name: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Formatted date
    pub date_format: *const c_char,
    /// ISIN
    pub isin: *const c_char,
    /// Last update time (unix seconds)
    pub last_update_time: i64,
    /// Net value
    pub value: *const c_char,
    /// Formatted net value
    pub value_format: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundNavValueOwned {
    change: CString,
    change_percent: CString,
    change_percent_format: CString,
    counter_id: CString,
    counter_name: CString,
    currency: CString,
    date_format: CString,
    isin: CString,
    last_update_time: i64,
    value: CString,
    value_format: CString,
}

impl From<FundNavValue> for CFundNavValueOwned {
    fn from(v: FundNavValue) -> Self {
        CFundNavValueOwned {
            change: v.change.into(),
            change_percent: v.change_percent.into(),
            change_percent_format: v.change_percent_format.into(),
            counter_id: v.counter_id.into(),
            counter_name: v.counter_name.into(),
            currency: v.currency.into(),
            date_format: v.date_format.into(),
            isin: v.isin.into(),
            last_update_time: v.last_update_time,
            value: v.value.into(),
            value_format: v.value_format.into(),
        }
    }
}

impl ToFFI for CFundNavValueOwned {
    type FFIType = CFundNavValue;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundNavValue {
            change: self.change.to_ffi_type(),
            change_percent: self.change_percent.to_ffi_type(),
            change_percent_format: self.change_percent_format.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            date_format: self.date_format.to_ffi_type(),
            isin: self.isin.to_ffi_type(),
            last_update_time: self.last_update_time,
            value: self.value.to_ffi_type(),
            value_format: self.value_format.to_ffi_type(),
        }
    }
}

// ── FundPerformancePoint ────────────────────────────────────────────────────

/// A recent performance point used by the hot-fund list.
#[repr(C)]
pub struct CFundPerformancePoint {
    /// Date
    pub date: *const c_char,
    /// Last done value
    pub last_done: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundPerformancePointOwned {
    date: CString,
    last_done: CString,
}

impl From<FundPerformancePoint> for CFundPerformancePointOwned {
    fn from(p: FundPerformancePoint) -> Self {
        CFundPerformancePointOwned {
            date: p.date.into(),
            last_done: p.last_done.into(),
        }
    }
}

impl ToFFI for CFundPerformancePointOwned {
    type FFIType = CFundPerformancePoint;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPerformancePoint {
            date: self.date.to_ffi_type(),
            last_done: self.last_done.to_ffi_type(),
        }
    }
}

// ── HotFund ─────────────────────────────────────────────────────────────────

/// A hot-selling fund entry.
#[repr(C)]
pub struct CHotFund {
    /// Asset class
    pub asset_class: i32,
    /// Asset class name
    pub asset_class_name: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Earning rate
    pub earning_rate: *const c_char,
    /// Recent performance points
    pub fund_performances: *const CFundPerformancePoint,
    /// Number of recent performance points
    pub num_fund_performances: usize,
    /// Fund name
    pub name: *const c_char,
    /// Minimum purchase amount
    pub purchase_amount: *const c_char,
    /// Recommendation text
    pub recommendation_text: *const c_char,
    /// Risk level
    pub risk_level: i32,
    /// Risk level name
    pub risk_level_name: *const c_char,
    /// Time interval of the earning rate
    pub time_interval: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CHotFundOwned {
    asset_class: i32,
    asset_class_name: CString,
    counter_id: CString,
    currency: CString,
    earning_rate: CString,
    fund_performances: CVec<CFundPerformancePointOwned>,
    name: CString,
    purchase_amount: CString,
    recommendation_text: CString,
    risk_level: i32,
    risk_level_name: CString,
    time_interval: CString,
}

impl From<HotFund> for CHotFundOwned {
    fn from(f: HotFund) -> Self {
        CHotFundOwned {
            asset_class: f.asset_class,
            asset_class_name: f.asset_class_name.into(),
            counter_id: f.counter_id.into(),
            currency: f.currency.into(),
            earning_rate: f.earning_rate.into(),
            fund_performances: f.fund_performances.into(),
            name: f.name.into(),
            purchase_amount: f.purchase_amount.into(),
            recommendation_text: f.recommendation_text.into(),
            risk_level: f.risk_level,
            risk_level_name: f.risk_level_name.into(),
            time_interval: f.time_interval.into(),
        }
    }
}

impl ToFFI for CHotFundOwned {
    type FFIType = CHotFund;

    fn to_ffi_type(&self) -> Self::FFIType {
        CHotFund {
            asset_class: self.asset_class,
            asset_class_name: self.asset_class_name.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            earning_rate: self.earning_rate.to_ffi_type(),
            fund_performances: self.fund_performances.to_ffi_type(),
            num_fund_performances: self.fund_performances.len(),
            name: self.name.to_ffi_type(),
            purchase_amount: self.purchase_amount.to_ffi_type(),
            recommendation_text: self.recommendation_text.to_ffi_type(),
            risk_level: self.risk_level,
            risk_level_name: self.risk_level_name.to_ffi_type(),
            time_interval: self.time_interval.to_ffi_type(),
        }
    }
}

// ── FundBrief ───────────────────────────────────────────────────────────────

/// A fund entry in the fund list.
#[repr(C)]
pub struct CFundBrief {
    /// Asset class
    pub asset_class: i32,
    /// Asset class name
    pub asset_class_name: *const c_char,
    /// Fund code
    pub code: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Description
    pub description: *const c_char,
    /// Earning rate
    pub earning_rate: *const c_char,
    /// Whether the user is holding this fund
    pub holding: bool,
    /// ISIN
    pub isin: *const c_char,
    /// Fund name
    pub name: *const c_char,
    /// Product
    pub product: *const c_char,
    /// Minimum purchase amount
    pub purchase_amount: *const c_char,
    /// Recommendation text
    pub recommendation_text: *const c_char,
    /// Risk level
    pub risk_level: i32,
    /// Risk level name
    pub risk_level_name: *const c_char,
    /// Time interval of the earning rate
    pub time_interval: *const c_char,
    /// Unit value
    pub unit_value: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundBriefOwned {
    asset_class: i32,
    asset_class_name: CString,
    code: CString,
    counter_id: CString,
    currency: CString,
    description: CString,
    earning_rate: CString,
    holding: bool,
    isin: CString,
    name: CString,
    product: CString,
    purchase_amount: CString,
    recommendation_text: CString,
    risk_level: i32,
    risk_level_name: CString,
    time_interval: CString,
    unit_value: CString,
}

impl From<FundBrief> for CFundBriefOwned {
    fn from(f: FundBrief) -> Self {
        CFundBriefOwned {
            asset_class: f.asset_class,
            asset_class_name: f.asset_class_name.into(),
            code: f.code.into(),
            counter_id: f.counter_id.into(),
            currency: f.currency.into(),
            description: f.description.into(),
            earning_rate: f.earning_rate.into(),
            holding: f.holding,
            isin: f.isin.into(),
            name: f.name.into(),
            product: f.product.into(),
            purchase_amount: f.purchase_amount.into(),
            recommendation_text: f.recommendation_text.into(),
            risk_level: f.risk_level,
            risk_level_name: f.risk_level_name.into(),
            time_interval: f.time_interval.into(),
            unit_value: f.unit_value.into(),
        }
    }
}

impl ToFFI for CFundBriefOwned {
    type FFIType = CFundBrief;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundBrief {
            asset_class: self.asset_class,
            asset_class_name: self.asset_class_name.to_ffi_type(),
            code: self.code.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            description: self.description.to_ffi_type(),
            earning_rate: self.earning_rate.to_ffi_type(),
            holding: self.holding,
            isin: self.isin.to_ffi_type(),
            name: self.name.to_ffi_type(),
            product: self.product.to_ffi_type(),
            purchase_amount: self.purchase_amount.to_ffi_type(),
            recommendation_text: self.recommendation_text.to_ffi_type(),
            risk_level: self.risk_level,
            risk_level_name: self.risk_level_name.to_ffi_type(),
            time_interval: self.time_interval.to_ffi_type(),
            unit_value: self.unit_value.to_ffi_type(),
        }
    }
}

// ── FundFilters ─────────────────────────────────────────────────────────────

/// Fund list filter options. Each list holds JSON-encoded option objects.
#[repr(C)]
pub struct CFundFilters {
    /// Asset class options (JSON strings)
    pub asset_class: *const *const c_char,
    /// Number of asset class options
    pub num_asset_class: usize,
    /// Company options (JSON strings)
    pub company: *const *const c_char,
    /// Number of company options
    pub num_company: usize,
    /// Currency options (JSON strings)
    pub currency: *const *const c_char,
    /// Number of currency options
    pub num_currency: usize,
    /// Industry category options (JSON strings)
    pub industry_category_name: *const *const c_char,
    /// Number of industry category options
    pub num_industry_category_name: usize,
    /// Risk level options (JSON strings)
    pub risk_level: *const *const c_char,
    /// Number of risk level options
    pub num_risk_level: usize,
}

#[derive(Debug)]
pub(crate) struct CFundFiltersOwned {
    asset_class: CVec<CString>,
    company: CVec<CString>,
    currency: CVec<CString>,
    industry_category_name: CVec<CString>,
    risk_level: CVec<CString>,
}

impl From<FundFilters> for CFundFiltersOwned {
    fn from(f: FundFilters) -> Self {
        CFundFiltersOwned {
            asset_class: values_to_cvec(f.asset_class),
            company: values_to_cvec(f.company),
            currency: values_to_cvec(f.currency),
            industry_category_name: values_to_cvec(f.industry_category_name),
            risk_level: values_to_cvec(f.risk_level),
        }
    }
}

impl ToFFI for CFundFiltersOwned {
    type FFIType = CFundFilters;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundFilters {
            asset_class: self.asset_class.to_ffi_type(),
            num_asset_class: self.asset_class.len(),
            company: self.company.to_ffi_type(),
            num_company: self.company.len(),
            currency: self.currency.to_ffi_type(),
            num_currency: self.currency.len(),
            industry_category_name: self.industry_category_name.to_ffi_type(),
            num_industry_category_name: self.industry_category_name.len(),
            risk_level: self.risk_level.to_ffi_type(),
            num_risk_level: self.risk_level.len(),
        }
    }
}

// ── FundAssetAllocation ─────────────────────────────────────────────────────

/// A single holding entry inside a fund's asset allocation.
#[repr(C)]
pub struct CFundAssetAllocationItem {
    /// Security code
    pub code: *const c_char,
    /// Security counter id
    pub counter_id: *const c_char,
    /// Name
    pub name: *const c_char,
    /// Position ratio
    pub position_ratio: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundAssetAllocationItemOwned {
    code: CString,
    counter_id: CString,
    name: CString,
    position_ratio: CString,
}

impl From<FundAssetAllocationItem> for CFundAssetAllocationItemOwned {
    fn from(i: FundAssetAllocationItem) -> Self {
        CFundAssetAllocationItemOwned {
            code: i.code.into(),
            counter_id: i.counter_id.into(),
            name: i.name.into(),
            position_ratio: i.position_ratio.into(),
        }
    }
}

impl ToFFI for CFundAssetAllocationItemOwned {
    type FFIType = CFundAssetAllocationItem;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundAssetAllocationItem {
            code: self.code.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            name: self.name.to_ffi_type(),
            position_ratio: self.position_ratio.to_ffi_type(),
        }
    }
}

/// A fund's asset allocation.
#[repr(C)]
pub struct CFundAssetAllocation {
    /// Asset type
    pub asset_type: i32,
    /// Allocation entries
    pub lists: *const CFundAssetAllocationItem,
    /// Number of allocation entries
    pub num_lists: usize,
    /// Report date
    pub report_date: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundAssetAllocationOwned {
    asset_type: i32,
    lists: CVec<CFundAssetAllocationItemOwned>,
    report_date: CString,
}

impl From<FundAssetAllocation> for CFundAssetAllocationOwned {
    fn from(a: FundAssetAllocation) -> Self {
        CFundAssetAllocationOwned {
            asset_type: a.asset_type,
            lists: a.lists.into(),
            report_date: a.report_date.into(),
        }
    }
}

impl ToFFI for CFundAssetAllocationOwned {
    type FFIType = CFundAssetAllocation;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundAssetAllocation {
            asset_type: self.asset_type,
            lists: self.lists.to_ffi_type(),
            num_lists: self.lists.len(),
            report_date: self.report_date.to_ffi_type(),
        }
    }
}

// ── FundDetail ──────────────────────────────────────────────────────────────

/// Fund detail.
#[repr(C)]
pub struct CFundDetail {
    /// Additional purchase amount
    pub additional_purchase_amount: *const c_char,
    /// Affirm day
    pub affirm_day: i32,
    /// Amount affirm day
    pub amount_affirm_day: *const c_char,
    /// Asset allocation
    pub asset_allocation: CFundAssetAllocation,
    /// Asset class
    pub asset_class: i32,
    /// Asset class name
    pub asset_class_name: *const c_char,
    /// Bill purchase rate
    pub bill_purchase_rate: *const c_char,
    /// Channel
    pub channel: *const c_char,
    /// Close period
    pub close_period: *const c_char,
    /// Fund code
    pub code: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Cut off time
    pub cut_off_time: *const c_char,
    /// Whether it is a derivative
    pub derivatives: bool,
    /// Done day
    pub done_day: i32,
    /// Excess return fee
    pub excess_return_fee: *const c_char,
    /// GST rate
    pub gst_rate: *const c_char,
    /// Introduction
    pub introduce: *const c_char,
    /// Whether it is a cash-plus fund
    pub is_cash_plus: bool,
    /// Whether it is a complex product
    pub is_complex: bool,
    /// Whether it is a new cash-plus fund
    pub is_new_cash_plus: bool,
    /// Whether it is a Yinghebao fund
    pub is_yinghebao: bool,
    /// ISIN
    pub isin: *const c_char,
    /// Management rate
    pub manage_rate: *const c_char,
    /// Manager
    pub manager: *const c_char,
    /// Minimum holding cash
    pub min_hold_cash: *const c_char,
    /// Minimum holding share
    pub min_hold_share: *const c_char,
    /// Minimum sell share
    pub min_sell_share: *const c_char,
    /// Month raise day
    pub month_raise_day: *const c_char,
    /// Fund name
    pub name: *const c_char,
    /// Net value deadline
    pub nav_deadline: *const c_char,
    /// Whether it is no-load
    pub no_load: bool,
    /// Open date
    pub open_date: *const c_char,
    /// Open period
    pub open_period: *const c_char,
    /// Product
    pub product: *const c_char,
    /// Product information locals
    pub product_information_locals: *const c_char,
    /// Profile
    pub profile: *const c_char,
    /// Whether purchasable
    pub purchasable: i32,
    /// Purchase affirm day
    pub purchase_affirm_day: *const c_char,
    /// Minimum purchase amount
    pub purchase_amount: *const c_char,
    /// Purchase rate
    pub purchase_rate: *const c_char,
    /// Rating
    pub rating: i32,
    /// Whether redeemable
    pub redeemable: i32,
    /// Redemption advance day
    pub redemption_advance_day: *const c_char,
    /// Redemption amount
    pub redemption_amount: *const c_char,
    /// Redemption close period text
    pub redemption_close_period_shows: *const c_char,
    /// Redemption done day
    pub redemption_done_day: *const c_char,
    /// Redemption open day text
    pub redemption_open_day_shows: *const c_char,
    /// Risk level
    pub risk_level: i32,
    /// Risk level name
    pub risk_level_name: *const c_char,
    /// Verify status
    pub verify_status: i32,
    /// Whether it is a virtual currency fund
    pub virtual_currency: bool,
    /// Year to date yield
    pub year_to_date_yield: *const c_char,
    /// Year to date yield type
    pub ytd_yield_type: i32,
}

#[derive(Debug)]
pub(crate) struct CFundDetailOwned {
    additional_purchase_amount: CString,
    affirm_day: i32,
    amount_affirm_day: CString,
    asset_allocation: CFundAssetAllocationOwned,
    asset_class: i32,
    asset_class_name: CString,
    bill_purchase_rate: CString,
    channel: CString,
    close_period: CString,
    code: CString,
    currency: CString,
    cut_off_time: CString,
    derivatives: bool,
    done_day: i32,
    excess_return_fee: CString,
    gst_rate: CString,
    introduce: CString,
    is_cash_plus: bool,
    is_complex: bool,
    is_new_cash_plus: bool,
    is_yinghebao: bool,
    isin: CString,
    manage_rate: CString,
    manager: CString,
    min_hold_cash: CString,
    min_hold_share: CString,
    min_sell_share: CString,
    month_raise_day: CString,
    name: CString,
    nav_deadline: CString,
    no_load: bool,
    open_date: CString,
    open_period: CString,
    product: CString,
    product_information_locals: CString,
    profile: CString,
    purchasable: i32,
    purchase_affirm_day: CString,
    purchase_amount: CString,
    purchase_rate: CString,
    rating: i32,
    redeemable: i32,
    redemption_advance_day: CString,
    redemption_amount: CString,
    redemption_close_period_shows: CString,
    redemption_done_day: CString,
    redemption_open_day_shows: CString,
    risk_level: i32,
    risk_level_name: CString,
    verify_status: i32,
    virtual_currency: bool,
    year_to_date_yield: CString,
    ytd_yield_type: i32,
}

impl From<FundDetail> for CFundDetailOwned {
    fn from(d: FundDetail) -> Self {
        CFundDetailOwned {
            additional_purchase_amount: d.additional_purchase_amount.into(),
            affirm_day: d.affirm_day,
            amount_affirm_day: d.amount_affirm_day.into(),
            asset_allocation: d.asset_allocation.into(),
            asset_class: d.asset_class,
            asset_class_name: d.asset_class_name.into(),
            bill_purchase_rate: d.bill_purchase_rate.into(),
            channel: d.channel.into(),
            close_period: d.close_period.into(),
            code: d.code.into(),
            currency: d.currency.into(),
            cut_off_time: d.cut_off_time.into(),
            derivatives: d.derivatives,
            done_day: d.done_day,
            excess_return_fee: d.excess_return_fee.into(),
            gst_rate: d.gst_rate.into(),
            introduce: d.introduce.into(),
            is_cash_plus: d.is_cash_plus,
            is_complex: d.is_complex,
            is_new_cash_plus: d.is_new_cash_plus,
            is_yinghebao: d.is_yinghebao,
            isin: d.isin.into(),
            manage_rate: d.manage_rate.into(),
            manager: d.manager.into(),
            min_hold_cash: d.min_hold_cash.into(),
            min_hold_share: d.min_hold_share.into(),
            min_sell_share: d.min_sell_share.into(),
            month_raise_day: d.month_raise_day.into(),
            name: d.name.into(),
            nav_deadline: d.nav_deadline.into(),
            no_load: d.no_load,
            open_date: d.open_date.into(),
            open_period: d.open_period.into(),
            product: d.product.into(),
            product_information_locals: d.product_information_locals.into(),
            profile: d.profile.into(),
            purchasable: d.purchasable,
            purchase_affirm_day: d.purchase_affirm_day.into(),
            purchase_amount: d.purchase_amount.into(),
            purchase_rate: d.purchase_rate.into(),
            rating: d.rating,
            redeemable: d.redeemable,
            redemption_advance_day: d.redemption_advance_day.into(),
            redemption_amount: d.redemption_amount.into(),
            redemption_close_period_shows: d.redemption_close_period_shows.into(),
            redemption_done_day: d.redemption_done_day.into(),
            redemption_open_day_shows: d.redemption_open_day_shows.into(),
            risk_level: d.risk_level,
            risk_level_name: d.risk_level_name.into(),
            verify_status: d.verify_status,
            virtual_currency: d.virtual_currency,
            year_to_date_yield: d.year_to_date_yield.into(),
            ytd_yield_type: d.ytd_yield_type,
        }
    }
}

impl ToFFI for CFundDetailOwned {
    type FFIType = CFundDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundDetail {
            additional_purchase_amount: self.additional_purchase_amount.to_ffi_type(),
            affirm_day: self.affirm_day,
            amount_affirm_day: self.amount_affirm_day.to_ffi_type(),
            asset_allocation: self.asset_allocation.to_ffi_type(),
            asset_class: self.asset_class,
            asset_class_name: self.asset_class_name.to_ffi_type(),
            bill_purchase_rate: self.bill_purchase_rate.to_ffi_type(),
            channel: self.channel.to_ffi_type(),
            close_period: self.close_period.to_ffi_type(),
            code: self.code.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            cut_off_time: self.cut_off_time.to_ffi_type(),
            derivatives: self.derivatives,
            done_day: self.done_day,
            excess_return_fee: self.excess_return_fee.to_ffi_type(),
            gst_rate: self.gst_rate.to_ffi_type(),
            introduce: self.introduce.to_ffi_type(),
            is_cash_plus: self.is_cash_plus,
            is_complex: self.is_complex,
            is_new_cash_plus: self.is_new_cash_plus,
            is_yinghebao: self.is_yinghebao,
            isin: self.isin.to_ffi_type(),
            manage_rate: self.manage_rate.to_ffi_type(),
            manager: self.manager.to_ffi_type(),
            min_hold_cash: self.min_hold_cash.to_ffi_type(),
            min_hold_share: self.min_hold_share.to_ffi_type(),
            min_sell_share: self.min_sell_share.to_ffi_type(),
            month_raise_day: self.month_raise_day.to_ffi_type(),
            name: self.name.to_ffi_type(),
            nav_deadline: self.nav_deadline.to_ffi_type(),
            no_load: self.no_load,
            open_date: self.open_date.to_ffi_type(),
            open_period: self.open_period.to_ffi_type(),
            product: self.product.to_ffi_type(),
            product_information_locals: self.product_information_locals.to_ffi_type(),
            profile: self.profile.to_ffi_type(),
            purchasable: self.purchasable,
            purchase_affirm_day: self.purchase_affirm_day.to_ffi_type(),
            purchase_amount: self.purchase_amount.to_ffi_type(),
            purchase_rate: self.purchase_rate.to_ffi_type(),
            rating: self.rating,
            redeemable: self.redeemable,
            redemption_advance_day: self.redemption_advance_day.to_ffi_type(),
            redemption_amount: self.redemption_amount.to_ffi_type(),
            redemption_close_period_shows: self.redemption_close_period_shows.to_ffi_type(),
            redemption_done_day: self.redemption_done_day.to_ffi_type(),
            redemption_open_day_shows: self.redemption_open_day_shows.to_ffi_type(),
            risk_level: self.risk_level,
            risk_level_name: self.risk_level_name.to_ffi_type(),
            verify_status: self.verify_status,
            virtual_currency: self.virtual_currency,
            year_to_date_yield: self.year_to_date_yield.to_ffi_type(),
            ytd_yield_type: self.ytd_yield_type,
        }
    }
}

// ── FundAnalysis / FundAnalysisDetail ───────────────────────────────────────

/// Fund analysis (level 1).
#[repr(C)]
pub struct CFundAnalysis {
    /// Actual period
    pub actual_period: i32,
    /// Cost level (JSON string)
    pub cost_level: *const c_char,
    /// Return ability (JSON string)
    pub return_ability: *const c_char,
    /// Risk ability (JSON string)
    pub risk_ability: *const c_char,
    /// Updated at
    pub updated_at: *const c_char,
    /// Value for money (JSON string)
    pub value_for_money: *const c_char,
    /// Whether visible
    pub visible: bool,
}

#[derive(Debug)]
pub(crate) struct CFundAnalysisOwned {
    actual_period: i32,
    cost_level: CString,
    return_ability: CString,
    risk_ability: CString,
    updated_at: CString,
    value_for_money: CString,
    visible: bool,
}

impl From<FundAnalysis> for CFundAnalysisOwned {
    fn from(a: FundAnalysis) -> Self {
        CFundAnalysisOwned {
            actual_period: a.actual_period,
            cost_level: value_to_cstring(&a.cost_level),
            return_ability: value_to_cstring(&a.return_ability),
            risk_ability: value_to_cstring(&a.risk_ability),
            updated_at: a.updated_at.into(),
            value_for_money: value_to_cstring(&a.value_for_money),
            visible: a.visible,
        }
    }
}

impl ToFFI for CFundAnalysisOwned {
    type FFIType = CFundAnalysis;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundAnalysis {
            actual_period: self.actual_period,
            cost_level: self.cost_level.to_ffi_type(),
            return_ability: self.return_ability.to_ffi_type(),
            risk_ability: self.risk_ability.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
            value_for_money: self.value_for_money.to_ffi_type(),
            visible: self.visible,
        }
    }
}

/// Fund analysis detail (level 2).
#[repr(C)]
pub struct CFundAnalysisDetail {
    /// Actual period
    pub actual_period: i32,
    /// Available periods
    pub available_periods: *const i32,
    /// Number of available periods
    pub num_available_periods: usize,
    /// Cost level (JSON string)
    pub cost_level: *const c_char,
    /// Return ability (JSON string)
    pub return_ability: *const c_char,
    /// Risk ability (JSON string)
    pub risk_ability: *const c_char,
    /// Updated at
    pub updated_at: *const c_char,
    /// Value for money (JSON string)
    pub value_for_money: *const c_char,
    /// Whether visible
    pub visible: bool,
}

#[derive(Debug)]
pub(crate) struct CFundAnalysisDetailOwned {
    actual_period: i32,
    available_periods: CVec<i32>,
    cost_level: CString,
    return_ability: CString,
    risk_ability: CString,
    updated_at: CString,
    value_for_money: CString,
    visible: bool,
}

impl From<FundAnalysisDetail> for CFundAnalysisDetailOwned {
    fn from(a: FundAnalysisDetail) -> Self {
        CFundAnalysisDetailOwned {
            actual_period: a.actual_period,
            available_periods: a.available_periods.into(),
            cost_level: value_to_cstring(&a.cost_level),
            return_ability: value_to_cstring(&a.return_ability),
            risk_ability: value_to_cstring(&a.risk_ability),
            updated_at: a.updated_at.into(),
            value_for_money: value_to_cstring(&a.value_for_money),
            visible: a.visible,
        }
    }
}

impl ToFFI for CFundAnalysisDetailOwned {
    type FFIType = CFundAnalysisDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundAnalysisDetail {
            actual_period: self.actual_period,
            available_periods: self.available_periods.to_ffi_type(),
            num_available_periods: self.available_periods.len(),
            cost_level: self.cost_level.to_ffi_type(),
            return_ability: self.return_ability.to_ffi_type(),
            risk_ability: self.risk_ability.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
            value_for_money: self.value_for_money.to_ffi_type(),
            visible: self.visible,
        }
    }
}

// ── FundTrend ───────────────────────────────────────────────────────────────

/// A benchmark contrast series in a fund trend chart.
#[repr(C)]
pub struct CFundTrendContrast {
    /// Benchmark name
    pub benchmark_name: *const c_char,
    /// Performance points (JSON strings)
    pub performances: *const *const c_char,
    /// Number of performance points
    pub num_performances: usize,
}

#[derive(Debug)]
pub(crate) struct CFundTrendContrastOwned {
    benchmark_name: CString,
    performances: CVec<CString>,
}

impl From<FundTrendContrast> for CFundTrendContrastOwned {
    fn from(c: FundTrendContrast) -> Self {
        CFundTrendContrastOwned {
            benchmark_name: c.benchmark_name.into(),
            performances: values_to_cvec(c.performances),
        }
    }
}

impl ToFFI for CFundTrendContrastOwned {
    type FFIType = CFundTrendContrast;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundTrendContrast {
            benchmark_name: self.benchmark_name.to_ffi_type(),
            performances: self.performances.to_ffi_type(),
            num_performances: self.performances.len(),
        }
    }
}

/// Fund trend chart.
#[repr(C)]
pub struct CFundTrend {
    /// Actual period
    pub actual_period: i32,
    /// Available periods
    pub available_periods: *const i32,
    /// Number of available periods
    pub num_available_periods: usize,
    /// Category average performances (JSON strings)
    pub category_average_performances: *const *const c_char,
    /// Number of category average performances
    pub num_category_average_performances: usize,
    /// Benchmark contrast performances
    pub contrast_performances: CFundTrendContrast,
    /// Fund performances (JSON strings)
    pub fund_performances: *const *const c_char,
    /// Number of fund performances
    pub num_fund_performances: usize,
}

#[derive(Debug)]
pub(crate) struct CFundTrendOwned {
    actual_period: i32,
    available_periods: CVec<i32>,
    category_average_performances: CVec<CString>,
    contrast_performances: CFundTrendContrastOwned,
    fund_performances: CVec<CString>,
}

impl From<FundTrend> for CFundTrendOwned {
    fn from(t: FundTrend) -> Self {
        CFundTrendOwned {
            actual_period: t.actual_period,
            available_periods: t.available_periods.into(),
            category_average_performances: values_to_cvec(t.category_average_performances),
            contrast_performances: t.contrast_performances.into(),
            fund_performances: values_to_cvec(t.fund_performances),
        }
    }
}

impl ToFFI for CFundTrendOwned {
    type FFIType = CFundTrend;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundTrend {
            actual_period: self.actual_period,
            available_periods: self.available_periods.to_ffi_type(),
            num_available_periods: self.available_periods.len(),
            category_average_performances: self.category_average_performances.to_ffi_type(),
            num_category_average_performances: self.category_average_performances.len(),
            contrast_performances: self.contrast_performances.to_ffi_type(),
            fund_performances: self.fund_performances.to_ffi_type(),
            num_fund_performances: self.fund_performances.len(),
        }
    }
}

// ── FundPerformanceComparison ───────────────────────────────────────────────

/// A named contrast performance series.
#[repr(C)]
pub struct CFundNamedContrast {
    /// Series name
    pub name: *const c_char,
    /// Performance points (JSON strings)
    pub performances: *const *const c_char,
    /// Number of performance points
    pub num_performances: usize,
}

#[derive(Debug)]
pub(crate) struct CFundNamedContrastOwned {
    name: CString,
    performances: CVec<CString>,
}

impl From<FundNamedContrast> for CFundNamedContrastOwned {
    fn from(c: FundNamedContrast) -> Self {
        CFundNamedContrastOwned {
            name: c.name.into(),
            performances: values_to_cvec(c.performances),
        }
    }
}

impl ToFFI for CFundNamedContrastOwned {
    type FFIType = CFundNamedContrast;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundNamedContrast {
            name: self.name.to_ffi_type(),
            performances: self.performances.to_ffi_type(),
            num_performances: self.performances.len(),
        }
    }
}

/// Fund performance comparison.
#[repr(C)]
pub struct CFundPerformanceComparison {
    /// Contrast performance series
    pub contrast_performances: *const CFundNamedContrast,
    /// Number of contrast performance series
    pub num_contrast_performances: usize,
    /// Fund performances (JSON strings)
    pub fund_performances: *const *const c_char,
    /// Number of fund performances
    pub num_fund_performances: usize,
}

#[derive(Debug)]
pub(crate) struct CFundPerformanceComparisonOwned {
    contrast_performances: CVec<CFundNamedContrastOwned>,
    fund_performances: CVec<CString>,
}

impl From<FundPerformanceComparison> for CFundPerformanceComparisonOwned {
    fn from(c: FundPerformanceComparison) -> Self {
        CFundPerformanceComparisonOwned {
            contrast_performances: c.contrast_performances.into(),
            fund_performances: values_to_cvec(c.fund_performances),
        }
    }
}

impl ToFFI for CFundPerformanceComparisonOwned {
    type FFIType = CFundPerformanceComparison;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPerformanceComparison {
            contrast_performances: self.contrast_performances.to_ffi_type(),
            num_contrast_performances: self.contrast_performances.len(),
            fund_performances: self.fund_performances.to_ffi_type(),
            num_fund_performances: self.fund_performances.len(),
        }
    }
}

// ── FundAnnualReturn / FundQuarterlyReturn ──────────────────────────────────

/// A fund annual return entry.
#[repr(C)]
pub struct CFundAnnualReturn {
    /// Change percent
    pub change_percent: *const c_char,
    /// Year
    pub year: i32,
}

#[derive(Debug)]
pub(crate) struct CFundAnnualReturnOwned {
    change_percent: CString,
    year: i32,
}

impl From<FundAnnualReturn> for CFundAnnualReturnOwned {
    fn from(r: FundAnnualReturn) -> Self {
        CFundAnnualReturnOwned {
            change_percent: r.change_percent.into(),
            year: r.year,
        }
    }
}

impl ToFFI for CFundAnnualReturnOwned {
    type FFIType = CFundAnnualReturn;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundAnnualReturn {
            change_percent: self.change_percent.to_ffi_type(),
            year: self.year,
        }
    }
}

/// A fund quarterly return entry.
#[repr(C)]
pub struct CFundQuarterlyReturn {
    /// Change percent
    pub change_percent: *const c_char,
    /// Quarter
    pub quarter: i32,
    /// Year
    pub year: i32,
}

#[derive(Debug)]
pub(crate) struct CFundQuarterlyReturnOwned {
    change_percent: CString,
    quarter: i32,
    year: i32,
}

impl From<FundQuarterlyReturn> for CFundQuarterlyReturnOwned {
    fn from(r: FundQuarterlyReturn) -> Self {
        CFundQuarterlyReturnOwned {
            change_percent: r.change_percent.into(),
            quarter: r.quarter,
            year: r.year,
        }
    }
}

impl ToFFI for CFundQuarterlyReturnOwned {
    type FFIType = CFundQuarterlyReturn;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundQuarterlyReturn {
            change_percent: self.change_percent.to_ffi_type(),
            quarter: self.quarter,
            year: self.year,
        }
    }
}

// ── FundPerformance ─────────────────────────────────────────────────────────

/// A fund's detailed performance figures.
#[repr(C)]
pub struct CFundPerformance {
    /// Annualized return (5y)
    pub annualized_return_five: *const c_char,
    /// Annualized return (1y)
    pub annualized_return_one: *const c_char,
    /// Annualized return (10y)
    pub annualized_return_ten: *const c_char,
    /// Annualized return (3y)
    pub annualized_return_three: *const c_char,
    /// Annualized return (2y)
    pub annualized_return_two: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Fund name
    pub fund_name: *const c_char,
    /// Rank (5y)
    pub performance_rank_five_years: i32,
    /// Rank (1d)
    pub performance_rank_one_day: i32,
    /// Rank (1m)
    pub performance_rank_one_month: i32,
    /// Rank (1w)
    pub performance_rank_one_week: i32,
    /// Rank (1y)
    pub performance_rank_one_year: i32,
    /// Rank (6m)
    pub performance_rank_six_months: i32,
    /// Rank (10y)
    pub performance_rank_ten_years: i32,
    /// Rank (3m)
    pub performance_rank_three_months: i32,
    /// Rank (3y)
    pub performance_rank_three_years: i32,
    /// Rank (2y)
    pub performance_rank_two_years: i32,
    /// Rank (ytd)
    pub performance_rank_ytd: i32,
    /// Return (5y)
    pub performance_return_five_years: *const c_char,
    /// Return (1d)
    pub performance_return_one_day: *const c_char,
    /// Return (1m)
    pub performance_return_one_month: *const c_char,
    /// Return (1w)
    pub performance_return_one_week: *const c_char,
    /// Return (1y)
    pub performance_return_one_year: *const c_char,
    /// Return (6m)
    pub performance_return_six_months: *const c_char,
    /// Return (10y)
    pub performance_return_ten_years: *const c_char,
    /// Return (3m)
    pub performance_return_three_months: *const c_char,
    /// Return (3y)
    pub performance_return_three_years: *const c_char,
    /// Return (2y)
    pub performance_return_two_years: *const c_char,
    /// Return (ytd)
    pub performance_return_ytd: *const c_char,
    /// Total peers (5y)
    pub performance_total_five_years: i32,
    /// Total peers (1d)
    pub performance_total_one_day: i32,
    /// Total peers (1m)
    pub performance_total_one_month: i32,
    /// Total peers (1w)
    pub performance_total_one_week: i32,
    /// Total peers (1y)
    pub performance_total_one_year: i32,
    /// Total peers (6m)
    pub performance_total_six_months: i32,
    /// Total peers (10y)
    pub performance_total_ten_years: i32,
    /// Total peers (3m)
    pub performance_total_three_months: i32,
    /// Total peers (3y)
    pub performance_total_three_years: i32,
    /// Total peers (2y)
    pub performance_total_two_years: i32,
    /// Total peers (ytd)
    pub performance_total_ytd: i32,
    /// Seven days annualized
    pub seven_days_annualized: *const c_char,
    /// Ten thousand price
    pub ten_thousand_price: *const c_char,
    /// Update time (unix seconds)
    pub update_time: i64,
}

#[derive(Debug)]
pub(crate) struct CFundPerformanceOwned {
    annualized_return_five: CString,
    annualized_return_one: CString,
    annualized_return_ten: CString,
    annualized_return_three: CString,
    annualized_return_two: CString,
    counter_id: CString,
    fund_name: CString,
    performance_rank_five_years: i32,
    performance_rank_one_day: i32,
    performance_rank_one_month: i32,
    performance_rank_one_week: i32,
    performance_rank_one_year: i32,
    performance_rank_six_months: i32,
    performance_rank_ten_years: i32,
    performance_rank_three_months: i32,
    performance_rank_three_years: i32,
    performance_rank_two_years: i32,
    performance_rank_ytd: i32,
    performance_return_five_years: CString,
    performance_return_one_day: CString,
    performance_return_one_month: CString,
    performance_return_one_week: CString,
    performance_return_one_year: CString,
    performance_return_six_months: CString,
    performance_return_ten_years: CString,
    performance_return_three_months: CString,
    performance_return_three_years: CString,
    performance_return_two_years: CString,
    performance_return_ytd: CString,
    performance_total_five_years: i32,
    performance_total_one_day: i32,
    performance_total_one_month: i32,
    performance_total_one_week: i32,
    performance_total_one_year: i32,
    performance_total_six_months: i32,
    performance_total_ten_years: i32,
    performance_total_three_months: i32,
    performance_total_three_years: i32,
    performance_total_two_years: i32,
    performance_total_ytd: i32,
    seven_days_annualized: CString,
    ten_thousand_price: CString,
    update_time: i64,
}

impl From<FundPerformance> for CFundPerformanceOwned {
    fn from(p: FundPerformance) -> Self {
        CFundPerformanceOwned {
            annualized_return_five: p.annualized_return_five.into(),
            annualized_return_one: p.annualized_return_one.into(),
            annualized_return_ten: p.annualized_return_ten.into(),
            annualized_return_three: p.annualized_return_three.into(),
            annualized_return_two: p.annualized_return_two.into(),
            counter_id: p.counter_id.into(),
            fund_name: p.fund_name.into(),
            performance_rank_five_years: p.performance_rank_five_years,
            performance_rank_one_day: p.performance_rank_one_day,
            performance_rank_one_month: p.performance_rank_one_month,
            performance_rank_one_week: p.performance_rank_one_week,
            performance_rank_one_year: p.performance_rank_one_year,
            performance_rank_six_months: p.performance_rank_six_months,
            performance_rank_ten_years: p.performance_rank_ten_years,
            performance_rank_three_months: p.performance_rank_three_months,
            performance_rank_three_years: p.performance_rank_three_years,
            performance_rank_two_years: p.performance_rank_two_years,
            performance_rank_ytd: p.performance_rank_ytd,
            performance_return_five_years: p.performance_return_five_years.into(),
            performance_return_one_day: p.performance_return_one_day.into(),
            performance_return_one_month: p.performance_return_one_month.into(),
            performance_return_one_week: p.performance_return_one_week.into(),
            performance_return_one_year: p.performance_return_one_year.into(),
            performance_return_six_months: p.performance_return_six_months.into(),
            performance_return_ten_years: p.performance_return_ten_years.into(),
            performance_return_three_months: p.performance_return_three_months.into(),
            performance_return_three_years: p.performance_return_three_years.into(),
            performance_return_two_years: p.performance_return_two_years.into(),
            performance_return_ytd: p.performance_return_ytd.into(),
            performance_total_five_years: p.performance_total_five_years,
            performance_total_one_day: p.performance_total_one_day,
            performance_total_one_month: p.performance_total_one_month,
            performance_total_one_week: p.performance_total_one_week,
            performance_total_one_year: p.performance_total_one_year,
            performance_total_six_months: p.performance_total_six_months,
            performance_total_ten_years: p.performance_total_ten_years,
            performance_total_three_months: p.performance_total_three_months,
            performance_total_three_years: p.performance_total_three_years,
            performance_total_two_years: p.performance_total_two_years,
            performance_total_ytd: p.performance_total_ytd,
            seven_days_annualized: p.seven_days_annualized.into(),
            ten_thousand_price: p.ten_thousand_price.into(),
            update_time: p.update_time,
        }
    }
}

impl ToFFI for CFundPerformanceOwned {
    type FFIType = CFundPerformance;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPerformance {
            annualized_return_five: self.annualized_return_five.to_ffi_type(),
            annualized_return_one: self.annualized_return_one.to_ffi_type(),
            annualized_return_ten: self.annualized_return_ten.to_ffi_type(),
            annualized_return_three: self.annualized_return_three.to_ffi_type(),
            annualized_return_two: self.annualized_return_two.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            fund_name: self.fund_name.to_ffi_type(),
            performance_rank_five_years: self.performance_rank_five_years,
            performance_rank_one_day: self.performance_rank_one_day,
            performance_rank_one_month: self.performance_rank_one_month,
            performance_rank_one_week: self.performance_rank_one_week,
            performance_rank_one_year: self.performance_rank_one_year,
            performance_rank_six_months: self.performance_rank_six_months,
            performance_rank_ten_years: self.performance_rank_ten_years,
            performance_rank_three_months: self.performance_rank_three_months,
            performance_rank_three_years: self.performance_rank_three_years,
            performance_rank_two_years: self.performance_rank_two_years,
            performance_rank_ytd: self.performance_rank_ytd,
            performance_return_five_years: self.performance_return_five_years.to_ffi_type(),
            performance_return_one_day: self.performance_return_one_day.to_ffi_type(),
            performance_return_one_month: self.performance_return_one_month.to_ffi_type(),
            performance_return_one_week: self.performance_return_one_week.to_ffi_type(),
            performance_return_one_year: self.performance_return_one_year.to_ffi_type(),
            performance_return_six_months: self.performance_return_six_months.to_ffi_type(),
            performance_return_ten_years: self.performance_return_ten_years.to_ffi_type(),
            performance_return_three_months: self.performance_return_three_months.to_ffi_type(),
            performance_return_three_years: self.performance_return_three_years.to_ffi_type(),
            performance_return_two_years: self.performance_return_two_years.to_ffi_type(),
            performance_return_ytd: self.performance_return_ytd.to_ffi_type(),
            performance_total_five_years: self.performance_total_five_years,
            performance_total_one_day: self.performance_total_one_day,
            performance_total_one_month: self.performance_total_one_month,
            performance_total_one_week: self.performance_total_one_week,
            performance_total_one_year: self.performance_total_one_year,
            performance_total_six_months: self.performance_total_six_months,
            performance_total_ten_years: self.performance_total_ten_years,
            performance_total_three_months: self.performance_total_three_months,
            performance_total_three_years: self.performance_total_three_years,
            performance_total_two_years: self.performance_total_two_years,
            performance_total_ytd: self.performance_total_ytd,
            seven_days_annualized: self.seven_days_annualized.to_ffi_type(),
            ten_thousand_price: self.ten_thousand_price.to_ffi_type(),
            update_time: self.update_time,
        }
    }
}

// ── FundHoldings ────────────────────────────────────────────────────────────

/// A single fund holding (top-10 holdings).
#[repr(C)]
pub struct CFundHolding {
    /// Bond type
    pub bond_type: *const c_char,
    /// Bond type name
    pub bond_type_name: *const c_char,
    /// Country name
    pub country_name: *const c_char,
    /// Holding type
    pub holding_type: *const c_char,
    /// Industry name
    pub industry_name: *const c_char,
    /// Market value
    pub market_value: *const c_char,
    /// Maturity date
    pub maturity_date: *const c_char,
    /// Name
    pub name: *const c_char,
    /// Share change
    pub share_change: *const c_char,
    /// Share change percent
    pub share_change_percent: *const c_char,
    /// Shares
    pub shares: *const c_char,
    /// Weighting
    pub weighting: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundHoldingOwned {
    bond_type: CString,
    bond_type_name: CString,
    country_name: CString,
    holding_type: CString,
    industry_name: CString,
    market_value: CString,
    maturity_date: CString,
    name: CString,
    share_change: CString,
    share_change_percent: CString,
    shares: CString,
    weighting: CString,
}

impl From<FundHolding> for CFundHoldingOwned {
    fn from(h: FundHolding) -> Self {
        CFundHoldingOwned {
            bond_type: h.bond_type.into(),
            bond_type_name: h.bond_type_name.into(),
            country_name: h.country_name.into(),
            holding_type: h.holding_type.into(),
            industry_name: h.industry_name.into(),
            market_value: h.market_value.into(),
            maturity_date: h.maturity_date.into(),
            name: h.name.into(),
            share_change: h.share_change.into(),
            share_change_percent: h.share_change_percent.into(),
            shares: h.shares.into(),
            weighting: h.weighting.into(),
        }
    }
}

impl ToFFI for CFundHoldingOwned {
    type FFIType = CFundHolding;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundHolding {
            bond_type: self.bond_type.to_ffi_type(),
            bond_type_name: self.bond_type_name.to_ffi_type(),
            country_name: self.country_name.to_ffi_type(),
            holding_type: self.holding_type.to_ffi_type(),
            industry_name: self.industry_name.to_ffi_type(),
            market_value: self.market_value.to_ffi_type(),
            maturity_date: self.maturity_date.to_ffi_type(),
            name: self.name.to_ffi_type(),
            share_change: self.share_change.to_ffi_type(),
            share_change_percent: self.share_change_percent.to_ffi_type(),
            shares: self.shares.to_ffi_type(),
            weighting: self.weighting.to_ffi_type(),
        }
    }
}

/// A fund's top-10 holdings.
#[repr(C)]
pub struct CFundHoldings {
    /// Holding entries
    pub holdings: *const CFundHolding,
    /// Number of holding entries
    pub num_holdings: usize,
    /// Report date
    pub report_date: *const c_char,
    /// Total weighting
    pub weighting: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundHoldingsOwned {
    holdings: CVec<CFundHoldingOwned>,
    report_date: CString,
    weighting: CString,
}

impl From<FundHoldings> for CFundHoldingsOwned {
    fn from(h: FundHoldings) -> Self {
        CFundHoldingsOwned {
            holdings: h.holdings.into(),
            report_date: h.report_date.into(),
            weighting: h.weighting.into(),
        }
    }
}

impl ToFFI for CFundHoldingsOwned {
    type FFIType = CFundHoldings;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundHoldings {
            holdings: self.holdings.to_ffi_type(),
            num_holdings: self.holdings.len(),
            report_date: self.report_date.to_ffi_type(),
            weighting: self.weighting.to_ffi_type(),
        }
    }
}

// ── FundStockHolding ────────────────────────────────────────────────────────

/// A stock held by the fund (reverse lookup).
#[repr(C)]
pub struct CFundStockHolding {
    /// Stock code
    pub code: *const c_char,
    /// Stock counter id
    pub counter_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Stock name
    pub name: *const c_char,
    /// Position ratio
    pub position_ratio: *const c_char,
    /// Report date
    pub report_date: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundStockHoldingOwned {
    code: CString,
    counter_id: CString,
    currency: CString,
    name: CString,
    position_ratio: CString,
    report_date: CString,
}

impl From<FundStockHolding> for CFundStockHoldingOwned {
    fn from(h: FundStockHolding) -> Self {
        CFundStockHoldingOwned {
            code: h.code.into(),
            counter_id: h.counter_id.into(),
            currency: h.currency.into(),
            name: h.name.into(),
            position_ratio: h.position_ratio.into(),
            report_date: h.report_date.into(),
        }
    }
}

impl ToFFI for CFundStockHoldingOwned {
    type FFIType = CFundStockHolding;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundStockHolding {
            code: self.code.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            name: self.name.to_ffi_type(),
            position_ratio: self.position_ratio.to_ffi_type(),
            report_date: self.report_date.to_ffi_type(),
        }
    }
}

// ── FundPositions ───────────────────────────────────────────────────────────

/// A single fund position held by the user.
#[repr(C)]
pub struct CFundPosition {
    /// Holding amount
    pub amount: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Frozen units
    pub freeze_units: *const c_char,
    /// Holding profit
    pub holding_profit: *const c_char,
    /// Holding units
    pub holding_units: *const c_char,
    /// Fund name
    pub name: *const c_char,
    /// Recent profit
    pub recent_profit: *const c_char,
    /// Recent trading day (unix seconds)
    pub recent_trading_day: i64,
    /// Accumulated recent profit
    pub sum_recent_profit: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundPositionOwned {
    amount: CString,
    counter_id: CString,
    currency: CString,
    freeze_units: CString,
    holding_profit: CString,
    holding_units: CString,
    name: CString,
    recent_profit: CString,
    recent_trading_day: i64,
    sum_recent_profit: CString,
}

impl From<FundPosition> for CFundPositionOwned {
    fn from(p: FundPosition) -> Self {
        CFundPositionOwned {
            amount: p.amount.into(),
            counter_id: p.counter_id.into(),
            currency: p.currency.into(),
            freeze_units: p.freeze_units.into(),
            holding_profit: p.holding_profit.into(),
            holding_units: p.holding_units.into(),
            name: p.name.into(),
            recent_profit: p.recent_profit.into(),
            recent_trading_day: p.recent_trading_day,
            sum_recent_profit: p.sum_recent_profit.into(),
        }
    }
}

impl ToFFI for CFundPositionOwned {
    type FFIType = CFundPosition;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPosition {
            amount: self.amount.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            freeze_units: self.freeze_units.to_ffi_type(),
            holding_profit: self.holding_profit.to_ffi_type(),
            holding_units: self.holding_units.to_ffi_type(),
            name: self.name.to_ffi_type(),
            recent_profit: self.recent_profit.to_ffi_type(),
            recent_trading_day: self.recent_trading_day,
            sum_recent_profit: self.sum_recent_profit.to_ffi_type(),
        }
    }
}

/// The user's fund positions overview.
#[repr(C)]
pub struct CFundPositions {
    /// Account channel
    pub account_channel: *const c_char,
    /// Position entries
    pub list: *const CFundPosition,
    /// Number of position entries
    pub num_list: usize,
    /// Pending buy orders amount
    pub pending_buy_orders: *const c_char,
    /// Recent trading day (unix seconds)
    pub recent_trading_day: i64,
    /// Sold pending credit orders amount
    pub sold_pending_credit_orders: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundPositionsOwned {
    account_channel: CString,
    list: CVec<CFundPositionOwned>,
    pending_buy_orders: CString,
    recent_trading_day: i64,
    sold_pending_credit_orders: CString,
}

impl From<FundPositions> for CFundPositionsOwned {
    fn from(p: FundPositions) -> Self {
        CFundPositionsOwned {
            account_channel: p.account_channel.into(),
            list: p.list.into(),
            pending_buy_orders: p.pending_buy_orders.into(),
            recent_trading_day: p.recent_trading_day,
            sold_pending_credit_orders: p.sold_pending_credit_orders.into(),
        }
    }
}

impl ToFFI for CFundPositionsOwned {
    type FFIType = CFundPositions;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPositions {
            account_channel: self.account_channel.to_ffi_type(),
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
            pending_buy_orders: self.pending_buy_orders.to_ffi_type(),
            recent_trading_day: self.recent_trading_day,
            sold_pending_credit_orders: self.sold_pending_credit_orders.to_ffi_type(),
        }
    }
}

// ── FundPositionDetail ──────────────────────────────────────────────────────

/// A dated value point.
#[repr(C)]
pub struct CFundDatedValue {
    /// Date (unix seconds)
    pub date: i64,
    /// Value
    pub value: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundDatedValueOwned {
    date: i64,
    value: CString,
}

impl From<FundDatedValue> for CFundDatedValueOwned {
    fn from(v: FundDatedValue) -> Self {
        CFundDatedValueOwned {
            date: v.date,
            value: v.value.into(),
        }
    }
}

impl ToFFI for CFundDatedValueOwned {
    type FFIType = CFundDatedValue;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundDatedValue {
            date: self.date,
            value: self.value.to_ffi_type(),
        }
    }
}

/// A fund unit-value point (position view).
#[repr(C)]
pub struct CFundUnitValue {
    /// Date (unix seconds)
    pub date: i64,
    /// Day increase rate
    pub day_increase_rate: *const c_char,
    /// Total value
    pub total_value: *const c_char,
    /// Unit value
    pub unit_value: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundUnitValueOwned {
    date: i64,
    day_increase_rate: CString,
    total_value: CString,
    unit_value: CString,
}

impl From<FundUnitValue> for CFundUnitValueOwned {
    fn from(v: FundUnitValue) -> Self {
        CFundUnitValueOwned {
            date: v.date,
            day_increase_rate: v.day_increase_rate.into(),
            total_value: v.total_value.into(),
            unit_value: v.unit_value.into(),
        }
    }
}

impl ToFFI for CFundUnitValueOwned {
    type FFIType = CFundUnitValue;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundUnitValue {
            date: self.date,
            day_increase_rate: self.day_increase_rate.to_ffi_type(),
            total_value: self.total_value.to_ffi_type(),
            unit_value: self.unit_value.to_ffi_type(),
        }
    }
}

/// Detail values of a single fund position.
#[repr(C)]
pub struct CFundPositionDetailValues {
    /// Amount
    pub amount: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Holding cost
    pub holding_cost: *const c_char,
    /// Holding profit
    pub holding_profit: *const c_char,
    /// Holding profit rate
    pub holding_profit_rate: *const c_char,
    /// Holding units
    pub holding_units: *const c_char,
    /// Holding value
    pub holding_value: *const c_char,
    /// Pending buy value
    pub pending_buy_value: *const c_char,
    /// Pending sell value
    pub pending_sell_value: *const c_char,
    /// Accumulated profit (to date)
    pub profit_amount_accum_td: *const c_char,
    /// Accumulated profit rate (to date)
    pub profit_amount_accum_td_rate: *const c_char,
    /// Recent profit
    pub recent_profit: *const c_char,
    /// Recent trading day (unix seconds)
    pub recent_tradingday: i64,
    /// Recent unit value
    pub recent_unit_value: *const c_char,
    /// Sold pending-confirm units
    pub sold_pending_confirm_units: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundPositionDetailValuesOwned {
    amount: CString,
    currency: CString,
    holding_cost: CString,
    holding_profit: CString,
    holding_profit_rate: CString,
    holding_units: CString,
    holding_value: CString,
    pending_buy_value: CString,
    pending_sell_value: CString,
    profit_amount_accum_td: CString,
    profit_amount_accum_td_rate: CString,
    recent_profit: CString,
    recent_tradingday: i64,
    recent_unit_value: CString,
    sold_pending_confirm_units: CString,
}

impl From<FundPositionDetailValues> for CFundPositionDetailValuesOwned {
    fn from(v: FundPositionDetailValues) -> Self {
        CFundPositionDetailValuesOwned {
            amount: v.amount.into(),
            currency: v.currency.into(),
            holding_cost: v.holding_cost.into(),
            holding_profit: v.holding_profit.into(),
            holding_profit_rate: v.holding_profit_rate.into(),
            holding_units: v.holding_units.into(),
            holding_value: v.holding_value.into(),
            pending_buy_value: v.pending_buy_value.into(),
            pending_sell_value: v.pending_sell_value.into(),
            profit_amount_accum_td: v.profit_amount_accum_td.into(),
            profit_amount_accum_td_rate: v.profit_amount_accum_td_rate.into(),
            recent_profit: v.recent_profit.into(),
            recent_tradingday: v.recent_tradingday,
            recent_unit_value: v.recent_unit_value.into(),
            sold_pending_confirm_units: v.sold_pending_confirm_units.into(),
        }
    }
}

impl ToFFI for CFundPositionDetailValuesOwned {
    type FFIType = CFundPositionDetailValues;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPositionDetailValues {
            amount: self.amount.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            holding_cost: self.holding_cost.to_ffi_type(),
            holding_profit: self.holding_profit.to_ffi_type(),
            holding_profit_rate: self.holding_profit_rate.to_ffi_type(),
            holding_units: self.holding_units.to_ffi_type(),
            holding_value: self.holding_value.to_ffi_type(),
            pending_buy_value: self.pending_buy_value.to_ffi_type(),
            pending_sell_value: self.pending_sell_value.to_ffi_type(),
            profit_amount_accum_td: self.profit_amount_accum_td.to_ffi_type(),
            profit_amount_accum_td_rate: self.profit_amount_accum_td_rate.to_ffi_type(),
            recent_profit: self.recent_profit.to_ffi_type(),
            recent_tradingday: self.recent_tradingday,
            recent_unit_value: self.recent_unit_value.to_ffi_type(),
            sold_pending_confirm_units: self.sold_pending_confirm_units.to_ffi_type(),
        }
    }
}

/// Detail of a single fund position.
#[repr(C)]
pub struct CFundPositionDetail {
    /// Detail values
    pub detail_values: CFundPositionDetailValues,
    /// Accumulated profit series
    pub sum_profit: *const CFundDatedValue,
    /// Number of accumulated-profit points
    pub num_sum_profit: usize,
    /// Unit value series
    pub ut_value: *const CFundUnitValue,
    /// Number of unit-value points
    pub num_ut_value: usize,
}

#[derive(Debug)]
pub(crate) struct CFundPositionDetailOwned {
    detail_values: CFundPositionDetailValuesOwned,
    sum_profit: CVec<CFundDatedValueOwned>,
    ut_value: CVec<CFundUnitValueOwned>,
}

impl From<FundPositionDetail> for CFundPositionDetailOwned {
    fn from(d: FundPositionDetail) -> Self {
        CFundPositionDetailOwned {
            detail_values: d.detail_values.into(),
            sum_profit: d.sum_profit.into(),
            ut_value: d.ut_value.into(),
        }
    }
}

impl ToFFI for CFundPositionDetailOwned {
    type FFIType = CFundPositionDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPositionDetail {
            detail_values: self.detail_values.to_ffi_type(),
            sum_profit: self.sum_profit.to_ffi_type(),
            num_sum_profit: self.sum_profit.len(),
            ut_value: self.ut_value.to_ffi_type(),
            num_ut_value: self.ut_value.len(),
        }
    }
}

// ── FundPositionPerformance ─────────────────────────────────────────────────

/// Performance figures for a held fund.
#[repr(C)]
pub struct CFundPositionPerformance {
    /// Annualized return (5y)
    pub annualized_return_five: *const c_char,
    /// Annualized return (1y)
    pub annualized_return_one: *const c_char,
    /// Annualized return (10y)
    pub annualized_return_ten: *const c_char,
    /// Annualized return (3y)
    pub annualized_return_three: *const c_char,
    /// Annualized return (2y)
    pub annualized_return_two: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Fund name
    pub fund_name: *const c_char,
    /// Return (5y)
    pub performance_return_five_years: *const c_char,
    /// Return (1d)
    pub performance_return_one_day: *const c_char,
    /// Return (1m)
    pub performance_return_one_month: *const c_char,
    /// Return (1w)
    pub performance_return_one_week: *const c_char,
    /// Return (1y)
    pub performance_return_one_year: *const c_char,
    /// Return (6m)
    pub performance_return_six_months: *const c_char,
    /// Return (10y)
    pub performance_return_ten_years: *const c_char,
    /// Return (3m)
    pub performance_return_three_months: *const c_char,
    /// Return (3y)
    pub performance_return_three_years: *const c_char,
    /// Return (2y)
    pub performance_return_two_years: *const c_char,
    /// Return (ytd)
    pub performance_return_ytd: *const c_char,
    /// Update time (unix seconds)
    pub update_time: i64,
}

#[derive(Debug)]
pub(crate) struct CFundPositionPerformanceOwned {
    annualized_return_five: CString,
    annualized_return_one: CString,
    annualized_return_ten: CString,
    annualized_return_three: CString,
    annualized_return_two: CString,
    counter_id: CString,
    fund_name: CString,
    performance_return_five_years: CString,
    performance_return_one_day: CString,
    performance_return_one_month: CString,
    performance_return_one_week: CString,
    performance_return_one_year: CString,
    performance_return_six_months: CString,
    performance_return_ten_years: CString,
    performance_return_three_months: CString,
    performance_return_three_years: CString,
    performance_return_two_years: CString,
    performance_return_ytd: CString,
    update_time: i64,
}

impl From<FundPositionPerformance> for CFundPositionPerformanceOwned {
    fn from(p: FundPositionPerformance) -> Self {
        CFundPositionPerformanceOwned {
            annualized_return_five: p.annualized_return_five.into(),
            annualized_return_one: p.annualized_return_one.into(),
            annualized_return_ten: p.annualized_return_ten.into(),
            annualized_return_three: p.annualized_return_three.into(),
            annualized_return_two: p.annualized_return_two.into(),
            counter_id: p.counter_id.into(),
            fund_name: p.fund_name.into(),
            performance_return_five_years: p.performance_return_five_years.into(),
            performance_return_one_day: p.performance_return_one_day.into(),
            performance_return_one_month: p.performance_return_one_month.into(),
            performance_return_one_week: p.performance_return_one_week.into(),
            performance_return_one_year: p.performance_return_one_year.into(),
            performance_return_six_months: p.performance_return_six_months.into(),
            performance_return_ten_years: p.performance_return_ten_years.into(),
            performance_return_three_months: p.performance_return_three_months.into(),
            performance_return_three_years: p.performance_return_three_years.into(),
            performance_return_two_years: p.performance_return_two_years.into(),
            performance_return_ytd: p.performance_return_ytd.into(),
            update_time: p.update_time,
        }
    }
}

impl ToFFI for CFundPositionPerformanceOwned {
    type FFIType = CFundPositionPerformance;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPositionPerformance {
            annualized_return_five: self.annualized_return_five.to_ffi_type(),
            annualized_return_one: self.annualized_return_one.to_ffi_type(),
            annualized_return_ten: self.annualized_return_ten.to_ffi_type(),
            annualized_return_three: self.annualized_return_three.to_ffi_type(),
            annualized_return_two: self.annualized_return_two.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            fund_name: self.fund_name.to_ffi_type(),
            performance_return_five_years: self.performance_return_five_years.to_ffi_type(),
            performance_return_one_day: self.performance_return_one_day.to_ffi_type(),
            performance_return_one_month: self.performance_return_one_month.to_ffi_type(),
            performance_return_one_week: self.performance_return_one_week.to_ffi_type(),
            performance_return_one_year: self.performance_return_one_year.to_ffi_type(),
            performance_return_six_months: self.performance_return_six_months.to_ffi_type(),
            performance_return_ten_years: self.performance_return_ten_years.to_ffi_type(),
            performance_return_three_months: self.performance_return_three_months.to_ffi_type(),
            performance_return_three_years: self.performance_return_three_years.to_ffi_type(),
            performance_return_two_years: self.performance_return_two_years.to_ffi_type(),
            performance_return_ytd: self.performance_return_ytd.to_ffi_type(),
            update_time: self.update_time,
        }
    }
}

// ── FundPositionProfits ─────────────────────────────────────────────────────

/// The user's cumulative profit for a held fund.
#[repr(C)]
pub struct CFundPositionProfits {
    /// Currency
    pub currency: *const c_char,
    /// Profit series
    pub history_value: *const CFundDatedValue,
    /// Number of profit points
    pub num_history_value: usize,
    /// Last update time (unix seconds)
    pub last_update_time: i64,
    /// Total profit
    pub sum_profit: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundPositionProfitsOwned {
    currency: CString,
    history_value: CVec<CFundDatedValueOwned>,
    last_update_time: i64,
    sum_profit: CString,
}

impl From<FundPositionProfits> for CFundPositionProfitsOwned {
    fn from(p: FundPositionProfits) -> Self {
        CFundPositionProfitsOwned {
            currency: p.currency.into(),
            history_value: p.history_value.into(),
            last_update_time: p.last_update_time,
            sum_profit: p.sum_profit.into(),
        }
    }
}

impl ToFFI for CFundPositionProfitsOwned {
    type FFIType = CFundPositionProfits;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPositionProfits {
            currency: self.currency.to_ffi_type(),
            history_value: self.history_value.to_ffi_type(),
            num_history_value: self.history_value.len(),
            last_update_time: self.last_update_time,
            sum_profit: self.sum_profit.to_ffi_type(),
        }
    }
}

// ── FundPositionNav ─────────────────────────────────────────────────────────

/// A held-fund net-value point (position view).
#[repr(C)]
pub struct CFundPositionNav {
    /// Net value change
    pub change: *const c_char,
    /// Net value change percent
    pub change_percent: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Fund name
    pub counter_name: *const c_char,
    /// Last update time (unix seconds)
    pub last_update_time: i64,
    /// Net value
    pub value: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundPositionNavOwned {
    change: CString,
    change_percent: CString,
    counter_id: CString,
    counter_name: CString,
    last_update_time: i64,
    value: CString,
}

impl From<FundPositionNav> for CFundPositionNavOwned {
    fn from(v: FundPositionNav) -> Self {
        CFundPositionNavOwned {
            change: v.change.into(),
            change_percent: v.change_percent.into(),
            counter_id: v.counter_id.into(),
            counter_name: v.counter_name.into(),
            last_update_time: v.last_update_time,
            value: v.value.into(),
        }
    }
}

impl ToFFI for CFundPositionNavOwned {
    type FFIType = CFundPositionNav;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundPositionNav {
            change: self.change.to_ffi_type(),
            change_percent: self.change_percent.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            last_update_time: self.last_update_time,
            value: self.value.to_ffi_type(),
        }
    }
}

// ── FundDividends ───────────────────────────────────────────────────────────

/// A cash dividend record for a held fund.
#[repr(C)]
pub struct CFundDividend {
    /// Amount
    pub amount: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Date (unix seconds)
    pub date: i64,
    /// Dividend method
    pub div_method: *const c_char,
    /// Fund name
    pub name: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundDividendOwned {
    amount: CString,
    counter_id: CString,
    currency: CString,
    date: i64,
    div_method: CString,
    name: CString,
}

impl From<FundDividend> for CFundDividendOwned {
    fn from(d: FundDividend) -> Self {
        CFundDividendOwned {
            amount: d.amount.into(),
            counter_id: d.counter_id.into(),
            currency: d.currency.into(),
            date: d.date,
            div_method: d.div_method.into(),
            name: d.name.into(),
        }
    }
}

impl ToFFI for CFundDividendOwned {
    type FFIType = CFundDividend;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundDividend {
            amount: self.amount.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            date: self.date,
            div_method: self.div_method.to_ffi_type(),
            name: self.name.to_ffi_type(),
        }
    }
}

/// The user's dividend records for a held fund.
#[repr(C)]
pub struct CFundDividends {
    /// Currency
    pub currency: *const c_char,
    /// Dividend records
    pub div_cash_infos: *const CFundDividend,
    /// Number of dividend records
    pub num_div_cash_infos: usize,
    /// Latest dividend date (unix seconds)
    pub lastest_date: i64,
    /// Total cash dividend
    pub total_div_cash: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundDividendsOwned {
    currency: CString,
    div_cash_infos: CVec<CFundDividendOwned>,
    lastest_date: i64,
    total_div_cash: CString,
}

impl From<FundDividends> for CFundDividendsOwned {
    fn from(d: FundDividends) -> Self {
        CFundDividendsOwned {
            currency: d.currency.into(),
            div_cash_infos: d.div_cash_infos.into(),
            lastest_date: d.lastest_date,
            total_div_cash: d.total_div_cash.into(),
        }
    }
}

impl ToFFI for CFundDividendsOwned {
    type FFIType = CFundDividends;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundDividends {
            currency: self.currency.to_ffi_type(),
            div_cash_infos: self.div_cash_infos.to_ffi_type(),
            num_div_cash_infos: self.div_cash_infos.len(),
            lastest_date: self.lastest_date,
            total_div_cash: self.total_div_cash.to_ffi_type(),
        }
    }
}

// ── FundOrder ───────────────────────────────────────────────────────────────

/// A fund order (list view).
#[repr(C)]
pub struct CFundOrder {
    /// Action (buy/sell)
    pub action: *const c_char,
    /// Amount
    pub amount: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Currency
    pub currency: *const c_char,
    /// Fund name
    pub fund_name: *const c_char,
    /// Order id
    pub id: i64,
    /// Whether it is an auto (DCA) order
    pub is_auto: bool,
    /// Net worth
    pub net_worth: *const c_char,
    /// Product type
    pub product_type: *const c_char,
    /// State
    pub state: *const c_char,
    /// State description
    pub state_desc: *const c_char,
    /// Units
    pub units: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundOrderOwned {
    action: CString,
    amount: CString,
    counter_id: CString,
    created_at: i64,
    currency: CString,
    fund_name: CString,
    id: i64,
    is_auto: bool,
    net_worth: CString,
    product_type: CString,
    state: CString,
    state_desc: CString,
    units: CString,
}

impl From<FundOrder> for CFundOrderOwned {
    fn from(o: FundOrder) -> Self {
        CFundOrderOwned {
            action: o.action.into(),
            amount: o.amount.into(),
            counter_id: o.counter_id.into(),
            created_at: o.created_at,
            currency: o.currency.into(),
            fund_name: o.fund_name.into(),
            id: o.id,
            is_auto: o.is_auto,
            net_worth: o.net_worth.into(),
            product_type: o.product_type.into(),
            state: o.state.into(),
            state_desc: o.state_desc.into(),
            units: o.units.into(),
        }
    }
}

impl ToFFI for CFundOrderOwned {
    type FFIType = CFundOrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrder {
            action: self.action.to_ffi_type(),
            amount: self.amount.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            created_at: self.created_at,
            currency: self.currency.to_ffi_type(),
            fund_name: self.fund_name.to_ffi_type(),
            id: self.id,
            is_auto: self.is_auto,
            net_worth: self.net_worth.to_ffi_type(),
            product_type: self.product_type.to_ffi_type(),
            state: self.state.to_ffi_type(),
            state_desc: self.state_desc.to_ffi_type(),
            units: self.units.to_ffi_type(),
        }
    }
}

// ── FundOrderDetail ─────────────────────────────────────────────────────────

/// A keyword block in a fund order detail.
#[repr(C)]
pub struct CFundOrderKeyword {
    /// Content
    pub content: *const c_char,
    /// Group
    pub group: *const c_char,
    /// Key
    pub key: *const c_char,
    /// Line strategy
    pub line_strategy: *const c_char,
    /// Title
    pub title: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundOrderKeywordOwned {
    content: CString,
    group: CString,
    key: CString,
    line_strategy: CString,
    title: CString,
}

impl From<FundOrderKeyword> for CFundOrderKeywordOwned {
    fn from(k: FundOrderKeyword) -> Self {
        CFundOrderKeywordOwned {
            content: k.content.into(),
            group: k.group.into(),
            key: k.key.into(),
            line_strategy: k.line_strategy.into(),
            title: k.title.into(),
        }
    }
}

impl ToFFI for CFundOrderKeywordOwned {
    type FFIType = CFundOrderKeyword;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrderKeyword {
            content: self.content.to_ffi_type(),
            group: self.group.to_ffi_type(),
            key: self.key.to_ffi_type(),
            line_strategy: self.line_strategy.to_ffi_type(),
            title: self.title.to_ffi_type(),
        }
    }
}

/// A processing stage in a fund order detail.
#[repr(C)]
pub struct CFundOrderStage {
    /// Description
    pub desc: *const c_char,
    /// Key
    pub key: *const c_char,
    /// Link
    pub link: *const c_char,
    /// Link text
    pub link_text: *const c_char,
    /// Progress
    pub progress: *const c_char,
    /// Stage
    pub stage: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundOrderStageOwned {
    desc: CString,
    key: CString,
    link: CString,
    link_text: CString,
    progress: CString,
    stage: CString,
}

impl From<FundOrderStage> for CFundOrderStageOwned {
    fn from(s: FundOrderStage) -> Self {
        CFundOrderStageOwned {
            desc: s.desc.into(),
            key: s.key.into(),
            link: s.link.into(),
            link_text: s.link_text.into(),
            progress: s.progress.into(),
            stage: s.stage.into(),
        }
    }
}

impl ToFFI for CFundOrderStageOwned {
    type FFIType = CFundOrderStage;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrderStage {
            desc: self.desc.to_ffi_type(),
            key: self.key.to_ffi_type(),
            link: self.link.to_ffi_type(),
            link_text: self.link_text.to_ffi_type(),
            progress: self.progress.to_ffi_type(),
            stage: self.stage.to_ffi_type(),
        }
    }
}

/// The full information of a fund order.
#[repr(C)]
pub struct CFundOrderInfo {
    /// Account id
    pub aaid: i64,
    /// Account channel
    pub account_channel: *const c_char,
    /// Action (buy/sell)
    pub action: *const c_char,
    /// Amount
    pub amount: *const c_char,
    /// Channel
    pub channel: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Currency
    pub currency: *const c_char,
    /// Dividend option
    pub dividend_option: *const c_char,
    /// Equity time (unix seconds)
    pub eq_at: i64,
    /// Fee
    pub fee: *const c_char,
    /// Fund name
    pub fund_name: *const c_char,
    /// Fund source
    pub fund_source: *const c_char,
    /// Histories
    pub histories: *const c_char,
    /// Order id
    pub id: i64,
    /// Message
    pub message: *const c_char,
    /// Net worth
    pub net_worth: *const c_char,
    /// Price time (unix seconds)
    pub price_at: i64,
    /// Processed at (unix seconds)
    pub processed_at: i64,
    /// Product type
    pub product_type: *const c_char,
    /// Whether repurchaseable
    pub repurchaseable: bool,
    /// Sale proceeds
    pub sale_proceeds: *const c_char,
    /// Sales charge
    pub sales_charge: *const c_char,
    /// Sales price
    pub sales_price: *const c_char,
    /// Sales unit
    pub sales_unit: *const c_char,
    /// State
    pub state: *const c_char,
    /// State description
    pub state_desc: *const c_char,
    /// Status
    pub status: i32,
    /// Extended status
    pub status_ex: i32,
    /// T+ description
    pub t_description: *const c_char,
    /// Time partition
    pub time_partition: *const c_char,
    /// Total amount
    pub total_amount: *const c_char,
    /// Transaction at (unix seconds)
    pub transaction_at: i64,
    /// Units
    pub units: *const c_char,
    /// Withdraw at (unix seconds)
    pub withdraw_at: i64,
    /// Whether withdrawable
    pub withdrawable: bool,
}

#[derive(Debug)]
pub(crate) struct CFundOrderInfoOwned {
    aaid: i64,
    account_channel: CString,
    action: CString,
    amount: CString,
    channel: CString,
    counter_id: CString,
    created_at: i64,
    currency: CString,
    dividend_option: CString,
    eq_at: i64,
    fee: CString,
    fund_name: CString,
    fund_source: CString,
    histories: CString,
    id: i64,
    message: CString,
    net_worth: CString,
    price_at: i64,
    processed_at: i64,
    product_type: CString,
    repurchaseable: bool,
    sale_proceeds: CString,
    sales_charge: CString,
    sales_price: CString,
    sales_unit: CString,
    state: CString,
    state_desc: CString,
    status: i32,
    status_ex: i32,
    t_description: CString,
    time_partition: CString,
    total_amount: CString,
    transaction_at: i64,
    units: CString,
    withdraw_at: i64,
    withdrawable: bool,
}

impl From<FundOrderInfo> for CFundOrderInfoOwned {
    fn from(o: FundOrderInfo) -> Self {
        CFundOrderInfoOwned {
            aaid: o.aaid,
            account_channel: o.account_channel.into(),
            action: o.action.into(),
            amount: o.amount.into(),
            channel: o.channel.into(),
            counter_id: o.counter_id.into(),
            created_at: o.created_at,
            currency: o.currency.into(),
            dividend_option: o.dividend_option.into(),
            eq_at: o.eq_at,
            fee: o.fee.into(),
            fund_name: o.fund_name.into(),
            fund_source: o.fund_source.into(),
            histories: o.histories.into(),
            id: o.id,
            message: o.message.into(),
            net_worth: o.net_worth.into(),
            price_at: o.price_at,
            processed_at: o.processed_at,
            product_type: o.product_type.into(),
            repurchaseable: o.repurchaseable,
            sale_proceeds: o.sale_proceeds.into(),
            sales_charge: o.sales_charge.into(),
            sales_price: o.sales_price.into(),
            sales_unit: o.sales_unit.into(),
            state: o.state.into(),
            state_desc: o.state_desc.into(),
            status: o.status,
            status_ex: o.status_ex,
            t_description: o.t_description.into(),
            time_partition: o.time_partition.into(),
            total_amount: o.total_amount.into(),
            transaction_at: o.transaction_at,
            units: o.units.into(),
            withdraw_at: o.withdraw_at,
            withdrawable: o.withdrawable,
        }
    }
}

impl ToFFI for CFundOrderInfoOwned {
    type FFIType = CFundOrderInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrderInfo {
            aaid: self.aaid,
            account_channel: self.account_channel.to_ffi_type(),
            action: self.action.to_ffi_type(),
            amount: self.amount.to_ffi_type(),
            channel: self.channel.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            created_at: self.created_at,
            currency: self.currency.to_ffi_type(),
            dividend_option: self.dividend_option.to_ffi_type(),
            eq_at: self.eq_at,
            fee: self.fee.to_ffi_type(),
            fund_name: self.fund_name.to_ffi_type(),
            fund_source: self.fund_source.to_ffi_type(),
            histories: self.histories.to_ffi_type(),
            id: self.id,
            message: self.message.to_ffi_type(),
            net_worth: self.net_worth.to_ffi_type(),
            price_at: self.price_at,
            processed_at: self.processed_at,
            product_type: self.product_type.to_ffi_type(),
            repurchaseable: self.repurchaseable,
            sale_proceeds: self.sale_proceeds.to_ffi_type(),
            sales_charge: self.sales_charge.to_ffi_type(),
            sales_price: self.sales_price.to_ffi_type(),
            sales_unit: self.sales_unit.to_ffi_type(),
            state: self.state.to_ffi_type(),
            state_desc: self.state_desc.to_ffi_type(),
            status: self.status,
            status_ex: self.status_ex,
            t_description: self.t_description.to_ffi_type(),
            time_partition: self.time_partition.to_ffi_type(),
            total_amount: self.total_amount.to_ffi_type(),
            transaction_at: self.transaction_at,
            units: self.units.to_ffi_type(),
            withdraw_at: self.withdraw_at,
            withdrawable: self.withdrawable,
        }
    }
}

/// Fund order detail.
#[repr(C)]
pub struct CFundOrderDetail {
    /// Keyword blocks
    pub keywords: *const CFundOrderKeyword,
    /// Number of keyword blocks
    pub num_keywords: usize,
    /// The order
    pub order: CFundOrderInfo,
    /// Processing stages
    pub stages: *const CFundOrderStage,
    /// Number of processing stages
    pub num_stages: usize,
}

#[derive(Debug)]
pub(crate) struct CFundOrderDetailOwned {
    keywords: CVec<CFundOrderKeywordOwned>,
    order: CFundOrderInfoOwned,
    stages: CVec<CFundOrderStageOwned>,
}

impl From<FundOrderDetail> for CFundOrderDetailOwned {
    fn from(d: FundOrderDetail) -> Self {
        CFundOrderDetailOwned {
            keywords: d.keywords.into(),
            order: d.order.into(),
            stages: d.stages.into(),
        }
    }
}

impl ToFFI for CFundOrderDetailOwned {
    type FFIType = CFundOrderDetail;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrderDetail {
            keywords: self.keywords.to_ffi_type(),
            num_keywords: self.keywords.len(),
            order: self.order.to_ffi_type(),
            stages: self.stages.to_ffi_type(),
            num_stages: self.stages.len(),
        }
    }
}

// ── FundTransaction ─────────────────────────────────────────────────────────

/// A fund transaction / cash-flow record.
#[repr(C)]
pub struct CFundTransaction {
    /// Amount
    pub amount: *const c_char,
    /// Category
    pub category: *const c_char,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Currency
    pub currency: *const c_char,
    /// Description
    pub description: *const c_char,
    /// Detail created at (unix seconds)
    pub detail_created_at: i64,
    /// Detail type
    pub detail_type: *const c_char,
    /// Done at (unix seconds)
    pub done_at: i64,
    /// Quantity description
    pub quantity_description: *const c_char,
    /// Redirect page
    pub redirect_page: *const c_char,
    /// Redirect page (v2)
    pub redirect_page_v2: *const c_char,
    /// Reference number
    pub ref_no: *const c_char,
    /// Stock quantity
    pub stock_quantity: *const c_char,
    /// Transaction type
    pub tx_type: *const c_char,
    /// Type name
    pub type_name: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundTransactionOwned {
    amount: CString,
    category: CString,
    created_at: i64,
    currency: CString,
    description: CString,
    detail_created_at: i64,
    detail_type: CString,
    done_at: i64,
    quantity_description: CString,
    redirect_page: CString,
    redirect_page_v2: CString,
    ref_no: CString,
    stock_quantity: CString,
    tx_type: CString,
    type_name: CString,
}

impl From<FundTransaction> for CFundTransactionOwned {
    fn from(t: FundTransaction) -> Self {
        CFundTransactionOwned {
            amount: t.amount.into(),
            category: t.category.into(),
            created_at: t.created_at,
            currency: t.currency.into(),
            description: t.description.into(),
            detail_created_at: t.detail_created_at,
            detail_type: t.detail_type.into(),
            done_at: t.done_at,
            quantity_description: t.quantity_description.into(),
            redirect_page: t.redirect_page.into(),
            redirect_page_v2: t.redirect_page_v2.into(),
            ref_no: t.ref_no.into(),
            stock_quantity: t.stock_quantity.into(),
            tx_type: t.tx_type.into(),
            type_name: t.type_name.into(),
        }
    }
}

impl ToFFI for CFundTransactionOwned {
    type FFIType = CFundTransaction;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundTransaction {
            amount: self.amount.to_ffi_type(),
            category: self.category.to_ffi_type(),
            created_at: self.created_at,
            currency: self.currency.to_ffi_type(),
            description: self.description.to_ffi_type(),
            detail_created_at: self.detail_created_at,
            detail_type: self.detail_type.to_ffi_type(),
            done_at: self.done_at,
            quantity_description: self.quantity_description.to_ffi_type(),
            redirect_page: self.redirect_page.to_ffi_type(),
            redirect_page_v2: self.redirect_page_v2.to_ffi_type(),
            ref_no: self.ref_no.to_ffi_type(),
            stock_quantity: self.stock_quantity.to_ffi_type(),
            tx_type: self.tx_type.to_ffi_type(),
            type_name: self.type_name.to_ffi_type(),
        }
    }
}

// ── FundOrderValidation ─────────────────────────────────────────────────────

/// The result of validating a fund order.
#[repr(C)]
pub struct CFundOrderValidation {
    /// Auth token to carry into submit
    pub auth_token: *const c_char,
    /// Risk-assessment eval address
    pub eval_address: *const c_char,
    /// Fund risk level
    pub fund_risk_level: i32,
    /// Message
    pub msg: *const c_char,
    /// User PI status
    pub user_pi: i32,
    /// User risk level
    pub user_risk_level: i32,
}

#[derive(Debug)]
pub(crate) struct CFundOrderValidationOwned {
    auth_token: CString,
    eval_address: CString,
    fund_risk_level: i32,
    msg: CString,
    user_pi: i32,
    user_risk_level: i32,
}

impl From<FundOrderValidation> for CFundOrderValidationOwned {
    fn from(v: FundOrderValidation) -> Self {
        CFundOrderValidationOwned {
            auth_token: v.auth_token.into(),
            eval_address: v.eval_address.into(),
            fund_risk_level: v.fund_risk_level,
            msg: v.msg.into(),
            user_pi: v.user_pi,
            user_risk_level: v.user_risk_level,
        }
    }
}

impl ToFFI for CFundOrderValidationOwned {
    type FFIType = CFundOrderValidation;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrderValidation {
            auth_token: self.auth_token.to_ffi_type(),
            eval_address: self.eval_address.to_ffi_type(),
            fund_risk_level: self.fund_risk_level,
            msg: self.msg.to_ffi_type(),
            user_pi: self.user_pi,
            user_risk_level: self.user_risk_level,
        }
    }
}

// ── FundOrderSubmitResponse ─────────────────────────────────────────────────

/// The result of submitting a fund order.
#[repr(C)]
pub struct CFundOrderSubmitResponse {
    /// Action (buy/sell)
    pub action: *const c_char,
    /// Amount
    pub amount: *const c_char,
    /// Fund counter id
    pub counter_id: *const c_char,
    /// Created at (unix seconds)
    pub created_at: i64,
    /// Fund name
    pub fund_name: *const c_char,
    /// Order id
    pub id: i64,
    /// Message
    pub msg: *const c_char,
    /// Status
    pub status: i32,
    /// Units
    pub units: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CFundOrderSubmitResponseOwned {
    action: CString,
    amount: CString,
    counter_id: CString,
    created_at: i64,
    fund_name: CString,
    id: i64,
    msg: CString,
    status: i32,
    units: CString,
}

impl From<FundOrderSubmitResponse> for CFundOrderSubmitResponseOwned {
    fn from(r: FundOrderSubmitResponse) -> Self {
        CFundOrderSubmitResponseOwned {
            action: r.action.into(),
            amount: r.amount.into(),
            counter_id: r.counter_id.into(),
            created_at: r.created_at,
            fund_name: r.fund_name.into(),
            id: r.id,
            msg: r.msg.into(),
            status: r.status,
            units: r.units.into(),
        }
    }
}

impl ToFFI for CFundOrderSubmitResponseOwned {
    type FFIType = CFundOrderSubmitResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CFundOrderSubmitResponse {
            action: self.action.to_ffi_type(),
            amount: self.amount.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            created_at: self.created_at,
            fund_name: self.fund_name.to_ffi_type(),
            id: self.id,
            msg: self.msg.to_ffi_type(),
            status: self.status,
            units: self.units.to_ffi_type(),
        }
    }
}

// ── Request option C structs ────────────────────────────────────────────────

/// Options for the fund list request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundsOptions {
    /// Server-defined filter object as a JSON string (can be null)
    pub filter: *const c_char,
    /// Quick-filter ids (can be null)
    pub quick_ids: *const i64,
    /// Number of quick-filter ids
    pub num_quick_ids: usize,
    /// Earning-rate time intervals (can be null)
    pub time_interval: *const *const c_char,
    /// Number of time intervals
    pub num_time_interval: usize,
}

/// Options for the fund analysis / trend / comparison request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundAnalysisOptions {
    /// Analysis period (can be null)
    pub period: *const i32,
}

/// Paging options (page / size).
#[derive(Debug)]
#[repr(C)]
pub struct CFundPageOptions {
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
}

/// Net-value range options (relative months / years before now).
#[derive(Debug)]
#[repr(C)]
pub struct CFundNavRangeOptions {
    /// Number of months before now (can be null)
    pub month_before: *const i32,
    /// Number of years before now (can be null)
    pub year_before: *const i32,
}

/// Options for the fund holdings request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundHoldingsOptions {
    /// Scene (can be null)
    pub scene: *const i32,
}

/// Options for the fund stock-holdings (reverse) request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundStockHoldingsOptions {
    /// Maximum number of stocks to return (can be null)
    pub limit: *const i32,
}

/// Options for the fund positions overview request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundPositionsOptions {
    /// Account channel (can be null)
    pub account_channel: *const c_char,
    /// Account id (can be null)
    pub aaid: *const i64,
}

/// Options for a single fund position detail request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundPositionOptions {
    /// Account channel (can be null)
    pub account_channel: *const c_char,
    /// Account id (can be null)
    pub aaid: *const i64,
    /// Range start (can be null)
    pub start: *const c_char,
    /// Range end (can be null)
    pub end: *const c_char,
}

/// Options for a single fund position cumulative-profit request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundPositionProfitsOptions {
    /// Account channel (can be null)
    pub account_channel: *const c_char,
    /// Account id (can be null)
    pub aaid: *const i64,
    /// Range start (can be null)
    pub start: *const c_char,
    /// Range end (can be null)
    pub end: *const c_char,
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
}

/// Options for a single fund position dividend request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundPositionDividendsOptions {
    /// Account channel (can be null)
    pub account_channel: *const c_char,
    /// Account id (can be null)
    pub aaid: *const i64,
    /// Currency (can be null)
    pub currency: *const c_char,
    /// Range start (unix seconds) (can be null)
    pub start: *const i64,
    /// Range end (unix seconds) (can be null)
    pub end: *const i64,
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
}

/// Options for the fund orders list request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundOrdersOptions {
    /// Filter by fund symbols (can be null)
    pub symbols: *const *const c_char,
    /// Number of fund symbols
    pub num_symbols: usize,
    /// Filter by actions (comma-separated) (can be null)
    pub actions: *const c_char,
    /// Filter by states (comma-separated) (can be null)
    pub states: *const c_char,
    /// Filter by currency (can be null)
    pub currency: *const c_char,
    /// Range start (unix seconds) (can be null)
    pub start: *const i64,
    /// Range end (unix seconds) (can be null)
    pub end: *const i64,
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
}

/// Options for the fund transactions (cash-flow) list request.
#[derive(Debug)]
#[repr(C)]
pub struct CGetFundTransactionsOptions {
    /// Account channel (can be null)
    pub account_channel: *const c_char,
    /// Business type (can be null)
    pub business_type: *const c_char,
    /// Category (can be null)
    pub category: *const c_char,
    /// Currencies (comma-separated) (can be null)
    pub currencies: *const c_char,
    /// Range start (unix seconds) (can be null)
    pub start: *const i64,
    /// Range end (unix seconds) (can be null)
    pub end: *const i64,
    /// Page number (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
}

/// Options for validating a fund order.
#[derive(Debug)]
#[repr(C)]
pub struct CValidateFundOrderOptions {
    /// Fund symbol
    pub symbol: *const c_char,
    /// Action (buy/sell)
    pub action: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Amount (for amount-based orders) (can be null)
    pub amount: *const c_char,
    /// Units (for unit-based orders) (can be null)
    pub units: *const c_char,
    /// Dividend option (can be null)
    pub dividend_option: *const i32,
    /// Fund source (can be null)
    pub fund_source: *const i32,
    /// Account channel (can be null)
    pub account_channel: *const c_char,
}

/// Options for submitting a fund order.
#[derive(Debug)]
#[repr(C)]
pub struct CSubmitFundOrderOptions {
    /// Fund symbol
    pub symbol: *const c_char,
    /// Action (buy/sell)
    pub action: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Amount (for amount-based orders) (can be null)
    pub amount: *const c_char,
    /// Units (for unit-based orders) (can be null)
    pub units: *const c_char,
    /// Dividend option (can be null)
    pub dividend_option: *const i32,
    /// Fee (can be null)
    pub fee: *const c_char,
    /// Whether to sell all (can be null)
    pub is_sell_all: *const bool,
    /// Remark (can be null)
    pub remark: *const c_char,
    /// Trade method (can be null)
    pub trade_method: *const i32,
}
