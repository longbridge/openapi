#![allow(missing_docs)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{types::Market, utils::counter::deserialize_counter_id_as_symbol};

// ── market_status ─────────────────────────────────────────────────

/// Response for [`crate::MarketContext::market_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatusResponse {
    /// Per-market trading status items
    pub market_time: Vec<MarketTimeItem>,
}

/// Trading status for one market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTimeItem {
    /// Market code
    pub market: Market,
    /// Raw trade status code (101=PreOpen, 102/103/105=Trading, 104=LunchBreak,
    /// 106=PostTrading, 108=Closed, 201=PreMarket, 204=PostMarket)
    pub trade_status: i32,
    /// Current market time (unix timestamp string)
    pub timestamp: String,
    /// Delayed-quote trade status code
    pub delay_trade_status: i32,
    /// Delayed-quote market time (unix timestamp string)
    pub delay_timestamp: String,
    /// Sub-status code
    pub sub_status: i32,
    /// Delayed-quote sub-status code
    pub delay_sub_status: i32,
}

// ── broker_holding ────────────────────────────────────────────────

/// Response for [`crate::MarketContext::broker_holding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingTop {
    /// Top brokers by net buying
    pub buy: Vec<BrokerHoldingEntry>,
    /// Top brokers by net selling
    pub sell: Vec<BrokerHoldingEntry>,
    /// Last updated (may be empty)
    #[serde(default)]
    pub updated_at: String,
}

/// One broker entry in a top-holding list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingEntry {
    /// Broker name
    pub name: String,
    /// Participant number / broker code
    pub parti_number: String,
    /// Net change in shares held
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg: Option<Decimal>,
    /// Whether this is a "strengthening" broker
    pub strong: bool,
}

// ── broker_holding_detail ─────────────────────────────────────────

/// Response for [`crate::MarketContext::broker_holding_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingDetail {
    /// Full list of broker holdings
    pub list: Vec<BrokerHoldingDetailItem>,
    /// Last updated (may be empty)
    #[serde(default)]
    pub updated_at: String,
}

/// One broker's full holding detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingDetailItem {
    /// Broker name
    pub name: String,
    /// Participant number / broker code
    pub parti_number: String,
    /// Holding ratio changes over various periods
    pub ratio: BrokerHoldingChanges,
    /// Share count changes over various periods
    pub shares: BrokerHoldingChanges,
    /// Whether this is a "strengthening" broker
    pub strong: bool,
}

/// Changes in broker holding over 1 / 5 / 20 / 60 day periods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingChanges {
    /// Current value
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub value: Option<Decimal>,
    /// 1-day change
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg_1: Option<Decimal>,
    /// 5-day change
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg_5: Option<Decimal>,
    /// 20-day change
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg_20: Option<Decimal>,
    /// 60-day change
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg_60: Option<Decimal>,
}

// ── broker_holding_daily ──────────────────────────────────────────

/// Response for [`crate::MarketContext::broker_holding_daily`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingDailyHistory {
    /// Daily broker holding records
    pub list: Vec<BrokerHoldingDailyItem>,
}

/// One day's broker holding record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerHoldingDailyItem {
    /// Date in `"2026.05.05"` format
    pub date: String,
    /// Total shares held
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub holding: Option<Decimal>,
    /// Holding ratio as a decimal
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub ratio: Option<Decimal>,
    /// Change vs previous day
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg: Option<Decimal>,
}

// ── ah_premium ────────────────────────────────────────────────────

/// Response for [`crate::MarketContext::ah_premium`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhPremiumKlines {
    /// K-line data points
    pub klines: Vec<AhPremiumKline>,
}

/// Response for [`crate::MarketContext::ah_premium_intraday`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhPremiumIntraday {
    /// Intraday data points (field name is `klines` in the API)
    pub klines: Vec<AhPremiumKline>,
}

/// One A/H premium data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhPremiumKline {
    /// A-share price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub aprice: Decimal,
    /// A-share previous close
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub apreclose: Decimal,
    /// H-share price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub hprice: Decimal,
    /// H-share previous close
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub hpreclose: Decimal,
    /// CNY/HKD exchange rate
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub currency_rate: Decimal,
    /// A/H premium rate (negative = H-share at premium)
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub ahpremium_rate: Decimal,
    /// Price spread
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub price_spread: Decimal,
    /// Data point timestamp
    #[serde(deserialize_with = "crate::serde_utils::deserialize_timestamp")]
    pub timestamp: OffsetDateTime,
}

// ── trade_stats ───────────────────────────────────────────────────

/// Response for [`crate::MarketContext::trade_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeStatsResponse {
    /// Summary statistics
    pub statistics: TradeStatistics,
    /// Per-price-level breakdown
    pub trades: Vec<TradePriceLevel>,
}

/// Summary trade statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeStatistics {
    /// Volume-weighted average price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub avgprice: Decimal,
    /// Total buy volume (shares)
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub buy: Decimal,
    /// Total neutral / unknown-direction volume
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub neutral: Decimal,
    /// Previous close price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub preclose: Decimal,
    /// Total sell volume (shares)
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub sell: Decimal,
    /// Data timestamp (unix timestamp string)
    pub timestamp: String,
    /// Total trading volume (shares)
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub total_amount: Decimal,
    /// Unix timestamps for the last 5 trading days
    pub trade_date: Vec<String>,
    /// Total number of trades
    pub trades_count: String,
}

/// Trade volume at one price level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePriceLevel {
    /// Buy volume at this price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub buy_amount: Decimal,
    /// Neutral (unknown direction) volume at this price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub neutral_amount: Decimal,
    /// Price level
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub price: Decimal,
    /// Sell volume at this price
    #[serde(with = "crate::serde_utils::decimal_empty_is_0")]
    pub sell_amount: Decimal,
}

// ── anomaly ───────────────────────────────────────────────────────

/// Response for [`crate::MarketContext::anomaly`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResponse {
    /// Whether anomaly alerts are globally disabled
    pub all_off: bool,
    /// List of market anomaly events
    pub changes: Vec<AnomalyItem>,
}

/// One market anomaly event (e.g. large block trade, margin buying surge)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyItem {
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Anomaly type name, e.g. `"大宗交易"`, `"融资买入"`
    pub alert_name: String,
    /// Time of the anomaly (unix timestamp in milliseconds)
    pub alert_time: i64,
    /// Change values — items are accessed as strings by the client
    pub change_values: Vec<String>,
    /// Sentiment direction: 1 = positive/up, 2 = negative/down
    pub emotion: i32,
}

// ── constituent ───────────────────────────────────────────────────

/// Response for [`crate::MarketContext::constituent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConstituents {
    /// Number of constituent stocks that fell today
    pub fall_num: i32,
    /// Number of constituent stocks unchanged today
    pub flat_num: i32,
    /// Number of constituent stocks that rose today
    pub rise_num: i32,
    /// Constituent stock details
    pub stocks: Vec<ConstituentStock>,
}

/// One constituent stock of an index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstituentStock {
    /// Security symbol
    #[serde(
        rename = "counter_id",
        deserialize_with = "deserialize_counter_id_as_symbol"
    )]
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Latest price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub last_done: Option<Decimal>,
    /// Previous close
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub prev_close: Option<Decimal>,
    /// Net capital inflow today
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub inflow: Option<Decimal>,
    /// Turnover amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub balance: Option<Decimal>,
    /// Trading volume (shares)
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub amount: Option<Decimal>,
    /// Total shares outstanding
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub total_shares: Option<Decimal>,
    /// Tags, e.g. `["领涨龙头"]`
    pub tags: Vec<String>,
    /// Brief description
    pub intro: String,
    /// Market, e.g. `"HK"`
    pub market: String,
    /// Circulating shares
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub circulating_shares: Option<Decimal>,
    /// Whether this is a delayed quote
    pub delay: bool,
    /// Day change percentage
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg: Option<Decimal>,
    /// Raw trade status code
    pub trade_status: i32,
}

// ── top_movers ────────────────────────────────────────────────────

/// Response for [`crate::MarketContext::top_movers`]
///
/// The raw data contains top-movers stock events from all requested markets.
/// The exact structure varies so the payload is preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMoversResponse {
    /// Raw top movers data
    pub data: serde_json::Value,
}

// ── rank_categories ───────────────────────────────────────────────

/// Response for [`crate::MarketContext::rank_categories`]
///
/// The raw data contains all available rank category keys and labels.
/// The exact structure varies so the payload is preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankCategoriesResponse {
    /// Raw rank category data
    pub data: serde_json::Value,
}

// ── rank_list ─────────────────────────────────────────────────────

/// Response for [`crate::MarketContext::rank_list`]
///
/// The raw data contains a ranked list of securities for the requested
/// category key.  The exact structure varies so the payload is
/// preserved as raw JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankListResponse {
    /// Raw rank list data
    pub data: serde_json::Value,
}

// ── enums ─────────────────────────────────────────────────────────

/// Broker holding lookback period
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum BrokerHoldingPeriod {
    /// 1-day change
    #[default]
    #[serde(rename = "rct_1")]
    Rct1,
    /// 5-day change
    #[serde(rename = "rct_5")]
    Rct5,
    /// 20-day change
    #[serde(rename = "rct_20")]
    Rct20,
    /// 60-day change
    #[serde(rename = "rct_60")]
    Rct60,
}

/// A/H premium K-line period
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum AhPremiumPeriod {
    /// 1-minute
    Min1,
    /// 5-minute
    Min5,
    /// 15-minute
    Min15,
    /// 30-minute
    Min30,
    /// 60-minute
    Min60,
    /// Daily
    #[default]
    Day,
    /// Weekly
    Week,
    /// Monthly
    Month,
    /// Yearly
    Year,
}

impl AhPremiumPeriod {
    /// Convert to the API's `line_type` parameter value
    pub(crate) fn to_line_type(self) -> &'static str {
        match self {
            AhPremiumPeriod::Min1 => "1",
            AhPremiumPeriod::Min5 => "5",
            AhPremiumPeriod::Min15 => "15",
            AhPremiumPeriod::Min30 => "30",
            AhPremiumPeriod::Min60 => "60",
            AhPremiumPeriod::Day => "1000",
            AhPremiumPeriod::Week => "2000",
            AhPremiumPeriod::Month => "3000",
            AhPremiumPeriod::Year => "4000",
        }
    }
}
