#![allow(missing_docs)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{FromRepr, IntoStaticStr};
use time::OffsetDateTime;

use crate::{types::Market, utils::counter::deserialize_counter_id_as_symbol};

// ── market_status ─────────────────────────────────────────────────

/// Market trading status code.
#[allow(non_camel_case_types)]
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    FromRepr,
    IntoStaticStr,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(i32)]
pub enum TradeStatus {
    /// Unknown status
    #[default]
    #[serde(other)]
    UNKNOWN = -1,
    /// Quote is not registered
    NO_REGISTER_QUOTE = 0,
    /// Clearing before the market opens.
    CLEAN = 101,
    /// Opening auction.
    OPEN_BID = 102,
    /// Morning break, currently used by VIX indexes.
    MORNING_CLOSING = 103,
    /// Regular trading.
    TRADING = 105,
    /// Midday break.
    NOON_CLOSING = 106,
    /// Closing auction.
    CLOSE_BID = 107,
    /// Market closed.
    CLOSING = 108,
    /// Dark trading waiting to open.
    DARK_WAIT = 110,
    /// Dark trading.
    DARK_TRADING = 111,
    /// Dark trading closed.
    DARK_CLOSING = 112,
    /// After-hours fixed-price trading.
    AFTER_FIX = 120,
    /// Half-day market closed. Defined by the market status table but currently
    /// unused.
    HALF_CLOSING = 121,
    /// Not opened because the exchange is waiting to open under special
    /// conditions.
    NOT_OPENED = 122,
    /// Temporary intraday break. The historical variant name is kept for
    /// compatibility.
    REALTIME_QUOTE = 123,
    /// US pre-market.
    US_PREV = 201,
    /// US regular trading.
    US_TRADING = 202,
    /// US post-market.
    US_AFTER = 203,
    /// US closed.
    US_CLOSING = 204,
    /// US halted.
    US_STOP = 205,
    /// US clearing plus pre-market.
    US_CLEAN = 206,
    /// US overnight trading.
    US_NIGHT = 207,
    /// US pre-market clearing alias returned by the quote engine.
    US_PREV_MARKET_CLEAN = 209,
    /// US post-market clearing alias returned by the quote engine.
    US_AFTER_MARKET_CLEAN = 210,
    /// Stock refresh. Deprecated in the status definition.
    REFRESH = 1000,
    /// Delisted.
    DELIST = 1001,
    /// Preparing to list.
    PREPARE = 1002,
    /// Code changed.
    CODE_CHANGE = 1003,
    /// Halted.
    STOP = 1004,
    /// Waiting to open, typically for a US IPO auction.
    WILL_OPEN = 1005,
    /// Split or merge suspended.
    COMMON_SUSPEND = 1006,
    /// Expired.
    EXPIRE = 1007,
    /// No quote data.
    NO_QUOTE = 1008,
    /// Not listed. The historical variant name is kept for compatibility.
    UNITED = 1009,
    /// Terminated trading, usually for warrants.
    TRADING_HALT = 1010,
    /// Waiting to list, usually for new warrants.
    WAIT_LISTING = 1011,
    /// Fuse.
    FUSE = 2001,
}

impl From<i32> for TradeStatus {
    fn from(value: i32) -> Self {
        Self::from_repr(value).unwrap_or_default()
    }
}

impl TradeStatus {
    /// Converts an isize value to a market trading status.
    pub fn from_isize(value: isize) -> TradeStatus {
        (value as i32).into()
    }

    /// Returns the raw numeric status code.
    pub fn code(self) -> i32 {
        self as i32
    }

    /// Returns the static enum variant name.
    pub fn as_static(self) -> &'static str {
        self.into()
    }

    /// Returns a simplified label for key display states.
    pub fn label(self) -> &'static str {
        let status = self.normalize();
        match status {
            TradeStatus::US_PREV
            | TradeStatus::US_TRADING
            | TradeStatus::US_AFTER
            | TradeStatus::US_NIGHT
            | TradeStatus::US_CLOSING
            | TradeStatus::TRADING
            | TradeStatus::CLOSING => status.name(),
            _ => "",
        }
    }

    /// Returns the full English status name.
    pub fn name(self) -> &'static str {
        match self.normalize() {
            TradeStatus::UNKNOWN | TradeStatus::NO_REGISTER_QUOTE => "Unknown",
            TradeStatus::OPEN_BID => "Open Bid",
            TradeStatus::MORNING_CLOSING => "Morning Break",
            TradeStatus::TRADING | TradeStatus::US_TRADING | TradeStatus::US_AFTER_MARKET_CLEAN => {
                "Trading"
            }
            TradeStatus::NOON_CLOSING => "Mid-Day Break",
            TradeStatus::CLOSE_BID => "Close Bid",
            TradeStatus::CLOSING
            | TradeStatus::CLEAN
            | TradeStatus::HALF_CLOSING
            | TradeStatus::US_CLOSING
            | TradeStatus::US_PREV_MARKET_CLEAN => "Closed",
            TradeStatus::DARK_WAIT => "Dark Wait",
            TradeStatus::DARK_TRADING => "Dark Trading",
            TradeStatus::DARK_CLOSING => "Closing",
            TradeStatus::AFTER_FIX => "After Fix",
            TradeStatus::NOT_OPENED => "Not Open",
            TradeStatus::REALTIME_QUOTE => "Temporary Break",
            TradeStatus::US_PREV | TradeStatus::US_CLEAN => "Pre-Market",
            TradeStatus::US_AFTER => "Post-Market",
            TradeStatus::US_STOP | TradeStatus::STOP => "Stop",
            TradeStatus::US_NIGHT => "Overnight",
            TradeStatus::REFRESH => "Refresh",
            TradeStatus::DELIST => "Delist",
            TradeStatus::PREPARE => "Prepare",
            TradeStatus::CODE_CHANGE => "Code Change",
            TradeStatus::WILL_OPEN => "Will Open",
            TradeStatus::COMMON_SUSPEND => "Common Suspend",
            TradeStatus::EXPIRE => "Expire",
            TradeStatus::NO_QUOTE => "No Quote",
            TradeStatus::UNITED => "Not Listed",
            TradeStatus::TRADING_HALT => "Terminated",
            TradeStatus::WAIT_LISTING => "Wait Listing",
            TradeStatus::FUSE => "Fuse",
        }
    }

    /// Returns whether this is a US market status.
    pub fn is_us_market(self) -> bool {
        self.code() >= 200 && self.code() < 300
    }

    /// Returns whether this is a US pre/post-market status.
    pub fn is_us_pre_post(self) -> bool {
        self.is_us_prev() || self.is_us_after()
    }

    /// Returns whether this is a US overnight status.
    pub fn is_us_night(self) -> bool {
        matches!(self, TradeStatus::US_NIGHT)
    }

    /// Returns whether this is a US closed status.
    pub fn is_us_closing(self) -> bool {
        matches!(
            self,
            TradeStatus::US_CLOSING | TradeStatus::US_PREV_MARKET_CLEAN
        )
    }

    /// Returns whether this is a closed status.
    pub fn is_closing(self) -> bool {
        matches!(
            self,
            TradeStatus::US_CLOSING
                | TradeStatus::US_PREV_MARKET_CLEAN
                | TradeStatus::CLOSING
                | TradeStatus::HALF_CLOSING
        )
    }

    /// Returns whether this is a US pre-market status.
    pub fn is_us_prev(self) -> bool {
        matches!(self, TradeStatus::US_PREV | TradeStatus::US_CLEAN)
    }

    /// Returns whether this is a US post-market status.
    pub fn is_us_after(self) -> bool {
        matches!(self, TradeStatus::US_AFTER)
    }

    /// Returns whether this is a trading status.
    pub fn is_trading(self) -> bool {
        matches!(
            self,
            TradeStatus::TRADING | TradeStatus::US_TRADING | TradeStatus::US_AFTER_MARKET_CLEAN
        )
    }

    /// Returns whether this is a dark-pool status.
    pub fn is_dark(self) -> bool {
        matches!(
            self,
            TradeStatus::DARK_WAIT | TradeStatus::DARK_TRADING | TradeStatus::DARK_CLOSING
        )
    }

    /// Returns whether this status allows trading.
    pub fn allow_trading(self) -> bool {
        matches!(
            self,
            TradeStatus::OPEN_BID
                | TradeStatus::TRADING
                | TradeStatus::CLOSE_BID
                | TradeStatus::NOT_OPENED
                | TradeStatus::NOON_CLOSING
                | TradeStatus::US_TRADING
                | TradeStatus::US_AFTER_MARKET_CLEAN
        )
    }

    /// Normalizes clearing aliases to their display-equivalent status.
    #[must_use]
    pub fn normalize(self) -> Self {
        match self {
            TradeStatus::CLEAN => TradeStatus::CLOSING,
            TradeStatus::US_PREV_MARKET_CLEAN => TradeStatus::US_CLOSING,
            TradeStatus::US_CLEAN => TradeStatus::US_PREV,
            TradeStatus::US_AFTER_MARKET_CLEAN => TradeStatus::US_TRADING,
            _ => self,
        }
    }

    /// Returns whether this is a special non-regular status.
    pub fn is_special(self) -> bool {
        self.code() < 100 || self == Self::US_STOP || self.code() >= 1000
    }
}

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
    /// Market trade status. See [`TradeStatus`] for the code table.
    pub trade_status: TradeStatus,
    /// Current market time (unix timestamp string)
    pub timestamp: String,
    /// Delayed-quote market trade status. See [`TradeStatus`] for the code
    /// table.
    pub delay_trade_status: TradeStatus,
    /// Delayed-quote market time (unix timestamp string)
    pub delay_timestamp: String,
    /// Sub-status code
    pub sub_status: i32,
    /// Delayed-quote sub-status code
    pub delay_sub_status: i32,
}

#[cfg(test)]
mod tests {
    use crate::market::TradeStatus;

    #[test]
    fn market_trade_status_deserializes_numeric_codes() {
        assert_eq!(
            serde_json::from_str::<TradeStatus>("202")
                .expect("202 should deserialize as market trade status"),
            TradeStatus::US_TRADING
        );
        assert_eq!(
            serde_json::from_str::<TradeStatus>("456")
                .expect("unknown numeric status should deserialize"),
            TradeStatus::UNKNOWN
        );
    }

    #[test]
    fn market_trade_status_serializes_as_numeric_code() {
        let value = serde_json::to_string(&TradeStatus::US_CLEAN)
            .expect("market trade status should serialize");
        assert_eq!(value, "206");
    }

    #[test]
    fn market_trade_status_normalizes_engine_aliases() {
        assert_eq!(TradeStatus::CLEAN.normalize(), TradeStatus::CLOSING);
        assert_eq!(TradeStatus::US_CLEAN.normalize(), TradeStatus::US_PREV);
        assert_eq!(
            TradeStatus::US_PREV_MARKET_CLEAN.normalize(),
            TradeStatus::US_CLOSING
        );
        assert_eq!(
            TradeStatus::US_AFTER_MARKET_CLEAN.normalize(),
            TradeStatus::US_TRADING
        );
    }

    #[test]
    fn market_trade_status_label_matches_engine_simplified_display() {
        assert_eq!(TradeStatus::US_PREV.label(), "Pre-Market");
        assert_eq!(TradeStatus::US_CLEAN.label(), "Pre-Market");
        assert_eq!(TradeStatus::US_AFTER.label(), "Post-Market");
        assert_eq!(TradeStatus::US_CLOSING.label(), "Closed");
        assert_eq!(TradeStatus::US_AFTER_MARKET_CLEAN.label(), "Trading");
        assert_eq!(TradeStatus::US_TRADING.label(), "Trading");
        assert_eq!(TradeStatus::TRADING.label(), "Trading");
        assert_eq!(TradeStatus::CLEAN.label(), "Closed");
        assert_eq!(TradeStatus::OPEN_BID.label(), "");
        assert_eq!(TradeStatus::NOON_CLOSING.label(), "");
    }

    #[test]
    fn market_trade_status_name_covers_full_status_set() {
        let cases = [
            (TradeStatus::MORNING_CLOSING, "Morning Break"),
            (TradeStatus::NOON_CLOSING, "Mid-Day Break"),
            (TradeStatus::REALTIME_QUOTE, "Temporary Break"),
            (TradeStatus::US_STOP, "Stop"),
            (TradeStatus::TRADING_HALT, "Terminated"),
            (TradeStatus::WAIT_LISTING, "Wait Listing"),
            (TradeStatus::FUSE, "Fuse"),
            (TradeStatus::UNKNOWN, "Unknown"),
            (TradeStatus::NO_REGISTER_QUOTE, "Unknown"),
        ];

        for (status, expected) in cases {
            assert_eq!(status.name(), expected, "status {status:?}");
        }
    }

    #[test]
    fn market_trade_status_codes_match_phase_definition_document() {
        let codes = [
            101, 102, 103, 105, 106, 107, 108, 110, 111, 112, 120, 121, 122, 123, 201, 202, 203,
            204, 206, 207, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011,
            2001,
        ];

        for code in codes {
            assert_eq!(TradeStatus::from(code).code(), code, "status code {code}");
        }
    }

    #[test]
    fn market_trade_status_names_match_phase_definition_document() {
        let cases = [
            (123, "Temporary Break"),
            (1009, "Not Listed"),
            (1010, "Terminated"),
            (2001, "Fuse"),
        ];

        for (code, expected) in cases {
            assert_eq!(
                TradeStatus::from(code).name(),
                expected,
                "status code {code}"
            );
        }
    }

    #[test]
    fn market_time_item_uses_market_trade_status_type() {
        let item = serde_json::from_str::<crate::market::MarketTimeItem>(
            r#"{
                "market": "US",
                "trade_status": 202,
                "timestamp": "1717200000",
                "delay_trade_status": 204,
                "delay_timestamp": "1717200000",
                "sub_status": 0,
                "delay_sub_status": 0
            }"#,
        )
        .expect("market time item should deserialize");

        assert_eq!(item.trade_status, TradeStatus::US_TRADING);
        assert_eq!(item.delay_trade_status, TradeStatus::US_CLOSING);
    }
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

/// Stock information within a top-movers event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMoversStock {
    /// Symbol (converted from counter_id, e.g. `"NVDA.US"`)
    pub symbol: String,
    /// Ticker code (e.g. `"NVDA"`)
    pub code: String,
    /// Security name
    pub name: String,
    /// Full name
    #[serde(default)]
    pub full_name: String,
    /// Price change (decimal ratio)
    pub change: String,
    /// Latest price
    pub last_done: String,
    /// Market code
    pub market: String,
    /// Labels / tags
    #[serde(default)]
    pub labels: Vec<String>,
    /// Logo URL
    #[serde(default)]
    pub logo: String,
}

/// One top-movers event entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMoversEvent {
    /// Event time (RFC 3339)
    pub timestamp: String,
    /// Alert reason description
    pub alert_reason: String,
    /// Alert type code
    pub alert_type: i64,
    /// Stock information
    pub stock: TopMoversStock,
    /// Associated news post (raw JSON, complex structure)
    pub post: serde_json::Value,
}

/// Response for [`crate::MarketContext::top_movers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMoversResponse {
    /// Top-mover events
    pub events: Vec<TopMoversEvent>,
    /// Pagination cursor for next page
    pub next_params: serde_json::Value,
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

/// One ranked security item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankListItem {
    /// Symbol (converted from counter_id, e.g. `"MU.US"`)
    pub symbol: String,
    /// Ticker code (e.g. `"MU"`)
    pub code: String,
    /// Security name
    pub name: String,
    /// Latest price
    pub last_done: String,
    /// Price change ratio (decimal)
    pub chg: String,
    /// Absolute price change
    pub change: String,
    /// Net inflow
    pub inflow: String,
    /// Market cap
    pub market_cap: String,
    /// Industry name
    pub industry: String,
    /// Pre/post market price
    #[serde(default)]
    pub pre_post_price: String,
    /// Pre/post market change
    #[serde(default)]
    pub pre_post_chg: String,
    /// Amplitude
    #[serde(default)]
    pub amplitude: String,
    /// 5-day change
    #[serde(default)]
    pub five_day_chg: String,
    /// Turnover rate
    #[serde(default)]
    pub turnover_rate: String,
    /// Volume ratio
    #[serde(default)]
    pub volume_rate: String,
    /// P/B ratio (TTM)
    #[serde(default)]
    pub pb_ttm: String,
}

/// Response for [`crate::MarketContext::rank_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankListResponse {
    /// Whether delayed / BMP data
    pub bmp: bool,
    /// Ranked security items
    pub lists: Vec<RankListItem>,
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
