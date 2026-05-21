use longbridge::market::types as lb;

/// Market trading status response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MarketStatusResponse {
    /// Per-market trading status items
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MarketTimeItem {
    /// Market
    pub market: crate::types::Market,
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

/// Top broker holdings response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BrokerHoldingTop {
    /// Top brokers by net buying
    pub buy: Vec<BrokerHoldingEntry>,
    /// Top brokers by net selling
    pub sell: Vec<BrokerHoldingEntry>,
    /// Last updated (may be empty)
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

/// One broker entry in a top-holding list
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BrokerHoldingEntry {
    /// Broker name
    pub name: String,
    /// Participant number / broker code
    pub parti_number: String,
    /// Net change in shares held
    pub chg: Option<String>,
    /// Whether this is a "strengthening" broker
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

/// Full broker holding detail response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BrokerHoldingDetail {
    /// Full list of broker holdings
    pub list: Vec<BrokerHoldingDetailItem>,
    /// Last updated (may be empty)
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
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

/// Changes in broker holding over 1 / 5 / 20 / 60 day periods
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BrokerHoldingChanges {
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

/// Daily broker holding history response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BrokerHoldingDailyHistory {
    /// Daily broker holding records
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct BrokerHoldingDailyItem {
    /// Date in `"2026.05.05"` format
    pub date: String,
    /// Total shares held
    pub holding: Option<String>,
    /// Holding ratio
    pub ratio: Option<String>,
    /// Change vs previous day
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

/// A/H premium K-lines response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AhPremiumKlines {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AhPremiumIntraday {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AhPremiumKline {
    /// A-share price
    pub aprice: String,
    /// A-share previous close
    pub apreclose: String,
    /// H-share price
    pub hprice: String,
    /// H-share previous close
    pub hpreclose: String,
    /// CNY/HKD exchange rate
    pub currency_rate: String,
    /// A/H premium rate (negative = H-share at premium)
    pub ahpremium_rate: String,
    /// Price spread
    pub price_spread: String,
    /// Data point timestamp (unix seconds)
    pub timestamp: i64,
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
            timestamp: v.timestamp.unix_timestamp(),
        }
    }
}

/// Trade statistics response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct TradeStatsResponse {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct TradeStatistics {
    /// Volume-weighted average price
    pub avgprice: String,
    /// Total buy volume (shares)
    pub buy: String,
    /// Total neutral / unknown-direction volume
    pub neutral: String,
    /// Previous close price
    pub preclose: String,
    /// Total sell volume (shares)
    pub sell: String,
    /// Data timestamp (unix timestamp string)
    pub timestamp: String,
    /// Total trading volume (shares)
    pub total_amount: String,
    /// Unix timestamps for the last 5 trading days
    pub trade_date: Vec<String>,
    /// Total number of trades
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct TradePriceLevel {
    /// Buy volume at this price
    pub buy_amount: String,
    /// Neutral (unknown direction) volume at this price
    pub neutral_amount: String,
    /// Price level
    pub price: String,
    /// Sell volume at this price
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

/// Market anomaly response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AnomalyResponse {
    /// Whether anomaly alerts are globally disabled
    pub all_off: bool,
    /// List of market anomaly events
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

/// One market anomaly event (e.g. large block trade, margin buying surge)
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AnomalyItem {
    /// Security symbol
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

/// Index constituents response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
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

/// One constituent stock of an index
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ConstituentStock {
    /// Security symbol
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Latest price
    pub last_done: Option<String>,
    /// Previous close
    pub prev_close: Option<String>,
    /// Net capital inflow today
    pub inflow: Option<String>,
    /// Turnover amount
    pub balance: Option<String>,
    /// Trading volume (shares)
    pub amount: Option<String>,
    /// Total shares outstanding
    pub total_shares: Option<String>,
    /// Tags, e.g. `["领涨龙头"]`
    pub tags: Vec<String>,
    /// Brief description
    pub intro: String,
    /// Market, e.g. `"HK"`
    pub market: String,
    /// Circulating shares
    pub circulating_shares: Option<String>,
    /// Whether this is a delayed quote
    pub delay: bool,
    /// Day change percentage
    pub chg: Option<String>,
    /// Raw trade status code
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

/// Broker holding lookback period
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum BrokerHoldingPeriod {
    /// 1-day change
    Rct1,
    /// 5-day change
    Rct5,
    /// 20-day change
    Rct20,
    /// 60-day change
    Rct60,
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
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
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
    Day,
    /// Weekly
    Week,
    /// Monthly
    Month,
    /// Yearly
    Year,
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

// ── TopMoversResponse ─────────────────────────────────────────────

/// Top movers response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct TopMoversResponse {
    /// Raw top movers data (JSON string)
    pub data: String,
}

impl From<lb::TopMoversResponse> for TopMoversResponse {
    fn from(v: lb::TopMoversResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── RankCategoriesResponse ────────────────────────────────────────

/// Rank categories response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct RankCategoriesResponse {
    /// Raw rank categories data (JSON string)
    pub data: String,
}

impl From<lb::RankCategoriesResponse> for RankCategoriesResponse {
    fn from(v: lb::RankCategoriesResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}

// ── RankListResponse ──────────────────────────────────────────────

/// Rank list response. `data` is a JSON string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct RankListResponse {
    /// Raw rank list data (JSON string)
    pub data: String,
}

impl From<lb::RankListResponse> for RankListResponse {
    fn from(v: lb::RankListResponse) -> Self {
        Self {
            data: v.data.to_string(),
        }
    }
}
