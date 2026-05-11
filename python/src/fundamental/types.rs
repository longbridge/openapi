use longbridge::fundamental::types as lb;
use longbridge_python_macros::PyEnum;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

/// Institutional analyst recommendation
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::fundamental::types::InstitutionRecommend")]
pub(crate) enum InstitutionRecommend {
    /// Unknown
    Unknown,
    /// Strong buy
    StrongBuy,
    /// Buy
    Buy,
    /// Hold
    Hold,
    /// Sell
    Sell,
    /// Strong sell
    StrongSell,
    /// Underperform
    Underperform,
    /// No opinion
    NoOpinion,
}

// ── JsonValue: Clone + IntoPyObject wrapper ───────────────────────

#[derive(Debug, Clone)]
pub(crate) struct JsonValue(pub(crate) serde_json::Value);

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

impl<'py> IntoPyObject<'py> for &JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

// ── FinancialReports ──────────────────────────────────────────────

/// Financial reports response.
///
/// The `list` field is a dict keyed by report kind (`"IS"`, `"BS"`, `"CF"`).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct FinancialReports {
    /// Raw nested financial data dict
    pub list: JsonValue,
}

impl From<lb::FinancialReports> for FinancialReports {
    fn from(v: lb::FinancialReports) -> Self {
        Self {
            list: JsonValue(v.list),
        }
    }
}

impl FinancialReports {
    pub(crate) fn from_lb(_py: Python<'_>, v: lb::FinancialReports) -> PyResult<Self> {
        Ok(v.into())
    }
}

// ── DividendList / DividendItem ───────────────────────────────────

/// Dividend history response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DividendList {
    /// List of dividend events
    pub list: Vec<DividendItem>,
}

impl From<lb::DividendList> for DividendList {
    fn from(v: lb::DividendList) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// A single dividend event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct DividendItem {
    /// Security symbol, e.g. `"700.HK"`
    pub symbol: String,
    /// Internal record ID
    pub id: String,
    /// Human-readable description
    pub desc: String,
    /// Record / book-close date
    pub record_date: String,
    /// Ex-dividend date
    pub ex_date: String,
    /// Payment date
    pub payment_date: String,
}

impl From<lb::DividendItem> for DividendItem {
    fn from(v: lb::DividendItem) -> Self {
        Self {
            symbol: v.symbol,
            id: v.id,
            desc: v.desc,
            record_date: v.record_date,
            ex_date: v.ex_date,
            payment_date: v.payment_date,
        }
    }
}

// ── InstitutionRating ─────────────────────────────────────────────

/// Combined analyst rating response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRating {
    /// Latest snapshot
    pub latest: InstitutionRatingLatest,
    /// Consensus summary
    pub summary: InstitutionRatingSummary,
}

impl From<lb::InstitutionRating> for InstitutionRating {
    fn from(v: lb::InstitutionRating) -> Self {
        Self {
            latest: v.latest.into(),
            summary: v.summary.into(),
        }
    }
}

/// Latest analyst rating snapshot
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingLatest {
    /// Rating distribution counts
    pub evaluate: RatingEvaluate,
    /// Target price range
    pub target: RatingTarget,
    /// Industry classification ID
    pub industry_id: i64,
    /// Industry name
    pub industry_name: String,
    /// Rank within the industry
    pub industry_rank: i32,
    /// Total securities in the industry
    pub industry_total: i32,
    /// Mean analyst count in the industry
    pub industry_mean: i32,
    /// Median analyst count in the industry
    pub industry_median: i32,
}

impl From<lb::InstitutionRatingLatest> for InstitutionRatingLatest {
    fn from(v: lb::InstitutionRatingLatest) -> Self {
        Self {
            evaluate: v.evaluate.into(),
            target: v.target.into(),
            industry_id: v.industry_id,
            industry_name: v.industry_name,
            industry_rank: v.industry_rank,
            industry_total: v.industry_total,
            industry_mean: v.industry_mean,
            industry_median: v.industry_median,
        }
    }
}

/// Analyst rating distribution counts
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct RatingEvaluate {
    /// Number of "Buy" ratings
    pub buy: i32,
    /// Number of "Strong Buy" / "Outperform" ratings
    pub over: i32,
    /// Number of "Hold" ratings
    pub hold: i32,
    /// Number of "Underperform" ratings
    pub under: i32,
    /// Number of "Sell" ratings
    pub sell: i32,
    /// Number of "No Opinion" ratings
    pub no_opinion: i32,
    /// Total analyst count
    pub total: i32,
    /// Window start (unix timestamp string)
    pub start_date: String,
    /// Window end (unix timestamp string)
    pub end_date: String,
}

impl From<lb::RatingEvaluate> for RatingEvaluate {
    fn from(v: lb::RatingEvaluate) -> Self {
        Self {
            buy: v.buy,
            over: v.over,
            hold: v.hold,
            under: v.under,
            sell: v.sell,
            no_opinion: v.no_opinion,
            total: v.total,
            start_date: v.start_date,
            end_date: v.end_date,
        }
    }
}

/// Analyst target price range
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct RatingTarget {
    /// Highest price target
    pub highest_price: Option<String>,
    /// Lowest price target
    pub lowest_price: Option<String>,
    /// Previous close price
    pub prev_close: Option<String>,
    /// Window start (unix timestamp string)
    pub start_date: String,
    /// Window end (unix timestamp string)
    pub end_date: String,
}

impl From<lb::RatingTarget> for RatingTarget {
    fn from(v: lb::RatingTarget) -> Self {
        Self {
            highest_price: v.highest_price.map(|d| d.to_string()),
            lowest_price: v.lowest_price.map(|d| d.to_string()),
            prev_close: v.prev_close.map(|d| d.to_string()),
            start_date: v.start_date,
            end_date: v.end_date,
        }
    }
}

/// Consensus summary
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingSummary {
    /// Currency symbol, e.g. `"HK$"`
    pub ccy_symbol: String,
    /// Change vs previous period
    pub change: Option<String>,
    /// Simplified rating distribution
    pub evaluate: RatingSummaryEvaluate,
    /// Consensus recommendation
    pub recommend: InstitutionRecommend,
    /// Consensus target price
    pub target: Option<String>,
    /// Last updated display string
    pub updated_at: String,
}

impl From<lb::InstitutionRatingSummary> for InstitutionRatingSummary {
    fn from(v: lb::InstitutionRatingSummary) -> Self {
        Self {
            ccy_symbol: v.ccy_symbol,
            change: v.change.map(|d| d.to_string()),
            evaluate: v.evaluate.into(),
            recommend: v.recommend.into(),
            target: v.target.map(|d| d.to_string()),
            updated_at: v.updated_at,
        }
    }
}

/// Simplified rating distribution
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct RatingSummaryEvaluate {
    /// Number of "Buy" ratings
    pub buy: i32,
    /// Date of the update
    pub date: String,
    /// Number of "Hold" ratings
    pub hold: i32,
    /// Number of "Sell" ratings
    pub sell: i32,
    /// Number of "Strong Buy" ratings
    pub strong_buy: i32,
    /// Number of "Underperform" ratings
    pub under: i32,
}

impl From<lb::RatingSummaryEvaluate> for RatingSummaryEvaluate {
    fn from(v: lb::RatingSummaryEvaluate) -> Self {
        Self {
            buy: v.buy,
            date: v.date,
            hold: v.hold,
            sell: v.sell,
            strong_buy: v.strong_buy,
            under: v.under,
        }
    }
}

// ── InstitutionRatingDetail ───────────────────────────────────────

/// Historical analyst rating detail response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingDetail {
    /// Currency symbol
    pub ccy_symbol: String,
    /// Historical rating distribution time-series
    pub evaluate: InstitutionRatingDetailEvaluate,
    /// Historical target price time-series
    pub target: InstitutionRatingDetailTarget,
}

impl From<lb::InstitutionRatingDetail> for InstitutionRatingDetail {
    fn from(v: lb::InstitutionRatingDetail) -> Self {
        Self {
            ccy_symbol: v.ccy_symbol,
            evaluate: v.evaluate.into(),
            target: v.target.into(),
        }
    }
}

/// Historical rating distribution time-series
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingDetailEvaluate {
    /// Weekly rating distribution snapshots
    pub list: Vec<InstitutionRatingDetailEvaluateItem>,
}

impl From<lb::InstitutionRatingDetailEvaluate> for InstitutionRatingDetailEvaluate {
    fn from(v: lb::InstitutionRatingDetailEvaluate) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One weekly rating distribution snapshot
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingDetailEvaluateItem {
    /// Number of "Buy" ratings
    pub buy: i32,
    /// Date in `"2021/05/14"` format
    pub date: String,
    /// Number of "Hold" ratings
    pub hold: i32,
    /// Number of "Sell" ratings
    pub sell: i32,
    /// Number of "Strong Buy" / "Outperform" ratings
    pub strong_buy: i32,
    /// Number of "No Opinion" ratings
    pub no_opinion: i32,
    /// Number of "Underperform" ratings
    pub under: i32,
}

impl From<lb::InstitutionRatingDetailEvaluateItem> for InstitutionRatingDetailEvaluateItem {
    fn from(v: lb::InstitutionRatingDetailEvaluateItem) -> Self {
        Self {
            buy: v.buy,
            date: v.date,
            hold: v.hold,
            sell: v.sell,
            strong_buy: v.strong_buy,
            no_opinion: v.no_opinion,
            under: v.under,
        }
    }
}

/// Historical target price time-series
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingDetailTarget {
    /// Prediction accuracy ratio (may be `None`)
    pub data_percent: Option<String>,
    /// Overall prediction accuracy
    pub prediction_accuracy: Option<String>,
    /// Last updated display string
    pub updated_at: String,
    /// Weekly target price snapshots
    pub list: Vec<InstitutionRatingDetailTargetItem>,
}

impl From<lb::InstitutionRatingDetailTarget> for InstitutionRatingDetailTarget {
    fn from(v: lb::InstitutionRatingDetailTarget) -> Self {
        Self {
            data_percent: v.data_percent.map(|d| d.to_string()),
            prediction_accuracy: v.prediction_accuracy.map(|d| d.to_string()),
            updated_at: v.updated_at,
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One weekly target price snapshot
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InstitutionRatingDetailTargetItem {
    /// Average target price
    pub avg_target: Option<String>,
    /// Date in `"2021/05/16"` format
    pub date: String,
    /// Highest target price
    pub max_target: Option<String>,
    /// Lowest target price
    pub min_target: Option<String>,
    /// Whether the stock price reached the target
    pub meet: bool,
    /// Actual stock price at this date
    pub price: Option<String>,
    /// Unix timestamp string
    pub timestamp: String,
}

impl From<lb::InstitutionRatingDetailTargetItem> for InstitutionRatingDetailTargetItem {
    fn from(v: lb::InstitutionRatingDetailTargetItem) -> Self {
        Self {
            avg_target: v.avg_target.map(|d| d.to_string()),
            date: v.date,
            max_target: v.max_target.map(|d| d.to_string()),
            min_target: v.min_target.map(|d| d.to_string()),
            meet: v.meet,
            price: v.price.map(|d| d.to_string()),
            timestamp: v.timestamp,
        }
    }
}

// ── ForecastEps ───────────────────────────────────────────────────

/// EPS forecast response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ForecastEps {
    /// EPS forecast snapshots
    pub items: Vec<ForecastEpsItem>,
}

impl From<lb::ForecastEps> for ForecastEps {
    fn from(v: lb::ForecastEps) -> Self {
        Self {
            items: v.items.into_iter().map(Into::into).collect(),
        }
    }
}

/// One EPS forecast snapshot
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ForecastEpsItem {
    /// Median EPS estimate
    pub forecast_eps_median: Option<String>,
    /// Mean EPS estimate
    pub forecast_eps_mean: Option<String>,
    /// Lowest EPS estimate
    pub forecast_eps_lowest: Option<String>,
    /// Highest EPS estimate
    pub forecast_eps_highest: Option<String>,
    /// Total number of forecasting institutions
    pub institution_total: i32,
    /// Number of institutions that raised their estimate
    pub institution_up: i32,
    /// Number of institutions that lowered their estimate
    pub institution_down: i32,
    /// Forecast window start (datetime)
    pub forecast_start_date: crate::time::PyOffsetDateTimeWrapper,
    /// Forecast window end (datetime)
    pub forecast_end_date: crate::time::PyOffsetDateTimeWrapper,
}

impl From<lb::ForecastEpsItem> for ForecastEpsItem {
    fn from(v: lb::ForecastEpsItem) -> Self {
        Self {
            forecast_eps_median: v.forecast_eps_median.map(|d| d.to_string()),
            forecast_eps_mean: v.forecast_eps_mean.map(|d| d.to_string()),
            forecast_eps_lowest: v.forecast_eps_lowest.map(|d| d.to_string()),
            forecast_eps_highest: v.forecast_eps_highest.map(|d| d.to_string()),
            institution_total: v.institution_total,
            institution_up: v.institution_up,
            institution_down: v.institution_down,
            forecast_start_date: crate::time::PyOffsetDateTimeWrapper(v.forecast_start_date),
            forecast_end_date: crate::time::PyOffsetDateTimeWrapper(v.forecast_end_date),
        }
    }
}

// ── FinancialConsensus ────────────────────────────────────────────

/// Financial consensus estimates response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct FinancialConsensus {
    /// Per-period consensus reports
    pub list: Vec<ConsensusReport>,
    /// Index of the most recently released period
    pub current_index: i32,
    /// Reporting currency
    pub currency: String,
    /// Available period types
    pub opt_periods: Vec<String>,
    /// Currently returned period type
    pub current_period: String,
}

impl From<lb::FinancialConsensus> for FinancialConsensus {
    fn from(v: lb::FinancialConsensus) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
            current_index: v.current_index,
            currency: v.currency,
            opt_periods: v.opt_periods,
            current_period: v.current_period,
        }
    }
}

/// Consensus report for one fiscal period
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ConsensusReport {
    /// Fiscal year
    pub fiscal_year: i32,
    /// Fiscal period code
    pub fiscal_period: String,
    /// Human-readable period label
    pub period_text: String,
    /// Per-metric consensus details
    pub details: Vec<ConsensusDetail>,
}

impl From<lb::ConsensusReport> for ConsensusReport {
    fn from(v: lb::ConsensusReport) -> Self {
        Self {
            fiscal_year: v.fiscal_year,
            fiscal_period: v.fiscal_period,
            period_text: v.period_text,
            details: v.details.into_iter().map(Into::into).collect(),
        }
    }
}

/// Consensus estimate for one financial metric
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ConsensusDetail {
    /// Metric key, e.g. `"revenue"`
    pub key: String,
    /// Display name
    pub name: String,
    /// Metric description
    pub description: String,
    /// Actual reported value
    pub actual: Option<String>,
    /// Consensus estimate value
    pub estimate: Option<String>,
    /// Actual minus estimate
    pub comp_value: Option<String>,
    /// Beat/miss description
    pub comp_desc: String,
    /// Comparison result code
    pub comp: String,
    /// Whether actual results have been published
    pub is_released: bool,
}

impl From<lb::ConsensusDetail> for ConsensusDetail {
    fn from(v: lb::ConsensusDetail) -> Self {
        Self {
            key: v.key,
            name: v.name,
            description: v.description,
            actual: v.actual.map(|d| d.to_string()),
            estimate: v.estimate.map(|d| d.to_string()),
            comp_value: v.comp_value.map(|d| d.to_string()),
            comp_desc: v.comp_desc,
            comp: v.comp,
            is_released: v.is_released,
        }
    }
}

// ── ValuationData ─────────────────────────────────────────────────

/// Valuation metrics response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationData {
    /// Valuation metrics (PE / PB / PS / dividend yield)
    pub metrics: ValuationMetricsData,
}

impl From<lb::ValuationData> for ValuationData {
    fn from(v: lb::ValuationData) -> Self {
        Self {
            metrics: v.metrics.into(),
        }
    }
}

/// Container for valuation metrics
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationMetricsData {
    /// Price-to-Earnings ratio history
    pub pe: Option<ValuationMetricData>,
    /// Price-to-Book ratio history
    pub pb: Option<ValuationMetricData>,
    /// Price-to-Sales ratio history
    pub ps: Option<ValuationMetricData>,
    /// Dividend yield history
    pub dvd_yld: Option<ValuationMetricData>,
}

impl From<lb::ValuationMetricsData> for ValuationMetricsData {
    fn from(v: lb::ValuationMetricsData) -> Self {
        Self {
            pe: v.pe.map(Into::into),
            pb: v.pb.map(Into::into),
            ps: v.ps.map(Into::into),
            dvd_yld: v.dvd_yld.map(Into::into),
        }
    }
}

/// Historical time-series for one valuation metric
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationMetricData {
    /// Human-readable description
    pub desc: String,
    /// Historical high
    pub high: Option<String>,
    /// Historical low
    pub low: Option<String>,
    /// Historical median
    pub median: Option<String>,
    /// Historical data points
    pub list: Vec<ValuationPoint>,
}

impl From<lb::ValuationMetricData> for ValuationMetricData {
    fn from(v: lb::ValuationMetricData) -> Self {
        Self {
            desc: v.desc,
            high: v.high.map(|d| d.to_string()),
            low: v.low.map(|d| d.to_string()),
            median: v.median.map(|d| d.to_string()),
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One valuation data point
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationPoint {
    /// Date of the data point (datetime)
    pub timestamp: crate::time::PyOffsetDateTimeWrapper,
    /// Metric value
    pub value: Option<String>,
}

impl From<lb::ValuationPoint> for ValuationPoint {
    fn from(v: lb::ValuationPoint) -> Self {
        Self {
            timestamp: crate::time::PyOffsetDateTimeWrapper(v.timestamp),
            value: v.value.map(|d| d.to_string()),
        }
    }
}

// ── ValuationHistoryResponse ──────────────────────────────────────

/// Historical valuation response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationHistoryResponse {
    /// Historical valuation data
    pub history: ValuationHistoryData,
}

impl From<lb::ValuationHistoryResponse> for ValuationHistoryResponse {
    fn from(v: lb::ValuationHistoryResponse) -> Self {
        Self {
            history: v.history.into(),
        }
    }
}

/// Historical valuation container
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationHistoryData {
    /// Historical metrics
    pub metrics: ValuationHistoryMetrics,
}

impl From<lb::ValuationHistoryData> for ValuationHistoryData {
    fn from(v: lb::ValuationHistoryData) -> Self {
        Self {
            metrics: v.metrics.into(),
        }
    }
}

/// Historical valuation metrics container
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationHistoryMetrics {
    /// Price-to-Earnings history
    pub pe: Option<ValuationHistoryMetric>,
    /// Price-to-Book history
    pub pb: Option<ValuationHistoryMetric>,
    /// Price-to-Sales history
    pub ps: Option<ValuationHistoryMetric>,
}

impl From<lb::ValuationHistoryMetrics> for ValuationHistoryMetrics {
    fn from(v: lb::ValuationHistoryMetrics) -> Self {
        Self {
            pe: v.pe.map(Into::into),
            pb: v.pb.map(Into::into),
            ps: v.ps.map(Into::into),
        }
    }
}

/// Historical data for one valuation metric
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationHistoryMetric {
    /// Human-readable description
    pub desc: String,
    /// Historical high over the period
    pub high: Option<String>,
    /// Historical low over the period
    pub low: Option<String>,
    /// Historical median over the period
    pub median: Option<String>,
    /// Historical data points
    pub list: Vec<ValuationPoint>,
}

impl From<lb::ValuationHistoryMetric> for ValuationHistoryMetric {
    fn from(v: lb::ValuationHistoryMetric) -> Self {
        Self {
            desc: v.desc,
            high: v.high.map(|d| d.to_string()),
            low: v.low.map(|d| d.to_string()),
            median: v.median.map(|d| d.to_string()),
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

// ── IndustryValuationList ─────────────────────────────────────────

/// Industry peer valuation comparison response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct IndustryValuationList {
    /// List of peer securities
    pub list: Vec<IndustryValuationItem>,
}

impl From<lb::IndustryValuationList> for IndustryValuationList {
    fn from(v: lb::IndustryValuationList) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Valuation data for one peer security
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct IndustryValuationItem {
    /// Security symbol
    pub symbol: String,
    /// Company name
    pub name: String,
    /// Reporting currency
    pub currency: String,
    /// Total assets
    pub assets: Option<String>,
    /// Book value per share
    pub bps: Option<String>,
    /// Earnings per share
    pub eps: Option<String>,
    /// Dividends per share
    pub dps: Option<String>,
    /// Dividend yield
    pub div_yld: Option<String>,
    /// Dividend payout ratio
    pub div_payout_ratio: Option<String>,
    /// 5-year average dividends per share
    pub five_y_avg_dps: Option<String>,
    /// Current PE ratio
    pub pe: Option<String>,
    /// Historical PE/PB/PS snapshots
    pub history: Vec<IndustryValuationHistory>,
}

impl From<lb::IndustryValuationItem> for IndustryValuationItem {
    fn from(v: lb::IndustryValuationItem) -> Self {
        Self {
            symbol: v.symbol,
            name: v.name,
            currency: v.currency,
            assets: v.assets.map(|d| d.to_string()),
            bps: v.bps.map(|d| d.to_string()),
            eps: v.eps.map(|d| d.to_string()),
            dps: v.dps.map(|d| d.to_string()),
            div_yld: v.div_yld.map(|d| d.to_string()),
            div_payout_ratio: v.div_payout_ratio.map(|d| d.to_string()),
            five_y_avg_dps: v.five_y_avg_dps.map(|d| d.to_string()),
            pe: v.pe.map(|d| d.to_string()),
            history: v.history.into_iter().map(Into::into).collect(),
        }
    }
}

/// Historical valuation snapshot for a peer
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct IndustryValuationHistory {
    /// Unix timestamp string
    pub date: String,
    /// Price-to-Earnings ratio
    pub pe: Option<String>,
    /// Price-to-Book ratio
    pub pb: Option<String>,
    /// Price-to-Sales ratio
    pub ps: Option<String>,
}

impl From<lb::IndustryValuationHistory> for IndustryValuationHistory {
    fn from(v: lb::IndustryValuationHistory) -> Self {
        Self {
            date: v.date,
            pe: v.pe.map(|d| d.to_string()),
            pb: v.pb.map(|d| d.to_string()),
            ps: v.ps.map(|d| d.to_string()),
        }
    }
}

// ── IndustryValuationDist ─────────────────────────────────────────

/// Industry valuation distribution response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct IndustryValuationDist {
    /// PE ratio distribution
    pub pe: Option<ValuationDist>,
    /// PB ratio distribution
    pub pb: Option<ValuationDist>,
    /// PS ratio distribution
    pub ps: Option<ValuationDist>,
}

impl From<lb::IndustryValuationDist> for IndustryValuationDist {
    fn from(v: lb::IndustryValuationDist) -> Self {
        Self {
            pe: v.pe.map(Into::into),
            pb: v.pb.map(Into::into),
            ps: v.ps.map(Into::into),
        }
    }
}

/// Distribution statistics for one valuation metric
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ValuationDist {
    /// Minimum value
    pub low: Option<String>,
    /// Maximum value
    pub high: Option<String>,
    /// Median value
    pub median: Option<String>,
    /// Current value of the queried security
    pub value: Option<String>,
    /// Percentile ranking (0–1)
    pub ranking: Option<String>,
    /// Ordinal rank index
    pub rank_index: String,
    /// Total securities in the industry
    pub rank_total: String,
}

impl From<lb::ValuationDist> for ValuationDist {
    fn from(v: lb::ValuationDist) -> Self {
        Self {
            low: v.low.map(|d| d.to_string()),
            high: v.high.map(|d| d.to_string()),
            median: v.median.map(|d| d.to_string()),
            value: v.value.map(|d| d.to_string()),
            ranking: v.ranking.map(|d| d.to_string()),
            rank_index: v.rank_index,
            rank_total: v.rank_total,
        }
    }
}

// ── CompanyOverview ───────────────────────────────────────────────

/// Company overview response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct CompanyOverview {
    /// Short name
    pub name: String,
    /// Full legal name
    pub company_name: String,
    /// Founding date
    pub founded: String,
    /// Listing date
    pub listing_date: String,
    /// Primary listing market
    pub market: String,
    /// Market region code
    pub region: String,
    /// Registered address
    pub address: String,
    /// Principal office address
    pub office_address: String,
    /// Company website
    pub website: String,
    /// IPO issue price
    pub issue_price: Option<String>,
    /// Shares offered at IPO
    pub shares_offered: String,
    /// Chairman name
    pub chairman: String,
    /// Company secretary
    pub secretary: String,
    /// Auditing institution
    pub audit_inst: String,
    /// Company category
    pub category: String,
    /// Fiscal year end
    pub year_end: String,
    /// Number of employees
    pub employees: String,
    /// Phone number
    pub phone: String,
    /// Fax number
    pub fax: String,
    /// Email address
    pub email: String,
    /// Legal representative
    pub legal_repr: String,
    /// CEO / Managing Director
    pub manager: String,
    /// Business licence number
    pub bus_license: String,
    /// Accounting firm
    pub accounting_firm: String,
    /// Securities representative
    pub securities_rep: String,
    /// Legal counsel
    pub legal_counsel: String,
    /// Postal code
    pub zip_code: String,
    /// Exchange ticker code
    pub ticker: String,
    /// Logo icon URL
    pub icon: String,
    /// Business profile
    pub profile: String,
    /// ADS ratio
    pub ads_ratio: String,
    /// Industry sector code
    pub sector: i32,
}

impl From<lb::CompanyOverview> for CompanyOverview {
    fn from(v: lb::CompanyOverview) -> Self {
        Self {
            name: v.name,
            company_name: v.company_name,
            founded: v.founded,
            listing_date: v.listing_date,
            market: v.market,
            region: v.region,
            address: v.address,
            office_address: v.office_address,
            website: v.website,
            issue_price: v.issue_price.map(|d| d.to_string()),
            shares_offered: v.shares_offered,
            chairman: v.chairman,
            secretary: v.secretary,
            audit_inst: v.audit_inst,
            category: v.category,
            year_end: v.year_end,
            employees: v.employees,
            phone: v.phone,
            fax: v.fax,
            email: v.email,
            legal_repr: v.legal_repr,
            manager: v.manager,
            bus_license: v.bus_license,
            accounting_firm: v.accounting_firm,
            securities_rep: v.securities_rep,
            legal_counsel: v.legal_counsel,
            zip_code: v.zip_code,
            ticker: v.ticker,
            icon: v.icon,
            profile: v.profile,
            ads_ratio: v.ads_ratio,
            sector: v.sector,
        }
    }
}

// ── ExecutiveList ─────────────────────────────────────────────────

/// Executive list response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ExecutiveList {
    /// Groups of executives per security
    pub professional_list: Vec<ExecutiveGroup>,
}

impl From<lb::ExecutiveList> for ExecutiveList {
    fn from(v: lb::ExecutiveList) -> Self {
        Self {
            professional_list: v.professional_list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Executives for one security
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ExecutiveGroup {
    /// Security symbol
    pub symbol: String,
    /// Link to company wiki page
    pub forward_url: String,
    /// Total number of executives
    pub total: i32,
    /// Individual executive entries
    pub professionals: Vec<Professional>,
}

impl From<lb::ExecutiveGroup> for ExecutiveGroup {
    fn from(v: lb::ExecutiveGroup) -> Self {
        Self {
            symbol: v.symbol,
            forward_url: v.forward_url,
            total: v.total,
            professionals: v.professionals.into_iter().map(Into::into).collect(),
        }
    }
}

/// One executive / board member
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Professional {
    /// Internal wiki person ID
    pub id: String,
    /// Full name
    pub name: String,
    /// Full name in Simplified Chinese
    pub name_zhcn: String,
    /// Full name in English
    pub name_en: String,
    /// Job title
    pub title: String,
    /// Biography text
    pub biography: String,
    /// Photo URL
    pub photo: String,
    /// Wiki profile URL
    pub wiki_url: String,
}

impl From<lb::Professional> for Professional {
    fn from(v: lb::Professional) -> Self {
        Self {
            id: v.id,
            name: v.name,
            name_zhcn: v.name_zhcn,
            name_en: v.name_en,
            title: v.title,
            biography: v.biography,
            photo: v.photo,
            wiki_url: v.wiki_url,
        }
    }
}

// ── ShareholderList ───────────────────────────────────────────────

/// Shareholder list response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ShareholderList {
    /// List of major shareholders
    pub shareholder_list: Vec<Shareholder>,
    /// Link to full shareholder page
    pub forward_url: String,
    /// Total number returned
    pub total: i32,
}

impl From<lb::ShareholderList> for ShareholderList {
    fn from(v: lb::ShareholderList) -> Self {
        Self {
            shareholder_list: v.shareholder_list.into_iter().map(Into::into).collect(),
            forward_url: v.forward_url,
            total: v.total,
        }
    }
}

/// One major shareholder
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Shareholder {
    /// Internal shareholder ID
    pub shareholder_id: String,
    /// Shareholder name
    pub shareholder_name: String,
    /// Institution type
    pub institution_type: String,
    /// Percentage of shares held
    pub percent_of_shares: Option<String>,
    /// Change in shares held
    pub shares_changed: Option<String>,
    /// Most recent filing date
    pub report_date: String,
    /// Other securities held by this shareholder
    pub stocks: Vec<ShareholderStock>,
}

impl From<lb::Shareholder> for Shareholder {
    fn from(v: lb::Shareholder) -> Self {
        Self {
            shareholder_id: v.shareholder_id,
            shareholder_name: v.shareholder_name,
            institution_type: v.institution_type,
            percent_of_shares: v.percent_of_shares.map(|d| d.to_string()),
            shares_changed: v.shares_changed.map(|d| d.to_string()),
            report_date: v.report_date,
            stocks: v.stocks.into_iter().map(Into::into).collect(),
        }
    }
}

/// A cross-held security in an institutional shareholder's portfolio
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ShareholderStock {
    /// Security symbol
    pub symbol: String,
    /// Ticker code
    pub code: String,
    /// Market
    pub market: String,
    /// Day change percentage
    pub chg: String,
}

impl From<lb::ShareholderStock> for ShareholderStock {
    fn from(v: lb::ShareholderStock) -> Self {
        Self {
            symbol: v.symbol,
            code: v.code,
            market: v.market,
            chg: v.chg,
        }
    }
}

// ── FundHolders ───────────────────────────────────────────────────

/// Fund/ETF holders response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct FundHolders {
    /// Funds and ETFs holding the queried security
    pub lists: Vec<FundHolder>,
}

impl From<lb::FundHolders> for FundHolders {
    fn from(v: lb::FundHolders) -> Self {
        Self {
            lists: v.lists.into_iter().map(Into::into).collect(),
        }
    }
}

/// A fund or ETF holding the queried security
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct FundHolder {
    /// Fund/ETF ticker code
    pub code: String,
    /// Fund/ETF symbol
    pub symbol: String,
    /// Reporting currency
    pub currency: String,
    /// Fund/ETF full name
    pub name: String,
    /// Position ratio percentage string
    pub position_ratio: String,
    /// Report date
    pub report_date: String,
}

impl From<lb::FundHolder> for FundHolder {
    fn from(v: lb::FundHolder) -> Self {
        Self {
            code: v.code,
            symbol: v.symbol,
            currency: v.currency,
            name: v.name,
            position_ratio: v.position_ratio.to_string(),
            report_date: v.report_date,
        }
    }
}

// ── CorpActions ───────────────────────────────────────────────────

/// Corporate actions response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct CorpActions {
    /// Corporate action events
    pub items: Vec<CorpActionItem>,
}

impl From<lb::CorpActions> for CorpActions {
    fn from(v: lb::CorpActions) -> Self {
        Self {
            items: v.items.into_iter().map(Into::into).collect(),
        }
    }
}

/// One corporate action event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct CorpActionItem {
    /// Internal event ID
    pub id: String,
    /// Date in `YYYYMMDD` format
    pub date: String,
    /// Short display date
    pub date_str: String,
    /// Date type label
    pub date_type: String,
    /// Time zone description
    pub date_zone: String,
    /// Event category
    pub act_type: String,
    /// Human-readable description
    pub act_desc: String,
    /// Machine-readable action code
    pub action: String,
    /// Whether this is a recent event
    pub recent: bool,
    /// Whether publication was delayed
    pub is_delay: bool,
    /// Delay announcement content
    pub delay_content: String,
    /// Associated live stream
    pub live: Option<CorpActionLive>,
}

impl From<lb::CorpActionItem> for CorpActionItem {
    fn from(v: lb::CorpActionItem) -> Self {
        Self {
            id: v.id,
            date: v.date,
            date_str: v.date_str,
            date_type: v.date_type,
            date_zone: v.date_zone,
            act_type: v.act_type,
            act_desc: v.act_desc,
            action: v.action,
            recent: v.recent,
            is_delay: v.is_delay,
            delay_content: v.delay_content,
            live: v.live.map(Into::into),
        }
    }
}

/// Live stream associated with a corp action
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct CorpActionLive {
    /// Live stream ID
    pub id: String,
    /// Status code
    pub status: String,
    /// Start time
    pub started_at: String,
    /// Stream title
    pub name: String,
    /// Icon URL
    pub icon: String,
}

impl From<lb::CorpActionLive> for CorpActionLive {
    fn from(v: lb::CorpActionLive) -> Self {
        Self {
            id: v.id,
            status: v.status.to_string(),
            started_at: v.started_at,
            name: v.name,
            icon: v.icon,
        }
    }
}

// ── InvestRelations ───────────────────────────────────────────────

/// Investor relations response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InvestRelations {
    /// Link to investor relations page
    pub forward_url: String,
    /// Securities in which the company has a stake
    pub invest_securities: Vec<InvestSecurity>,
}

impl From<lb::InvestRelations> for InvestRelations {
    fn from(v: lb::InvestRelations) -> Self {
        Self {
            forward_url: v.forward_url,
            invest_securities: v.invest_securities.into_iter().map(Into::into).collect(),
        }
    }
}

/// A security in which the company has an investment stake
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct InvestSecurity {
    /// Internal company ID
    pub company_id: String,
    /// Company name (locale-aware)
    pub company_name: String,
    /// Company name in English
    pub company_name_en: String,
    /// Company name in Simplified Chinese
    pub company_name_zhcn: String,
    /// Security symbol
    pub symbol: String,
    /// Reporting currency
    pub currency: String,
    /// Percentage of shares held
    pub percent_of_shares: Option<String>,
    /// Shareholder rank
    pub shares_rank: String,
    /// Market value of the holding
    pub shares_value: Option<String>,
}

impl From<lb::InvestSecurity> for InvestSecurity {
    fn from(v: lb::InvestSecurity) -> Self {
        Self {
            company_id: v.company_id,
            company_name: v.company_name,
            company_name_en: v.company_name_en,
            company_name_zhcn: v.company_name_zhcn,
            symbol: v.symbol,
            currency: v.currency,
            percent_of_shares: v.percent_of_shares.map(|d| d.to_string()),
            shares_rank: v.shares_rank,
            shares_value: v.shares_value.map(|d| d.to_string()),
        }
    }
}

// ── OperatingList ─────────────────────────────────────────────────

/// Operating metrics response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct OperatingList {
    /// Operating summary reports
    pub list: Vec<OperatingItem>,
}

impl From<lb::OperatingList> for OperatingList {
    fn from(v: lb::OperatingList) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One operating summary report
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct OperatingItem {
    /// Internal report ID
    pub id: String,
    /// Report period code
    pub report: String,
    /// Report title
    pub title: String,
    /// Management discussion text
    pub txt: String,
    /// Whether this is the most recent report
    pub latest: bool,
    /// URL to the community report page
    pub web_url: String,
    /// Key financial metrics
    pub financial: OperatingFinancial,
}

impl From<lb::OperatingItem> for OperatingItem {
    fn from(v: lb::OperatingItem) -> Self {
        Self {
            id: v.id,
            report: v.report,
            title: v.title,
            txt: v.txt,
            latest: v.latest,
            web_url: v.web_url,
            financial: v.financial.into(),
        }
    }
}

/// Key financial metrics from an operating report
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct OperatingFinancial {
    /// Ticker code
    pub code: String,
    /// Reporting currency
    pub currency: String,
    /// Company name
    pub name: String,
    /// Market region
    pub region: String,
    /// Report period code
    pub report: String,
    /// Financial indicators
    pub indicators: Vec<OperatingIndicator>,
}

impl From<lb::OperatingFinancial> for OperatingFinancial {
    fn from(v: lb::OperatingFinancial) -> Self {
        Self {
            code: v.code,
            currency: v.currency,
            name: v.name,
            region: v.region,
            report: v.report,
            indicators: v.indicators.into_iter().map(Into::into).collect(),
        }
    }
}

/// One financial indicator from an operating report
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct OperatingIndicator {
    /// Field name key
    pub field_name: String,
    /// Display name
    pub indicator_name: String,
    /// Formatted value
    pub indicator_value: String,
    /// Year-over-year change
    pub yoy: Option<String>,
}

impl From<lb::OperatingIndicator> for OperatingIndicator {
    fn from(v: lb::OperatingIndicator) -> Self {
        Self {
            field_name: v.field_name,
            indicator_name: v.indicator_name,
            indicator_value: v.indicator_value,
            yoy: v.yoy.map(|d| d.to_string()),
        }
    }
}

// ── enums ─────────────────────────────────────────────────────────

/// Financial report kind
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum FinancialReportKind {
    /// Income statement
    IncomeStatement = 0,
    /// Balance sheet
    BalanceSheet = 1,
    /// Cash flow statement
    CashFlow = 2,
    /// All statements
    All = 3,
}

impl From<FinancialReportKind> for lb::FinancialReportKind {
    fn from(v: FinancialReportKind) -> Self {
        match v {
            FinancialReportKind::IncomeStatement => lb::FinancialReportKind::IncomeStatement,
            FinancialReportKind::BalanceSheet => lb::FinancialReportKind::BalanceSheet,
            FinancialReportKind::CashFlow => lb::FinancialReportKind::CashFlow,
            FinancialReportKind::All => lb::FinancialReportKind::All,
        }
    }
}

// ── BuybackData ───────────────────────────────────────────────────

/// TTM buyback summary
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct RecentBuybacks {
    pub currency: String,
    pub net_buyback_ttm: String,
    pub net_buyback_yield_ttm: String,
}

impl From<lb::RecentBuybacks> for RecentBuybacks {
    fn from(v: lb::RecentBuybacks) -> Self {
        Self {
            currency: v.currency,
            net_buyback_ttm: v.net_buyback_ttm.map(|d| d.to_string()).unwrap_or_default(),
            net_buyback_yield_ttm: v
                .net_buyback_yield_ttm
                .map(|d| d.to_string())
                .unwrap_or_default(),
        }
    }
}

/// Historical annual buyback data item
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct BuybackHistoryItem {
    pub fiscal_year: String,
    pub fiscal_year_range: String,
    pub net_buyback: String,
    pub net_buyback_yield: String,
    pub net_buyback_growth_rate: String,
    pub currency: String,
}

impl From<lb::BuybackHistoryItem> for BuybackHistoryItem {
    fn from(v: lb::BuybackHistoryItem) -> Self {
        Self {
            fiscal_year: v.fiscal_year,
            fiscal_year_range: v.fiscal_year_range,
            net_buyback: v.net_buyback.map(|d| d.to_string()).unwrap_or_default(),
            net_buyback_yield: v
                .net_buyback_yield
                .map(|d| d.to_string())
                .unwrap_or_default(),
            net_buyback_growth_rate: v
                .net_buyback_growth_rate
                .map(|d| d.to_string())
                .unwrap_or_default(),
            currency: v.currency,
        }
    }
}

/// Buyback payout and cash-flow ratios
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct BuybackRatios {
    pub net_buyback_payout_ratio: String,
    pub net_buyback_to_cashflow_ratio: String,
}

impl From<lb::BuybackRatios> for BuybackRatios {
    fn from(v: lb::BuybackRatios) -> Self {
        Self {
            net_buyback_payout_ratio: v
                .net_buyback_payout_ratio
                .map(|d| d.to_string())
                .unwrap_or_default(),
            net_buyback_to_cashflow_ratio: v
                .net_buyback_to_cashflow_ratio
                .map(|d| d.to_string())
                .unwrap_or_default(),
        }
    }
}

/// Buyback data response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct BuybackData {
    pub recent_buybacks: Option<RecentBuybacks>,
    pub buyback_history: Vec<BuybackHistoryItem>,
    pub buyback_ratios: Vec<BuybackRatios>,
}

impl From<lb::BuybackData> for BuybackData {
    fn from(v: lb::BuybackData) -> Self {
        Self {
            recent_buybacks: v.recent_buybacks.map(Into::into),
            buyback_history: v.buyback_history.into_iter().map(Into::into).collect(),
            buyback_ratios: v.buyback_ratios.into_iter().map(Into::into).collect(),
        }
    }
}

// ── StockRatings ──────────────────────────────────────────────────

/// Stock ratings response.
///
/// `ratings_json` contains the full nested ratings structure as a JSON string.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct StockRatings {
    pub style_txt_name: String,
    pub scale_txt_name: String,
    pub report_period_txt: String,
    /// Composite score (string representation of the JSON value)
    pub multi_score: String,
    pub multi_letter: String,
    pub multi_score_change: i32,
    pub industry_name: String,
    pub industry_rank: i64,
    /// Full ratings JSON string
    pub ratings_json: String,
}

impl From<lb::StockRatings> for StockRatings {
    fn from(v: lb::StockRatings) -> Self {
        let industry_rank = v.industry_rank.as_i64().unwrap_or(0);
        Self {
            style_txt_name: v.style_txt_name,
            scale_txt_name: v.scale_txt_name,
            report_period_txt: v.report_period_txt,
            multi_score: v.multi_score.to_string(),
            multi_letter: v.multi_letter,
            multi_score_change: v.multi_score_change,
            industry_name: v.industry_name,
            industry_rank,
            ratings_json: serde_json::to_string(&v.ratings).unwrap_or_default(),
        }
    }
}

/// Financial report period type
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum FinancialReportPeriod {
    /// Annual report
    Annual = 0,
    /// Semi-annual report
    SemiAnnual = 1,
    /// Q1 report
    Q1 = 2,
    /// Q2 report
    Q2 = 3,
    /// Q3 report
    Q3 = 4,
    /// Full quarterly report
    QuarterlyFull = 5,
    /// Three-quarter report (first three quarters)
    ThreeQ = 6,
}

impl From<FinancialReportPeriod> for lb::FinancialReportPeriod {
    fn from(v: FinancialReportPeriod) -> Self {
        match v {
            FinancialReportPeriod::Annual => lb::FinancialReportPeriod::Annual,
            FinancialReportPeriod::SemiAnnual => lb::FinancialReportPeriod::SemiAnnual,
            FinancialReportPeriod::Q1 => lb::FinancialReportPeriod::Q1,
            FinancialReportPeriod::Q2 => lb::FinancialReportPeriod::Q2,
            FinancialReportPeriod::Q3 => lb::FinancialReportPeriod::Q3,
            FinancialReportPeriod::QuarterlyFull => lb::FinancialReportPeriod::QuarterlyFull,
            FinancialReportPeriod::ThreeQ => lb::FinancialReportPeriod::ThreeQ,
        }
    }
}
