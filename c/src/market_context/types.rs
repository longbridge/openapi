use std::os::raw::c_char;

use longbridge::market::{
    AhPremiumIntraday, AhPremiumKline, AhPremiumKlines, AnomalyItem, AnomalyResponse,
    BrokerHoldingChanges, BrokerHoldingDailyHistory, BrokerHoldingDailyItem, BrokerHoldingDetail,
    BrokerHoldingDetailItem, BrokerHoldingEntry, BrokerHoldingTop, ConstituentStock,
    IndexConstituents, MarketStatusResponse, MarketTimeItem, RankCategoriesResponse, RankListItem,
    RankListResponse, TopMoversEvent, TopMoversResponse, TopMoversStock, TradePriceLevel,
    TradeStatistics, TradeStatsResponse,
};

use crate::types::{CMarket, CString, CVec, ToFFI};

// ── MarketStatusResponse ──────────────────────────────────────────

/// Market trading time item describing the current status of a single market.
#[repr(C)]
pub struct CMarketTimeItem {
    /// Market identifier.
    pub market: CMarket,
    /// Current trade status code for the market.
    pub trade_status: i32,
    /// Timestamp of the current trade status as an ISO-8601 string.
    pub timestamp: *const c_char,
    /// Delayed trade status code for the market.
    pub delay_trade_status: i32,
    /// Timestamp of the delayed trade status as an ISO-8601 string.
    pub delay_timestamp: *const c_char,
    /// Sub-status code for the current trade status.
    pub sub_status: i32,
    /// Sub-status code for the delayed trade status.
    pub delay_sub_status: i32,
}

pub(crate) struct CMarketTimeItemOwned {
    market: CMarket,
    trade_status: i32,
    timestamp: CString,
    delay_trade_status: i32,
    delay_timestamp: CString,
    sub_status: i32,
    delay_sub_status: i32,
}

impl From<MarketTimeItem> for CMarketTimeItemOwned {
    fn from(v: MarketTimeItem) -> Self {
        Self {
            market: v.market.into(),
            trade_status: v.trade_status.code(),
            timestamp: v.timestamp.into(),
            delay_trade_status: v.delay_trade_status.code(),
            delay_timestamp: v.delay_timestamp.into(),
            sub_status: v.sub_status,
            delay_sub_status: v.delay_sub_status,
        }
    }
}

impl ToFFI for CMarketTimeItemOwned {
    type FFIType = CMarketTimeItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CMarketTimeItem {
            market: self.market,
            trade_status: self.trade_status,
            timestamp: self.timestamp.to_ffi_type(),
            delay_trade_status: self.delay_trade_status,
            delay_timestamp: self.delay_timestamp.to_ffi_type(),
            sub_status: self.sub_status,
            delay_sub_status: self.delay_sub_status,
        }
    }
}

/// Response containing the trading status for all markets.
#[repr(C)]
pub struct CMarketStatusResponse {
    /// Pointer to array of market time items.
    pub market_time: *const CMarketTimeItem,
    /// Number of elements in the array.
    pub num_market_time: usize,
}

pub(crate) struct CMarketStatusResponseOwned {
    market_time: CVec<CMarketTimeItemOwned>,
}

impl From<MarketStatusResponse> for CMarketStatusResponseOwned {
    fn from(v: MarketStatusResponse) -> Self {
        Self {
            market_time: v.market_time.into(),
        }
    }
}

impl ToFFI for CMarketStatusResponseOwned {
    type FFIType = CMarketStatusResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CMarketStatusResponse {
            market_time: self.market_time.to_ffi_type(),
            num_market_time: self.market_time.len(),
        }
    }
}

// ── BrokerHolding ─────────────────────────────────────────────────

/// A single broker entry in a broker holding top list.
#[repr(C)]
pub struct CBrokerHoldingEntry {
    /// Name of the broker.
    pub name: *const c_char,
    /// Participant number identifying the broker.
    pub parti_number: *const c_char,
    /// Change value as a decimal string.
    pub chg: *const c_char,
    /// Whether this broker is marked as a strong holder.
    pub strong: bool,
}

pub(crate) struct CBrokerHoldingEntryOwned {
    name: CString,
    parti_number: CString,
    chg: CString,
    strong: bool,
}

impl From<BrokerHoldingEntry> for CBrokerHoldingEntryOwned {
    fn from(v: BrokerHoldingEntry) -> Self {
        Self {
            name: v.name.into(),
            parti_number: v.parti_number.into(),
            chg: v.chg.map(|d| d.to_string()).unwrap_or_default().into(),
            strong: v.strong,
        }
    }
}

impl ToFFI for CBrokerHoldingEntryOwned {
    type FFIType = CBrokerHoldingEntry;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingEntry {
            name: self.name.to_ffi_type(),
            parti_number: self.parti_number.to_ffi_type(),
            chg: self.chg.to_ffi_type(),
            strong: self.strong,
        }
    }
}

/// Top broker holdings for a security, split into top buyers and top sellers.
#[repr(C)]
pub struct CBrokerHoldingTop {
    /// Pointer to array of top-buying broker entries.
    pub buy: *const CBrokerHoldingEntry,
    /// Number of elements in the buy array.
    pub num_buy: usize,
    /// Pointer to array of top-selling broker entries.
    pub sell: *const CBrokerHoldingEntry,
    /// Number of elements in the sell array.
    pub num_sell: usize,
    /// Timestamp of the last update as an ISO-8601 string.
    pub updated_at: *const c_char,
}

pub(crate) struct CBrokerHoldingTopOwned {
    buy: CVec<CBrokerHoldingEntryOwned>,
    sell: CVec<CBrokerHoldingEntryOwned>,
    updated_at: CString,
}

impl From<BrokerHoldingTop> for CBrokerHoldingTopOwned {
    fn from(v: BrokerHoldingTop) -> Self {
        Self {
            buy: v.buy.into(),
            sell: v.sell.into(),
            updated_at: v.updated_at.into(),
        }
    }
}

impl ToFFI for CBrokerHoldingTopOwned {
    type FFIType = CBrokerHoldingTop;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingTop {
            buy: self.buy.to_ffi_type(),
            num_buy: self.buy.len(),
            sell: self.sell.to_ffi_type(),
            num_sell: self.sell.len(),
            updated_at: self.updated_at.to_ffi_type(),
        }
    }
}

// ── BrokerHoldingDetail ───────────────────────────────────────────

/// A set of holding change values over multiple time windows.
#[repr(C)]
pub struct CBrokerHoldingChanges {
    /// Current value as a decimal string.
    pub value: *const c_char,
    /// Change over 1 day as a decimal string.
    pub chg_1: *const c_char,
    /// Change over 5 days as a decimal string.
    pub chg_5: *const c_char,
    /// Change over 20 days as a decimal string.
    pub chg_20: *const c_char,
    /// Change over 60 days as a decimal string.
    pub chg_60: *const c_char,
}

pub(crate) struct CBrokerHoldingChangesOwned {
    value: CString,
    chg_1: CString,
    chg_5: CString,
    chg_20: CString,
    chg_60: CString,
}

impl From<BrokerHoldingChanges> for CBrokerHoldingChangesOwned {
    fn from(v: BrokerHoldingChanges) -> Self {
        Self {
            value: v.value.map(|d| d.to_string()).unwrap_or_default().into(),
            chg_1: v.chg_1.map(|d| d.to_string()).unwrap_or_default().into(),
            chg_5: v.chg_5.map(|d| d.to_string()).unwrap_or_default().into(),
            chg_20: v.chg_20.map(|d| d.to_string()).unwrap_or_default().into(),
            chg_60: v.chg_60.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CBrokerHoldingChangesOwned {
    type FFIType = CBrokerHoldingChanges;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingChanges {
            value: self.value.to_ffi_type(),
            chg_1: self.chg_1.to_ffi_type(),
            chg_5: self.chg_5.to_ffi_type(),
            chg_20: self.chg_20.to_ffi_type(),
            chg_60: self.chg_60.to_ffi_type(),
        }
    }
}

/// Detailed holding information for a single broker in the broker holding
/// detail list.
#[repr(C)]
pub struct CBrokerHoldingDetailItem {
    /// Name of the broker.
    pub name: *const c_char,
    /// Participant number identifying the broker.
    pub parti_number: *const c_char,
    /// Holding ratio and its changes over multiple time windows.
    pub ratio: CBrokerHoldingChanges,
    /// Absolute share count and its changes over multiple time windows.
    pub shares: CBrokerHoldingChanges,
    /// Whether this broker is marked as a strong holder.
    pub strong: bool,
}

pub(crate) struct CBrokerHoldingDetailItemOwned {
    name: CString,
    parti_number: CString,
    ratio: CBrokerHoldingChangesOwned,
    shares: CBrokerHoldingChangesOwned,
    strong: bool,
}

impl From<BrokerHoldingDetailItem> for CBrokerHoldingDetailItemOwned {
    fn from(v: BrokerHoldingDetailItem) -> Self {
        Self {
            name: v.name.into(),
            parti_number: v.parti_number.into(),
            ratio: v.ratio.into(),
            shares: v.shares.into(),
            strong: v.strong,
        }
    }
}

impl ToFFI for CBrokerHoldingDetailItemOwned {
    type FFIType = CBrokerHoldingDetailItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingDetailItem {
            name: self.name.to_ffi_type(),
            parti_number: self.parti_number.to_ffi_type(),
            ratio: self.ratio.to_ffi_type(),
            shares: self.shares.to_ffi_type(),
            strong: self.strong,
        }
    }
}

/// Full broker holding detail response for a security.
#[repr(C)]
pub struct CBrokerHoldingDetail {
    /// Pointer to array of broker holding detail items.
    pub list: *const CBrokerHoldingDetailItem,
    /// Number of elements in the array.
    pub num_list: usize,
    /// Timestamp of the last update as an ISO-8601 string.
    pub updated_at: *const c_char,
}

pub(crate) struct CBrokerHoldingDetailOwned {
    list: CVec<CBrokerHoldingDetailItemOwned>,
    updated_at: CString,
}

impl From<BrokerHoldingDetail> for CBrokerHoldingDetailOwned {
    fn from(v: BrokerHoldingDetail) -> Self {
        Self {
            list: v.list.into(),
            updated_at: v.updated_at.into(),
        }
    }
}

impl ToFFI for CBrokerHoldingDetailOwned {
    type FFIType = CBrokerHoldingDetail;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingDetail {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
            updated_at: self.updated_at.to_ffi_type(),
        }
    }
}

// ── BrokerHoldingDaily ────────────────────────────────────────────

/// A single day's broker holding record.
#[repr(C)]
pub struct CBrokerHoldingDailyItem {
    /// Date of the record as a string (e.g. `"2024-01-15"`).
    pub date: *const c_char,
    /// Total shares held by the broker on this date as a decimal string.
    pub holding: *const c_char,
    /// Holding ratio as a decimal string.
    pub ratio: *const c_char,
    /// Day-over-day change in holdings as a decimal string.
    pub chg: *const c_char,
}

pub(crate) struct CBrokerHoldingDailyItemOwned {
    date: CString,
    holding: CString,
    ratio: CString,
    chg: CString,
}

impl From<BrokerHoldingDailyItem> for CBrokerHoldingDailyItemOwned {
    fn from(v: BrokerHoldingDailyItem) -> Self {
        Self {
            date: v.date.into(),
            holding: v.holding.map(|d| d.to_string()).unwrap_or_default().into(),
            ratio: v.ratio.map(|d| d.to_string()).unwrap_or_default().into(),
            chg: v.chg.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CBrokerHoldingDailyItemOwned {
    type FFIType = CBrokerHoldingDailyItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingDailyItem {
            date: self.date.to_ffi_type(),
            holding: self.holding.to_ffi_type(),
            ratio: self.ratio.to_ffi_type(),
            chg: self.chg.to_ffi_type(),
        }
    }
}

/// Historical daily broker holding records for a security.
#[repr(C)]
pub struct CBrokerHoldingDailyHistory {
    /// Pointer to array of daily broker holding items.
    pub list: *const CBrokerHoldingDailyItem,
    /// Number of elements in the array.
    pub num_list: usize,
}

pub(crate) struct CBrokerHoldingDailyHistoryOwned {
    list: CVec<CBrokerHoldingDailyItemOwned>,
}

impl From<BrokerHoldingDailyHistory> for CBrokerHoldingDailyHistoryOwned {
    fn from(v: BrokerHoldingDailyHistory) -> Self {
        Self {
            list: v.list.into(),
        }
    }
}

impl ToFFI for CBrokerHoldingDailyHistoryOwned {
    type FFIType = CBrokerHoldingDailyHistory;
    fn to_ffi_type(&self) -> Self::FFIType {
        CBrokerHoldingDailyHistory {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
        }
    }
}

// ── AhPremium ─────────────────────────────────────────────────────

/// A single candlestick data point for the A/H share premium.
#[repr(C)]
pub struct CAhPremiumKline {
    /// A-share price as a decimal string.
    pub aprice: *const c_char,
    /// A-share previous close price as a decimal string.
    pub apreclose: *const c_char,
    /// H-share price as a decimal string.
    pub hprice: *const c_char,
    /// H-share previous close price as a decimal string.
    pub hpreclose: *const c_char,
    /// CNY/HKD currency exchange rate as a decimal string.
    pub currency_rate: *const c_char,
    /// A/H premium rate as a decimal string.
    pub ahpremium_rate: *const c_char,
    /// Price spread between A-share and H-share as a decimal string.
    pub price_spread: *const c_char,
    /// Unix timestamp (seconds) of this data point.
    pub timestamp: i64,
}

pub(crate) struct CAhPremiumKlineOwned {
    aprice: CString,
    apreclose: CString,
    hprice: CString,
    hpreclose: CString,
    currency_rate: CString,
    ahpremium_rate: CString,
    price_spread: CString,
    timestamp: i64,
}

impl From<AhPremiumKline> for CAhPremiumKlineOwned {
    fn from(v: AhPremiumKline) -> Self {
        Self {
            aprice: v.aprice.to_string().into(),
            apreclose: v.apreclose.to_string().into(),
            hprice: v.hprice.to_string().into(),
            hpreclose: v.hpreclose.to_string().into(),
            currency_rate: v.currency_rate.to_string().into(),
            ahpremium_rate: v.ahpremium_rate.to_string().into(),
            price_spread: v.price_spread.to_string().into(),
            timestamp: v.timestamp.unix_timestamp(),
        }
    }
}

impl ToFFI for CAhPremiumKlineOwned {
    type FFIType = CAhPremiumKline;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAhPremiumKline {
            aprice: self.aprice.to_ffi_type(),
            apreclose: self.apreclose.to_ffi_type(),
            hprice: self.hprice.to_ffi_type(),
            hpreclose: self.hpreclose.to_ffi_type(),
            currency_rate: self.currency_rate.to_ffi_type(),
            ahpremium_rate: self.ahpremium_rate.to_ffi_type(),
            price_spread: self.price_spread.to_ffi_type(),
            timestamp: self.timestamp,
        }
    }
}

/// Historical A/H premium kline data.
#[repr(C)]
pub struct CAhPremiumKlines {
    /// Pointer to array of A/H premium kline data points.
    pub klines: *const CAhPremiumKline,
    /// Number of elements in the array.
    pub num_klines: usize,
}

pub(crate) struct CAhPremiumKlinesOwned {
    klines: CVec<CAhPremiumKlineOwned>,
}

impl From<AhPremiumKlines> for CAhPremiumKlinesOwned {
    fn from(v: AhPremiumKlines) -> Self {
        Self {
            klines: v.klines.into(),
        }
    }
}

impl ToFFI for CAhPremiumKlinesOwned {
    type FFIType = CAhPremiumKlines;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAhPremiumKlines {
            klines: self.klines.to_ffi_type(),
            num_klines: self.klines.len(),
        }
    }
}

/// Intraday A/H premium data for the current trading session.
#[repr(C)]
pub struct CAhPremiumIntraday {
    /// Pointer to array of intraday A/H premium kline data points.
    pub klines: *const CAhPremiumKline,
    /// Number of elements in the array.
    pub num_klines: usize,
}

pub(crate) struct CAhPremiumIntradayOwned {
    klines: CVec<CAhPremiumKlineOwned>,
}

impl From<AhPremiumIntraday> for CAhPremiumIntradayOwned {
    fn from(v: AhPremiumIntraday) -> Self {
        Self {
            klines: v.klines.into(),
        }
    }
}

impl ToFFI for CAhPremiumIntradayOwned {
    type FFIType = CAhPremiumIntraday;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAhPremiumIntraday {
            klines: self.klines.to_ffi_type(),
            num_klines: self.klines.len(),
        }
    }
}

// ── TradeStats ────────────────────────────────────────────────────

/// Trade volume breakdown at a single price level.
#[repr(C)]
pub struct CTradePriceLevel {
    /// Total buy-side trade amount at this price level as a decimal string.
    pub buy_amount: *const c_char,
    /// Total neutral (unknown direction) trade amount at this price level as a
    /// decimal string.
    pub neutral_amount: *const c_char,
    /// Price of this level as a decimal string.
    pub price: *const c_char,
    /// Total sell-side trade amount at this price level as a decimal string.
    pub sell_amount: *const c_char,
}

pub(crate) struct CTradePriceLevelOwned {
    buy_amount: CString,
    neutral_amount: CString,
    price: CString,
    sell_amount: CString,
}

impl From<TradePriceLevel> for CTradePriceLevelOwned {
    fn from(v: TradePriceLevel) -> Self {
        Self {
            buy_amount: v.buy_amount.to_string().into(),
            neutral_amount: v.neutral_amount.to_string().into(),
            price: v.price.to_string().into(),
            sell_amount: v.sell_amount.to_string().into(),
        }
    }
}

impl ToFFI for CTradePriceLevelOwned {
    type FFIType = CTradePriceLevel;
    fn to_ffi_type(&self) -> Self::FFIType {
        CTradePriceLevel {
            buy_amount: self.buy_amount.to_ffi_type(),
            neutral_amount: self.neutral_amount.to_ffi_type(),
            price: self.price.to_ffi_type(),
            sell_amount: self.sell_amount.to_ffi_type(),
        }
    }
}

/// Aggregated trade statistics for a security over a period.
#[repr(C)]
pub struct CTradeStatistics {
    /// Volume-weighted average price as a decimal string.
    pub avgprice: *const c_char,
    /// Total buy-side trade amount as a decimal string.
    pub buy: *const c_char,
    /// Total neutral (unknown direction) trade amount as a decimal string.
    pub neutral: *const c_char,
    /// Previous close price as a decimal string.
    pub preclose: *const c_char,
    /// Total sell-side trade amount as a decimal string.
    pub sell: *const c_char,
    /// Timestamp of the statistics snapshot as an ISO-8601 string.
    pub timestamp: *const c_char,
    /// Total traded amount (buy + sell + neutral) as a decimal string.
    pub total_amount: *const c_char,
    /// Pointer to array of trade date strings (e.g. `"2024-01-15"`).
    pub trade_date: *const *const c_char,
    /// Number of elements in the trade_date array.
    pub num_trade_date: usize,
    /// Total number of individual trades as a decimal string.
    pub trades_count: *const c_char,
}

pub(crate) struct CTradeStatisticsOwned {
    avgprice: CString,
    buy: CString,
    neutral: CString,
    preclose: CString,
    sell: CString,
    timestamp: CString,
    total_amount: CString,
    trade_date: CVec<CString>,
    trades_count: CString,
}

impl From<TradeStatistics> for CTradeStatisticsOwned {
    fn from(v: TradeStatistics) -> Self {
        Self {
            avgprice: v.avgprice.to_string().into(),
            buy: v.buy.to_string().into(),
            neutral: v.neutral.to_string().into(),
            preclose: v.preclose.to_string().into(),
            sell: v.sell.to_string().into(),
            timestamp: v.timestamp.into(),
            total_amount: v.total_amount.to_string().into(),
            trade_date: v
                .trade_date
                .into_iter()
                .map(CString::from)
                .collect::<Vec<_>>()
                .into(),
            trades_count: v.trades_count.into(),
        }
    }
}

impl ToFFI for CTradeStatisticsOwned {
    type FFIType = CTradeStatistics;
    fn to_ffi_type(&self) -> Self::FFIType {
        CTradeStatistics {
            avgprice: self.avgprice.to_ffi_type(),
            buy: self.buy.to_ffi_type(),
            neutral: self.neutral.to_ffi_type(),
            preclose: self.preclose.to_ffi_type(),
            sell: self.sell.to_ffi_type(),
            timestamp: self.timestamp.to_ffi_type(),
            total_amount: self.total_amount.to_ffi_type(),
            trade_date: self.trade_date.to_ffi_type(),
            num_trade_date: self.trade_date.len(),
            trades_count: self.trades_count.to_ffi_type(),
        }
    }
}

/// Full trade statistics response combining aggregate stats and per-price-level
/// breakdown.
#[repr(C)]
pub struct CTradeStatsResponse {
    /// Aggregated trade statistics for the security.
    pub statistics: CTradeStatistics,
    /// Pointer to array of per-price-level trade breakdowns.
    pub trades: *const CTradePriceLevel,
    /// Number of elements in the trades array.
    pub num_trades: usize,
}

pub(crate) struct CTradeStatsResponseOwned {
    statistics: CTradeStatisticsOwned,
    trades: CVec<CTradePriceLevelOwned>,
}

impl From<TradeStatsResponse> for CTradeStatsResponseOwned {
    fn from(v: TradeStatsResponse) -> Self {
        Self {
            statistics: v.statistics.into(),
            trades: v.trades.into(),
        }
    }
}

impl ToFFI for CTradeStatsResponseOwned {
    type FFIType = CTradeStatsResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CTradeStatsResponse {
            statistics: self.statistics.to_ffi_type(),
            trades: self.trades.to_ffi_type(),
            num_trades: self.trades.len(),
        }
    }
}

// ── Anomaly ───────────────────────────────────────────────────────

/// A single market anomaly alert item.
#[repr(C)]
pub struct CAnomalyItem {
    /// Security symbol (e.g. `"700.HK"`).
    pub symbol: *const c_char,
    /// Security name string.
    pub name: *const c_char,
    /// Name of the anomaly alert type.
    pub alert_name: *const c_char,
    /// Unix timestamp (seconds) when the alert was triggered.
    pub alert_time: i64,
    /// Pointer to array of change value strings describing the anomaly.
    pub change_values: *const *const c_char,
    /// Number of elements in the change_values array.
    pub num_change_values: usize,
    /// Sentiment/emotion indicator for the anomaly (positive/negative
    /// direction).
    pub emotion: i32,
}

pub(crate) struct CAnomalyItemOwned {
    symbol: CString,
    name: CString,
    alert_name: CString,
    alert_time: i64,
    change_values: CVec<CString>,
    emotion: i32,
}

impl From<AnomalyItem> for CAnomalyItemOwned {
    fn from(v: AnomalyItem) -> Self {
        Self {
            symbol: v.symbol.into(),
            name: v.name.into(),
            alert_name: v.alert_name.into(),
            alert_time: v.alert_time,
            change_values: v
                .change_values
                .into_iter()
                .map(CString::from)
                .collect::<Vec<_>>()
                .into(),
            emotion: v.emotion,
        }
    }
}

impl ToFFI for CAnomalyItemOwned {
    type FFIType = CAnomalyItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAnomalyItem {
            symbol: self.symbol.to_ffi_type(),
            name: self.name.to_ffi_type(),
            alert_name: self.alert_name.to_ffi_type(),
            alert_time: self.alert_time,
            change_values: self.change_values.to_ffi_type(),
            num_change_values: self.change_values.len(),
            emotion: self.emotion,
        }
    }
}

/// Response containing a list of market anomaly alerts.
#[repr(C)]
pub struct CAnomalyResponse {
    /// Whether all anomaly alerts are turned off.
    pub all_off: bool,
    /// Pointer to array of anomaly alert items.
    pub changes: *const CAnomalyItem,
    /// Number of elements in the changes array.
    pub num_changes: usize,
}

pub(crate) struct CAnomalyResponseOwned {
    all_off: bool,
    changes: CVec<CAnomalyItemOwned>,
}

impl From<AnomalyResponse> for CAnomalyResponseOwned {
    fn from(v: AnomalyResponse) -> Self {
        Self {
            all_off: v.all_off,
            changes: v.changes.into(),
        }
    }
}

impl ToFFI for CAnomalyResponseOwned {
    type FFIType = CAnomalyResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAnomalyResponse {
            all_off: self.all_off,
            changes: self.changes.to_ffi_type(),
            num_changes: self.changes.len(),
        }
    }
}

// ── IndexConstituents ─────────────────────────────────────────────

/// A constituent stock within an index.
#[repr(C)]
pub struct CConstituentStock {
    /// Security symbol (e.g. `"700.HK"`).
    pub symbol: *const c_char,
    /// Security name string.
    pub name: *const c_char,
    /// Latest traded price as a decimal string.
    pub last_done: *const c_char,
    /// Previous close price as a decimal string.
    pub prev_close: *const c_char,
    /// Net capital inflow for the stock as a decimal string.
    pub inflow: *const c_char,
    /// Outstanding balance (remaining sell-side liquidity) as a decimal string.
    pub balance: *const c_char,
    /// Total traded amount for the session as a decimal string.
    pub amount: *const c_char,
    /// Total issued shares as a decimal string.
    pub total_shares: *const c_char,
    /// Pointer to array of tag strings associated with the stock.
    pub tags: *const *const c_char,
    /// Number of elements in the tags array.
    pub num_tags: usize,
    /// Brief introductory description of the stock.
    pub intro: *const c_char,
    /// Market identifier string (e.g. `"HK"`, `"US"`).
    pub market: *const c_char,
    /// Number of circulating (publicly tradeable) shares as a decimal string.
    pub circulating_shares: *const c_char,
    /// Whether the quote data for this stock is delayed.
    pub delay: bool,
    /// Price change (from previous close) as a decimal string.
    pub chg: *const c_char,
    /// Current trade status code for the stock.
    pub trade_status: i32,
}

pub(crate) struct CConstituentStockOwned {
    symbol: CString,
    name: CString,
    last_done: CString,
    prev_close: CString,
    inflow: CString,
    balance: CString,
    amount: CString,
    total_shares: CString,
    tags: CVec<CString>,
    intro: CString,
    market: CString,
    circulating_shares: CString,
    delay: bool,
    chg: CString,
    trade_status: i32,
}

impl From<ConstituentStock> for CConstituentStockOwned {
    fn from(v: ConstituentStock) -> Self {
        Self {
            symbol: v.symbol.into(),
            name: v.name.into(),
            last_done: v
                .last_done
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            prev_close: v
                .prev_close
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            inflow: v.inflow.map(|d| d.to_string()).unwrap_or_default().into(),
            balance: v.balance.map(|d| d.to_string()).unwrap_or_default().into(),
            amount: v.amount.map(|d| d.to_string()).unwrap_or_default().into(),
            total_shares: v
                .total_shares
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            tags: v
                .tags
                .into_iter()
                .map(CString::from)
                .collect::<Vec<_>>()
                .into(),
            intro: v.intro.into(),
            market: v.market.into(),
            circulating_shares: v
                .circulating_shares
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            delay: v.delay,
            chg: v.chg.map(|d| d.to_string()).unwrap_or_default().into(),
            trade_status: v.trade_status,
        }
    }
}

impl ToFFI for CConstituentStockOwned {
    type FFIType = CConstituentStock;
    fn to_ffi_type(&self) -> Self::FFIType {
        CConstituentStock {
            symbol: self.symbol.to_ffi_type(),
            name: self.name.to_ffi_type(),
            last_done: self.last_done.to_ffi_type(),
            prev_close: self.prev_close.to_ffi_type(),
            inflow: self.inflow.to_ffi_type(),
            balance: self.balance.to_ffi_type(),
            amount: self.amount.to_ffi_type(),
            total_shares: self.total_shares.to_ffi_type(),
            tags: self.tags.to_ffi_type(),
            num_tags: self.tags.len(),
            intro: self.intro.to_ffi_type(),
            market: self.market.to_ffi_type(),
            circulating_shares: self.circulating_shares.to_ffi_type(),
            delay: self.delay,
            chg: self.chg.to_ffi_type(),
            trade_status: self.trade_status,
        }
    }
}

/// Index constituent data including breadth statistics and the list of member
/// stocks.
#[repr(C)]
pub struct CIndexConstituents {
    /// Number of constituent stocks that declined in this session.
    pub fall_num: i32,
    /// Number of constituent stocks that were unchanged in this session.
    pub flat_num: i32,
    /// Number of constituent stocks that advanced in this session.
    pub rise_num: i32,
    /// Pointer to array of constituent stock data.
    pub stocks: *const CConstituentStock,
    /// Number of elements in the stocks array.
    pub num_stocks: usize,
}

pub(crate) struct CIndexConstituentsOwned {
    fall_num: i32,
    flat_num: i32,
    rise_num: i32,
    stocks: CVec<CConstituentStockOwned>,
}

impl From<IndexConstituents> for CIndexConstituentsOwned {
    fn from(v: IndexConstituents) -> Self {
        Self {
            fall_num: v.fall_num,
            flat_num: v.flat_num,
            rise_num: v.rise_num,
            stocks: v.stocks.into(),
        }
    }
}

impl ToFFI for CIndexConstituentsOwned {
    type FFIType = CIndexConstituents;
    fn to_ffi_type(&self) -> Self::FFIType {
        CIndexConstituents {
            fall_num: self.fall_num,
            flat_num: self.flat_num,
            rise_num: self.rise_num,
            stocks: self.stocks.to_ffi_type(),
            num_stocks: self.stocks.len(),
        }
    }
}

// ── TopMoversResponse ─────────────────────────────────────────────

/// Stock information within a top-movers event.
#[repr(C)]
pub struct CTopMoversStock {
    /// Symbol, e.g. "TSLA.US"
    pub symbol: *const c_char,
    /// Ticker code
    pub code: *const c_char,
    /// Security name
    pub name: *const c_char,
    /// Full name
    pub full_name: *const c_char,
    /// Price change (decimal ratio)
    pub change: *const c_char,
    /// Latest price
    pub last_done: *const c_char,
    /// Market code
    pub market: *const c_char,
    /// Logo URL
    pub logo: *const c_char,
    /// Labels / tags
    pub labels: *const *const c_char,
    /// Number of items in `labels`
    pub num_labels: usize,
}

pub(crate) struct CTopMoversStockOwned {
    symbol: CString,
    code: CString,
    name: CString,
    full_name: CString,
    change: CString,
    last_done: CString,
    market: CString,
    logo: CString,
    labels: CVec<CString>,
}

impl From<TopMoversStock> for CTopMoversStockOwned {
    fn from(v: TopMoversStock) -> Self {
        Self {
            symbol: v.symbol.into(),
            code: v.code.into(),
            name: v.name.into(),
            full_name: v.full_name.into(),
            change: v.change.into(),
            last_done: v.last_done.into(),
            market: v.market.into(),
            logo: v.logo.into(),
            labels: v.labels.into(),
        }
    }
}

impl ToFFI for CTopMoversStockOwned {
    type FFIType = CTopMoversStock;
    fn to_ffi_type(&self) -> Self::FFIType {
        CTopMoversStock {
            symbol: self.symbol.to_ffi_type(),
            code: self.code.to_ffi_type(),
            name: self.name.to_ffi_type(),
            full_name: self.full_name.to_ffi_type(),
            change: self.change.to_ffi_type(),
            last_done: self.last_done.to_ffi_type(),
            market: self.market.to_ffi_type(),
            logo: self.logo.to_ffi_type(),
            labels: self.labels.to_ffi_type(),
            num_labels: self.labels.len(),
        }
    }
}

/// One top-movers event entry.
#[repr(C)]
pub struct CTopMoversEvent {
    /// Event time (RFC 3339)
    pub timestamp: *const c_char,
    /// Alert reason description
    pub alert_reason: *const c_char,
    /// Alert type code
    pub alert_type: i64,
    /// Stock information
    pub stock: CTopMoversStock,
    /// Associated news post as a JSON string (may be null)
    pub post: *const c_char,
}

pub(crate) struct CTopMoversEventOwned {
    timestamp: CString,
    alert_reason: CString,
    alert_type: i64,
    stock: CTopMoversStockOwned,
    post: CString,
}

impl From<TopMoversEvent> for CTopMoversEventOwned {
    fn from(v: TopMoversEvent) -> Self {
        Self {
            timestamp: v.timestamp.into(),
            alert_reason: v.alert_reason.into(),
            alert_type: v.alert_type,
            stock: v.stock.into(),
            post: v.post.to_string().into(),
        }
    }
}

impl ToFFI for CTopMoversEventOwned {
    type FFIType = CTopMoversEvent;
    fn to_ffi_type(&self) -> Self::FFIType {
        CTopMoversEvent {
            timestamp: self.timestamp.to_ffi_type(),
            alert_reason: self.alert_reason.to_ffi_type(),
            alert_type: self.alert_type,
            stock: self.stock.to_ffi_type(),
            post: self.post.to_ffi_type(),
        }
    }
}

/// Top movers response.
#[repr(C)]
pub struct CTopMoversResponse {
    /// Pointer to the array of top-mover events
    pub events: *const CTopMoversEvent,
    /// Number of items in `events`
    pub num_events: usize,
    /// Pagination cursor as a JSON string
    pub next_params: *const c_char,
}

pub(crate) struct CTopMoversResponseOwned {
    events: CVec<CTopMoversEventOwned>,
    next_params: CString,
}

impl From<TopMoversResponse> for CTopMoversResponseOwned {
    fn from(v: TopMoversResponse) -> Self {
        Self {
            events: v.events.into(),
            next_params: v.next_params.to_string().into(),
        }
    }
}

impl ToFFI for CTopMoversResponseOwned {
    type FFIType = CTopMoversResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CTopMoversResponse {
            events: self.events.to_ffi_type(),
            num_events: self.events.len(),
            next_params: self.next_params.to_ffi_type(),
        }
    }
}

// ── RankCategoriesResponse ────────────────────────────────────────

/// Rank categories response. `data` is a NUL-terminated JSON string.
#[repr(C)]
pub struct CRankCategoriesResponse {
    /// Raw rank categories data as a JSON string
    pub data: *const c_char,
}

pub(crate) struct CRankCategoriesResponseOwned {
    data: CString,
}

impl From<RankCategoriesResponse> for CRankCategoriesResponseOwned {
    fn from(v: RankCategoriesResponse) -> Self {
        let json = serde_json::to_string(&v.data).unwrap_or_default();
        Self { data: json.into() }
    }
}

impl ToFFI for CRankCategoriesResponseOwned {
    type FFIType = CRankCategoriesResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRankCategoriesResponse {
            data: self.data.to_ffi_type(),
        }
    }
}

// ── RankListResponse ──────────────────────────────────────────────

/// One ranked security item.
#[repr(C)]
pub struct CRankListItem {
    /// Symbol, e.g. "MU.US"
    pub symbol: *const c_char,
    /// Ticker code
    pub code: *const c_char,
    /// Security name
    pub name: *const c_char,
    /// Latest price
    pub last_done: *const c_char,
    /// Price change ratio (decimal)
    pub chg: *const c_char,
    /// Absolute price change
    pub change: *const c_char,
    /// Net inflow
    pub inflow: *const c_char,
    /// Market cap
    pub market_cap: *const c_char,
    /// Industry name
    pub industry: *const c_char,
    /// Pre/post market price
    pub pre_post_price: *const c_char,
    /// Pre/post market change
    pub pre_post_chg: *const c_char,
    /// Amplitude
    pub amplitude: *const c_char,
    /// 5-day change
    pub five_day_chg: *const c_char,
    /// Turnover rate
    pub turnover_rate: *const c_char,
    /// Volume ratio
    pub volume_rate: *const c_char,
    /// P/B ratio (TTM)
    pub pb_ttm: *const c_char,
}

pub(crate) struct CRankListItemOwned {
    symbol: CString,
    code: CString,
    name: CString,
    last_done: CString,
    chg: CString,
    change: CString,
    inflow: CString,
    market_cap: CString,
    industry: CString,
    pre_post_price: CString,
    pre_post_chg: CString,
    amplitude: CString,
    five_day_chg: CString,
    turnover_rate: CString,
    volume_rate: CString,
    pb_ttm: CString,
}

impl From<RankListItem> for CRankListItemOwned {
    fn from(v: RankListItem) -> Self {
        Self {
            symbol: v.symbol.into(),
            code: v.code.into(),
            name: v.name.into(),
            last_done: v.last_done.into(),
            chg: v.chg.into(),
            change: v.change.into(),
            inflow: v.inflow.into(),
            market_cap: v.market_cap.into(),
            industry: v.industry.into(),
            pre_post_price: v.pre_post_price.into(),
            pre_post_chg: v.pre_post_chg.into(),
            amplitude: v.amplitude.into(),
            five_day_chg: v.five_day_chg.into(),
            turnover_rate: v.turnover_rate.into(),
            volume_rate: v.volume_rate.into(),
            pb_ttm: v.pb_ttm.into(),
        }
    }
}

impl ToFFI for CRankListItemOwned {
    type FFIType = CRankListItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRankListItem {
            symbol: self.symbol.to_ffi_type(),
            code: self.code.to_ffi_type(),
            name: self.name.to_ffi_type(),
            last_done: self.last_done.to_ffi_type(),
            chg: self.chg.to_ffi_type(),
            change: self.change.to_ffi_type(),
            inflow: self.inflow.to_ffi_type(),
            market_cap: self.market_cap.to_ffi_type(),
            industry: self.industry.to_ffi_type(),
            pre_post_price: self.pre_post_price.to_ffi_type(),
            pre_post_chg: self.pre_post_chg.to_ffi_type(),
            amplitude: self.amplitude.to_ffi_type(),
            five_day_chg: self.five_day_chg.to_ffi_type(),
            turnover_rate: self.turnover_rate.to_ffi_type(),
            volume_rate: self.volume_rate.to_ffi_type(),
            pb_ttm: self.pb_ttm.to_ffi_type(),
        }
    }
}

/// Rank list response.
#[repr(C)]
pub struct CRankListResponse {
    /// Whether the response is delayed / BMP data
    pub bmp: bool,
    /// Pointer to the array of ranked security items
    pub lists: *const CRankListItem,
    /// Number of items in `lists`
    pub num_lists: usize,
}

pub(crate) struct CRankListResponseOwned {
    bmp: bool,
    lists: CVec<CRankListItemOwned>,
}

impl From<RankListResponse> for CRankListResponseOwned {
    fn from(v: RankListResponse) -> Self {
        Self {
            bmp: v.bmp,
            lists: v.lists.into(),
        }
    }
}

impl ToFFI for CRankListResponseOwned {
    type FFIType = CRankListResponse;
    fn to_ffi_type(&self) -> Self::FFIType {
        CRankListResponse {
            bmp: self.bmp,
            lists: self.lists.to_ffi_type(),
            num_lists: self.lists.len(),
        }
    }
}
