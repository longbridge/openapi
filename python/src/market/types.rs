use longbridge::market::types as lb;
use pyo3::prelude::*;

// ── MarketStatusResponse ──────────────────────────────────────────

/// Market trading status response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct MarketStatusResponse {
    /// Per-market status items
    pub market_time: Vec<MarketTimeItem>,
}

impl From<lb::MarketStatusResponse> for MarketStatusResponse {
    fn from(v: lb::MarketStatusResponse) -> Self {
        Self {
            market_time: v.market_time.into_iter().map(Into::into).collect(),
        }
    }
}

/// Trading status for one market
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct MarketTimeItem {
    /// Market
    pub market: crate::types::Market,
    /// Raw trade status code
    pub trade_status: i32,
    /// Current market time (unix timestamp string)
    pub timestamp: String,
    /// Delayed-quote trade status code
    pub delay_trade_status: i32,
    /// Delayed-quote time (unix timestamp string)
    pub delay_timestamp: String,
    /// Sub-status code
    pub sub_status: i32,
    /// Delayed sub-status code
    pub delay_sub_status: i32,
}

impl From<lb::MarketTimeItem> for MarketTimeItem {
    fn from(v: lb::MarketTimeItem) -> Self {
        Self {
            market: v.market.into(),
            trade_status: v.trade_status,
            timestamp: v.timestamp,
            delay_trade_status: v.delay_trade_status,
            delay_timestamp: v.delay_timestamp,
            sub_status: v.sub_status,
            delay_sub_status: v.delay_sub_status,
        }
    }
}

// ── BrokerHoldingTop ──────────────────────────────────────────────

/// Top broker holdings response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingTop {
    /// Top buying brokers
    pub buy: Vec<BrokerHoldingEntry>,
    /// Top selling brokers
    pub sell: Vec<BrokerHoldingEntry>,
    /// Last updated string
    pub updated_at: String,
}

impl From<lb::BrokerHoldingTop> for BrokerHoldingTop {
    fn from(v: lb::BrokerHoldingTop) -> Self {
        Self {
            buy: v.buy.into_iter().map(Into::into).collect(),
            sell: v.sell.into_iter().map(Into::into).collect(),
            updated_at: v.updated_at,
        }
    }
}

/// One broker entry
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingEntry {
    /// Broker name
    pub name: String,
    /// Participant number
    pub parti_number: String,
    /// Net change in shares
    pub chg: Option<String>,
    /// Whether strengthening
    pub strong: bool,
}

impl From<lb::BrokerHoldingEntry> for BrokerHoldingEntry {
    fn from(v: lb::BrokerHoldingEntry) -> Self {
        Self {
            name: v.name,
            parti_number: v.parti_number,
            chg: v.chg.map(|d| d.to_string()),
            strong: v.strong,
        }
    }
}

// ── BrokerHoldingDetail ───────────────────────────────────────────

/// Full broker holding detail response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingDetail {
    /// Full broker list
    pub list: Vec<BrokerHoldingDetailItem>,
    /// Last updated string
    pub updated_at: String,
}

impl From<lb::BrokerHoldingDetail> for BrokerHoldingDetail {
    fn from(v: lb::BrokerHoldingDetail) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
            updated_at: v.updated_at,
        }
    }
}

/// One broker's full holding detail
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingDetailItem {
    /// Broker name
    pub name: String,
    /// Participant number
    pub parti_number: String,
    /// Holding ratio changes
    pub ratio: BrokerHoldingChanges,
    /// Share count changes
    pub shares: BrokerHoldingChanges,
    /// Whether strengthening
    pub strong: bool,
}

impl From<lb::BrokerHoldingDetailItem> for BrokerHoldingDetailItem {
    fn from(v: lb::BrokerHoldingDetailItem) -> Self {
        Self {
            name: v.name,
            parti_number: v.parti_number,
            ratio: v.ratio.into(),
            shares: v.shares.into(),
            strong: v.strong,
        }
    }
}

/// Holding changes over multiple periods
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingChanges {
    /// Current value
    pub value: Option<String>,
    /// 1-day change
    pub chg_1: Option<String>,
    /// 5-day change
    pub chg_5: Option<String>,
    /// 20-day change
    pub chg_20: Option<String>,
    /// 60-day change
    pub chg_60: Option<String>,
}

impl From<lb::BrokerHoldingChanges> for BrokerHoldingChanges {
    fn from(v: lb::BrokerHoldingChanges) -> Self {
        Self {
            value: v.value.map(|d| d.to_string()),
            chg_1: v.chg_1.map(|d| d.to_string()),
            chg_5: v.chg_5.map(|d| d.to_string()),
            chg_20: v.chg_20.map(|d| d.to_string()),
            chg_60: v.chg_60.map(|d| d.to_string()),
        }
    }
}

// ── BrokerHoldingDailyHistory ─────────────────────────────────────

/// Daily broker holding history response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingDailyHistory {
    /// Daily records
    pub list: Vec<BrokerHoldingDailyItem>,
}

impl From<lb::BrokerHoldingDailyHistory> for BrokerHoldingDailyHistory {
    fn from(v: lb::BrokerHoldingDailyHistory) -> Self {
        Self {
            list: v.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// One day's broker holding record
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct BrokerHoldingDailyItem {
    /// Date string
    pub date: String,
    /// Total shares held
    pub holding: Option<String>,
    /// Holding ratio
    pub ratio: Option<String>,
    /// Daily change
    pub chg: Option<String>,
}

impl From<lb::BrokerHoldingDailyItem> for BrokerHoldingDailyItem {
    fn from(v: lb::BrokerHoldingDailyItem) -> Self {
        Self {
            date: v.date,
            holding: v.holding.map(|d| d.to_string()),
            ratio: v.ratio.map(|d| d.to_string()),
            chg: v.chg.map(|d| d.to_string()),
        }
    }
}

// ── AhPremiumKlines / AhPremiumIntraday ───────────────────────────

/// A/H premium K-line response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct AhPremiumKlines {
    /// K-line data points
    pub klines: Vec<AhPremiumKline>,
}

impl From<lb::AhPremiumKlines> for AhPremiumKlines {
    fn from(v: lb::AhPremiumKlines) -> Self {
        Self {
            klines: v.klines.into_iter().map(Into::into).collect(),
        }
    }
}

/// A/H premium intraday response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct AhPremiumIntraday {
    /// Intraday data points
    pub klines: Vec<AhPremiumKline>,
}

impl From<lb::AhPremiumIntraday> for AhPremiumIntraday {
    fn from(v: lb::AhPremiumIntraday) -> Self {
        Self {
            klines: v.klines.into_iter().map(Into::into).collect(),
        }
    }
}

/// One A/H premium data point
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct AhPremiumKline {
    /// A-share price
    pub aprice: String,
    /// A-share previous close
    pub apreclose: String,
    /// H-share price
    pub hprice: String,
    /// H-share previous close
    pub hpreclose: String,
    /// Exchange rate
    pub currency_rate: String,
    /// A/H premium rate
    pub ahpremium_rate: String,
    /// Price spread
    pub price_spread: String,
    /// Timestamp (datetime)
    pub timestamp: crate::time::PyOffsetDateTimeWrapper,
}

impl From<lb::AhPremiumKline> for AhPremiumKline {
    fn from(v: lb::AhPremiumKline) -> Self {
        Self {
            aprice: v.aprice.to_string(),
            apreclose: v.apreclose.to_string(),
            hprice: v.hprice.to_string(),
            hpreclose: v.hpreclose.to_string(),
            currency_rate: v.currency_rate.to_string(),
            ahpremium_rate: v.ahpremium_rate.to_string(),
            price_spread: v.price_spread.to_string(),
            timestamp: crate::time::PyOffsetDateTimeWrapper(v.timestamp),
        }
    }
}

// ── TradeStatsResponse ────────────────────────────────────────────

/// Trade statistics response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct TradeStatsResponse {
    /// Summary statistics
    pub statistics: TradeStatistics,
    /// Per-price-level breakdown
    pub trades: Vec<TradePriceLevel>,
}

impl From<lb::TradeStatsResponse> for TradeStatsResponse {
    fn from(v: lb::TradeStatsResponse) -> Self {
        Self {
            statistics: v.statistics.into(),
            trades: v.trades.into_iter().map(Into::into).collect(),
        }
    }
}

/// Summary trade statistics
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct TradeStatistics {
    /// Volume-weighted average price
    pub avgprice: String,
    /// Total buy volume
    pub buy: String,
    /// Neutral volume
    pub neutral: String,
    /// Previous close
    pub preclose: String,
    /// Total sell volume
    pub sell: String,
    /// Data timestamp string
    pub timestamp: String,
    /// Total volume
    pub total_amount: String,
    /// Last 5 trading day timestamps
    pub trade_date: Vec<String>,
    /// Total trade count
    pub trades_count: String,
}

impl From<lb::TradeStatistics> for TradeStatistics {
    fn from(v: lb::TradeStatistics) -> Self {
        Self {
            avgprice: v.avgprice.to_string(),
            buy: v.buy.to_string(),
            neutral: v.neutral.to_string(),
            preclose: v.preclose.to_string(),
            sell: v.sell.to_string(),
            timestamp: v.timestamp,
            total_amount: v.total_amount.to_string(),
            trade_date: v.trade_date,
            trades_count: v.trades_count,
        }
    }
}

/// Trade volume at one price level
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct TradePriceLevel {
    /// Buy volume
    pub buy_amount: String,
    /// Neutral volume
    pub neutral_amount: String,
    /// Price level
    pub price: String,
    /// Sell volume
    pub sell_amount: String,
}

impl From<lb::TradePriceLevel> for TradePriceLevel {
    fn from(v: lb::TradePriceLevel) -> Self {
        Self {
            buy_amount: v.buy_amount.to_string(),
            neutral_amount: v.neutral_amount.to_string(),
            price: v.price.to_string(),
            sell_amount: v.sell_amount.to_string(),
        }
    }
}

// ── AnomalyResponse ───────────────────────────────────────────────

/// Market anomaly response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct AnomalyResponse {
    /// Whether anomaly alerts are disabled
    pub all_off: bool,
    /// Anomaly events
    pub changes: Vec<AnomalyItem>,
}

impl From<lb::AnomalyResponse> for AnomalyResponse {
    fn from(v: lb::AnomalyResponse) -> Self {
        Self {
            all_off: v.all_off,
            changes: v.changes.into_iter().map(Into::into).collect(),
        }
    }
}

/// One anomaly event
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct AnomalyItem {
    /// Security symbol
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Anomaly type name
    pub alert_name: String,
    /// Anomaly time (unix ms)
    pub alert_time: i64,
    /// Change value strings
    pub change_values: Vec<String>,
    /// Sentiment direction
    pub emotion: i32,
}

impl From<lb::AnomalyItem> for AnomalyItem {
    fn from(v: lb::AnomalyItem) -> Self {
        Self {
            symbol: v.symbol,
            name: v.name,
            alert_name: v.alert_name,
            alert_time: v.alert_time,
            change_values: v.change_values,
            emotion: v.emotion,
        }
    }
}

// ── IndexConstituents ─────────────────────────────────────────────

/// Index constituents response
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct IndexConstituents {
    /// Number of falling stocks today
    pub fall_num: i32,
    /// Number of unchanged stocks today
    pub flat_num: i32,
    /// Number of rising stocks today
    pub rise_num: i32,
    /// Constituent stocks
    pub stocks: Vec<ConstituentStock>,
}

impl From<lb::IndexConstituents> for IndexConstituents {
    fn from(v: lb::IndexConstituents) -> Self {
        Self {
            fall_num: v.fall_num,
            flat_num: v.flat_num,
            rise_num: v.rise_num,
            stocks: v.stocks.into_iter().map(Into::into).collect(),
        }
    }
}

/// One constituent stock
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct ConstituentStock {
    /// Security symbol
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Latest price
    pub last_done: Option<String>,
    /// Previous close
    pub prev_close: Option<String>,
    /// Net capital inflow
    pub inflow: Option<String>,
    /// Turnover
    pub balance: Option<String>,
    /// Volume
    pub amount: Option<String>,
    /// Total shares
    pub total_shares: Option<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Description
    pub intro: String,
    /// Market
    pub market: String,
    /// Circulating shares
    pub circulating_shares: Option<String>,
    /// Whether delayed quote
    pub delay: bool,
    /// Day change %
    pub chg: Option<String>,
    /// Trade status code
    pub trade_status: i32,
}

impl From<lb::ConstituentStock> for ConstituentStock {
    fn from(v: lb::ConstituentStock) -> Self {
        Self {
            symbol: v.symbol,
            name: v.name,
            last_done: v.last_done.map(|d| d.to_string()),
            prev_close: v.prev_close.map(|d| d.to_string()),
            inflow: v.inflow.map(|d| d.to_string()),
            balance: v.balance.map(|d| d.to_string()),
            amount: v.amount.map(|d| d.to_string()),
            total_shares: v.total_shares.map(|d| d.to_string()),
            tags: v.tags,
            intro: v.intro,
            market: v.market,
            circulating_shares: v.circulating_shares.map(|d| d.to_string()),
            delay: v.delay,
            chg: v.chg.map(|d| d.to_string()),
            trade_status: v.trade_status,
        }
    }
}

// ── enums ─────────────────────────────────────────────────────────

/// Broker holding lookback period
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum BrokerHoldingPeriod {
    /// 1-day
    Rct1 = 0,
    /// 5-day
    Rct5 = 1,
    /// 20-day
    Rct20 = 2,
    /// 60-day
    Rct60 = 3,
}

impl From<BrokerHoldingPeriod> for lb::BrokerHoldingPeriod {
    fn from(v: BrokerHoldingPeriod) -> Self {
        match v {
            BrokerHoldingPeriod::Rct1 => lb::BrokerHoldingPeriod::Rct1,
            BrokerHoldingPeriod::Rct5 => lb::BrokerHoldingPeriod::Rct5,
            BrokerHoldingPeriod::Rct20 => lb::BrokerHoldingPeriod::Rct20,
            BrokerHoldingPeriod::Rct60 => lb::BrokerHoldingPeriod::Rct60,
        }
    }
}

/// A/H premium K-line period
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum AhPremiumPeriod {
    /// 1-minute
    Min1 = 0,
    /// 5-minute
    Min5 = 1,
    /// 15-minute
    Min15 = 2,
    /// 30-minute
    Min30 = 3,
    /// 60-minute
    Min60 = 4,
    /// Daily
    Day = 5,
    /// Weekly
    Week = 6,
    /// Monthly
    Month = 7,
    /// Yearly
    Year = 8,
}

impl From<AhPremiumPeriod> for lb::AhPremiumPeriod {
    fn from(v: AhPremiumPeriod) -> Self {
        match v {
            AhPremiumPeriod::Min1 => lb::AhPremiumPeriod::Min1,
            AhPremiumPeriod::Min5 => lb::AhPremiumPeriod::Min5,
            AhPremiumPeriod::Min15 => lb::AhPremiumPeriod::Min15,
            AhPremiumPeriod::Min30 => lb::AhPremiumPeriod::Min30,
            AhPremiumPeriod::Min60 => lb::AhPremiumPeriod::Min60,
            AhPremiumPeriod::Day => lb::AhPremiumPeriod::Day,
            AhPremiumPeriod::Week => lb::AhPremiumPeriod::Week,
            AhPremiumPeriod::Month => lb::AhPremiumPeriod::Month,
            AhPremiumPeriod::Year => lb::AhPremiumPeriod::Year,
        }
    }
}
