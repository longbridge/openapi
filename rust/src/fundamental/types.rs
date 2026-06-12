#![allow(missing_docs)]

use std::collections::HashMap;

use num_enum::{FromPrimitive, IntoPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use time::OffsetDateTime;

use crate::utils::counter::deserialize_counter_id_as_symbol;

// ── financial_report ─────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::financial_report`]
///
/// The `list` field contains deeply-nested indicator/account/value data keyed
/// by report kind (`"IS"`, `"BS"`, `"CF"`).  The exact structure varies and is
/// preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialReports {
    /// Raw nested financial data. Top-level keys are report kinds such as
    /// `"IS"` (income statement), `"BS"` (balance sheet), `"CF"` (cash flow).
    pub list: serde_json::Value,
}

// ── dividend ─────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::dividend`] and
/// [`crate::FundamentalContext::dividend_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendList {
    /// List of dividend events
    pub list: Vec<DividendItem>,
}

/// A single dividend / distribution event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendItem {
    /// Security symbol, e.g. `"700.HK"`
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Internal record ID (may be absent in dividend_detail response)
    #[serde(default)]
    pub id: String,
    /// Human-readable description, e.g. `"每股派息 5.3 HKD"`
    pub desc: String,
    /// Record / book-close date, e.g. `"2026.05.18"`
    pub record_date: String,
    /// Ex-dividend date, e.g. `"2026.05.15"`
    pub ex_date: String,
    /// Payment date, e.g. `"2026.06.01"`
    pub payment_date: String,
}

// ── institution_rating ────────────────────────────────────────────

/// Combined analyst-rating response for
/// [`crate::FundamentalContext::institution_rating`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRating {
    /// Latest snapshot from `/v1/quote/institution-rating-latest`
    pub latest: InstitutionRatingLatest,
    /// Consensus summary from `/v1/quote/institution-ratings`
    pub summary: InstitutionRatingSummary,
}

/// Latest analyst-rating snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingLatest {
    /// Rating distribution counts and date range
    pub evaluate: RatingEvaluate,
    /// Target price range
    pub target: RatingTarget,
    /// Industry classification ID
    pub industry_id: i64,
    /// Industry name
    pub industry_name: String,
    /// Rank of this security within the industry (1 = highest)
    pub industry_rank: i32,
    /// Total number of securities in the industry
    pub industry_total: i32,
    /// Mean analyst count in the industry
    pub industry_mean: i32,
    /// Median analyst count in the industry
    pub industry_median: i32,
}

/// Analyst rating distribution counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingEvaluate {
    /// Number of "Buy" ratings
    pub buy: i32,
    /// Number of "Strong Buy" / "Outperform" ratings
    pub over: i32,
    /// Number of "Hold" / "Neutral" ratings
    pub hold: i32,
    /// Number of "Underperform" ratings
    pub under: i32,
    /// Number of "Sell" ratings
    pub sell: i32,
    /// Number of "No Opinion" ratings
    pub no_opinion: i32,
    /// Total analyst count
    pub total: i32,
    /// Window start (unix timestamp string; `"0"` means unset)
    pub start_date: String,
    /// Window end (unix timestamp string; `"0"` means unset)
    pub end_date: String,
}

/// Analyst target price range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingTarget {
    /// Highest price target
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub highest_price: Option<Decimal>,
    /// Lowest price target
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub lowest_price: Option<Decimal>,
    /// Previous close price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub prev_close: Option<Decimal>,
    /// Window start (unix timestamp string)
    pub start_date: String,
    /// Window end (unix timestamp string)
    pub end_date: String,
}

/// Consensus summary from `/v1/quote/institution-ratings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingSummary {
    /// Currency symbol, e.g. `"HK$"`
    pub ccy_symbol: String,
    /// Change vs previous period
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub change: Option<Decimal>,
    /// Simplified rating distribution
    pub evaluate: RatingSummaryEvaluate,
    /// Overall recommendation
    pub recommend: InstitutionRecommend,
    /// Consensus target price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub target: Option<Decimal>,
    /// Last updated display string, e.g. `"2026 年 5 月 5 日"`
    pub updated_at: String,
}

/// Simplified rating distribution for the consensus summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingSummaryEvaluate {
    /// Number of "Buy" ratings
    pub buy: i32,
    /// Date of the latest update
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

// ── institution_rating_detail ─────────────────────────────────────

/// Response for [`crate::FundamentalContext::institution_rating_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingDetail {
    /// Currency symbol, e.g. `"HK$"`
    pub ccy_symbol: String,
    /// Historical rating distribution time-series
    pub evaluate: InstitutionRatingDetailEvaluate,
    /// Historical target price time-series
    pub target: InstitutionRatingDetailTarget,
}

/// Historical rating distribution time-series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingDetailEvaluate {
    /// Weekly snapshots ordered from oldest to newest
    pub list: Vec<InstitutionRatingDetailEvaluateItem>,
}

/// One weekly rating distribution snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default)]
    pub strong_buy: i32,
    /// Number of "No Opinion" ratings
    #[serde(default)]
    pub no_opinion: i32,
    /// Number of "Underperform" ratings
    pub under: i32,
}

/// Historical target price time-series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingDetailTarget {
    /// Prediction accuracy ratio, e.g. `"0.9934"` (may be `null`)
    pub data_percent: Option<Decimal>,
    /// Overall prediction accuracy percentage string
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub prediction_accuracy: Option<Decimal>,
    /// Last updated display string
    pub updated_at: String,
    /// Weekly target price snapshots
    pub list: Vec<InstitutionRatingDetailTargetItem>,
}

/// One weekly target price snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingDetailTargetItem {
    /// Average target price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub avg_target: Option<Decimal>,
    /// Date in `"2021/05/16"` format
    pub date: String,
    /// Highest target price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub max_target: Option<Decimal>,
    /// Lowest target price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub min_target: Option<Decimal>,
    /// Whether the stock price reached the target
    pub meet: bool,
    /// Actual stock price at this date
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub price: Option<Decimal>,
    /// Unix timestamp string
    pub timestamp: String,
}

// ── forecast_eps ──────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::forecast_eps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastEps {
    /// EPS forecast snapshots ordered by `forecast_start_date` ascending
    pub items: Vec<ForecastEpsItem>,
}

/// One EPS forecast snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastEpsItem {
    /// Median EPS estimate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub forecast_eps_median: Option<Decimal>,
    /// Mean EPS estimate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub forecast_eps_mean: Option<Decimal>,
    /// Lowest EPS estimate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub forecast_eps_lowest: Option<Decimal>,
    /// Highest EPS estimate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub forecast_eps_highest: Option<Decimal>,
    /// Total number of forecasting institutions
    pub institution_total: i32,
    /// Number of institutions that raised their estimate
    pub institution_up: i32,
    /// Number of institutions that lowered their estimate
    pub institution_down: i32,
    /// Forecast window start
    #[serde(deserialize_with = "crate::serde_utils::deserialize_timestamp")]
    pub forecast_start_date: OffsetDateTime,
    /// Forecast window end
    #[serde(deserialize_with = "crate::serde_utils::deserialize_timestamp")]
    pub forecast_end_date: OffsetDateTime,
}

// ── consensus ─────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::consensus`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialConsensus {
    /// Per-period consensus reports
    pub list: Vec<ConsensusReport>,
    /// Index into `list` of the most recently released period
    pub current_index: i32,
    /// Reporting currency, e.g. `"HKD"`
    pub currency: String,
    /// Available period types, e.g. `["qf", "saf", "af"]`
    #[serde(default)]
    pub opt_periods: Vec<String>,
    /// Currently returned period type
    pub current_period: String,
}

/// Consensus report for one fiscal period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusReport {
    /// Fiscal year, e.g. `2025`
    pub fiscal_year: i32,
    /// Fiscal period code, e.g. `"Q4"`
    pub fiscal_period: String,
    /// Human-readable period label, e.g. `"Q4 FY2025"`
    pub period_text: String,
    /// Per-metric consensus details
    pub details: Vec<ConsensusDetail>,
}

/// Consensus estimate for one financial metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusDetail {
    /// Metric key, e.g. `"revenue"`, `"eps"`
    pub key: String,
    /// Display name
    pub name: String,
    /// Metric description
    pub description: String,
    /// Actual reported value (empty string if not yet released)
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub actual: Option<Decimal>,
    /// Consensus estimate value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub estimate: Option<Decimal>,
    /// Actual minus estimate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub comp_value: Option<Decimal>,
    /// Beat/miss description, e.g. `"超出预期"`
    pub comp_desc: String,
    /// Comparison result code for colour coding
    pub comp: String,
    /// Whether the actual results have been published
    pub is_released: bool,
}

// ── valuation ─────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::valuation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationData {
    /// Valuation metrics (PE / PB / PS / dividend yield)
    pub metrics: ValuationMetricsData,
}

/// Container for all valuation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationMetricsData {
    /// Price-to-Earnings ratio history
    pub pe: Option<ValuationMetricData>,
    /// Price-to-Book ratio history
    pub pb: Option<ValuationMetricData>,
    /// Price-to-Sales ratio history
    pub ps: Option<ValuationMetricData>,
    /// Dividend yield history
    pub dvd_yld: Option<ValuationMetricData>,
}

/// Historical time-series for one valuation metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationMetricData {
    /// Human-readable description with current value and percentile
    pub desc: String,
    /// Historical high value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub high: Option<Decimal>,
    /// Historical low value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub low: Option<Decimal>,
    /// Historical median value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub median: Option<Decimal>,
    /// Historical data points
    pub list: Vec<ValuationPoint>,
}

/// One valuation data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationPoint {
    /// Date of the data point
    #[serde(deserialize_with = "crate::serde_utils::deserialize_timestamp")]
    pub timestamp: OffsetDateTime,
    /// Metric value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub value: Option<Decimal>,
}

// ── valuation_history ─────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::valuation_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationHistoryResponse {
    /// Historical valuation data
    pub history: ValuationHistoryData,
}

/// Container for historical valuation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationHistoryData {
    /// Historical metrics (PE / PB / PS)
    pub metrics: ValuationHistoryMetrics,
}

/// Historical valuation metrics container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationHistoryMetrics {
    /// Price-to-Earnings history
    pub pe: Option<ValuationHistoryMetric>,
    /// Price-to-Book history
    pub pb: Option<ValuationHistoryMetric>,
    /// Price-to-Sales history
    pub ps: Option<ValuationHistoryMetric>,
}

/// Historical data for one valuation metric including statistical bounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationHistoryMetric {
    /// Human-readable description
    pub desc: String,
    /// Historical high over the period
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub high: Option<Decimal>,
    /// Historical low over the period
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub low: Option<Decimal>,
    /// Historical median over the period
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub median: Option<Decimal>,
    /// Historical data points
    pub list: Vec<ValuationPoint>,
}

// ── industry_valuation ────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::industry_valuation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryValuationList {
    /// List of peer securities with their valuation data
    pub list: Vec<IndustryValuationItem>,
}

/// Valuation data for one peer security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryValuationItem {
    /// Security symbol, e.g. `"700.HK"`
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Company name
    pub name: String,
    /// Reporting currency
    pub currency: String,
    /// Total assets
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub assets: Option<Decimal>,
    /// Book value per share
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub bps: Option<Decimal>,
    /// Earnings per share
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub eps: Option<Decimal>,
    /// Dividends per share
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub dps: Option<Decimal>,
    /// Dividend yield
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub div_yld: Option<Decimal>,
    /// Dividend payout ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub div_payout_ratio: Option<Decimal>,
    /// 5-year average dividends per share
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub five_y_avg_dps: Option<Decimal>,
    /// Current PE ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub pe: Option<Decimal>,
    /// Historical PE/PB/PS snapshots
    pub history: Vec<IndustryValuationHistory>,
}

/// Historical valuation snapshot for an industry peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryValuationHistory {
    /// Unix timestamp string
    pub date: String,
    /// Price-to-Earnings ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub pe: Option<Decimal>,
    /// Price-to-Book ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub pb: Option<Decimal>,
    /// Price-to-Sales ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub ps: Option<Decimal>,
}

// ── industry_valuation_dist ───────────────────────────────────────

/// Response for [`crate::FundamentalContext::industry_valuation_dist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryValuationDist {
    /// PE ratio distribution within the industry
    pub pe: Option<ValuationDist>,
    /// PB ratio distribution within the industry
    pub pb: Option<ValuationDist>,
    /// PS ratio distribution within the industry
    pub ps: Option<ValuationDist>,
}

/// Distribution statistics for one valuation metric within an industry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationDist {
    /// Minimum value in the industry
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub low: Option<Decimal>,
    /// Maximum value in the industry
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub high: Option<Decimal>,
    /// Median value in the industry
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub median: Option<Decimal>,
    /// Current value of the queried security
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub value: Option<Decimal>,
    /// Percentile ranking (0–1 range as string)
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub ranking: Option<Decimal>,
    /// Ordinal rank index (1-based)
    pub rank_index: String,
    /// Total number of securities in the industry
    pub rank_total: String,
}

// ── company ───────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyOverview {
    /// Short name, e.g. `"腾讯控股"`
    pub name: String,
    /// Full legal name
    pub company_name: String,
    /// Founding date
    pub founded: String,
    /// Listing date
    pub listing_date: String,
    /// Primary listing market display name
    pub market: String,
    /// Market region code, e.g. `"HK"`
    pub region: String,
    /// Registered address
    pub address: String,
    /// Principal office address
    pub office_address: String,
    /// Company website
    pub website: String,
    /// IPO issue price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub issue_price: Option<Decimal>,
    /// Number of shares offered at IPO
    pub shares_offered: String,
    /// Chairman name
    pub chairman: String,
    /// Company secretary name
    pub secretary: String,
    /// Auditing institution
    pub audit_inst: String,
    /// Company classification category
    pub category: String,
    /// Fiscal year end, e.g. `"12 月 31 日"`
    pub year_end: String,
    /// Number of employees
    pub employees: String,
    /// Phone number (API field name is `"Phone"`)
    #[serde(rename = "Phone")]
    pub phone: String,
    /// Fax number
    pub fax: String,
    /// Investor relations email
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
    /// Exchange ticker code, e.g. `"00700"`
    pub ticker: String,
    /// URL to the company's logo icon
    pub icon: String,
    /// Business profile / description
    pub profile: String,
    /// ADS ratio (may be empty)
    #[serde(default)]
    pub ads_ratio: String,
    /// Industry sector code
    pub sector: i32,
}

// ── executive ─────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::executive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveList {
    /// Groups of executives per security (usually one group)
    pub professional_list: Vec<ExecutiveGroup>,
}

/// Executives for one security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveGroup {
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Link to the company wiki page
    pub forward_url: String,
    /// Total number of executives
    pub total: i32,
    /// Individual executive entries
    pub professionals: Vec<Professional>,
}

/// One executive / board member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Professional {
    /// Internal wiki person ID (string form)
    pub id: String,
    /// Full name
    pub name: String,
    /// Full name in Simplified Chinese
    pub name_zhcn: String,
    /// Full name in English
    pub name_en: String,
    /// Job title, e.g. `"Co-Founder, Chairman & CEO"`
    pub title: String,
    /// Biography text
    pub biography: String,
    /// URL to the person's photo
    pub photo: String,
    /// URL to the wiki profile page
    pub wiki_url: String,
}

// ── shareholder ───────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::shareholder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderList {
    /// List of major shareholders
    pub shareholder_list: Vec<Shareholder>,
    /// Link to the full shareholder page
    #[serde(default)]
    pub forward_url: String,
    /// Total number of shareholders returned
    pub total: i32,
}

/// One major shareholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shareholder {
    /// Internal shareholder ID (string form)
    pub shareholder_id: String,
    /// Shareholder name
    pub shareholder_name: String,
    /// Institution type (may be empty)
    pub institution_type: String,
    /// Percentage of shares held
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub percent_of_shares: Option<Decimal>,
    /// Change in shares held (positive = bought, negative = sold)
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub shares_changed: Option<Decimal>,
    /// Date of the most recent filing, e.g. `"2026-05-04"`
    pub report_date: String,
    /// Other securities held by this shareholder (cross-holdings)
    #[serde(default)]
    pub stocks: Vec<ShareholderStock>,
}

/// A security in an institutional shareholder's cross-holdings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderStock {
    /// Security symbol of the cross-held stock
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Ticker code, e.g. `"BLK"`
    pub code: String,
    /// Market, e.g. `"US"`
    pub market: String,
    /// Day change percentage, e.g. `"-0.32%"`
    pub chg: String,
}

// ── fund_holder ───────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::fund_holder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundHolders {
    /// Funds and ETFs that hold the queried security
    pub lists: Vec<FundHolder>,
}

/// A fund or ETF that holds the queried security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundHolder {
    /// Fund/ETF ticker code, e.g. `"513050"`
    pub code: String,
    /// Fund/ETF symbol, e.g. `"ETF/SH/513050"` → converted to `"513050.SH"`
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Reporting currency, e.g. `"CNY"`
    pub currency: String,
    /// Fund/ETF full name
    pub name: String,
    /// Position ratio as a percentage decimal
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub position_ratio: Decimal,
    /// Report date, e.g. `"2025.12.31"`
    pub report_date: String,
}

// ── corp_action ───────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::corp_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpActions {
    /// Corporate action events
    pub items: Vec<CorpActionItem>,
}

/// One corporate action event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpActionItem {
    /// Internal event ID
    pub id: String,
    /// Date in `YYYYMMDD` format, e.g. `"20260601"`
    pub date: String,
    /// Short display date, e.g. `"06.01"`
    pub date_str: String,
    /// Date type label, e.g. `"派息日"`, `"除权日"`
    pub date_type: String,
    /// Time zone description, e.g. `"北京时间"`
    pub date_zone: String,
    /// Event category, e.g. `"分配方案"`
    pub act_type: String,
    /// Human-readable event description
    pub act_desc: String,
    /// Machine-readable action code, e.g. `"DividendExDate"`
    pub action: String,
    /// Whether this is a recent event
    pub recent: bool,
    /// Whether publication was delayed
    pub is_delay: bool,
    /// Delay announcement content (if `is_delay` is `true`)
    pub delay_content: String,
    /// Associated live stream (if any)
    pub live: Option<CorpActionLive>,
    /// Associated security info (rarely populated; preserved as raw JSON)
    pub security: Option<serde_json::Value>,
}

/// Live stream associated with a corporate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpActionLive {
    /// Live stream ID
    pub id: String,
    /// Status code: 1=preview, 2=live, 3=ended, 4=replay, 5=processing
    pub status: serde_json::Value, // API may return int or string
    /// Start time
    pub started_at: String,
    /// Stream title
    pub name: String,
    /// Icon URL
    pub icon: String,
}

// ── invest_relation ───────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::invest_relation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestRelations {
    /// Link to the full investor-relations page
    #[serde(default)]
    pub forward_url: String,
    /// Securities in which the queried company holds a stake
    pub invest_securities: Vec<InvestSecurity>,
}

/// A security in which the queried company has an investment stake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestSecurity {
    /// Internal company ID (string form; may be `"0"`)
    pub company_id: String,
    /// Company name (locale-aware)
    pub company_name: String,
    /// Company name in English
    pub company_name_en: String,
    /// Company name in Simplified Chinese
    pub company_name_zhcn: String,
    /// Security symbol of the invested company
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Reporting currency
    pub currency: String,
    /// Percentage of shares held
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub percent_of_shares: Option<Decimal>,
    /// Shareholder rank, e.g. `"1"` = largest shareholder
    pub shares_rank: String,
    /// Market value of the holding
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub shares_value: Option<Decimal>,
}

// ── operating ─────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::operating`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingList {
    /// List of operating summary reports
    pub list: Vec<OperatingItem>,
}

/// One operating summary report (annual / quarterly)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingItem {
    /// Internal report ID
    pub id: String,
    /// Report period code, e.g. `"af"` (annual), `"qf"` (quarterly)
    pub report: String,
    /// Report title, e.g. `"2025 财年年报"`
    pub title: String,
    /// Management discussion text
    pub txt: String,
    /// Whether this is the most recent report
    pub latest: bool,
    /// Keyword tags (structure undocumented; usually empty)
    #[serde(default)]
    pub keywords: Vec<serde_json::Value>,
    /// URL to the full community report page
    #[serde(default)]
    pub web_url: String,
    /// Key financial metrics extracted from the report
    pub financial: OperatingFinancial,
}

/// Key financial metrics extracted from an operating report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingFinancial {
    /// Ticker code (may be empty)
    pub code: String,
    /// Symbol in `CODE.MARKET` format (may be empty)
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Reporting currency
    pub currency: String,
    /// Company name
    pub name: String,
    /// Market region
    pub region: String,
    /// Report period code
    pub report: String,
    /// Report period display text
    pub report_txt: String,
    /// Financial indicators
    pub indicators: Vec<OperatingIndicator>,
}

/// One financial indicator in an operating report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingIndicator {
    /// Field name key, e.g. `"operating_revenue"`
    pub field_name: String,
    /// Display name, e.g. `"营业收入"`
    pub indicator_name: String,
    /// Formatted value, e.g. `"8217 亿"`
    pub indicator_value: String,
    /// Year-over-year change
    #[serde(default, with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub yoy: Option<Decimal>,
}

// ── buyback ───────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::buyback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuybackData {
    /// Most recent buyback summary (TTM)
    #[serde(default)]
    pub recent_buybacks: Option<RecentBuybacks>,
    /// Historical annual buyback data
    #[serde(default)]
    pub buyback_history: Vec<BuybackHistoryItem>,
    /// Buyback payout and cash-flow ratios
    #[serde(default)]
    pub buyback_ratios: Vec<BuybackRatios>,
}

/// TTM (trailing twelve months) buyback summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentBuybacks {
    /// Reporting currency
    pub currency: String,
    /// Net buyback amount TTM
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback_ttm: Option<Decimal>,
    /// Net buyback yield TTM
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback_yield_ttm: Option<Decimal>,
}

/// Historical annual buyback data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuybackHistoryItem {
    /// Fiscal year label, e.g. `"FY2024"`
    pub fiscal_year: String,
    /// Fiscal year date range string
    pub fiscal_year_range: String,
    /// Net buyback amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback: Option<Decimal>,
    /// Net buyback yield
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback_yield: Option<Decimal>,
    /// Year-over-year net buyback growth rate
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback_growth_rate: Option<Decimal>,
    /// Reporting currency
    pub currency: String,
}

/// Buyback payout and cash-flow ratios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuybackRatios {
    /// Net buyback payout ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback_payout_ratio: Option<Decimal>,
    /// Net buyback to free cash-flow ratio
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub net_buyback_to_cashflow_ratio: Option<Decimal>,
}

// ── ratings ───────────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::ratings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockRatings {
    /// Style display name
    #[serde(default)]
    pub style_txt_name: String,
    /// Scale display name
    #[serde(default)]
    pub scale_txt_name: String,
    /// Report period display text
    #[serde(default)]
    pub report_period_txt: String,
    /// Composite score (may be int, float, or null)
    #[serde(default)]
    pub multi_score: serde_json::Value,
    /// Composite score letter grade
    #[serde(default)]
    pub multi_letter: String,
    /// Score change vs previous period
    #[serde(default)]
    pub multi_score_change: i32,
    /// Industry name
    #[serde(default)]
    pub industry_name: String,
    /// Industry rank (may be int or null)
    #[serde(default)]
    pub industry_rank: serde_json::Value,
    /// Total securities in the industry
    #[serde(default)]
    pub industry_total: serde_json::Value,
    /// Industry mean score
    #[serde(default)]
    pub industry_mean_score: serde_json::Value,
    /// Industry median score
    #[serde(default)]
    pub industry_median_score: serde_json::Value,
    /// Detailed rating categories
    #[serde(default)]
    pub ratings: Vec<RatingCategory>,
}

/// One rating category (e.g. growth, profitability)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingCategory {
    /// Category type code
    #[serde(rename = "type")]
    pub kind: i32,
    /// Sub-indicator groups within this category
    #[serde(default)]
    pub sub_indicators: Vec<RatingSubIndicatorGroup>,
}

/// A group of sub-indicators under one category indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingSubIndicatorGroup {
    /// Parent indicator for this group
    pub indicator: RatingIndicator,
    /// Leaf sub-indicators
    #[serde(default)]
    pub sub_indicators: Vec<RatingLeafIndicator>,
}

/// A rating indicator node (may be a parent or a leaf)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingIndicator {
    /// Indicator display name
    pub name: String,
    /// Score (may be int, float, or null)
    #[serde(default)]
    pub score: serde_json::Value,
    /// Letter grade
    #[serde(default)]
    pub letter: String,
}

/// A leaf rating indicator with a raw value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingLeafIndicator {
    /// Indicator display name
    pub name: String,
    /// Formatted value string
    #[serde(default)]
    pub value: String,
    /// Value type hint, e.g. `"percent"`
    #[serde(default)]
    pub value_type: String,
    /// Score (may be int, float, or null)
    #[serde(default)]
    pub score: serde_json::Value,
    /// Letter grade
    #[serde(default)]
    pub letter: String,
}

// ── enums ─────────────────────────────────────────────────────────

/// Institutional analyst recommendation
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum InstitutionRecommend {
    /// Unknown
    Unknown,
    /// Strong buy
    #[strum(serialize = "strong_buy")]
    StrongBuy,
    /// Buy
    #[strum(serialize = "buy")]
    Buy,
    /// Hold
    #[strum(serialize = "hold")]
    Hold,
    /// Sell
    #[strum(serialize = "sell")]
    Sell,
    /// Strong sell
    #[strum(serialize = "strong_sell")]
    StrongSell,
    /// Underperform
    #[strum(serialize = "underperform")]
    Underperform,
    /// No opinion
    #[strum(serialize = "no_opinion")]
    NoOpinion,
}

impl_default_for_enum_string!(InstitutionRecommend);
impl_serde_for_enum_string!(InstitutionRecommend);

/// Financial report kind
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum FinancialReportKind {
    /// Income statement
    #[serde(rename = "IS")]
    IncomeStatement,
    /// Balance sheet
    #[serde(rename = "BS")]
    BalanceSheet,
    /// Cash flow statement
    #[serde(rename = "CF")]
    CashFlow,
    /// All statements
    #[default]
    #[serde(rename = "ALL")]
    All,
}

// ── business_segments ─────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::business_segments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSegments {
    /// Report date
    pub date: String,
    /// Total revenue
    pub total: String,
    /// Reporting currency
    pub currency: String,
    /// Business segment breakdown
    #[serde(default)]
    pub business: Vec<BusinessSegmentItem>,
}

/// One business segment item (latest snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSegmentItem {
    /// Segment name
    pub name: String,
    /// Percentage of total revenue
    pub percent: String,
}

/// Response for [`crate::FundamentalContext::business_segments_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSegmentsHistory {
    /// Historical snapshots
    #[serde(default)]
    pub historical: Vec<BusinessSegmentsHistoricalItem>,
}

/// One historical business segments snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSegmentsHistoricalItem {
    /// Report date
    pub date: String,
    /// Total revenue
    pub total: String,
    /// Reporting currency
    pub currency: String,
    /// Business segment breakdown
    #[serde(default)]
    pub business: Vec<BusinessSegmentHistoryItem>,
    /// Regional breakdown
    #[serde(default)]
    pub regionals: Vec<BusinessSegmentHistoryItem>,
}

/// One business/regional segment item in a historical snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSegmentHistoryItem {
    /// Segment name
    pub name: String,
    /// Percentage of total
    pub percent: String,
    /// Absolute value
    pub value: String,
}

// ── institution_rating_views ──────────────────────────────────────

/// Response for [`crate::FundamentalContext::institution_rating_views`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingViews {
    /// Historical rating distribution snapshots
    #[serde(default)]
    pub elist: Vec<InstitutionRatingViewItem>,
}

/// One historical rating distribution snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionRatingViewItem {
    /// Date as unix timestamp string (API returns as quoted or bare integer)
    pub date: String,
    /// Number of "Buy" ratings (API returns as string)
    pub buy: String,
    /// Number of "Outperform" ratings (API returns as string)
    pub over: String,
    /// Number of "Hold" ratings (API returns as string)
    pub hold: String,
    /// Number of "Underperform" ratings (API returns as string)
    pub under: String,
    /// Number of "Sell" ratings (API returns as string)
    pub sell: String,
    /// Total analyst count (API returns as string)
    pub total: String,
}

// ── industry_rank ─────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::industry_rank`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryRankResponse {
    /// Grouped rank items
    #[serde(default)]
    pub items: Vec<IndustryRankGroup>,
}

/// A group of ranked industry items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryRankGroup {
    /// Items in this group
    #[serde(default)]
    pub lists: Vec<IndustryRankItem>,
}

/// One ranked industry item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryRankItem {
    /// Industry / sector name
    pub name: String,
    /// Counter ID of the industry
    pub counter_id: String,
    /// Change percentage
    pub chg: String,
    /// Name of the leading stock
    pub leading_name: String,
    /// Ticker of the leading stock
    pub leading_ticker: String,
    /// Change percentage of the leading stock
    pub leading_chg: String,
    /// Value label name
    pub value_name: String,
    /// Value data
    pub value_data: String,
}

// ── industry_peers ────────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::industry_peers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryPeersResponse {
    /// Top-level industry node info
    pub top: IndustryPeersTop,
    /// Root peer chain node (may be absent if no data)
    pub chain: Option<IndustryPeerNode>,
}

/// Top-level industry info in the peers response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryPeersTop {
    /// Industry name
    pub name: String,
    /// Market code
    pub market: String,
}

/// A node in the recursive industry peer chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryPeerNode {
    /// Node name
    pub name: String,
    /// Counter ID
    pub counter_id: String,
    /// Number of stocks in this node (API returns as integer)
    pub stock_num: i32,
    /// Change percentage
    pub chg: String,
    /// Year-to-date change
    pub ytd_chg: String,
    /// Child nodes (recursive)
    #[serde(default)]
    pub next: Vec<IndustryPeerNode>,
}

// ── financial_report_snapshot ─────────────────────────────────────

/// Response for [`crate::FundamentalContext::financial_report_snapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialReportSnapshot {
    /// Company name
    pub name: String,
    /// Ticker code
    pub ticker: String,
    /// Fiscal period start date
    pub fp_start: String,
    /// Fiscal period end date
    pub fp_end: String,
    /// Reporting currency
    pub currency: String,
    /// Report description
    pub report_desc: String,
    /// Forecast revenue
    pub fo_revenue: Option<SnapshotForecastMetric>,
    /// Forecast EBIT
    pub fo_ebit: Option<SnapshotForecastMetric>,
    /// Forecast EPS
    pub fo_eps: Option<SnapshotForecastMetric>,
    /// Reported revenue
    pub fr_revenue: Option<SnapshotReportedMetric>,
    /// Reported net profit
    pub fr_profit: Option<SnapshotReportedMetric>,
    /// Reported operating cash flow
    pub fr_operate_cash: Option<SnapshotReportedMetric>,
    /// Reported investing cash flow
    pub fr_invest_cash: Option<SnapshotReportedMetric>,
    /// Reported financing cash flow
    pub fr_finance_cash: Option<SnapshotReportedMetric>,
    /// Reported total assets
    pub fr_total_assets: Option<SnapshotReportedMetric>,
    /// Reported total liabilities
    pub fr_total_liability: Option<SnapshotReportedMetric>,
    /// ROE TTM
    pub fr_roe_ttm: String,
    /// Profit margin
    pub fr_profit_margin: String,
    /// Profit margin TTM
    pub fr_profit_margin_ttm: String,
    /// Asset turnover TTM
    pub fr_asset_turn_ttm: String,
    /// Leverage TTM
    pub fr_leverage_ttm: String,
    /// Debt-to-assets ratio
    pub fr_debt_assets_ratio: String,
}

/// A forecast metric in the financial report snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotForecastMetric {
    /// Actual value
    pub value: String,
    /// Year-over-year change
    pub yoy: String,
    /// Beat/miss description
    pub cmp_desc: String,
    /// Consensus estimate value
    pub est_value: String,
}

/// A reported metric in the financial report snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotReportedMetric {
    /// Actual value
    pub value: String,
    /// Year-over-year change
    pub yoy: String,
}

// ── shareholder_top ───────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::shareholder_top`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderTopResponse {
    /// Raw top-shareholder data
    pub data: serde_json::Value,
}

// ── shareholder_detail ────────────────────────────────────────────

/// Response for [`crate::FundamentalContext::shareholder_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderDetailResponse {
    /// Raw shareholder detail data
    pub data: serde_json::Value,
}

// ── valuation_comparison ──────────────────────────────────────────

/// One historical valuation data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationHistoryPoint {
    /// Date (RFC 3339, converted from Unix timestamp)
    pub date: String,
    /// P/E ratio
    pub pe: String,
    /// P/B ratio
    pub pb: String,
    /// P/S ratio
    pub ps: String,
}

/// One security's valuation comparison item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationComparisonItem {
    /// Symbol (converted from counter_id)
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

/// Response for [`crate::FundamentalContext::valuation_comparison`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationComparisonResponse {
    /// Valuation comparison items
    pub list: Vec<ValuationComparisonItem>,
}

/// Financial report period type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum FinancialReportPeriod {
    /// Annual report
    #[serde(rename = "af")]
    Annual,
    /// Semi-annual report
    #[serde(rename = "saf")]
    SemiAnnual,
    /// Q1 report
    #[serde(rename = "q1")]
    Q1,
    /// Q2 report
    #[serde(rename = "q2")]
    Q2,
    /// Q3 report
    #[serde(rename = "q3")]
    Q3,
    /// Full quarterly report
    #[serde(rename = "qf")]
    QuarterlyFull,
    /// Three-quarter report (first three quarters)
    #[serde(rename = "3q")]
    ThreeQ,
}

// ── etf_asset_allocation ──────────────────────────────────────────

/// ETF asset allocation element type
#[derive(Debug, FromPrimitive, IntoPrimitive, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(i32)]
pub enum ElementType {
    /// Unknown
    #[num_enum(default)]
    Unknown = 0,
    /// Holdings
    Holdings = 1,
    /// Regional
    Regional = 2,
    /// Asset class
    AssetClass = 3,
    /// Industry
    Industry = 4,
}

impl Serialize for ElementType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32((*self).into())
    }
}

impl<'de> Deserialize<'de> for ElementType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ElementType::from(i32::deserialize(deserializer)?))
    }
}

/// Holding detail of an ETF asset allocation element (holdings only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingDetail {
    /// Industry ID
    #[serde(default)]
    pub industry_id: String,
    /// Industry name
    #[serde(default)]
    pub industry_name: String,
    /// Index counter ID (e.g. `BK/US/CP99000`)
    #[serde(default)]
    pub index: String,
    /// Index name
    #[serde(default)]
    pub index_name: String,
    /// Holding type (e.g. `E` for stock)
    #[serde(default)]
    pub holding_type: String,
    /// Holding type name
    #[serde(default)]
    pub holding_type_name: String,
}

/// One element of an ETF asset allocation group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAllocationItem {
    /// Element name
    pub name: String,
    /// Security code (holdings only, e.g. `NVDA`)
    #[serde(default)]
    pub code: String,
    /// Position ratio (e.g. `0.0861114`)
    pub position_ratio: String,
    /// Security symbol (holdings only, e.g. `NVDA.US`)
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol",
        default
    )]
    pub symbol: String,
    /// Localized names (locale → name, e.g. `zh-CN` → `英伟达`)
    #[serde(rename = "name_locales_map", default)]
    pub name_locales: HashMap<String, String>,
    /// Holding detail (holdings only)
    #[serde(default)]
    pub holding_detail: Option<HoldingDetail>,
}

/// One ETF asset allocation group (grouped by element type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAllocationGroup {
    /// Report date (e.g. `20260601`)
    pub report_date: String,
    /// Element type of this group
    pub asset_type: ElementType,
    /// Elements
    #[serde(default)]
    pub lists: Vec<AssetAllocationItem>,
}

/// Response for [`crate::FundamentalContext::etf_asset_allocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAllocationResponse {
    /// Asset allocation groups
    #[serde(default)]
    pub info: Vec<AssetAllocationGroup>,
}

// ── macroeconomic ─────────────────────────────────────────────────────

/// Country for filtering macroeconomic indicators
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MacroeconomicCountry {
    /// Hong Kong SAR China
    #[serde(rename = "Hong Kong SAR China")]
    HongKong,
    /// China (Mainland)
    #[serde(rename = "China (Mainland)")]
    China,
    /// United States
    #[serde(rename = "United States")]
    UnitedStates,
    /// Euro Zone
    #[serde(rename = "Euro Zone")]
    EuroZone,
    /// Japan
    #[serde(rename = "Japan")]
    Japan,
    /// Singapore
    #[serde(rename = "Singapore")]
    Singapore,
}

/// Importance level of a macroeconomic indicator
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MacroeconomicImportance {
    /// Low importance
    Low = 1,
    /// Medium importance
    Medium = 2,
    /// High importance
    High = 3,
}

impl MacroeconomicImportance {
    /// Convert from raw API integer value
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            1 => Some(Self::Low),
            2 => Some(Self::Medium),
            3 => Some(Self::High),
            _ => None,
        }
    }
}

/// Localized text in simplified Chinese, traditional Chinese, and English
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiLanguageText {
    /// English
    #[serde(default)]
    pub english: String,
    /// Simplified Chinese
    #[serde(default)]
    pub simplified_chinese: String,
    /// Traditional Chinese
    #[serde(default)]
    pub traditional_chinese: String,
}

/// Metadata for one macroeconomic indicator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MacroeconomicIndicator {
    /// External vendor code (used as input to `macroeconomic`)
    pub indicator_code: String,
    /// Publishing organisation
    #[serde(default)]
    pub source_org: String,
    /// Country
    #[serde(default)]
    pub country: String,
    /// Indicator name (multilingual)
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub name: MultiLanguageText,
    /// Adjustment factor
    #[serde(default)]
    pub adjustment_factor: String,
    /// Release periodicity (e.g. `monthly` / `quarterly`)
    #[serde(default)]
    pub periodicity: String,
    /// Indicator category
    #[serde(default)]
    pub category: String,
    /// Description (multilingual)
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub describe: MultiLanguageText,
    /// Importance — higher is more important
    #[serde(default)]
    pub importance: i32,
    /// Start date of data coverage
    #[serde(
        default,
        with = "crate::serde_utils::rfc3339_opt",
        rename = "start_date"
    )]
    pub start_date: Option<OffsetDateTime>,
}

/// Response for [`crate::FundamentalContext::macroeconomic_indicators`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroeconomicIndicatorListResponse {
    /// Indicator list
    #[serde(default, rename = "list")]
    pub data: Vec<MacroeconomicIndicator>,
    /// Total number of indicators matching the query
    #[serde(default)]
    pub count: i32,
}

/// One historical data point for a macroeconomic indicator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Macroeconomic {
    /// Statistical period (e.g. `2024-Q1`, `2024-03`)
    #[serde(default)]
    pub period: String,
    /// Release datetime
    #[serde(default, with = "crate::serde_utils::rfc3339_opt")]
    pub release_at: Option<OffsetDateTime>,
    /// Actual value
    #[serde(default)]
    pub actual_value: String,
    /// Previous value
    #[serde(default)]
    pub previous_value: String,
    /// Forecast value (market consensus)
    #[serde(default)]
    pub forecast_value: String,
    /// Revised value
    #[serde(default)]
    pub revised_value: String,
    /// Next release datetime
    #[serde(default, with = "crate::serde_utils::rfc3339_opt")]
    pub next_release_at: Option<OffsetDateTime>,
    /// Unit (multilingual)
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub unit: MultiLanguageText,
    /// Unit prefix / data scale (multilingual, e.g. millions / billions)
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub unit_prefix: MultiLanguageText,
}

/// Response for [`crate::FundamentalContext::macroeconomic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroeconomicResponse {
    /// Indicator metadata
    #[serde(default, deserialize_with = "crate::serde_utils::null_as_default")]
    pub info: MacroeconomicIndicator,
    /// Historical data points
    #[serde(default)]
    pub data: Vec<Macroeconomic>,
    /// Total number of historical data points
    #[serde(default)]
    pub count: i32,
}

// ── v2 wire types (internal, used for mapping to existing public types) ──────

/// v2 wire: one indicator from GET /v2/quote/macrodata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct V2MacroIndicator {
    #[serde(default)]
    pub indicator_id: i32,
    #[serde(default)]
    pub indicator_name: String,
    #[serde(default)]
    pub market: String,
}

/// v2 wire: response from GET /v2/quote/macrodata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct V2MacroIndicatorListResponse {
    #[serde(default)]
    pub indicator_list: Vec<V2MacroIndicator>,
    /// Total count for pagination
    #[serde(default)]
    pub total: i32,
}

/// v2 wire: one data point from GET /v2/quote/macrodata/:id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct V2IndicatorDataDetail {
    #[serde(default)]
    pub actual_data: String,
    #[serde(default)]
    pub previous_data: String,
    #[serde(default)]
    pub estimated_data: String,
    #[serde(default)]
    pub published_time: String,
    #[serde(default)]
    pub observation_date: String,
}

/// v2 wire: one indicator with data from GET /v2/quote/macrodata/:id
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct V2MacroIndicatorDetail {
    #[serde(default)]
    pub indicator_id: i32,
    #[serde(default)]
    pub indicator_name: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub indicator_data: Vec<V2IndicatorDataDetail>,
}

/// v2 wire: response from GET /v2/quote/macrodata/:id
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct V2MacroIndicatorDataResponse {
    #[serde(default)]
    pub indicator_data_list: Vec<V2MacroIndicatorDetail>,
    /// Total count for pagination
    #[serde(default)]
    pub total: i32,
}
