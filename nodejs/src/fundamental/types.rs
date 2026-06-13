use longbridge::fundamental::types as lb;

// ── FinancialReports ──────────────────────────────────────────────

/// Financial reports response.
/// The `list` field is a nested object keyed by report kind.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct FinancialReports {
    /// Raw nested financial data object
    pub list: serde_json::Value,
}

impl From<lb::FinancialReports> for FinancialReports {
    fn from(v: lb::FinancialReports) -> Self {
        Self { list: v.list }
    }
}

// ── DividendList ──────────────────────────────────────────────────

/// Dividend history response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DividendList {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct DividendItem {
    /// Security symbol
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRating {
    /// Latest rating snapshot
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingLatest {
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
    /// Mean analyst count
    pub industry_mean: i32,
    /// Median analyst count
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct RatingEvaluate {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct RatingTarget {
    /// Highest price target
    pub highest_price: Option<String>,
    /// Lowest price target
    pub lowest_price: Option<String>,
    /// Previous close price
    pub prev_close: Option<String>,
    /// Window start
    pub start_date: String,
    /// Window end
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingSummary {
    /// Currency symbol
    pub ccy_symbol: String,
    /// Change vs previous period
    pub change: Option<String>,
    /// Simplified rating distribution
    pub evaluate: RatingSummaryEvaluate,
    /// Consensus recommendation
    pub recommend: crate::types::InstitutionRecommend,
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct RatingSummaryEvaluate {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingDetail {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingDetailEvaluate {
    /// Weekly rating snapshots
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingDetailEvaluateItem {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingDetailTarget {
    /// Prediction accuracy ratio (may be null)
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InstitutionRatingDetailTargetItem {
    /// Average target price
    pub avg_target: Option<String>,
    /// Date string
    pub date: String,
    /// Highest target price
    pub max_target: Option<String>,
    /// Lowest target price
    pub min_target: Option<String>,
    /// Whether the stock reached the target
    pub meet: bool,
    /// Actual stock price
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ForecastEps {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ForecastEpsItem {
    /// Median EPS estimate
    pub forecast_eps_median: Option<String>,
    /// Mean EPS estimate
    pub forecast_eps_mean: Option<String>,
    /// Lowest EPS estimate
    pub forecast_eps_lowest: Option<String>,
    /// Highest EPS estimate
    pub forecast_eps_highest: Option<String>,
    /// Total forecasting institutions
    pub institution_total: i32,
    /// Institutions that raised their estimate
    pub institution_up: i32,
    /// Institutions that lowered their estimate
    pub institution_down: i32,
    /// Forecast window start (ms timestamp)
    pub forecast_start_date: i64,
    /// Forecast window end (ms timestamp)
    pub forecast_end_date: i64,
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
            forecast_start_date: v.forecast_start_date.unix_timestamp(),
            forecast_end_date: v.forecast_end_date.unix_timestamp(),
        }
    }
}

// ── FinancialConsensus ────────────────────────────────────────────

/// Financial consensus estimates response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct FinancialConsensus {
    /// Per-period consensus reports
    pub list: Vec<ConsensusReport>,
    /// Index of most recently released period
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ConsensusReport {
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

/// Consensus estimate for one metric
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ConsensusDetail {
    /// Metric key
    pub key: String,
    /// Display name
    pub name: String,
    /// Metric description
    pub description: String,
    /// Actual value
    pub actual: Option<String>,
    /// Consensus estimate
    pub estimate: Option<String>,
    /// Actual minus estimate
    pub comp_value: Option<String>,
    /// Beat/miss description
    pub comp_desc: String,
    /// Comparison code
    pub comp: String,
    /// Whether actual results are published
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationData {
    /// Valuation metrics
    pub metrics: ValuationMetricsData,
}

impl From<lb::ValuationData> for ValuationData {
    fn from(v: lb::ValuationData) -> Self {
        Self {
            metrics: v.metrics.into(),
        }
    }
}

/// Valuation metrics container
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationMetricsData {
    /// PE ratio history
    pub pe: Option<ValuationMetricData>,
    /// PB ratio history
    pub pb: Option<ValuationMetricData>,
    /// PS ratio history
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationMetricData {
    /// Description
    pub desc: String,
    /// Historical high
    pub high: Option<String>,
    /// Historical low
    pub low: Option<String>,
    /// Historical median
    pub median: Option<String>,
    /// Data points
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationPoint {
    /// Unix timestamp (seconds)
    pub timestamp: i64,
    /// Metric value
    pub value: Option<String>,
}

impl From<lb::ValuationPoint> for ValuationPoint {
    fn from(v: lb::ValuationPoint) -> Self {
        Self {
            timestamp: v.timestamp.unix_timestamp(),
            value: v.value.map(|d| d.to_string()),
        }
    }
}

// ── ValuationHistoryResponse ──────────────────────────────────────

/// Historical valuation response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationHistoryResponse {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationHistoryData {
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

/// Historical metrics container
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationHistoryMetrics {
    /// PE history
    pub pe: Option<ValuationHistoryMetric>,
    /// PB history
    pub pb: Option<ValuationHistoryMetric>,
    /// PS history
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationHistoryMetric {
    /// Description
    pub desc: String,
    /// High
    pub high: Option<String>,
    /// Low
    pub low: Option<String>,
    /// Median
    pub median: Option<String>,
    /// Data points
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct IndustryValuationList {
    /// Peer securities
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct IndustryValuationItem {
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
    /// 5-year avg dividends per share
    pub five_y_avg_dps: Option<String>,
    /// PE ratio
    pub pe: Option<String>,
    /// Historical snapshots
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct IndustryValuationHistory {
    /// Unix timestamp string
    pub date: String,
    /// PE ratio
    pub pe: Option<String>,
    /// PB ratio
    pub pb: Option<String>,
    /// PS ratio
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct IndustryValuationDist {
    /// PE distribution
    pub pe: Option<ValuationDist>,
    /// PB distribution
    pub pb: Option<ValuationDist>,
    /// PS distribution
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationDist {
    /// Minimum value
    pub low: Option<String>,
    /// Maximum value
    pub high: Option<String>,
    /// Median value
    pub median: Option<String>,
    /// Current value
    pub value: Option<String>,
    /// Percentile ranking
    pub ranking: Option<String>,
    /// Ordinal rank index
    pub rank_index: String,
    /// Total securities in industry
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CompanyOverview {
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
    /// Office address
    pub office_address: String,
    /// Website
    pub website: String,
    /// IPO price
    pub issue_price: Option<String>,
    /// Shares offered at IPO
    pub shares_offered: String,
    /// Chairman
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
    /// Email
    pub email: String,
    /// Legal representative
    pub legal_repr: String,
    /// CEO / MD
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
    /// Exchange ticker
    pub ticker: String,
    /// Logo URL
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ExecutiveList {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ExecutiveGroup {
    /// Security symbol
    pub symbol: String,
    /// Company wiki URL
    pub forward_url: String,
    /// Total executives
    pub total: i32,
    /// Individual executives
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Professional {
    /// Internal wiki ID
    pub id: String,
    /// Full name
    pub name: String,
    /// Name in Simplified Chinese
    pub name_zhcn: String,
    /// Name in English
    pub name_en: String,
    /// Job title
    pub title: String,
    /// Biography
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ShareholderList {
    /// Major shareholders
    pub shareholder_list: Vec<Shareholder>,
    /// Link to full shareholder page
    pub forward_url: String,
    /// Total returned
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Shareholder {
    /// Internal ID
    pub shareholder_id: String,
    /// Name
    pub shareholder_name: String,
    /// Institution type
    pub institution_type: String,
    /// Percentage held
    pub percent_of_shares: Option<String>,
    /// Change in shares held
    pub shares_changed: Option<String>,
    /// Report date
    pub report_date: String,
    /// Cross-holdings
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

/// A cross-held security
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ShareholderStock {
    /// Symbol
    pub symbol: String,
    /// Ticker code
    pub code: String,
    /// Market
    pub market: String,
    /// Day change
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct FundHolders {
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

/// A fund or ETF holding the security
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct FundHolder {
    /// Ticker code
    pub code: String,
    /// Symbol
    pub symbol: String,
    /// Currency
    pub currency: String,
    /// Name
    pub name: String,
    /// Position ratio %
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CorpActions {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CorpActionItem {
    /// Internal ID
    pub id: String,
    /// Date in YYYYMMDD format
    pub date: String,
    /// Short display date
    pub date_str: String,
    /// Date type label
    pub date_type: String,
    /// Time zone description
    pub date_zone: String,
    /// Event category
    pub act_type: String,
    /// Description
    pub act_desc: String,
    /// Machine-readable action code
    pub action: String,
    /// Whether recent
    pub recent: bool,
    /// Whether delayed
    pub is_delay: bool,
    /// Delay content
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

/// Live stream for a corp action
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct CorpActionLive {
    /// Stream ID
    pub id: String,
    /// Status code (may be integer or string in API)
    pub status: String,
    /// Start time
    pub started_at: String,
    /// Title
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InvestRelations {
    /// Link to IR page
    pub forward_url: String,
    /// Securities with a stake
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

/// A security in which the company has a stake
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct InvestSecurity {
    /// Company ID
    pub company_id: String,
    /// Company name
    pub company_name: String,
    /// Company name in English
    pub company_name_en: String,
    /// Company name in Simplified Chinese
    pub company_name_zhcn: String,
    /// Security symbol
    pub symbol: String,
    /// Currency
    pub currency: String,
    /// Percentage held
    pub percent_of_shares: Option<String>,
    /// Shareholder rank
    pub shares_rank: String,
    /// Market value of holding
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct OperatingList {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct OperatingItem {
    /// Report ID
    pub id: String,
    /// Period code
    pub report: String,
    /// Title
    pub title: String,
    /// Management discussion text
    pub txt: String,
    /// Whether most recent
    pub latest: bool,
    /// Community page URL
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct OperatingFinancial {
    /// Ticker code
    pub code: String,
    /// Currency
    pub currency: String,
    /// Company name
    pub name: String,
    /// Region
    pub region: String,
    /// Report period code
    pub report: String,
    /// Indicators
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

/// One financial indicator
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct OperatingIndicator {
    /// Field key
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
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum FinancialReportKind {
    /// Income statement
    IncomeStatement,
    /// Balance sheet
    BalanceSheet,
    /// Cash flow statement
    CashFlow,
    /// All statements
    All,
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

/// Financial report period type
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum FinancialReportPeriod {
    /// Annual report
    Annual,
    /// Semi-annual report
    SemiAnnual,
    /// Q1 report
    Q1,
    /// Q2 report
    Q2,
    /// Q3 report
    Q3,
    /// Full quarterly report
    QuarterlyFull,
    /// Three-quarter report (first three quarters)
    ThreeQ,
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

// ── BuybackData ───────────────────────────────────────────────────

/// TTM buyback summary
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct RecentBuybacks {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BuybackHistoryItem {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BuybackRatios {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BuybackData {
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
/// `ratingsJson` contains the full nested ratings structure as a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct StockRatings {
    pub style_txt_name: String,
    pub scale_txt_name: String,
    pub report_period_txt: String,
    /// Composite score as a JSON string
    pub multi_score: String,
    pub multi_letter: String,
    pub multi_score_change: i32,
    pub industry_name: String,
    pub industry_rank: i64,
    /// Full ratings array as a JSON string
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

// ── ShareholderTopResponse ────────────────────────────────────────

/// Top-shareholder list response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ShareholderTopResponse {
    /// Raw top-shareholder data (JSON string)
    pub data: String,
}

impl From<lb::ShareholderTopResponse> for ShareholderTopResponse {
    fn from(v: lb::ShareholderTopResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── ShareholderDetailResponse ─────────────────────────────────────

/// Shareholder detail response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ShareholderDetailResponse {
    /// Raw shareholder detail data (JSON string)
    pub data: String,
}

impl From<lb::ShareholderDetailResponse> for ShareholderDetailResponse {
    fn from(v: lb::ShareholderDetailResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── ValuationComparisonResponse ───────────────────────────────────

/// One historical valuation data point.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationHistoryPoint {
    /// Date (RFC 3339)
    pub date: String,
    /// P/E ratio
    pub pe: String,
    /// P/B ratio
    pub pb: String,
    /// P/S ratio
    pub ps: String,
}

impl From<lb::ValuationHistoryPoint> for ValuationHistoryPoint {
    fn from(v: lb::ValuationHistoryPoint) -> Self {
        Self {
            date: v.date,
            pe: v.pe,
            pb: v.pb,
            ps: v.ps,
        }
    }
}

/// One security's valuation comparison item.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationComparisonItem {
    /// Symbol (e.g. `"AAPL.US"`)
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Currency
    pub currency: String,
    /// Market capitalisation
    pub market_value: String,
    /// Latest closing price
    pub price_close: String,
    /// P/E ratio
    pub pe: String,
    /// P/B ratio
    pub pb: String,
    /// P/S ratio
    pub ps: String,
    /// Return on equity
    pub roe: String,
    /// Earnings per share
    pub eps: String,
    /// Book value per share
    pub bps: String,
    /// Dividends per share
    pub dps: String,
    /// Dividend yield
    pub div_yld: String,
    /// Total assets
    pub assets: String,
    /// Historical valuation points
    pub history: Vec<ValuationHistoryPoint>,
}

impl From<lb::ValuationComparisonItem> for ValuationComparisonItem {
    fn from(v: lb::ValuationComparisonItem) -> Self {
        Self {
            symbol: v.symbol,
            name: v.name,
            currency: v.currency,
            market_value: v.market_value,
            price_close: v.price_close,
            pe: v.pe,
            pb: v.pb,
            ps: v.ps,
            roe: v.roe,
            eps: v.eps,
            bps: v.bps,
            dps: v.dps,
            div_yld: v.div_yld,
            assets: v.assets,
            history: v.history.into_iter().map(Into::into).collect(),
        }
    }
}

/// Valuation comparison response.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ValuationComparisonResponse {
    /// Valuation comparison items
    pub list: Vec<ValuationComparisonItem>,
}

impl From<lb::ValuationComparisonResponse> for ValuationComparisonResponse {
    fn from(v: lb::ValuationComparisonResponse) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

// ── etf_asset_allocation ──────────────────────────────────────────

/// ETF asset allocation element type
#[napi_derive::napi]
#[derive(longbridge_nodejs_macros::JsEnum, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::fundamental::types::ElementType")]
pub enum ElementType {
    /// Unknown
    Unknown,
    /// Holdings
    Holdings,
    /// Regional
    Regional,
    /// Asset class
    AssetClass,
    /// Industry
    Industry,
}

/// Holding detail of an ETF asset allocation element (holdings only)
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct HoldingDetail {
    /// Industry ID
    pub industry_id: String,
    /// Industry name
    pub industry_name: String,
    /// Index counter ID (e.g. `BK/US/CP99000`)
    pub index: String,
    /// Index name
    pub index_name: String,
    /// Holding type (e.g. `E` for stock)
    pub holding_type: String,
    /// Holding type name
    pub holding_type_name: String,
}

impl From<lb::HoldingDetail> for HoldingDetail {
    fn from(v: lb::HoldingDetail) -> Self {
        Self {
            industry_id: v.industry_id,
            industry_name: v.industry_name,
            index: v.index,
            index_name: v.index_name,
            holding_type: v.holding_type,
            holding_type_name: v.holding_type_name,
        }
    }
}

/// One element of an ETF asset allocation group
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AssetAllocationItem {
    /// Element name
    pub name: String,
    /// Security code (holdings only, e.g. `NVDA`)
    pub code: String,
    /// Position ratio (e.g. `0.0861114`)
    pub position_ratio: String,
    /// Security symbol (holdings only, e.g. `NVDA.US`)
    pub symbol: String,
    /// Localized names (locale → name)
    pub name_locales: std::collections::HashMap<String, String>,
    /// Holding detail (holdings only)
    pub holding_detail: Option<HoldingDetail>,
}

impl From<lb::AssetAllocationItem> for AssetAllocationItem {
    fn from(v: lb::AssetAllocationItem) -> Self {
        Self {
            name: v.name,
            code: v.code,
            position_ratio: v.position_ratio,
            symbol: v.symbol,
            name_locales: v.name_locales,
            holding_detail: v.holding_detail.map(Into::into),
        }
    }
}

/// One ETF asset allocation group (grouped by element type)
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AssetAllocationGroup {
    /// Report date (e.g. `20260601`)
    pub report_date: String,
    /// Element type of this group
    pub asset_type: ElementType,
    /// Elements
    pub lists: Vec<AssetAllocationItem>,
}

impl From<lb::AssetAllocationGroup> for AssetAllocationGroup {
    fn from(v: lb::AssetAllocationGroup) -> Self {
        Self {
            report_date: v.report_date,
            asset_type: v.asset_type.into(),
            lists: v.lists.into_iter().map(Into::into).collect(),
        }
    }
}

/// ETF asset allocation response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AssetAllocationResponse {
    /// Asset allocation groups
    pub info: Vec<AssetAllocationGroup>,
}

impl From<lb::AssetAllocationResponse> for AssetAllocationResponse {
    fn from(v: lb::AssetAllocationResponse) -> Self {
        Self {
            info: v.info.into_iter().map(Into::into).collect(),
        }
    }
}

// ── economic_indicator ─────────────────────────────────────────────────────

/// Localized text in simplified Chinese, traditional Chinese, and English
#[napi_derive::napi(object)]
#[derive(Debug, Clone, Default)]
pub struct MultiLanguageText {
    pub english: String,
    pub simplified_chinese: String,
    pub traditional_chinese: String,
}

impl From<lb::MultiLanguageText> for MultiLanguageText {
    fn from(v: lb::MultiLanguageText) -> Self {
        Self {
            english: v.english,
            simplified_chinese: v.simplified_chinese,
            traditional_chinese: v.traditional_chinese,
        }
    }
}

/// Country code for filtering macroeconomic indicators
#[napi_derive::napi]
#[derive(Debug, Copy, Clone)]
pub enum MacroeconomicCountry {
    /// Hong Kong SAR China
    HongKong,
    /// China (Mainland)
    China,
    /// United States
    UnitedStates,
    /// Euro Zone
    EuroZone,
    /// Japan
    Japan,
    /// Singapore
    Singapore,
}

impl From<MacroeconomicCountry> for lb::MacroeconomicCountry {
    fn from(v: MacroeconomicCountry) -> Self {
        match v {
            MacroeconomicCountry::HongKong => lb::MacroeconomicCountry::HongKong,
            MacroeconomicCountry::China => lb::MacroeconomicCountry::China,
            MacroeconomicCountry::UnitedStates => lb::MacroeconomicCountry::UnitedStates,
            MacroeconomicCountry::EuroZone => lb::MacroeconomicCountry::EuroZone,
            MacroeconomicCountry::Japan => lb::MacroeconomicCountry::Japan,
            MacroeconomicCountry::Singapore => lb::MacroeconomicCountry::Singapore,
        }
    }
}

/// Response for macroeconomic_indicators
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MacroeconomicIndicatorListResponse {
    pub data: Vec<MacroeconomicIndicator>,
    pub count: i32,
}

impl From<lb::MacroeconomicIndicatorListResponse> for MacroeconomicIndicatorListResponse {
    fn from(v: lb::MacroeconomicIndicatorListResponse) -> Self {
        Self {
            data: v.data.into_iter().map(Into::into).collect(),
            count: v.count,
        }
    }
}

/// Metadata for one macroeconomic indicator
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MacroeconomicIndicator {
    pub indicator_code: String,
    pub source_org: String,
    pub country: String,
    pub name: String,
    pub adjustment_factor: String,
    pub periodicity: String,
    pub category: String,
    pub describe: String,
    pub importance: i32,
    /// Start date of data coverage (unix timestamp in seconds; null if unset)
    pub start_date: Option<i64>,
}

impl From<lb::MacroeconomicIndicator> for MacroeconomicIndicator {
    fn from(v: lb::MacroeconomicIndicator) -> Self {
        Self {
            indicator_code: v.indicator_code,
            source_org: v.source_org,
            country: v.country,
            name: v.name,
            adjustment_factor: v.adjustment_factor,
            periodicity: v.periodicity,
            category: v.category,
            describe: v.describe,
            importance: v.importance,
            start_date: v.start_date.map(|dt| dt.unix_timestamp()),
        }
    }
}

/// One historical data point for a macroeconomic indicator
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Macroeconomic {
    pub period: String,
    /// Release datetime (unix timestamp in seconds; null if unset)
    pub release_at: Option<i64>,
    pub actual_value: String,
    pub previous_value: String,
    pub forecast_value: String,
    pub revised_value: String,
    /// Next release datetime (unix timestamp in seconds; null if unset)
    pub next_release_at: Option<i64>,
    pub unit: String,
    pub unit_prefix: String,
}

impl From<lb::Macroeconomic> for Macroeconomic {
    fn from(v: lb::Macroeconomic) -> Self {
        Self {
            period: v.period,
            release_at: v.release_at.map(|dt| dt.unix_timestamp()),
            actual_value: v.actual_value,
            previous_value: v.previous_value,
            forecast_value: v.forecast_value,
            revised_value: v.revised_value,
            next_release_at: v.next_release_at.map(|dt| dt.unix_timestamp()),
            unit: v.unit,
            unit_prefix: v.unit_prefix,
        }
    }
}

/// Response for macroeconomic
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MacroeconomicResponse {
    pub info: MacroeconomicIndicator,
    pub data: Vec<Macroeconomic>,
    pub count: i32,
}

impl From<lb::MacroeconomicResponse> for MacroeconomicResponse {
    fn from(v: lb::MacroeconomicResponse) -> Self {
        Self {
            info: v.info.into(),
            data: v.data.into_iter().map(Into::into).collect(),
            count: v.count,
        }
    }
}
