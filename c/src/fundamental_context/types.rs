use std::os::raw::c_char;

use longbridge::fundamental::types::*;

use crate::{
    fundamental_context::enum_types::CInstitutionRecommend,
    types::{COption, CString, CVec, ToFFI},
};

// ── Helper macro for all-string structs ──────────────────────────
// Each string field: String → CString (owned), *const c_char (FFI)

// ── DividendList ──────────────────────────────────────────────────

/// A single dividend event for a security (C-facing FFI type).
#[repr(C)]
pub struct CDividendItem {
    /// Security symbol (e.g. `"700.HK"`).
    pub symbol: *const c_char,
    /// Unique identifier for the dividend event.
    pub id: *const c_char,
    /// Human-readable description of the dividend.
    pub desc: *const c_char,
    /// Record date ("YYYY-MM-DD").
    pub record_date: *const c_char,
    /// Ex-dividend date ("YYYY-MM-DD").
    pub ex_date: *const c_char,
    /// Payment date ("YYYY-MM-DD").
    pub payment_date: *const c_char,
}

pub(crate) struct CDividendItemOwned {
    symbol: CString,
    id: CString,
    desc: CString,
    record_date: CString,
    ex_date: CString,
    payment_date: CString,
}

impl From<DividendItem> for CDividendItemOwned {
    fn from(v: DividendItem) -> Self {
        Self {
            symbol: v.symbol.into(),
            id: v.id.into(),
            desc: v.desc.into(),
            record_date: v.record_date.into(),
            ex_date: v.ex_date.into(),
            payment_date: v.payment_date.into(),
        }
    }
}

impl ToFFI for CDividendItemOwned {
    type FFIType = CDividendItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDividendItem {
            symbol: self.symbol.to_ffi_type(),
            id: self.id.to_ffi_type(),
            desc: self.desc.to_ffi_type(),
            record_date: self.record_date.to_ffi_type(),
            ex_date: self.ex_date.to_ffi_type(),
            payment_date: self.payment_date.to_ffi_type(),
        }
    }
}

/// List of dividend items for a security (C-facing FFI type).
#[repr(C)]
pub struct CDividendList {
    /// Pointer to the array of dividend items.
    pub list: *const CDividendItem,
    /// Number of items in the array.
    pub num_list: usize,
}

pub(crate) struct CDividendListOwned {
    list: CVec<CDividendItemOwned>,
}

impl From<DividendList> for CDividendListOwned {
    fn from(v: DividendList) -> Self {
        Self {
            list: v.list.into(),
        }
    }
}

impl ToFFI for CDividendListOwned {
    type FFIType = CDividendList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CDividendList {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
        }
    }
}

// ── InstitutionRating ─────────────────────────────────────────────

/// Aggregated institutional rating opinion counts over a date range (C-facing
/// FFI type).
#[repr(C)]
pub struct CRatingEvaluate {
    /// Number of "buy" ratings.
    pub buy: i32,
    /// Number of "outperform" ratings.
    pub over: i32,
    /// Number of "hold" ratings.
    pub hold: i32,
    /// Number of "underperform" ratings.
    pub under: i32,
    /// Number of "sell" ratings.
    pub sell: i32,
    /// Number of "no opinion" ratings.
    pub no_opinion: i32,
    /// Total number of ratings.
    pub total: i32,
    /// Start date of the evaluation period ("YYYY-MM-DD").
    pub start_date: *const c_char,
    /// End date of the evaluation period ("YYYY-MM-DD").
    pub end_date: *const c_char,
}

pub(crate) struct CRatingEvaluateOwned {
    buy: i32,
    over: i32,
    hold: i32,
    under: i32,
    sell: i32,
    no_opinion: i32,
    total: i32,
    start_date: CString,
    end_date: CString,
}

impl From<RatingEvaluate> for CRatingEvaluateOwned {
    fn from(v: RatingEvaluate) -> Self {
        Self {
            buy: v.buy,
            over: v.over,
            hold: v.hold,
            under: v.under,
            sell: v.sell,
            no_opinion: v.no_opinion,
            total: v.total,
            start_date: v.start_date.into(),
            end_date: v.end_date.into(),
        }
    }
}

impl ToFFI for CRatingEvaluateOwned {
    type FFIType = CRatingEvaluate;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingEvaluate {
            buy: self.buy,
            over: self.over,
            hold: self.hold,
            under: self.under,
            sell: self.sell,
            no_opinion: self.no_opinion,
            total: self.total,
            start_date: self.start_date.to_ffi_type(),
            end_date: self.end_date.to_ffi_type(),
        }
    }
}

/// Institutional price-target range over a date period (C-facing FFI type).
#[repr(C)]
pub struct CRatingTarget {
    /// Highest analyst price target in the period.
    pub highest_price: *const c_char,
    /// Lowest analyst price target in the period.
    pub lowest_price: *const c_char,
    /// Previous closing price at the start of the period.
    pub prev_close: *const c_char,
    /// Start date of the target period ("YYYY-MM-DD").
    pub start_date: *const c_char,
    /// End date of the target period ("YYYY-MM-DD").
    pub end_date: *const c_char,
}

pub(crate) struct CRatingTargetOwned {
    highest_price: CString,
    lowest_price: CString,
    prev_close: CString,
    start_date: CString,
    end_date: CString,
}

impl From<RatingTarget> for CRatingTargetOwned {
    fn from(v: RatingTarget) -> Self {
        Self {
            highest_price: v
                .highest_price
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            lowest_price: v
                .lowest_price
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            prev_close: v
                .prev_close
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            start_date: v.start_date.into(),
            end_date: v.end_date.into(),
        }
    }
}

impl ToFFI for CRatingTargetOwned {
    type FFIType = CRatingTarget;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingTarget {
            highest_price: self.highest_price.to_ffi_type(),
            lowest_price: self.lowest_price.to_ffi_type(),
            prev_close: self.prev_close.to_ffi_type(),
            start_date: self.start_date.to_ffi_type(),
            end_date: self.end_date.to_ffi_type(),
        }
    }
}

/// Summary of rating opinion counts on a specific date (C-facing FFI type).
#[repr(C)]
pub struct CRatingSummaryEvaluate {
    /// Number of "buy" ratings.
    pub buy: i32,
    /// Date of the rating summary ("YYYY-MM-DD").
    pub date: *const c_char,
    /// Number of "hold" ratings.
    pub hold: i32,
    /// Number of "sell" ratings.
    pub sell: i32,
    /// Number of "strong buy" ratings.
    pub strong_buy: i32,
    /// Number of "underperform" ratings.
    pub under: i32,
}

pub(crate) struct CRatingSummaryEvaluateOwned {
    buy: i32,
    date: CString,
    hold: i32,
    sell: i32,
    strong_buy: i32,
    under: i32,
}

impl From<RatingSummaryEvaluate> for CRatingSummaryEvaluateOwned {
    fn from(v: RatingSummaryEvaluate) -> Self {
        Self {
            buy: v.buy,
            date: v.date.into(),
            hold: v.hold,
            sell: v.sell,
            strong_buy: v.strong_buy,
            under: v.under,
        }
    }
}

impl ToFFI for CRatingSummaryEvaluateOwned {
    type FFIType = CRatingSummaryEvaluate;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingSummaryEvaluate {
            buy: self.buy,
            date: self.date.to_ffi_type(),
            hold: self.hold,
            sell: self.sell,
            strong_buy: self.strong_buy,
            under: self.under,
        }
    }
}

/// Latest institutional rating data including evaluate counts, price targets,
/// and industry context (C-facing FFI type).
#[repr(C)]
pub struct CInstitutionRatingLatest {
    /// Aggregated opinion counts for the current period.
    pub evaluate: CRatingEvaluate,
    /// Consensus price target range for the current period.
    pub target: CRatingTarget,
    /// Industry identifier.
    pub industry_id: i64,
    /// Industry name.
    pub industry_name: *const c_char,
    /// Rank of the security within its industry by rating.
    pub industry_rank: i32,
    /// Total number of securities in the industry.
    pub industry_total: i32,
    /// Mean rating score for the industry.
    pub industry_mean: i32,
    /// Median rating score for the industry.
    pub industry_median: i32,
}

pub(crate) struct CInstitutionRatingLatestOwned {
    evaluate: CRatingEvaluateOwned,
    target: CRatingTargetOwned,
    industry_id: i64,
    industry_name: CString,
    industry_rank: i32,
    industry_total: i32,
    industry_mean: i32,
    industry_median: i32,
}

impl From<InstitutionRatingLatest> for CInstitutionRatingLatestOwned {
    fn from(v: InstitutionRatingLatest) -> Self {
        Self {
            evaluate: v.evaluate.into(),
            target: v.target.into(),
            industry_id: v.industry_id,
            industry_name: v.industry_name.into(),
            industry_rank: v.industry_rank,
            industry_total: v.industry_total,
            industry_mean: v.industry_mean,
            industry_median: v.industry_median,
        }
    }
}

impl ToFFI for CInstitutionRatingLatestOwned {
    type FFIType = CInstitutionRatingLatest;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInstitutionRatingLatest {
            evaluate: self.evaluate.to_ffi_type(),
            target: self.target.to_ffi_type(),
            industry_id: self.industry_id,
            industry_name: self.industry_name.to_ffi_type(),
            industry_rank: self.industry_rank,
            industry_total: self.industry_total,
            industry_mean: self.industry_mean,
            industry_median: self.industry_median,
        }
    }
}

/// Summary of the latest institutional rating for a security (C-facing FFI
/// type).
#[repr(C)]
pub struct CInstitutionRatingSummary {
    /// Currency symbol used for price targets (e.g. `"HKD"`).
    pub ccy_symbol: *const c_char,
    /// Price change since the previous rating cycle.
    pub change: *const c_char,
    /// Aggregated opinion counts on the summary date.
    pub evaluate: CRatingSummaryEvaluate,
    /// Consensus recommendation.
    pub recommend: CInstitutionRecommend,
    /// Consensus price target value.
    pub target: *const c_char,
    /// Timestamp of the last update.
    pub updated_at: *const c_char,
}

pub(crate) struct CInstitutionRatingSummaryOwned {
    ccy_symbol: CString,
    change: CString,
    evaluate: CRatingSummaryEvaluateOwned,
    recommend: CInstitutionRecommend,
    target: CString,
    updated_at: CString,
}

impl From<InstitutionRatingSummary> for CInstitutionRatingSummaryOwned {
    fn from(v: InstitutionRatingSummary) -> Self {
        Self {
            ccy_symbol: v.ccy_symbol.into(),
            change: v.change.map(|d| d.to_string()).unwrap_or_default().into(),
            evaluate: v.evaluate.into(),
            recommend: v.recommend.into(),
            target: v.target.map(|d| d.to_string()).unwrap_or_default().into(),
            updated_at: v.updated_at.into(),
        }
    }
}

impl ToFFI for CInstitutionRatingSummaryOwned {
    type FFIType = CInstitutionRatingSummary;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInstitutionRatingSummary {
            ccy_symbol: self.ccy_symbol.to_ffi_type(),
            change: self.change.to_ffi_type(),
            evaluate: self.evaluate.to_ffi_type(),
            recommend: self.recommend,
            target: self.target.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
        }
    }
}

/// Full institutional rating for a security, combining latest details and a
/// summary (C-facing FFI type).
#[repr(C)]
pub struct CInstitutionRating {
    /// Most recent detailed rating data.
    pub latest: CInstitutionRatingLatest,
    /// High-level summary of the rating.
    pub summary: CInstitutionRatingSummary,
}

pub(crate) struct CInstitutionRatingOwned {
    latest: CInstitutionRatingLatestOwned,
    summary: CInstitutionRatingSummaryOwned,
}

impl From<InstitutionRating> for CInstitutionRatingOwned {
    fn from(v: InstitutionRating) -> Self {
        Self {
            latest: v.latest.into(),
            summary: v.summary.into(),
        }
    }
}

impl ToFFI for CInstitutionRatingOwned {
    type FFIType = CInstitutionRating;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInstitutionRating {
            latest: self.latest.to_ffi_type(),
            summary: self.summary.to_ffi_type(),
        }
    }
}

// ── InstitutionRatingDetail ───────────────────────────────────────

/// A single data point in the historical evaluate series for institution rating
/// detail (C-facing FFI type).
#[repr(C)]
pub struct CInstitutionRatingDetailEvaluateItem {
    /// Number of "buy" ratings on this date.
    pub buy: i32,
    /// Date of this evaluate snapshot ("YYYY-MM-DD").
    pub date: *const c_char,
    /// Number of "hold" ratings on this date.
    pub hold: i32,
    /// Number of "sell" ratings on this date.
    pub sell: i32,
    /// Number of "strong buy" / "outperform" ratings on this date.
    pub strong_buy: i32,
    /// Number of "no opinion" ratings on this date.
    pub no_opinion: i32,
    /// Number of "underperform" ratings on this date.
    pub under: i32,
}

pub(crate) struct CInstitutionRatingDetailEvaluateItemOwned {
    buy: i32,
    date: CString,
    hold: i32,
    sell: i32,
    strong_buy: i32,
    no_opinion: i32,
    under: i32,
}

impl From<InstitutionRatingDetailEvaluateItem> for CInstitutionRatingDetailEvaluateItemOwned {
    fn from(v: InstitutionRatingDetailEvaluateItem) -> Self {
        Self {
            buy: v.buy,
            date: v.date.into(),
            hold: v.hold,
            sell: v.sell,
            strong_buy: v.strong_buy,
            no_opinion: v.no_opinion,
            under: v.under,
        }
    }
}

impl ToFFI for CInstitutionRatingDetailEvaluateItemOwned {
    type FFIType = CInstitutionRatingDetailEvaluateItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInstitutionRatingDetailEvaluateItem {
            buy: self.buy,
            date: self.date.to_ffi_type(),
            hold: self.hold,
            sell: self.sell,
            strong_buy: self.strong_buy,
            no_opinion: self.no_opinion,
            under: self.under,
        }
    }
}

/// A single data point in the historical price-target series for institution
/// rating detail (C-facing FFI type).
#[repr(C)]
pub struct CInstitutionRatingDetailTargetItem {
    /// Average analyst price target on this date.
    pub avg_target: *const c_char,
    /// Date of this target snapshot ("YYYY-MM-DD").
    pub date: *const c_char,
    /// Maximum analyst price target on this date.
    pub max_target: *const c_char,
    /// Minimum analyst price target on this date.
    pub min_target: *const c_char,
    /// Whether the price target was met.
    pub meet: bool,
    /// Actual price on this date.
    pub price: *const c_char,
    /// Unix timestamp of this data point.
    pub timestamp: *const c_char,
}

pub(crate) struct CInstitutionRatingDetailTargetItemOwned {
    avg_target: CString,
    date: CString,
    max_target: CString,
    min_target: CString,
    meet: bool,
    price: CString,
    timestamp: CString,
}

impl From<InstitutionRatingDetailTargetItem> for CInstitutionRatingDetailTargetItemOwned {
    fn from(v: InstitutionRatingDetailTargetItem) -> Self {
        Self {
            avg_target: v
                .avg_target
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            date: v.date.into(),
            max_target: v
                .max_target
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            min_target: v
                .min_target
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            meet: v.meet,
            price: v.price.map(|d| d.to_string()).unwrap_or_default().into(),
            timestamp: v.timestamp.into(),
        }
    }
}

impl ToFFI for CInstitutionRatingDetailTargetItemOwned {
    type FFIType = CInstitutionRatingDetailTargetItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInstitutionRatingDetailTargetItem {
            avg_target: self.avg_target.to_ffi_type(),
            date: self.date.to_ffi_type(),
            max_target: self.max_target.to_ffi_type(),
            min_target: self.min_target.to_ffi_type(),
            meet: self.meet,
            price: self.price.to_ffi_type(),
            timestamp: self.timestamp.to_ffi_type(),
        }
    }
}

/// Detailed historical institution rating data including evaluate and
/// price-target series (C-facing FFI type).
#[repr(C)]
pub struct CInstitutionRatingDetail {
    /// Currency symbol used for price targets (e.g. `"HKD"`).
    pub ccy_symbol: *const c_char,
    /// Pointer to the array of historical evaluate snapshots.
    pub evaluate_list: *const CInstitutionRatingDetailEvaluateItem,
    /// Number of items in `evaluate_list`.
    pub num_evaluate_list: usize,
    /// Percentage of price targets that were met (as a string).
    pub data_percent: *const c_char,
    /// Prediction accuracy rate for price targets (as a string).
    pub prediction_accuracy: *const c_char,
    /// Timestamp of the last update.
    pub updated_at: *const c_char,
    /// Pointer to the array of historical price-target snapshots.
    pub target_list: *const CInstitutionRatingDetailTargetItem,
    /// Number of items in `target_list`.
    pub num_target_list: usize,
}

pub(crate) struct CInstitutionRatingDetailOwned {
    ccy_symbol: CString,
    evaluate_list: CVec<CInstitutionRatingDetailEvaluateItemOwned>,
    data_percent: CString,
    prediction_accuracy: CString,
    updated_at: CString,
    target_list: CVec<CInstitutionRatingDetailTargetItemOwned>,
}

impl From<InstitutionRatingDetail> for CInstitutionRatingDetailOwned {
    fn from(v: InstitutionRatingDetail) -> Self {
        Self {
            ccy_symbol: v.ccy_symbol.into(),
            evaluate_list: v.evaluate.list.into(),
            data_percent: v
                .target
                .data_percent
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            prediction_accuracy: v
                .target
                .prediction_accuracy
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            updated_at: v.target.updated_at.into(),
            target_list: v.target.list.into(),
        }
    }
}

impl ToFFI for CInstitutionRatingDetailOwned {
    type FFIType = CInstitutionRatingDetail;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInstitutionRatingDetail {
            ccy_symbol: self.ccy_symbol.to_ffi_type(),
            evaluate_list: self.evaluate_list.to_ffi_type(),
            num_evaluate_list: self.evaluate_list.len(),
            data_percent: self.data_percent.to_ffi_type(),
            prediction_accuracy: self.prediction_accuracy.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
            target_list: self.target_list.to_ffi_type(),
            num_target_list: self.target_list.len(),
        }
    }
}

// ── ForecastEps ───────────────────────────────────────────────────

/// A single EPS forecast item covering a fiscal period (C-facing FFI type).
#[repr(C)]
pub struct CForecastEpsItem {
    /// Median EPS forecast across all contributing institutions.
    pub forecast_eps_median: *const c_char,
    /// Mean EPS forecast across all contributing institutions.
    pub forecast_eps_mean: *const c_char,
    /// Lowest individual EPS forecast.
    pub forecast_eps_lowest: *const c_char,
    /// Highest individual EPS forecast.
    pub forecast_eps_highest: *const c_char,
    /// Total number of institutions providing an EPS forecast.
    pub institution_total: i32,
    /// Number of institutions that revised their forecast upward.
    pub institution_up: i32,
    /// Number of institutions that revised their forecast downward.
    pub institution_down: i32,
    /// Unix timestamp of the forecast period start date.
    pub forecast_start_date: i64,
    /// Unix timestamp of the forecast period end date.
    pub forecast_end_date: i64,
}

pub(crate) struct CForecastEpsItemOwned {
    forecast_eps_median: CString,
    forecast_eps_mean: CString,
    forecast_eps_lowest: CString,
    forecast_eps_highest: CString,
    institution_total: i32,
    institution_up: i32,
    institution_down: i32,
    forecast_start_date: i64,
    forecast_end_date: i64,
}

impl From<ForecastEpsItem> for CForecastEpsItemOwned {
    fn from(v: ForecastEpsItem) -> Self {
        Self {
            forecast_eps_median: v
                .forecast_eps_median
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            forecast_eps_mean: v
                .forecast_eps_mean
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            forecast_eps_lowest: v
                .forecast_eps_lowest
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            forecast_eps_highest: v
                .forecast_eps_highest
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            institution_total: v.institution_total,
            institution_up: v.institution_up,
            institution_down: v.institution_down,
            forecast_start_date: v.forecast_start_date.unix_timestamp(),
            forecast_end_date: v.forecast_end_date.unix_timestamp(),
        }
    }
}

impl ToFFI for CForecastEpsItemOwned {
    type FFIType = CForecastEpsItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CForecastEpsItem {
            forecast_eps_median: self.forecast_eps_median.to_ffi_type(),
            forecast_eps_mean: self.forecast_eps_mean.to_ffi_type(),
            forecast_eps_lowest: self.forecast_eps_lowest.to_ffi_type(),
            forecast_eps_highest: self.forecast_eps_highest.to_ffi_type(),
            institution_total: self.institution_total,
            institution_up: self.institution_up,
            institution_down: self.institution_down,
            forecast_start_date: self.forecast_start_date,
            forecast_end_date: self.forecast_end_date,
        }
    }
}

/// Collection of EPS forecast items (C-facing FFI type).
#[repr(C)]
pub struct CForecastEps {
    /// Pointer to the array of EPS forecast items.
    pub items: *const CForecastEpsItem,
    /// Number of items in the array.
    pub num_items: usize,
}

pub(crate) struct CForecastEpsOwned {
    items: CVec<CForecastEpsItemOwned>,
}

impl From<ForecastEps> for CForecastEpsOwned {
    fn from(v: ForecastEps) -> Self {
        Self {
            items: v.items.into(),
        }
    }
}

impl ToFFI for CForecastEpsOwned {
    type FFIType = CForecastEps;
    fn to_ffi_type(&self) -> Self::FFIType {
        CForecastEps {
            items: self.items.to_ffi_type(),
            num_items: self.items.len(),
        }
    }
}

// ── ValuationPoint / ValuationMetricData ──────────────────────────

/// A single (timestamp, value) data point in a valuation time series (C-facing
/// FFI type).
#[repr(C)]
pub struct CValuationPoint {
    /// Unix timestamp of the data point.
    pub timestamp: i64,
    /// Valuation metric value at this timestamp (as a decimal string).
    pub value: *const c_char,
}

pub(crate) struct CValuationPointOwned {
    timestamp: i64,
    value: CString,
}

impl From<ValuationPoint> for CValuationPointOwned {
    fn from(v: ValuationPoint) -> Self {
        Self {
            timestamp: v.timestamp.unix_timestamp(),
            value: v.value.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CValuationPointOwned {
    type FFIType = CValuationPoint;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationPoint {
            timestamp: self.timestamp,
            value: self.value.to_ffi_type(),
        }
    }
}

/// Historical data for a single valuation metric (e.g. PE, PB) including
/// summary statistics (C-facing FFI type).
#[repr(C)]
pub struct CValuationMetricData {
    /// Description or label of the valuation metric.
    pub desc: *const c_char,
    /// Highest value of the metric over the series.
    pub high: *const c_char,
    /// Lowest value of the metric over the series.
    pub low: *const c_char,
    /// Median value of the metric over the series.
    pub median: *const c_char,
    /// Pointer to the array of time-series data points.
    pub list: *const CValuationPoint,
    /// Number of data points in `list`.
    pub num_list: usize,
}

pub(crate) struct CValuationMetricDataOwned {
    desc: CString,
    high: CString,
    low: CString,
    median: CString,
    list: CVec<CValuationPointOwned>,
}

impl From<ValuationMetricData> for CValuationMetricDataOwned {
    fn from(v: ValuationMetricData) -> Self {
        Self {
            desc: v.desc.into(),
            high: v.high.map(|d| d.to_string()).unwrap_or_default().into(),
            low: v.low.map(|d| d.to_string()).unwrap_or_default().into(),
            median: v.median.map(|d| d.to_string()).unwrap_or_default().into(),
            list: v.list.into(),
        }
    }
}

impl ToFFI for CValuationMetricDataOwned {
    type FFIType = CValuationMetricData;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationMetricData {
            desc: self.desc.to_ffi_type(),
            high: self.high.to_ffi_type(),
            low: self.low.to_ffi_type(),
            median: self.median.to_ffi_type(),
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
        }
    }
}

// Use same type for history metrics
pub type CValuationHistoryMetric = CValuationMetricData;
pub(crate) type CValuationHistoryMetricOwned = CValuationMetricDataOwned;

impl From<ValuationHistoryMetric> for CValuationHistoryMetricOwned {
    fn from(v: ValuationHistoryMetric) -> Self {
        Self {
            desc: v.desc.into(),
            high: v.high.map(|d| d.to_string()).unwrap_or_default().into(),
            low: v.low.map(|d| d.to_string()).unwrap_or_default().into(),
            median: v.median.map(|d| d.to_string()).unwrap_or_default().into(),
            list: v.list.into(),
        }
    }
}

/// Set of valuation metric data series for a security (C-facing FFI type).
#[repr(C)]
pub struct CValuationMetricsData {
    /// Price-to-earnings ratio series, or null if unavailable.
    pub pe: *const CValuationMetricData,
    /// Price-to-book ratio series, or null if unavailable.
    pub pb: *const CValuationMetricData,
    /// Price-to-sales ratio series, or null if unavailable.
    pub ps: *const CValuationMetricData,
    /// Dividend yield series, or null if unavailable.
    pub dvd_yld: *const CValuationMetricData,
}

pub(crate) struct CValuationMetricsDataOwned {
    pe: COption<CValuationMetricDataOwned>,
    pb: COption<CValuationMetricDataOwned>,
    ps: COption<CValuationMetricDataOwned>,
    dvd_yld: COption<CValuationMetricDataOwned>,
}

impl From<ValuationMetricsData> for CValuationMetricsDataOwned {
    fn from(v: ValuationMetricsData) -> Self {
        Self {
            pe: v.pe.into(),
            pb: v.pb.into(),
            ps: v.ps.into(),
            dvd_yld: v.dvd_yld.into(),
        }
    }
}

impl ToFFI for CValuationMetricsDataOwned {
    type FFIType = CValuationMetricsData;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationMetricsData {
            pe: self.pe.to_ffi_type(),
            pb: self.pb.to_ffi_type(),
            ps: self.ps.to_ffi_type(),
            dvd_yld: self.dvd_yld.to_ffi_type(),
        }
    }
}

/// Valuation data container holding all metric series for a security (C-facing
/// FFI type).
#[repr(C)]
pub struct CValuationData {
    /// The set of valuation metric data series (PE, PB, PS, dividend yield).
    pub metrics: CValuationMetricsData,
}

pub(crate) struct CValuationDataOwned {
    metrics: CValuationMetricsDataOwned,
}

impl From<ValuationData> for CValuationDataOwned {
    fn from(v: ValuationData) -> Self {
        Self {
            metrics: v.metrics.into(),
        }
    }
}

impl ToFFI for CValuationDataOwned {
    type FFIType = CValuationData;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationData {
            metrics: self.metrics.to_ffi_type(),
        }
    }
}

/// Set of historical valuation metric series (PE, PB, PS) for a security
/// (C-facing FFI type).
#[repr(C)]
pub struct CValuationHistoryMetrics {
    /// Historical price-to-earnings ratio series, or null if unavailable.
    pub pe: *const CValuationHistoryMetric,
    /// Historical price-to-book ratio series, or null if unavailable.
    pub pb: *const CValuationHistoryMetric,
    /// Historical price-to-sales ratio series, or null if unavailable.
    pub ps: *const CValuationHistoryMetric,
}

pub(crate) struct CValuationHistoryMetricsOwned {
    pe: COption<CValuationHistoryMetricOwned>,
    pb: COption<CValuationHistoryMetricOwned>,
    ps: COption<CValuationHistoryMetricOwned>,
}

impl From<ValuationHistoryMetrics> for CValuationHistoryMetricsOwned {
    fn from(v: ValuationHistoryMetrics) -> Self {
        Self {
            pe: v.pe.into(),
            pb: v.pb.into(),
            ps: v.ps.into(),
        }
    }
}

impl ToFFI for CValuationHistoryMetricsOwned {
    type FFIType = CValuationHistoryMetrics;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationHistoryMetrics {
            pe: self.pe.to_ffi_type(),
            pb: self.pb.to_ffi_type(),
            ps: self.ps.to_ffi_type(),
        }
    }
}

/// Response containing historical valuation metric series (C-facing FFI type).
#[repr(C)]
pub struct CValuationHistoryResponse {
    /// Historical price-to-earnings ratio series, or null if unavailable.
    pub pe: *const CValuationHistoryMetric,
    /// Historical price-to-book ratio series, or null if unavailable.
    pub pb: *const CValuationHistoryMetric,
    /// Historical price-to-sales ratio series, or null if unavailable.
    pub ps: *const CValuationHistoryMetric,
}

pub(crate) struct CValuationHistoryResponseOwned {
    metrics: CValuationHistoryMetricsOwned,
}

impl From<ValuationHistoryResponse> for CValuationHistoryResponseOwned {
    fn from(v: ValuationHistoryResponse) -> Self {
        Self {
            metrics: v.history.metrics.into(),
        }
    }
}

impl ToFFI for CValuationHistoryResponseOwned {
    type FFIType = CValuationHistoryResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        let m = self.metrics.to_ffi_type();
        CValuationHistoryResponse {
            pe: m.pe,
            pb: m.pb,
            ps: m.ps,
        }
    }
}

// ── CompanyOverview ───────────────────────────────────────────────

/// High-level company profile and metadata (C-facing FFI type).
#[repr(C)]
pub struct CCompanyOverview {
    /// Short display name of the company.
    pub name: *const c_char,
    /// Full legal company name.
    pub company_name: *const c_char,
    /// Year the company was founded.
    pub founded: *const c_char,
    /// Stock listing date ("YYYY-MM-DD").
    pub listing_date: *const c_char,
    /// Exchange or market where the stock is listed.
    pub market: *const c_char,
    /// Geographic region of the company's primary operations.
    pub region: *const c_char,
    /// Registered address of the company.
    pub address: *const c_char,
    /// Principal office address.
    pub office_address: *const c_char,
    /// Company website URL.
    pub website: *const c_char,
    /// IPO issue price.
    pub issue_price: *const c_char,
    /// Number of shares offered at IPO.
    pub shares_offered: *const c_char,
    /// Name of the board chairman.
    pub chairman: *const c_char,
    /// Name of the company secretary.
    pub secretary: *const c_char,
    /// Name of the auditing institution.
    pub audit_inst: *const c_char,
    /// Business category or industry classification label.
    pub category: *const c_char,
    /// Fiscal year-end date (e.g. `"12/31"`).
    pub year_end: *const c_char,
    /// Number of employees (as a string).
    pub employees: *const c_char,
    /// Corporate phone number.
    pub phone: *const c_char,
    /// Corporate fax number.
    pub fax: *const c_char,
    /// Corporate email address.
    pub email: *const c_char,
    /// Legal representative of the company.
    pub legal_repr: *const c_char,
    /// General manager or CEO name.
    pub manager: *const c_char,
    /// Stock ticker symbol.
    pub ticker: *const c_char,
    /// Business description / company profile text.
    pub profile: *const c_char,
    /// Numeric sector code.
    pub sector: i32,
}

pub(crate) struct CCompanyOverviewOwned {
    name: CString,
    company_name: CString,
    founded: CString,
    listing_date: CString,
    market: CString,
    region: CString,
    address: CString,
    office_address: CString,
    website: CString,
    issue_price: CString,
    shares_offered: CString,
    chairman: CString,
    secretary: CString,
    audit_inst: CString,
    category: CString,
    year_end: CString,
    employees: CString,
    phone: CString,
    fax: CString,
    email: CString,
    legal_repr: CString,
    manager: CString,
    ticker: CString,
    profile: CString,
    sector: i32,
}

impl From<CompanyOverview> for CCompanyOverviewOwned {
    fn from(v: CompanyOverview) -> Self {
        Self {
            name: v.name.into(),
            company_name: v.company_name.into(),
            founded: v.founded.into(),
            listing_date: v.listing_date.into(),
            market: v.market.into(),
            region: v.region.into(),
            address: v.address.into(),
            office_address: v.office_address.into(),
            website: v.website.into(),
            issue_price: v
                .issue_price
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            shares_offered: v.shares_offered.into(),
            chairman: v.chairman.into(),
            secretary: v.secretary.into(),
            audit_inst: v.audit_inst.into(),
            category: v.category.into(),
            year_end: v.year_end.into(),
            employees: v.employees.into(),
            phone: v.phone.into(),
            fax: v.fax.into(),
            email: v.email.into(),
            legal_repr: v.legal_repr.into(),
            manager: v.manager.into(),
            ticker: v.ticker.into(),
            profile: v.profile.into(),
            sector: v.sector,
        }
    }
}

impl ToFFI for CCompanyOverviewOwned {
    type FFIType = CCompanyOverview;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCompanyOverview {
            name: self.name.to_ffi_type(),
            company_name: self.company_name.to_ffi_type(),
            founded: self.founded.to_ffi_type(),
            listing_date: self.listing_date.to_ffi_type(),
            market: self.market.to_ffi_type(),
            region: self.region.to_ffi_type(),
            address: self.address.to_ffi_type(),
            office_address: self.office_address.to_ffi_type(),
            website: self.website.to_ffi_type(),
            issue_price: self.issue_price.to_ffi_type(),
            shares_offered: self.shares_offered.to_ffi_type(),
            chairman: self.chairman.to_ffi_type(),
            secretary: self.secretary.to_ffi_type(),
            audit_inst: self.audit_inst.to_ffi_type(),
            category: self.category.to_ffi_type(),
            year_end: self.year_end.to_ffi_type(),
            employees: self.employees.to_ffi_type(),
            phone: self.phone.to_ffi_type(),
            fax: self.fax.to_ffi_type(),
            email: self.email.to_ffi_type(),
            legal_repr: self.legal_repr.to_ffi_type(),
            manager: self.manager.to_ffi_type(),
            ticker: self.ticker.to_ffi_type(),
            profile: self.profile.to_ffi_type(),
            sector: self.sector,
        }
    }
}

// ── ShareholderList ───────────────────────────────────────────────

/// A stock position held by a shareholder (C-facing FFI type).
#[repr(C)]
pub struct CShareholderStock {
    /// Security symbol (e.g. `"700.HK"`).
    pub symbol: *const c_char,
    /// Stock code.
    pub code: *const c_char,
    /// Exchange or market of the stock.
    pub market: *const c_char,
    /// Change in the holding since the previous report.
    pub chg: *const c_char,
}

pub(crate) struct CShareholderStockOwned {
    symbol: CString,
    code: CString,
    market: CString,
    chg: CString,
}

impl From<ShareholderStock> for CShareholderStockOwned {
    fn from(v: ShareholderStock) -> Self {
        Self {
            symbol: v.symbol.into(),
            code: v.code.into(),
            market: v.market.into(),
            chg: v.chg.into(),
        }
    }
}

impl ToFFI for CShareholderStockOwned {
    type FFIType = CShareholderStock;
    fn to_ffi_type(&self) -> Self::FFIType {
        CShareholderStock {
            symbol: self.symbol.to_ffi_type(),
            code: self.code.to_ffi_type(),
            market: self.market.to_ffi_type(),
            chg: self.chg.to_ffi_type(),
        }
    }
}

/// A single institutional or major shareholder entry (C-facing FFI type).
#[repr(C)]
pub struct CShareholder {
    /// Unique identifier for the shareholder.
    pub shareholder_id: *const c_char,
    /// Display name of the shareholder.
    pub shareholder_name: *const c_char,
    /// Type of institution (e.g. fund, insurance company).
    pub institution_type: *const c_char,
    /// Percentage of total shares held.
    pub percent_of_shares: *const c_char,
    /// Change in shares held since the previous report.
    pub shares_changed: *const c_char,
    /// Date of the holdings report ("YYYY-MM-DD").
    pub report_date: *const c_char,
    /// Pointer to the array of stock positions held by this shareholder.
    pub stocks: *const CShareholderStock,
    /// Number of stock positions in `stocks`.
    pub num_stocks: usize,
}

pub(crate) struct CShareholderOwned {
    shareholder_id: CString,
    shareholder_name: CString,
    institution_type: CString,
    percent_of_shares: CString,
    shares_changed: CString,
    report_date: CString,
    stocks: CVec<CShareholderStockOwned>,
}

impl From<Shareholder> for CShareholderOwned {
    fn from(v: Shareholder) -> Self {
        Self {
            shareholder_id: v.shareholder_id.into(),
            shareholder_name: v.shareholder_name.into(),
            institution_type: v.institution_type.into(),
            percent_of_shares: v
                .percent_of_shares
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            shares_changed: v
                .shares_changed
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            report_date: v.report_date.into(),
            stocks: v.stocks.into(),
        }
    }
}

impl ToFFI for CShareholderOwned {
    type FFIType = CShareholder;
    fn to_ffi_type(&self) -> Self::FFIType {
        CShareholder {
            shareholder_id: self.shareholder_id.to_ffi_type(),
            shareholder_name: self.shareholder_name.to_ffi_type(),
            institution_type: self.institution_type.to_ffi_type(),
            percent_of_shares: self.percent_of_shares.to_ffi_type(),
            shares_changed: self.shares_changed.to_ffi_type(),
            report_date: self.report_date.to_ffi_type(),
            stocks: self.stocks.to_ffi_type(),
            num_stocks: self.stocks.len(),
        }
    }
}

/// Paginated list of shareholders for a security (C-facing FFI type).
#[repr(C)]
pub struct CShareholderList {
    /// Pointer to the array of shareholder entries.
    pub shareholder_list: *const CShareholder,
    /// Number of entries in `shareholder_list`.
    pub num_shareholder_list: usize,
    /// URL to fetch the next page of results, or empty if no next page.
    pub forward_url: *const c_char,
    /// Total number of shareholders across all pages.
    pub total: i32,
}

pub(crate) struct CShareholderListOwned {
    shareholder_list: CVec<CShareholderOwned>,
    forward_url: CString,
    total: i32,
}

impl From<ShareholderList> for CShareholderListOwned {
    fn from(v: ShareholderList) -> Self {
        Self {
            shareholder_list: v.shareholder_list.into(),
            forward_url: v.forward_url.into(),
            total: v.total,
        }
    }
}

impl ToFFI for CShareholderListOwned {
    type FFIType = CShareholderList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CShareholderList {
            shareholder_list: self.shareholder_list.to_ffi_type(),
            num_shareholder_list: self.shareholder_list.len(),
            forward_url: self.forward_url.to_ffi_type(),
            total: self.total,
        }
    }
}

// ── FundHolders ───────────────────────────────────────────────────

/// A single fund that holds a position in a security (C-facing FFI type).
#[repr(C)]
pub struct CFundHolder {
    /// Fund code.
    pub code: *const c_char,
    /// Security symbol held by the fund.
    pub symbol: *const c_char,
    /// Currency of the fund's reported holding value.
    pub currency: *const c_char,
    /// Fund name.
    pub name: *const c_char,
    /// Proportion of the fund's portfolio allocated to this position.
    pub position_ratio: *const c_char,
    /// Date of the holdings report ("YYYY-MM-DD").
    pub report_date: *const c_char,
}

pub(crate) struct CFundHolderOwned {
    code: CString,
    symbol: CString,
    currency: CString,
    name: CString,
    position_ratio: CString,
    report_date: CString,
}

impl From<FundHolder> for CFundHolderOwned {
    fn from(v: FundHolder) -> Self {
        Self {
            code: v.code.into(),
            symbol: v.symbol.into(),
            currency: v.currency.into(),
            name: v.name.into(),
            position_ratio: v.position_ratio.to_string().into(),
            report_date: v.report_date.into(),
        }
    }
}

impl ToFFI for CFundHolderOwned {
    type FFIType = CFundHolder;
    fn to_ffi_type(&self) -> Self::FFIType {
        CFundHolder {
            code: self.code.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            name: self.name.to_ffi_type(),
            position_ratio: self.position_ratio.to_ffi_type(),
            report_date: self.report_date.to_ffi_type(),
        }
    }
}

/// Collection of fund holders for a security (C-facing FFI type).
#[repr(C)]
pub struct CFundHolders {
    /// Pointer to the array of fund holder entries.
    pub lists: *const CFundHolder,
    /// Number of entries in `lists`.
    pub num_lists: usize,
}

pub(crate) struct CFundHoldersOwned {
    lists: CVec<CFundHolderOwned>,
}

impl From<FundHolders> for CFundHoldersOwned {
    fn from(v: FundHolders) -> Self {
        Self {
            lists: v.lists.into(),
        }
    }
}

impl ToFFI for CFundHoldersOwned {
    type FFIType = CFundHolders;
    fn to_ffi_type(&self) -> Self::FFIType {
        CFundHolders {
            lists: self.lists.to_ffi_type(),
            num_lists: self.lists.len(),
        }
    }
}

// ── CorpActions ───────────────────────────────────────────────────

/// A single corporate action event for a security (C-facing FFI type).
#[repr(C)]
pub struct CCorpActionItem {
    /// Unique identifier for the corporate action.
    pub id: *const c_char,
    /// Action date as a Unix timestamp string.
    pub date: *const c_char,
    /// Human-readable action date string.
    pub date_str: *const c_char,
    /// Type classification of the date (e.g. record date, ex-date).
    pub date_type: *const c_char,
    /// Time zone associated with the action date.
    pub date_zone: *const c_char,
    /// Type of corporate action (e.g. dividend, split).
    pub act_type: *const c_char,
    /// Human-readable description of the action type.
    pub act_desc: *const c_char,
    /// Action details or ratio string.
    pub action: *const c_char,
    /// Whether this action occurred recently.
    pub recent: bool,
    /// Whether announcement of this action was delayed.
    pub is_delay: bool,
    /// Additional content explaining any delay.
    pub delay_content: *const c_char,
}

pub(crate) struct CCorpActionItemOwned {
    id: CString,
    date: CString,
    date_str: CString,
    date_type: CString,
    date_zone: CString,
    act_type: CString,
    act_desc: CString,
    action: CString,
    recent: bool,
    is_delay: bool,
    delay_content: CString,
}

impl From<CorpActionItem> for CCorpActionItemOwned {
    fn from(v: CorpActionItem) -> Self {
        Self {
            id: v.id.into(),
            date: v.date.into(),
            date_str: v.date_str.into(),
            date_type: v.date_type.into(),
            date_zone: v.date_zone.into(),
            act_type: v.act_type.into(),
            act_desc: v.act_desc.into(),
            action: v.action.into(),
            recent: v.recent,
            is_delay: v.is_delay,
            delay_content: v.delay_content.into(),
        }
    }
}

impl ToFFI for CCorpActionItemOwned {
    type FFIType = CCorpActionItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCorpActionItem {
            id: self.id.to_ffi_type(),
            date: self.date.to_ffi_type(),
            date_str: self.date_str.to_ffi_type(),
            date_type: self.date_type.to_ffi_type(),
            date_zone: self.date_zone.to_ffi_type(),
            act_type: self.act_type.to_ffi_type(),
            act_desc: self.act_desc.to_ffi_type(),
            action: self.action.to_ffi_type(),
            recent: self.recent,
            is_delay: self.is_delay,
            delay_content: self.delay_content.to_ffi_type(),
        }
    }
}

/// Collection of corporate action events for a security (C-facing FFI type).
#[repr(C)]
pub struct CCorpActions {
    /// Pointer to the array of corporate action items.
    pub items: *const CCorpActionItem,
    /// Number of items in the array.
    pub num_items: usize,
}

pub(crate) struct CCorpActionsOwned {
    items: CVec<CCorpActionItemOwned>,
}

impl From<CorpActions> for CCorpActionsOwned {
    fn from(v: CorpActions) -> Self {
        Self {
            items: v.items.into(),
        }
    }
}

impl ToFFI for CCorpActionsOwned {
    type FFIType = CCorpActions;
    fn to_ffi_type(&self) -> Self::FFIType {
        CCorpActions {
            items: self.items.to_ffi_type(),
            num_items: self.items.len(),
        }
    }
}

// ── InvestRelations ───────────────────────────────────────────────

/// A security held by an institutional investor (C-facing FFI type).
#[repr(C)]
pub struct CInvestSecurity {
    /// Unique identifier for the investing company.
    pub company_id: *const c_char,
    /// Display name of the investing company.
    pub company_name: *const c_char,
    /// English name of the investing company.
    pub company_name_en: *const c_char,
    /// Simplified Chinese name of the investing company.
    pub company_name_zhcn: *const c_char,
    /// Security symbol held (e.g. `"700.HK"`).
    pub symbol: *const c_char,
    /// Currency of the holding value.
    pub currency: *const c_char,
    /// Percentage of total shares held.
    pub percent_of_shares: *const c_char,
    /// Ranking of the holding within the investor's portfolio.
    pub shares_rank: *const c_char,
    /// Market value of the holding.
    pub shares_value: *const c_char,
}

pub(crate) struct CInvestSecurityOwned {
    company_id: CString,
    company_name: CString,
    company_name_en: CString,
    company_name_zhcn: CString,
    symbol: CString,
    currency: CString,
    percent_of_shares: CString,
    shares_rank: CString,
    shares_value: CString,
}

impl From<InvestSecurity> for CInvestSecurityOwned {
    fn from(v: InvestSecurity) -> Self {
        Self {
            company_id: v.company_id.into(),
            company_name: v.company_name.into(),
            company_name_en: v.company_name_en.into(),
            company_name_zhcn: v.company_name_zhcn.into(),
            symbol: v.symbol.into(),
            currency: v.currency.into(),
            percent_of_shares: v
                .percent_of_shares
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            shares_rank: v.shares_rank.into(),
            shares_value: v
                .shares_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CInvestSecurityOwned {
    type FFIType = CInvestSecurity;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInvestSecurity {
            company_id: self.company_id.to_ffi_type(),
            company_name: self.company_name.to_ffi_type(),
            company_name_en: self.company_name_en.to_ffi_type(),
            company_name_zhcn: self.company_name_zhcn.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            percent_of_shares: self.percent_of_shares.to_ffi_type(),
            shares_rank: self.shares_rank.to_ffi_type(),
            shares_value: self.shares_value.to_ffi_type(),
        }
    }
}

/// Paginated list of investment relations for a security (C-facing FFI type).
#[repr(C)]
pub struct CInvestRelations {
    /// URL to fetch the next page of results, or empty if no next page.
    pub forward_url: *const c_char,
    /// Pointer to the array of invested securities.
    pub invest_securities: *const CInvestSecurity,
    /// Number of entries in `invest_securities`.
    pub num_invest_securities: usize,
}

pub(crate) struct CInvestRelationsOwned {
    forward_url: CString,
    invest_securities: CVec<CInvestSecurityOwned>,
}

impl From<InvestRelations> for CInvestRelationsOwned {
    fn from(v: InvestRelations) -> Self {
        Self {
            forward_url: v.forward_url.into(),
            invest_securities: v.invest_securities.into(),
        }
    }
}

impl ToFFI for CInvestRelationsOwned {
    type FFIType = CInvestRelations;
    fn to_ffi_type(&self) -> Self::FFIType {
        CInvestRelations {
            forward_url: self.forward_url.to_ffi_type(),
            invest_securities: self.invest_securities.to_ffi_type(),
            num_invest_securities: self.invest_securities.len(),
        }
    }
}

// ── OperatingList ─────────────────────────────────────────────────

/// A single operating/financial indicator within an operating report item
/// (C-facing FFI type).
#[repr(C)]
pub struct COperatingIndicator {
    /// Machine-readable field name for the indicator.
    pub field_name: *const c_char,
    /// Human-readable display name for the indicator.
    pub indicator_name: *const c_char,
    /// Value of the indicator (as a decimal string).
    pub indicator_value: *const c_char,
    /// Year-over-year change for the indicator.
    pub yoy: *const c_char,
}

pub(crate) struct COperatingIndicatorOwned {
    field_name: CString,
    indicator_name: CString,
    indicator_value: CString,
    yoy: CString,
}

impl From<OperatingIndicator> for COperatingIndicatorOwned {
    fn from(v: OperatingIndicator) -> Self {
        Self {
            field_name: v.field_name.into(),
            indicator_name: v.indicator_name.into(),
            indicator_value: v.indicator_value.into(),
            yoy: v.yoy.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for COperatingIndicatorOwned {
    type FFIType = COperatingIndicator;
    fn to_ffi_type(&self) -> Self::FFIType {
        COperatingIndicator {
            field_name: self.field_name.to_ffi_type(),
            indicator_name: self.indicator_name.to_ffi_type(),
            indicator_value: self.indicator_value.to_ffi_type(),
            yoy: self.yoy.to_ffi_type(),
        }
    }
}

/// A single operating report entry including associated financial indicators
/// (C-facing FFI type).
#[repr(C)]
pub struct COperatingItem {
    /// Unique identifier for the operating report item.
    pub id: *const c_char,
    /// Report period identifier (e.g. `"2024Q1"`).
    pub report: *const c_char,
    /// Title of the operating report.
    pub title: *const c_char,
    /// Plain-text content of the operating report.
    pub txt: *const c_char,
    /// Whether this is the most recent operating report.
    pub latest: bool,
    /// URL to the original web page for this report.
    pub web_url: *const c_char,
    /// Currency used in the financial data.
    pub financial_currency: *const c_char,
    /// Name of the financial reporting entity.
    pub financial_name: *const c_char,
    /// Region associated with the financial report.
    pub financial_region: *const c_char,
    /// Financial report period label.
    pub financial_report: *const c_char,
    /// Pointer to the array of operating indicators for this item.
    pub indicators: *const COperatingIndicator,
    /// Number of indicators in the `indicators` array.
    pub num_indicators: usize,
}

pub(crate) struct COperatingItemOwned {
    id: CString,
    report: CString,
    title: CString,
    txt: CString,
    latest: bool,
    web_url: CString,
    financial_currency: CString,
    financial_name: CString,
    financial_region: CString,
    financial_report: CString,
    indicators: CVec<COperatingIndicatorOwned>,
}

impl From<OperatingItem> for COperatingItemOwned {
    fn from(v: OperatingItem) -> Self {
        Self {
            id: v.id.into(),
            report: v.report.into(),
            title: v.title.into(),
            txt: v.txt.into(),
            latest: v.latest,
            web_url: v.web_url.into(),
            financial_currency: v.financial.currency.into(),
            financial_name: v.financial.name.into(),
            financial_region: v.financial.region.into(),
            financial_report: v.financial.report.into(),
            indicators: v.financial.indicators.into(),
        }
    }
}

impl ToFFI for COperatingItemOwned {
    type FFIType = COperatingItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        COperatingItem {
            id: self.id.to_ffi_type(),
            report: self.report.to_ffi_type(),
            title: self.title.to_ffi_type(),
            txt: self.txt.to_ffi_type(),
            latest: self.latest,
            web_url: self.web_url.to_ffi_type(),
            financial_currency: self.financial_currency.to_ffi_type(),
            financial_name: self.financial_name.to_ffi_type(),
            financial_region: self.financial_region.to_ffi_type(),
            financial_report: self.financial_report.to_ffi_type(),
            indicators: self.indicators.to_ffi_type(),
            num_indicators: self.indicators.len(),
        }
    }
}

/// Collection of operating report items for a security (C-facing FFI type).
#[repr(C)]
pub struct COperatingList {
    /// Pointer to the array of operating report items.
    pub list: *const COperatingItem,
    /// Number of items in the array.
    pub num_list: usize,
}

pub(crate) struct COperatingListOwned {
    list: CVec<COperatingItemOwned>,
}

impl From<OperatingList> for COperatingListOwned {
    fn from(v: OperatingList) -> Self {
        Self {
            list: v.list.into(),
        }
    }
}

impl ToFFI for COperatingListOwned {
    type FFIType = COperatingList;
    fn to_ffi_type(&self) -> Self::FFIType {
        COperatingList {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
        }
    }
}

// ── FinancialReports (serde_json::Value → JSON string) ────────────

/// Financial reports serialised as a JSON string (C-facing FFI type).
#[repr(C)]
pub struct CFinancialReports {
    /// JSON-encoded array of financial report entries.
    pub list_json: *const c_char,
}

pub(crate) struct CFinancialReportsOwned {
    list_json: CString,
}

impl From<longbridge::fundamental::FinancialReports> for CFinancialReportsOwned {
    fn from(v: longbridge::fundamental::FinancialReports) -> Self {
        Self {
            list_json: serde_json::to_string(&v.list).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CFinancialReportsOwned {
    type FFIType = CFinancialReports;
    fn to_ffi_type(&self) -> Self::FFIType {
        CFinancialReports {
            list_json: self.list_json.to_ffi_type(),
        }
    }
}

// ── FinancialConsensus ────────────────────────────────────────────

/// One consensus estimate detail for a financial metric.
#[repr(C)]
pub struct CConsensusDetail {
    /// Metric key, e.g. "revenue", "eps".
    pub key: *const c_char,
    /// Display name.
    pub name: *const c_char,
    /// Metric description.
    pub description: *const c_char,
    /// Actual reported value (empty string if not yet released).
    pub actual: *const c_char,
    /// Consensus estimate value.
    pub estimate: *const c_char,
    /// Actual minus estimate.
    pub comp_value: *const c_char,
    /// Beat/miss description.
    pub comp_desc: *const c_char,
    /// Comparison result code.
    pub comp: *const c_char,
    /// Whether actual results have been published.
    pub is_released: bool,
}

pub(crate) struct CConsensusDetailOwned {
    key: CString,
    name: CString,
    description: CString,
    actual: CString,
    estimate: CString,
    comp_value: CString,
    comp_desc: CString,
    comp: CString,
    is_released: bool,
}

impl From<longbridge::fundamental::ConsensusDetail> for CConsensusDetailOwned {
    fn from(v: longbridge::fundamental::ConsensusDetail) -> Self {
        Self {
            key: v.key.into(),
            name: v.name.into(),
            description: v.description.into(),
            actual: v.actual.map(|d| d.to_string()).unwrap_or_default().into(),
            estimate: v.estimate.map(|d| d.to_string()).unwrap_or_default().into(),
            comp_value: v
                .comp_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            comp_desc: v.comp_desc.into(),
            comp: v.comp.into(),
            is_released: v.is_released,
        }
    }
}

impl ToFFI for CConsensusDetailOwned {
    type FFIType = CConsensusDetail;
    fn to_ffi_type(&self) -> Self::FFIType {
        CConsensusDetail {
            key: self.key.to_ffi_type(),
            name: self.name.to_ffi_type(),
            description: self.description.to_ffi_type(),
            actual: self.actual.to_ffi_type(),
            estimate: self.estimate.to_ffi_type(),
            comp_value: self.comp_value.to_ffi_type(),
            comp_desc: self.comp_desc.to_ffi_type(),
            comp: self.comp.to_ffi_type(),
            is_released: self.is_released,
        }
    }
}

/// Consensus report for one fiscal period.
#[repr(C)]
pub struct CConsensusReport {
    /// Fiscal year, e.g. 2025.
    pub fiscal_year: i32,
    /// Fiscal period code, e.g. "Q4".
    pub fiscal_period: *const c_char,
    /// Human-readable period label, e.g. "Q4 FY2025".
    pub period_text: *const c_char,
    /// Pointer to the array of consensus detail items.
    pub details: *const CConsensusDetail,
    /// Number of items in `details`.
    pub num_details: usize,
}

pub(crate) struct CConsensusReportOwned {
    fiscal_year: i32,
    fiscal_period: CString,
    period_text: CString,
    details: CVec<CConsensusDetailOwned>,
}

impl From<longbridge::fundamental::ConsensusReport> for CConsensusReportOwned {
    fn from(v: longbridge::fundamental::ConsensusReport) -> Self {
        Self {
            fiscal_year: v.fiscal_year,
            fiscal_period: v.fiscal_period.into(),
            period_text: v.period_text.into(),
            details: v.details.into(),
        }
    }
}

impl ToFFI for CConsensusReportOwned {
    type FFIType = CConsensusReport;
    fn to_ffi_type(&self) -> Self::FFIType {
        CConsensusReport {
            fiscal_year: self.fiscal_year,
            fiscal_period: self.fiscal_period.to_ffi_type(),
            period_text: self.period_text.to_ffi_type(),
            details: self.details.to_ffi_type(),
            num_details: self.details.len(),
        }
    }
}

/// Financial consensus response.
#[repr(C)]
pub struct CFinancialConsensus {
    /// Pointer to the array of consensus reports.
    pub list: *const CConsensusReport,
    /// Number of reports in `list`.
    pub num_list: usize,
    /// Index of the most recently released period.
    pub current_index: i32,
    /// Reporting currency, e.g. "HKD".
    pub currency: *const c_char,
    /// Pointer to the array of available period type strings.
    pub opt_periods: *const *const c_char,
    /// Number of items in `opt_periods`.
    pub num_opt_periods: usize,
    /// Currently returned period type.
    pub current_period: *const c_char,
}

pub(crate) struct CFinancialConsensusOwned {
    list: CVec<CConsensusReportOwned>,
    current_index: i32,
    currency: CString,
    opt_periods: CVec<CString>,
    current_period: CString,
}

impl From<longbridge::fundamental::FinancialConsensus> for CFinancialConsensusOwned {
    fn from(v: longbridge::fundamental::FinancialConsensus) -> Self {
        Self {
            list: v.list.into(),
            current_index: v.current_index,
            currency: v.currency.into(),
            opt_periods: v
                .opt_periods
                .into_iter()
                .map(CString::from)
                .collect::<Vec<_>>()
                .into(),
            current_period: v.current_period.into(),
        }
    }
}

impl ToFFI for CFinancialConsensusOwned {
    type FFIType = CFinancialConsensus;
    fn to_ffi_type(&self) -> Self::FFIType {
        CFinancialConsensus {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
            current_index: self.current_index,
            currency: self.currency.to_ffi_type(),
            opt_periods: self.opt_periods.to_ffi_type(),
            num_opt_periods: self.opt_periods.len(),
            current_period: self.current_period.to_ffi_type(),
        }
    }
}

// ── IndustryValuation ─────────────────────────────────────────────

/// Historical valuation snapshot for an industry peer.
#[repr(C)]
pub struct CIndustryValuationHistory {
    /// Unix timestamp string.
    pub date: *const c_char,
    /// Price-to-Earnings ratio.
    pub pe: *const c_char,
    /// Price-to-Book ratio.
    pub pb: *const c_char,
    /// Price-to-Sales ratio.
    pub ps: *const c_char,
}

pub(crate) struct CIndustryValuationHistoryOwned {
    date: CString,
    pe: CString,
    pb: CString,
    ps: CString,
}

impl From<longbridge::fundamental::IndustryValuationHistory> for CIndustryValuationHistoryOwned {
    fn from(v: longbridge::fundamental::IndustryValuationHistory) -> Self {
        Self {
            date: v.date.into(),
            pe: v.pe.map(|d| d.to_string()).unwrap_or_default().into(),
            pb: v.pb.map(|d| d.to_string()).unwrap_or_default().into(),
            ps: v.ps.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CIndustryValuationHistoryOwned {
    type FFIType = CIndustryValuationHistory;
    fn to_ffi_type(&self) -> Self::FFIType {
        CIndustryValuationHistory {
            date: self.date.to_ffi_type(),
            pe: self.pe.to_ffi_type(),
            pb: self.pb.to_ffi_type(),
            ps: self.ps.to_ffi_type(),
        }
    }
}

/// Valuation data for one industry peer security.
#[repr(C)]
pub struct CIndustryValuationItem {
    /// Security symbol.
    pub symbol: *const c_char,
    /// Company name.
    pub name: *const c_char,
    /// Reporting currency.
    pub currency: *const c_char,
    /// Total assets.
    pub assets: *const c_char,
    /// Book value per share.
    pub bps: *const c_char,
    /// Earnings per share.
    pub eps: *const c_char,
    /// Dividends per share.
    pub dps: *const c_char,
    /// Dividend yield.
    pub div_yld: *const c_char,
    /// Dividend payout ratio.
    pub div_payout_ratio: *const c_char,
    /// 5-year average dividends per share.
    pub five_y_avg_dps: *const c_char,
    /// Current PE ratio.
    pub pe: *const c_char,
    /// Pointer to the array of historical snapshots.
    pub history: *const CIndustryValuationHistory,
    /// Number of items in `history`.
    pub num_history: usize,
}

pub(crate) struct CIndustryValuationItemOwned {
    symbol: CString,
    name: CString,
    currency: CString,
    assets: CString,
    bps: CString,
    eps: CString,
    dps: CString,
    div_yld: CString,
    div_payout_ratio: CString,
    five_y_avg_dps: CString,
    pe: CString,
    history: CVec<CIndustryValuationHistoryOwned>,
}

impl From<longbridge::fundamental::IndustryValuationItem> for CIndustryValuationItemOwned {
    fn from(v: longbridge::fundamental::IndustryValuationItem) -> Self {
        Self {
            symbol: v.symbol.into(),
            name: v.name.into(),
            currency: v.currency.into(),
            assets: v.assets.map(|d| d.to_string()).unwrap_or_default().into(),
            bps: v.bps.map(|d| d.to_string()).unwrap_or_default().into(),
            eps: v.eps.map(|d| d.to_string()).unwrap_or_default().into(),
            dps: v.dps.map(|d| d.to_string()).unwrap_or_default().into(),
            div_yld: v.div_yld.map(|d| d.to_string()).unwrap_or_default().into(),
            div_payout_ratio: v
                .div_payout_ratio
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            five_y_avg_dps: v
                .five_y_avg_dps
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            pe: v.pe.map(|d| d.to_string()).unwrap_or_default().into(),
            history: v.history.into(),
        }
    }
}

impl ToFFI for CIndustryValuationItemOwned {
    type FFIType = CIndustryValuationItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CIndustryValuationItem {
            symbol: self.symbol.to_ffi_type(),
            name: self.name.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            assets: self.assets.to_ffi_type(),
            bps: self.bps.to_ffi_type(),
            eps: self.eps.to_ffi_type(),
            dps: self.dps.to_ffi_type(),
            div_yld: self.div_yld.to_ffi_type(),
            div_payout_ratio: self.div_payout_ratio.to_ffi_type(),
            five_y_avg_dps: self.five_y_avg_dps.to_ffi_type(),
            pe: self.pe.to_ffi_type(),
            history: self.history.to_ffi_type(),
            num_history: self.history.len(),
        }
    }
}

/// List of industry valuation items.
#[repr(C)]
pub struct CIndustryValuationList {
    /// Pointer to the array of industry valuation items.
    pub list: *const CIndustryValuationItem,
    /// Number of items in `list`.
    pub num_list: usize,
}

pub(crate) struct CIndustryValuationListOwned {
    list: CVec<CIndustryValuationItemOwned>,
}

impl From<longbridge::fundamental::IndustryValuationList> for CIndustryValuationListOwned {
    fn from(v: longbridge::fundamental::IndustryValuationList) -> Self {
        Self {
            list: v.list.into(),
        }
    }
}

impl ToFFI for CIndustryValuationListOwned {
    type FFIType = CIndustryValuationList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CIndustryValuationList {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
        }
    }
}

// ── IndustryValuationDist ─────────────────────────────────────────

/// Distribution statistics for one valuation metric within an industry.
#[repr(C)]
pub struct CValuationDist {
    /// Minimum value in the industry.
    pub low: *const c_char,
    /// Maximum value in the industry.
    pub high: *const c_char,
    /// Median value in the industry.
    pub median: *const c_char,
    /// Current value of the queried security.
    pub value: *const c_char,
    /// Percentile ranking (0-1 range as string).
    pub ranking: *const c_char,
    /// Ordinal rank index (1-based).
    pub rank_index: *const c_char,
    /// Total number of securities in the industry.
    pub rank_total: *const c_char,
}

pub(crate) struct CValuationDistOwned {
    low: CString,
    high: CString,
    median: CString,
    value: CString,
    ranking: CString,
    rank_index: CString,
    rank_total: CString,
}

impl From<longbridge::fundamental::ValuationDist> for CValuationDistOwned {
    fn from(v: longbridge::fundamental::ValuationDist) -> Self {
        Self {
            low: v.low.map(|d| d.to_string()).unwrap_or_default().into(),
            high: v.high.map(|d| d.to_string()).unwrap_or_default().into(),
            median: v.median.map(|d| d.to_string()).unwrap_or_default().into(),
            value: v.value.map(|d| d.to_string()).unwrap_or_default().into(),
            ranking: v.ranking.map(|d| d.to_string()).unwrap_or_default().into(),
            rank_index: v.rank_index.into(),
            rank_total: v.rank_total.into(),
        }
    }
}

impl ToFFI for CValuationDistOwned {
    type FFIType = CValuationDist;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationDist {
            low: self.low.to_ffi_type(),
            high: self.high.to_ffi_type(),
            median: self.median.to_ffi_type(),
            value: self.value.to_ffi_type(),
            ranking: self.ranking.to_ffi_type(),
            rank_index: self.rank_index.to_ffi_type(),
            rank_total: self.rank_total.to_ffi_type(),
        }
    }
}

/// Industry valuation distribution for PE, PB, PS ratios.
#[repr(C)]
pub struct CIndustryValuationDist {
    /// PE ratio distribution, or null if unavailable.
    pub pe: *const CValuationDist,
    /// PB ratio distribution, or null if unavailable.
    pub pb: *const CValuationDist,
    /// PS ratio distribution, or null if unavailable.
    pub ps: *const CValuationDist,
}

pub(crate) struct CIndustryValuationDistOwned {
    pe: COption<CValuationDistOwned>,
    pb: COption<CValuationDistOwned>,
    ps: COption<CValuationDistOwned>,
}

impl From<longbridge::fundamental::IndustryValuationDist> for CIndustryValuationDistOwned {
    fn from(v: longbridge::fundamental::IndustryValuationDist) -> Self {
        Self {
            pe: v.pe.into(),
            pb: v.pb.into(),
            ps: v.ps.into(),
        }
    }
}

impl ToFFI for CIndustryValuationDistOwned {
    type FFIType = CIndustryValuationDist;
    fn to_ffi_type(&self) -> Self::FFIType {
        CIndustryValuationDist {
            pe: self.pe.to_ffi_type(),
            pb: self.pb.to_ffi_type(),
            ps: self.ps.to_ffi_type(),
        }
    }
}

// ── ExecutiveList ─────────────────────────────────────────────────

/// One executive or board member.
#[repr(C)]
pub struct CProfessional {
    /// Internal wiki person ID.
    pub id: *const c_char,
    /// Full name.
    pub name: *const c_char,
    /// Full name in Simplified Chinese.
    pub name_zhcn: *const c_char,
    /// Full name in English.
    pub name_en: *const c_char,
    /// Job title.
    pub title: *const c_char,
    /// Biography text.
    pub biography: *const c_char,
    /// URL to the person's photo.
    pub photo: *const c_char,
    /// URL to the wiki profile page.
    pub wiki_url: *const c_char,
}

pub(crate) struct CProfessionalOwned {
    id: CString,
    name: CString,
    name_zhcn: CString,
    name_en: CString,
    title: CString,
    biography: CString,
    photo: CString,
    wiki_url: CString,
}

impl From<longbridge::fundamental::Professional> for CProfessionalOwned {
    fn from(v: longbridge::fundamental::Professional) -> Self {
        Self {
            id: v.id.into(),
            name: v.name.into(),
            name_zhcn: v.name_zhcn.into(),
            name_en: v.name_en.into(),
            title: v.title.into(),
            biography: v.biography.into(),
            photo: v.photo.into(),
            wiki_url: v.wiki_url.into(),
        }
    }
}

impl ToFFI for CProfessionalOwned {
    type FFIType = CProfessional;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfessional {
            id: self.id.to_ffi_type(),
            name: self.name.to_ffi_type(),
            name_zhcn: self.name_zhcn.to_ffi_type(),
            name_en: self.name_en.to_ffi_type(),
            title: self.title.to_ffi_type(),
            biography: self.biography.to_ffi_type(),
            photo: self.photo.to_ffi_type(),
            wiki_url: self.wiki_url.to_ffi_type(),
        }
    }
}

/// Executives for one security.
#[repr(C)]
pub struct CExecutiveGroup {
    /// Security symbol.
    pub symbol: *const c_char,
    /// Link to the company wiki page.
    pub forward_url: *const c_char,
    /// Total number of executives.
    pub total: i32,
    /// Pointer to the array of professionals.
    pub professionals: *const CProfessional,
    /// Number of items in `professionals`.
    pub num_professionals: usize,
}

pub(crate) struct CExecutiveGroupOwned {
    symbol: CString,
    forward_url: CString,
    total: i32,
    professionals: CVec<CProfessionalOwned>,
}

impl From<longbridge::fundamental::ExecutiveGroup> for CExecutiveGroupOwned {
    fn from(v: longbridge::fundamental::ExecutiveGroup) -> Self {
        Self {
            symbol: v.symbol.into(),
            forward_url: v.forward_url.into(),
            total: v.total,
            professionals: v.professionals.into(),
        }
    }
}

impl ToFFI for CExecutiveGroupOwned {
    type FFIType = CExecutiveGroup;
    fn to_ffi_type(&self) -> Self::FFIType {
        CExecutiveGroup {
            symbol: self.symbol.to_ffi_type(),
            forward_url: self.forward_url.to_ffi_type(),
            total: self.total,
            professionals: self.professionals.to_ffi_type(),
            num_professionals: self.professionals.len(),
        }
    }
}

/// List of executive groups per security.
#[repr(C)]
pub struct CExecutiveList {
    /// Pointer to the array of executive groups.
    pub professional_list: *const CExecutiveGroup,
    /// Number of groups in `professional_list`.
    pub num_professional_list: usize,
}

pub(crate) struct CExecutiveListOwned {
    professional_list: CVec<CExecutiveGroupOwned>,
}

impl From<longbridge::fundamental::ExecutiveList> for CExecutiveListOwned {
    fn from(v: longbridge::fundamental::ExecutiveList) -> Self {
        Self {
            professional_list: v.professional_list.into(),
        }
    }
}

impl ToFFI for CExecutiveListOwned {
    type FFIType = CExecutiveList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CExecutiveList {
            professional_list: self.professional_list.to_ffi_type(),
            num_professional_list: self.professional_list.len(),
        }
    }
}

// ── BuybackData ───────────────────────────────────────────────────

/// TTM (trailing twelve months) buyback summary.
#[repr(C)]
pub struct CRecentBuybacks {
    /// Reporting currency.
    pub currency: *const c_char,
    /// Net buyback amount TTM.
    pub net_buyback_ttm: *const c_char,
    /// Net buyback yield TTM.
    pub net_buyback_yield_ttm: *const c_char,
}

pub(crate) struct CRecentBuybacksOwned {
    currency: CString,
    net_buyback_ttm: CString,
    net_buyback_yield_ttm: CString,
}

impl From<longbridge::fundamental::RecentBuybacks> for CRecentBuybacksOwned {
    fn from(v: longbridge::fundamental::RecentBuybacks) -> Self {
        Self {
            currency: v.currency.into(),
            net_buyback_ttm: v
                .net_buyback_ttm
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            net_buyback_yield_ttm: v
                .net_buyback_yield_ttm
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CRecentBuybacksOwned {
    type FFIType = CRecentBuybacks;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRecentBuybacks {
            currency: self.currency.to_ffi_type(),
            net_buyback_ttm: self.net_buyback_ttm.to_ffi_type(),
            net_buyback_yield_ttm: self.net_buyback_yield_ttm.to_ffi_type(),
        }
    }
}

/// Historical annual buyback data point.
#[repr(C)]
pub struct CBuybackHistoryItem {
    /// Fiscal year label, e.g. "FY2024".
    pub fiscal_year: *const c_char,
    /// Fiscal year date range string.
    pub fiscal_year_range: *const c_char,
    /// Net buyback amount.
    pub net_buyback: *const c_char,
    /// Net buyback yield.
    pub net_buyback_yield: *const c_char,
    /// Year-over-year net buyback growth rate.
    pub net_buyback_growth_rate: *const c_char,
    /// Reporting currency.
    pub currency: *const c_char,
}

pub(crate) struct CBuybackHistoryItemOwned {
    fiscal_year: CString,
    fiscal_year_range: CString,
    net_buyback: CString,
    net_buyback_yield: CString,
    net_buyback_growth_rate: CString,
    currency: CString,
}

impl From<longbridge::fundamental::BuybackHistoryItem> for CBuybackHistoryItemOwned {
    fn from(v: longbridge::fundamental::BuybackHistoryItem) -> Self {
        Self {
            fiscal_year: v.fiscal_year.into(),
            fiscal_year_range: v.fiscal_year_range.into(),
            net_buyback: v
                .net_buyback
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            net_buyback_yield: v
                .net_buyback_yield
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            net_buyback_growth_rate: v
                .net_buyback_growth_rate
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            currency: v.currency.into(),
        }
    }
}

impl ToFFI for CBuybackHistoryItemOwned {
    type FFIType = CBuybackHistoryItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBuybackHistoryItem {
            fiscal_year: self.fiscal_year.to_ffi_type(),
            fiscal_year_range: self.fiscal_year_range.to_ffi_type(),
            net_buyback: self.net_buyback.to_ffi_type(),
            net_buyback_yield: self.net_buyback_yield.to_ffi_type(),
            net_buyback_growth_rate: self.net_buyback_growth_rate.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
        }
    }
}

/// Buyback payout and cash-flow ratios.
#[repr(C)]
pub struct CBuybackRatios {
    /// Net buyback payout ratio.
    pub net_buyback_payout_ratio: *const c_char,
    /// Net buyback to free cash-flow ratio.
    pub net_buyback_to_cashflow_ratio: *const c_char,
}

pub(crate) struct CBuybackRatiosOwned {
    net_buyback_payout_ratio: CString,
    net_buyback_to_cashflow_ratio: CString,
}

impl From<longbridge::fundamental::BuybackRatios> for CBuybackRatiosOwned {
    fn from(v: longbridge::fundamental::BuybackRatios) -> Self {
        Self {
            net_buyback_payout_ratio: v
                .net_buyback_payout_ratio
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            net_buyback_to_cashflow_ratio: v
                .net_buyback_to_cashflow_ratio
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CBuybackRatiosOwned {
    type FFIType = CBuybackRatios;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBuybackRatios {
            net_buyback_payout_ratio: self.net_buyback_payout_ratio.to_ffi_type(),
            net_buyback_to_cashflow_ratio: self.net_buyback_to_cashflow_ratio.to_ffi_type(),
        }
    }
}

/// Buyback data response.
#[repr(C)]
pub struct CBuybackData {
    /// TTM buyback summary, or null if unavailable.
    pub recent_buybacks: *const CRecentBuybacks,
    /// Pointer to the array of historical buyback items.
    pub buyback_history: *const CBuybackHistoryItem,
    /// Number of items in `buyback_history`.
    pub num_buyback_history: usize,
    /// Pointer to the array of buyback ratios.
    pub buyback_ratios: *const CBuybackRatios,
    /// Number of items in `buyback_ratios`.
    pub num_buyback_ratios: usize,
}

pub(crate) struct CBuybackDataOwned {
    recent_buybacks: COption<CRecentBuybacksOwned>,
    buyback_history: CVec<CBuybackHistoryItemOwned>,
    buyback_ratios: CVec<CBuybackRatiosOwned>,
}

impl From<longbridge::fundamental::BuybackData> for CBuybackDataOwned {
    fn from(v: longbridge::fundamental::BuybackData) -> Self {
        Self {
            recent_buybacks: v.recent_buybacks.into(),
            buyback_history: v.buyback_history.into(),
            buyback_ratios: v.buyback_ratios.into(),
        }
    }
}

impl ToFFI for CBuybackDataOwned {
    type FFIType = CBuybackData;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBuybackData {
            recent_buybacks: self.recent_buybacks.to_ffi_type(),
            buyback_history: self.buyback_history.to_ffi_type(),
            num_buyback_history: self.buyback_history.len(),
            buyback_ratios: self.buyback_ratios.to_ffi_type(),
            num_buyback_ratios: self.buyback_ratios.len(),
        }
    }
}

// ── StockRatings ──────────────────────────────────────────────────

/// A leaf rating indicator with a raw value.
#[repr(C)]
pub struct CRatingLeafIndicator {
    /// Indicator display name.
    pub name: *const c_char,
    /// Formatted value string.
    pub value: *const c_char,
    /// Value type hint, e.g. "percent".
    pub value_type: *const c_char,
    /// Score (serialised as JSON string).
    pub score: *const c_char,
    /// Letter grade.
    pub letter: *const c_char,
}

pub(crate) struct CRatingLeafIndicatorOwned {
    name: CString,
    value: CString,
    value_type: CString,
    score: CString,
    letter: CString,
}

impl From<longbridge::fundamental::RatingLeafIndicator> for CRatingLeafIndicatorOwned {
    fn from(v: longbridge::fundamental::RatingLeafIndicator) -> Self {
        Self {
            name: v.name.into(),
            value: v.value.into(),
            value_type: v.value_type.into(),
            score: v.score.to_string().into(),
            letter: v.letter.into(),
        }
    }
}

impl ToFFI for CRatingLeafIndicatorOwned {
    type FFIType = CRatingLeafIndicator;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingLeafIndicator {
            name: self.name.to_ffi_type(),
            value: self.value.to_ffi_type(),
            value_type: self.value_type.to_ffi_type(),
            score: self.score.to_ffi_type(),
            letter: self.letter.to_ffi_type(),
        }
    }
}

/// A rating indicator node (parent or leaf).
#[repr(C)]
pub struct CRatingIndicator {
    /// Indicator display name.
    pub name: *const c_char,
    /// Score (serialised as JSON string).
    pub score: *const c_char,
    /// Letter grade.
    pub letter: *const c_char,
}

pub(crate) struct CRatingIndicatorOwned {
    name: CString,
    score: CString,
    letter: CString,
}

impl From<longbridge::fundamental::RatingIndicator> for CRatingIndicatorOwned {
    fn from(v: longbridge::fundamental::RatingIndicator) -> Self {
        Self {
            name: v.name.into(),
            score: v.score.to_string().into(),
            letter: v.letter.into(),
        }
    }
}

impl ToFFI for CRatingIndicatorOwned {
    type FFIType = CRatingIndicator;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingIndicator {
            name: self.name.to_ffi_type(),
            score: self.score.to_ffi_type(),
            letter: self.letter.to_ffi_type(),
        }
    }
}

/// A group of sub-indicators under one category indicator.
#[repr(C)]
pub struct CRatingSubIndicatorGroup {
    /// Parent indicator for this group.
    pub indicator: CRatingIndicator,
    /// Pointer to the array of leaf sub-indicators.
    pub sub_indicators: *const CRatingLeafIndicator,
    /// Number of items in `sub_indicators`.
    pub num_sub_indicators: usize,
}

pub(crate) struct CRatingSubIndicatorGroupOwned {
    indicator: CRatingIndicatorOwned,
    sub_indicators: CVec<CRatingLeafIndicatorOwned>,
}

impl From<longbridge::fundamental::RatingSubIndicatorGroup> for CRatingSubIndicatorGroupOwned {
    fn from(v: longbridge::fundamental::RatingSubIndicatorGroup) -> Self {
        Self {
            indicator: v.indicator.into(),
            sub_indicators: v.sub_indicators.into(),
        }
    }
}

impl ToFFI for CRatingSubIndicatorGroupOwned {
    type FFIType = CRatingSubIndicatorGroup;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingSubIndicatorGroup {
            indicator: self.indicator.to_ffi_type(),
            sub_indicators: self.sub_indicators.to_ffi_type(),
            num_sub_indicators: self.sub_indicators.len(),
        }
    }
}

/// One rating category (e.g. growth, profitability).
#[repr(C)]
pub struct CRatingCategory {
    /// Category type code.
    pub kind: i32,
    /// Pointer to the array of sub-indicator groups.
    pub sub_indicators: *const CRatingSubIndicatorGroup,
    /// Number of items in `sub_indicators`.
    pub num_sub_indicators: usize,
}

pub(crate) struct CRatingCategoryOwned {
    kind: i32,
    sub_indicators: CVec<CRatingSubIndicatorGroupOwned>,
}

impl From<longbridge::fundamental::RatingCategory> for CRatingCategoryOwned {
    fn from(v: longbridge::fundamental::RatingCategory) -> Self {
        Self {
            kind: v.kind,
            sub_indicators: v.sub_indicators.into(),
        }
    }
}

impl ToFFI for CRatingCategoryOwned {
    type FFIType = CRatingCategory;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRatingCategory {
            kind: self.kind,
            sub_indicators: self.sub_indicators.to_ffi_type(),
            num_sub_indicators: self.sub_indicators.len(),
        }
    }
}

/// Stock ratings response.
#[repr(C)]
pub struct CStockRatings {
    /// Style display name.
    pub style_txt_name: *const c_char,
    /// Scale display name.
    pub scale_txt_name: *const c_char,
    /// Report period display text.
    pub report_period_txt: *const c_char,
    /// Composite score (JSON string).
    pub multi_score: *const c_char,
    /// Composite score letter grade.
    pub multi_letter: *const c_char,
    /// Score change vs previous period.
    pub multi_score_change: i32,
    /// Industry name.
    pub industry_name: *const c_char,
    /// Industry rank (JSON string).
    pub industry_rank: *const c_char,
    /// Total securities in the industry (JSON string).
    pub industry_total: *const c_char,
    /// Industry mean score (JSON string).
    pub industry_mean_score: *const c_char,
    /// Industry median score (JSON string).
    pub industry_median_score: *const c_char,
    /// Pointer to the array of rating categories.
    pub ratings: *const CRatingCategory,
    /// Number of items in `ratings`.
    pub num_ratings: usize,
}

pub(crate) struct CStockRatingsOwned {
    style_txt_name: CString,
    scale_txt_name: CString,
    report_period_txt: CString,
    multi_score: CString,
    multi_letter: CString,
    multi_score_change: i32,
    industry_name: CString,
    industry_rank: CString,
    industry_total: CString,
    industry_mean_score: CString,
    industry_median_score: CString,
    ratings: CVec<CRatingCategoryOwned>,
}

impl From<longbridge::fundamental::StockRatings> for CStockRatingsOwned {
    fn from(v: longbridge::fundamental::StockRatings) -> Self {
        Self {
            style_txt_name: v.style_txt_name.into(),
            scale_txt_name: v.scale_txt_name.into(),
            report_period_txt: v.report_period_txt.into(),
            multi_score: v.multi_score.to_string().into(),
            multi_letter: v.multi_letter.into(),
            multi_score_change: v.multi_score_change,
            industry_name: v.industry_name.into(),
            industry_rank: v.industry_rank.to_string().into(),
            industry_total: v.industry_total.to_string().into(),
            industry_mean_score: v.industry_mean_score.to_string().into(),
            industry_median_score: v.industry_median_score.to_string().into(),
            ratings: v.ratings.into(),
        }
    }
}

impl ToFFI for CStockRatingsOwned {
    type FFIType = CStockRatings;
    fn to_ffi_type(&self) -> Self::FFIType {
        CStockRatings {
            style_txt_name: self.style_txt_name.to_ffi_type(),
            scale_txt_name: self.scale_txt_name.to_ffi_type(),
            report_period_txt: self.report_period_txt.to_ffi_type(),
            multi_score: self.multi_score.to_ffi_type(),
            multi_letter: self.multi_letter.to_ffi_type(),
            multi_score_change: self.multi_score_change,
            industry_name: self.industry_name.to_ffi_type(),
            industry_rank: self.industry_rank.to_ffi_type(),
            industry_total: self.industry_total.to_ffi_type(),
            industry_mean_score: self.industry_mean_score.to_ffi_type(),
            industry_median_score: self.industry_median_score.to_ffi_type(),
            ratings: self.ratings.to_ffi_type(),
            num_ratings: self.ratings.len(),
        }
    }
}

// ── ShareholderTopResponse ────────────────────────────────────────

/// Top-shareholder list response. `data` is a NUL-terminated JSON string.
#[repr(C)]
pub struct CShareholderTopResponse {
    /// Raw top-shareholder data as a JSON string
    pub data: *const c_char,
}

pub(crate) struct CShareholderTopResponseOwned {
    data: CString,
}

impl From<ShareholderTopResponse> for CShareholderTopResponseOwned {
    fn from(v: ShareholderTopResponse) -> Self {
        let json = serde_json::to_string(&v.data).unwrap_or_default();
        Self { data: json.into() }
    }
}

impl ToFFI for CShareholderTopResponseOwned {
    type FFIType = CShareholderTopResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CShareholderTopResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── ShareholderDetailResponse ─────────────────────────────────────

/// Shareholder detail response. `data` is a NUL-terminated JSON string.
#[repr(C)]
pub struct CShareholderDetailResponse {
    /// Raw shareholder detail data as a JSON string
    pub data: *const c_char,
}

pub(crate) struct CShareholderDetailResponseOwned {
    data: CString,
}

impl From<ShareholderDetailResponse> for CShareholderDetailResponseOwned {
    fn from(v: ShareholderDetailResponse) -> Self {
        let json = serde_json::to_string(&v.data).unwrap_or_default();
        Self { data: json.into() }
    }
}

impl ToFFI for CShareholderDetailResponseOwned {
    type FFIType = CShareholderDetailResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CShareholderDetailResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── ValuationComparisonResponse ───────────────────────────────────

/// Valuation comparison response. `data` is a NUL-terminated JSON string.
#[repr(C)]
pub struct CValuationComparisonResponse {
    /// Raw valuation comparison data as a JSON string
    pub data: *const c_char,
}

pub(crate) struct CValuationComparisonResponseOwned {
    data: CString,
}

impl From<ValuationComparisonResponse> for CValuationComparisonResponseOwned {
    fn from(v: ValuationComparisonResponse) -> Self {
        let json = serde_json::to_string(&v.data).unwrap_or_default();
        Self { data: json.into() }
    }
}

impl ToFFI for CValuationComparisonResponseOwned {
    type FFIType = CValuationComparisonResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CValuationComparisonResponse {
            data: self.data.to_ffi_type(),
        }
    }
}
