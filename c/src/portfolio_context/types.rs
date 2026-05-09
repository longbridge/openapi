use std::os::raw::c_char;

use longbridge::portfolio::types::*;

use crate::{
    portfolio_context::enum_types::{CAssetType, CFlowDirection},
    types::{CString, CVec, ToFFI},
};

/// A single currency exchange rate entry.
#[repr(C)]
pub struct CExchangeRate {
    /// Mid (average) exchange rate between the two currencies.
    pub average_rate: f64,
    /// Base currency code (e.g. "USD").
    pub base_currency: *const c_char,
    /// Bid rate (buy price) for the base currency.
    pub bid_rate: f64,
    /// Offer rate (sell price) for the base currency.
    pub offer_rate: f64,
    /// Counter currency code (e.g. "HKD").
    pub other_currency: *const c_char,
}
pub(crate) struct CExchangeRateOwned {
    average_rate: f64,
    base_currency: CString,
    bid_rate: f64,
    offer_rate: f64,
    other_currency: CString,
}
impl From<ExchangeRate> for CExchangeRateOwned {
    fn from(v: ExchangeRate) -> Self {
        Self {
            average_rate: v.average_rate,
            base_currency: v.base_currency.into(),
            bid_rate: v.bid_rate,
            offer_rate: v.offer_rate,
            other_currency: v.other_currency.into(),
        }
    }
}
impl ToFFI for CExchangeRateOwned {
    type FFIType = CExchangeRate;
    fn to_ffi_type(&self) -> Self::FFIType {
        CExchangeRate {
            average_rate: self.average_rate,
            base_currency: self.base_currency.to_ffi_type(),
            bid_rate: self.bid_rate,
            offer_rate: self.offer_rate,
            other_currency: self.other_currency.to_ffi_type(),
        }
    }
}

/// Collection of exchange rate entries.
#[repr(C)]
pub struct CExchangeRates {
    /// Pointer to an array of exchange rate items.
    pub exchanges: *const CExchangeRate,
    /// Number of elements in the `exchanges` array.
    pub num_exchanges: usize,
}
pub(crate) struct CExchangeRatesOwned {
    exchanges: CVec<CExchangeRateOwned>,
}
impl From<ExchangeRates> for CExchangeRatesOwned {
    fn from(v: ExchangeRates) -> Self {
        Self {
            exchanges: v.exchanges.into(),
        }
    }
}
impl ToFFI for CExchangeRatesOwned {
    type FFIType = CExchangeRates;
    fn to_ffi_type(&self) -> Self::FFIType {
        CExchangeRates {
            exchanges: self.exchanges.to_ffi_type(),
            num_exchanges: self.exchanges.len(),
        }
    }
}

// ── ProfitAnalysis ────────────────────────────────────────────────

/// P&L summary for one asset category.
#[repr(C)]
pub struct CProfitSummaryInfo {
    /// Asset type.
    pub asset_type: CAssetType,
    /// Security with the maximum profit.
    pub profit_max: *const c_char,
    /// Name of the max-profit security.
    pub profit_max_name: *const c_char,
    /// Security with the maximum loss.
    pub loss_max: *const c_char,
    /// Name of the max-loss security.
    pub loss_max_name: *const c_char,
}

pub(crate) struct CProfitSummaryInfoOwned {
    asset_type: CAssetType,
    profit_max: CString,
    profit_max_name: CString,
    loss_max: CString,
    loss_max_name: CString,
}

impl From<ProfitSummaryInfo> for CProfitSummaryInfoOwned {
    fn from(v: ProfitSummaryInfo) -> Self {
        Self {
            asset_type: v.asset_type.into(),
            profit_max: v.profit_max.into(),
            profit_max_name: v.profit_max_name.into(),
            loss_max: v.loss_max.into(),
            loss_max_name: v.loss_max_name.into(),
        }
    }
}

impl ToFFI for CProfitSummaryInfoOwned {
    type FFIType = CProfitSummaryInfo;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitSummaryInfo {
            asset_type: self.asset_type,
            profit_max: self.profit_max.to_ffi_type(),
            profit_max_name: self.profit_max_name.to_ffi_type(),
            loss_max: self.loss_max.to_ffi_type(),
            loss_max_name: self.loss_max_name.to_ffi_type(),
        }
    }
}

/// P&L breakdown by asset type.
#[repr(C)]
pub struct CProfitSummaryBreakdown {
    /// Stock P&L.
    pub stock: *const c_char,
    /// Fund P&L.
    pub fund: *const c_char,
    /// Crypto P&L.
    pub crypto: *const c_char,
    /// Money market fund P&L.
    pub mmf: *const c_char,
    /// Other P&L.
    pub other: *const c_char,
    /// Cumulative transaction amount.
    pub cumulative_transaction_amount: *const c_char,
    /// Total number of orders.
    pub trade_order_num: *const c_char,
    /// Total number of traded securities.
    pub trade_stock_num: *const c_char,
    /// IPO P&L.
    pub ipo: *const c_char,
    /// IPO hits.
    pub ipo_hit: i32,
    /// IPO subscriptions.
    pub ipo_subscription: i32,
    /// Pointer to array of per-category summary info.
    pub summary_info: *const CProfitSummaryInfo,
    /// Number of items in `summary_info`.
    pub num_summary_info: usize,
}

pub(crate) struct CProfitSummaryBreakdownOwned {
    stock: CString,
    fund: CString,
    crypto: CString,
    mmf: CString,
    other: CString,
    cumulative_transaction_amount: CString,
    trade_order_num: CString,
    trade_stock_num: CString,
    ipo: CString,
    ipo_hit: i32,
    ipo_subscription: i32,
    summary_info: CVec<CProfitSummaryInfoOwned>,
}

impl From<ProfitSummaryBreakdown> for CProfitSummaryBreakdownOwned {
    fn from(v: ProfitSummaryBreakdown) -> Self {
        Self {
            stock: v.stock.map(|d| d.to_string()).unwrap_or_default().into(),
            fund: v.fund.map(|d| d.to_string()).unwrap_or_default().into(),
            crypto: v.crypto.map(|d| d.to_string()).unwrap_or_default().into(),
            mmf: v.mmf.map(|d| d.to_string()).unwrap_or_default().into(),
            other: v.other.map(|d| d.to_string()).unwrap_or_default().into(),
            cumulative_transaction_amount: v
                .cumulative_transaction_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            trade_order_num: v.trade_order_num.into(),
            trade_stock_num: v.trade_stock_num.into(),
            ipo: v.ipo.map(|d| d.to_string()).unwrap_or_default().into(),
            ipo_hit: v.ipo_hit,
            ipo_subscription: v.ipo_subscription,
            summary_info: v.summary_info.into(),
        }
    }
}

impl ToFFI for CProfitSummaryBreakdownOwned {
    type FFIType = CProfitSummaryBreakdown;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitSummaryBreakdown {
            stock: self.stock.to_ffi_type(),
            fund: self.fund.to_ffi_type(),
            crypto: self.crypto.to_ffi_type(),
            mmf: self.mmf.to_ffi_type(),
            other: self.other.to_ffi_type(),
            cumulative_transaction_amount: self.cumulative_transaction_amount.to_ffi_type(),
            trade_order_num: self.trade_order_num.to_ffi_type(),
            trade_stock_num: self.trade_stock_num.to_ffi_type(),
            ipo: self.ipo.to_ffi_type(),
            ipo_hit: self.ipo_hit,
            ipo_subscription: self.ipo_subscription,
            summary_info: self.summary_info.to_ffi_type(),
            num_summary_info: self.summary_info.len(),
        }
    }
}

/// Account-level P&L summary.
#[repr(C)]
pub struct CProfitAnalysisSummary {
    /// Account currency.
    pub currency: *const c_char,
    /// Current total asset value.
    pub current_total_asset: *const c_char,
    /// Query start date string.
    pub start_date: *const c_char,
    /// Query end date string.
    pub end_date: *const c_char,
    /// Start time (unix timestamp string).
    pub start_time: *const c_char,
    /// End time (unix timestamp string).
    pub end_time: *const c_char,
    /// Ending asset value.
    pub ending_asset_value: *const c_char,
    /// Initial asset value.
    pub initial_asset_value: *const c_char,
    /// Total invested amount.
    pub invest_amount: *const c_char,
    /// Whether any trades occurred.
    pub is_traded: bool,
    /// Total profit/loss.
    pub sum_profit: *const c_char,
    /// Total profit/loss rate.
    pub sum_profit_rate: *const c_char,
    /// Per-asset-type breakdown (inline).
    pub profits: CProfitSummaryBreakdown,
}

pub(crate) struct CProfitAnalysisSummaryOwned {
    currency: CString,
    current_total_asset: CString,
    start_date: CString,
    end_date: CString,
    start_time: CString,
    end_time: CString,
    ending_asset_value: CString,
    initial_asset_value: CString,
    invest_amount: CString,
    is_traded: bool,
    sum_profit: CString,
    sum_profit_rate: CString,
    profits: CProfitSummaryBreakdownOwned,
}

impl From<ProfitAnalysisSummary> for CProfitAnalysisSummaryOwned {
    fn from(v: ProfitAnalysisSummary) -> Self {
        Self {
            currency: v.currency.into(),
            current_total_asset: v
                .current_total_asset
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            start_date: v.start_date.into(),
            end_date: v.end_date.into(),
            start_time: v.start_time.into(),
            end_time: v.end_time.into(),
            ending_asset_value: v
                .ending_asset_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            initial_asset_value: v
                .initial_asset_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            invest_amount: v
                .invest_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            is_traded: v.is_traded,
            sum_profit: v
                .sum_profit
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            sum_profit_rate: v
                .sum_profit_rate
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            profits: v.profits.into(),
        }
    }
}

impl ToFFI for CProfitAnalysisSummaryOwned {
    type FFIType = CProfitAnalysisSummary;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisSummary {
            currency: self.currency.to_ffi_type(),
            current_total_asset: self.current_total_asset.to_ffi_type(),
            start_date: self.start_date.to_ffi_type(),
            end_date: self.end_date.to_ffi_type(),
            start_time: self.start_time.to_ffi_type(),
            end_time: self.end_time.to_ffi_type(),
            ending_asset_value: self.ending_asset_value.to_ffi_type(),
            initial_asset_value: self.initial_asset_value.to_ffi_type(),
            invest_amount: self.invest_amount.to_ffi_type(),
            is_traded: self.is_traded,
            sum_profit: self.sum_profit.to_ffi_type(),
            sum_profit_rate: self.sum_profit_rate.to_ffi_type(),
            profits: self.profits.to_ffi_type(),
        }
    }
}

/// P&L for one security.
#[repr(C)]
pub struct CProfitAnalysisItem {
    /// Security name.
    pub name: *const c_char,
    /// Market.
    pub market: *const c_char,
    /// Whether still holding.
    pub is_holding: bool,
    /// Profit/loss amount.
    pub profit: *const c_char,
    /// Profit/loss rate.
    pub profit_rate: *const c_char,
    /// Number of completed trades.
    pub clearance_times: i64,
    /// Asset type.
    pub item_type: CAssetType,
    /// Currency.
    pub currency: *const c_char,
    /// Security symbol.
    pub symbol: *const c_char,
    /// Holding period display string.
    pub holding_period: *const c_char,
    /// Ticker code.
    pub security_code: *const c_char,
    /// ISIN (for funds).
    pub isin: *const c_char,
    /// Underlying stock P&L.
    pub underlying_profit: *const c_char,
    /// Derivatives P&L.
    pub derivatives_profit: *const c_char,
    /// P&L in order currency.
    pub order_profit: *const c_char,
}

pub(crate) struct CProfitAnalysisItemOwned {
    name: CString,
    market: CString,
    is_holding: bool,
    profit: CString,
    profit_rate: CString,
    clearance_times: i64,
    item_type: CAssetType,
    currency: CString,
    symbol: CString,
    holding_period: CString,
    security_code: CString,
    isin: CString,
    underlying_profit: CString,
    derivatives_profit: CString,
    order_profit: CString,
}

impl From<ProfitAnalysisItem> for CProfitAnalysisItemOwned {
    fn from(v: ProfitAnalysisItem) -> Self {
        Self {
            name: v.name.into(),
            market: v.market.into(),
            is_holding: v.is_holding,
            profit: v.profit.map(|d| d.to_string()).unwrap_or_default().into(),
            profit_rate: v
                .profit_rate
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            clearance_times: v.clearance_times,
            item_type: v.item_type.into(),
            currency: v.currency.into(),
            symbol: v.symbol.into(),
            holding_period: v.holding_period.into(),
            security_code: v.security_code.into(),
            isin: v.isin.into(),
            underlying_profit: v
                .underlying_profit
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            derivatives_profit: v
                .derivatives_profit
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            order_profit: v
                .order_profit
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CProfitAnalysisItemOwned {
    type FFIType = CProfitAnalysisItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisItem {
            name: self.name.to_ffi_type(),
            market: self.market.to_ffi_type(),
            is_holding: self.is_holding,
            profit: self.profit.to_ffi_type(),
            profit_rate: self.profit_rate.to_ffi_type(),
            clearance_times: self.clearance_times,
            item_type: self.item_type,
            currency: self.currency.to_ffi_type(),
            symbol: self.symbol.to_ffi_type(),
            holding_period: self.holding_period.to_ffi_type(),
            security_code: self.security_code.to_ffi_type(),
            isin: self.isin.to_ffi_type(),
            underlying_profit: self.underlying_profit.to_ffi_type(),
            derivatives_profit: self.derivatives_profit.to_ffi_type(),
            order_profit: self.order_profit.to_ffi_type(),
        }
    }
}

/// Per-security P&L breakdown.
#[repr(C)]
pub struct CProfitAnalysisSublist {
    /// Start time (unix timestamp string).
    pub start: *const c_char,
    /// End time (unix timestamp string).
    pub end: *const c_char,
    /// Start date string.
    pub start_date: *const c_char,
    /// End date string.
    pub end_date: *const c_char,
    /// Last updated time (unix timestamp string).
    pub updated_at: *const c_char,
    /// Last updated date string.
    pub updated_date: *const c_char,
    /// Pointer to array of per-security items.
    pub items: *const CProfitAnalysisItem,
    /// Number of items.
    pub num_items: usize,
}

pub(crate) struct CProfitAnalysisSublistOwned {
    start: CString,
    end: CString,
    start_date: CString,
    end_date: CString,
    updated_at: CString,
    updated_date: CString,
    items: CVec<CProfitAnalysisItemOwned>,
}

impl From<ProfitAnalysisSublist> for CProfitAnalysisSublistOwned {
    fn from(v: ProfitAnalysisSublist) -> Self {
        Self {
            start: v.start.into(),
            end: v.end.into(),
            start_date: v.start_date.into(),
            end_date: v.end_date.into(),
            updated_at: v.updated_at.into(),
            updated_date: v.updated_date.into(),
            items: v.items.into(),
        }
    }
}

impl ToFFI for CProfitAnalysisSublistOwned {
    type FFIType = CProfitAnalysisSublist;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisSublist {
            start: self.start.to_ffi_type(),
            end: self.end.to_ffi_type(),
            start_date: self.start_date.to_ffi_type(),
            end_date: self.end_date.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
            updated_date: self.updated_date.to_ffi_type(),
            items: self.items.to_ffi_type(),
            num_items: self.items.len(),
        }
    }
}

/// Combined portfolio P&L analysis response.
#[repr(C)]
pub struct CProfitAnalysis {
    /// Account-level summary (inline).
    pub summary: CProfitAnalysisSummary,
    /// Per-security breakdown (inline).
    pub sublist: CProfitAnalysisSublist,
}

pub(crate) struct CProfitAnalysisOwned {
    summary: CProfitAnalysisSummaryOwned,
    sublist: CProfitAnalysisSublistOwned,
}

impl From<ProfitAnalysis> for CProfitAnalysisOwned {
    fn from(v: ProfitAnalysis) -> Self {
        Self {
            summary: v.summary.into(),
            sublist: v.sublist.into(),
        }
    }
}

impl ToFFI for CProfitAnalysisOwned {
    type FFIType = CProfitAnalysis;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysis {
            summary: self.summary.to_ffi_type(),
            sublist: self.sublist.to_ffi_type(),
        }
    }
}

// ── ProfitAnalysisByMarket ────────────────────────────────────────

/// One security entry in a by-market P&L response.
#[repr(C)]
pub struct CProfitAnalysisByMarketItem {
    /// Security symbol (ticker code).
    pub code: *const c_char,
    /// Security name.
    pub name: *const c_char,
    /// Market, e.g. "HK", "US".
    pub market: *const c_char,
    /// Profit/loss amount.
    pub profit: *const c_char,
}

pub(crate) struct CProfitAnalysisByMarketItemOwned {
    code: CString,
    name: CString,
    market: CString,
    profit: CString,
}

impl From<ProfitAnalysisByMarketItem> for CProfitAnalysisByMarketItemOwned {
    fn from(v: ProfitAnalysisByMarketItem) -> Self {
        Self {
            code: v.code.into(),
            name: v.name.into(),
            market: v.market.into(),
            profit: v.profit.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CProfitAnalysisByMarketItemOwned {
    type FFIType = CProfitAnalysisByMarketItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisByMarketItem {
            code: self.code.to_ffi_type(),
            name: self.name.to_ffi_type(),
            market: self.market.to_ffi_type(),
            profit: self.profit.to_ffi_type(),
        }
    }
}

/// P&L grouped by market response.
#[repr(C)]
pub struct CProfitAnalysisByMarket {
    /// Total P&L across all returned items.
    pub profit: *const c_char,
    /// Whether more pages are available.
    pub has_more: bool,
    /// Pointer to array of per-security items.
    pub stock_items: *const CProfitAnalysisByMarketItem,
    /// Number of items in `stock_items`.
    pub num_stock_items: usize,
}

pub(crate) struct CProfitAnalysisByMarketOwned {
    profit: CString,
    has_more: bool,
    stock_items: CVec<CProfitAnalysisByMarketItemOwned>,
}

impl From<ProfitAnalysisByMarket> for CProfitAnalysisByMarketOwned {
    fn from(v: ProfitAnalysisByMarket) -> Self {
        Self {
            profit: v.profit.map(|d| d.to_string()).unwrap_or_default().into(),
            has_more: v.has_more,
            stock_items: v.stock_items.into(),
        }
    }
}

impl ToFFI for CProfitAnalysisByMarketOwned {
    type FFIType = CProfitAnalysisByMarket;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisByMarket {
            profit: self.profit.to_ffi_type(),
            has_more: self.has_more,
            stock_items: self.stock_items.to_ffi_type(),
            num_stock_items: self.stock_items.len(),
        }
    }
}

// ── ProfitAnalysisFlows ───────────────────────────────────────────

/// One profit-analysis flow record.
#[repr(C)]
pub struct CFlowItem {
    /// Execution date string, e.g. "2024-01-15".
    pub executed_date: *const c_char,
    /// Execution timestamp (serialised as JSON string).
    pub executed_timestamp: *const c_char,
    /// Security code / ticker.
    pub code: *const c_char,
    /// Direction of the flow.
    pub direction: CFlowDirection,
    /// Executed quantity.
    pub executed_quantity: *const c_char,
    /// Executed price.
    pub executed_price: *const c_char,
    /// Executed cost.
    pub executed_cost: *const c_char,
    /// Human-readable description.
    pub describe: *const c_char,
}

pub(crate) struct CFlowItemOwned {
    executed_date: CString,
    executed_timestamp: CString,
    code: CString,
    direction: CFlowDirection,
    executed_quantity: CString,
    executed_price: CString,
    executed_cost: CString,
    describe: CString,
}

impl From<FlowItem> for CFlowItemOwned {
    fn from(v: FlowItem) -> Self {
        Self {
            executed_date: v.executed_date.into(),
            executed_timestamp: v.executed_timestamp.to_string().into(),
            code: v.code.into(),
            direction: v.direction.into(),
            executed_quantity: v
                .executed_quantity
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            executed_price: v
                .executed_price
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            executed_cost: v
                .executed_cost
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            describe: v.describe.into(),
        }
    }
}

impl ToFFI for CFlowItemOwned {
    type FFIType = CFlowItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CFlowItem {
            executed_date: self.executed_date.to_ffi_type(),
            executed_timestamp: self.executed_timestamp.to_ffi_type(),
            code: self.code.to_ffi_type(),
            direction: self.direction,
            executed_quantity: self.executed_quantity.to_ffi_type(),
            executed_price: self.executed_price.to_ffi_type(),
            executed_cost: self.executed_cost.to_ffi_type(),
            describe: self.describe.to_ffi_type(),
        }
    }
}

/// Paginated list of profit-analysis flow records.
#[repr(C)]
pub struct CProfitAnalysisFlows {
    /// Pointer to array of flow items.
    pub flows_list: *const CFlowItem,
    /// Number of items in `flows_list`.
    pub num_flows_list: usize,
    /// Whether there are more pages.
    pub has_more: bool,
}

pub(crate) struct CProfitAnalysisFlowsOwned {
    flows_list: CVec<CFlowItemOwned>,
    has_more: bool,
}

impl From<ProfitAnalysisFlows> for CProfitAnalysisFlowsOwned {
    fn from(v: ProfitAnalysisFlows) -> Self {
        Self {
            flows_list: v.flows_list.into(),
            has_more: v.has_more,
        }
    }
}

impl ToFFI for CProfitAnalysisFlowsOwned {
    type FFIType = CProfitAnalysisFlows;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisFlows {
            flows_list: self.flows_list.to_ffi_type(),
            num_flows_list: self.flows_list.len(),
            has_more: self.has_more,
        }
    }
}

// ── ProfitAnalysisDetail ──────────────────────────────────────────

/// One P&L detail line item (credit, debit, or fee).
#[repr(C)]
pub struct CProfitDetailEntry {
    /// Description.
    pub describe: *const c_char,
    /// Amount.
    pub amount: *const c_char,
}

pub(crate) struct CProfitDetailEntryOwned {
    describe: CString,
    amount: CString,
}

impl From<ProfitDetailEntry> for CProfitDetailEntryOwned {
    fn from(v: ProfitDetailEntry) -> Self {
        Self {
            describe: v.describe.into(),
            amount: v.amount.map(|d| d.to_string()).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CProfitDetailEntryOwned {
    type FFIType = CProfitDetailEntry;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitDetailEntry {
            describe: self.describe.to_ffi_type(),
            amount: self.amount.to_ffi_type(),
        }
    }
}

/// Detailed P&L breakdown for one asset class.
#[repr(C)]
pub struct CProfitDetails {
    /// Current holding market value.
    pub holding_value: *const c_char,
    /// Total profit/loss.
    pub profit: *const c_char,
    /// Cumulative credited amount.
    pub cumulative_credited_amount: *const c_char,
    /// Pointer to array of credit detail entries.
    pub credited_details: *const CProfitDetailEntry,
    /// Number of items in `credited_details`.
    pub num_credited_details: usize,
    /// Cumulative debited amount.
    pub cumulative_debited_amount: *const c_char,
    /// Pointer to array of debit detail entries.
    pub debited_details: *const CProfitDetailEntry,
    /// Number of items in `debited_details`.
    pub num_debited_details: usize,
    /// Cumulative fee amount.
    pub cumulative_fee_amount: *const c_char,
    /// Pointer to array of fee detail entries.
    pub fee_details: *const CProfitDetailEntry,
    /// Number of items in `fee_details`.
    pub num_fee_details: usize,
    /// Short position holding value.
    pub short_holding_value: *const c_char,
    /// Long position holding value.
    pub long_holding_value: *const c_char,
    /// Opening position market value at period start.
    pub holding_value_at_beginning: *const c_char,
    /// Closing position market value at period end.
    pub holding_value_at_ending: *const c_char,
}

pub(crate) struct CProfitDetailsOwned {
    holding_value: CString,
    profit: CString,
    cumulative_credited_amount: CString,
    credited_details: CVec<CProfitDetailEntryOwned>,
    cumulative_debited_amount: CString,
    debited_details: CVec<CProfitDetailEntryOwned>,
    cumulative_fee_amount: CString,
    fee_details: CVec<CProfitDetailEntryOwned>,
    short_holding_value: CString,
    long_holding_value: CString,
    holding_value_at_beginning: CString,
    holding_value_at_ending: CString,
}

impl From<ProfitDetails> for CProfitDetailsOwned {
    fn from(v: ProfitDetails) -> Self {
        Self {
            holding_value: v
                .holding_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            profit: v.profit.map(|d| d.to_string()).unwrap_or_default().into(),
            cumulative_credited_amount: v
                .cumulative_credited_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            credited_details: v.credited_details.into(),
            cumulative_debited_amount: v
                .cumulative_debited_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            debited_details: v.debited_details.into(),
            cumulative_fee_amount: v
                .cumulative_fee_amount
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            fee_details: v.fee_details.into(),
            short_holding_value: v
                .short_holding_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            long_holding_value: v
                .long_holding_value
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            holding_value_at_beginning: v
                .holding_value_at_beginning
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            holding_value_at_ending: v
                .holding_value_at_ending
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CProfitDetailsOwned {
    type FFIType = CProfitDetails;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitDetails {
            holding_value: self.holding_value.to_ffi_type(),
            profit: self.profit.to_ffi_type(),
            cumulative_credited_amount: self.cumulative_credited_amount.to_ffi_type(),
            credited_details: self.credited_details.to_ffi_type(),
            num_credited_details: self.credited_details.len(),
            cumulative_debited_amount: self.cumulative_debited_amount.to_ffi_type(),
            debited_details: self.debited_details.to_ffi_type(),
            num_debited_details: self.debited_details.len(),
            cumulative_fee_amount: self.cumulative_fee_amount.to_ffi_type(),
            fee_details: self.fee_details.to_ffi_type(),
            num_fee_details: self.fee_details.len(),
            short_holding_value: self.short_holding_value.to_ffi_type(),
            long_holding_value: self.long_holding_value.to_ffi_type(),
            holding_value_at_beginning: self.holding_value_at_beginning.to_ffi_type(),
            holding_value_at_ending: self.holding_value_at_ending.to_ffi_type(),
        }
    }
}

/// Detailed P&L for one security.
#[repr(C)]
pub struct CProfitAnalysisDetail {
    /// Total profit/loss.
    pub profit: *const c_char,
    /// Underlying stock P&L details (inline).
    pub underlying_details: CProfitDetails,
    /// Derivative P&L details (inline).
    pub derivative_pnl_details: CProfitDetails,
    /// Security name.
    pub name: *const c_char,
    /// Last updated time (unix timestamp string).
    pub updated_at: *const c_char,
    /// Last updated date string.
    pub updated_date: *const c_char,
    /// Currency.
    pub currency: *const c_char,
    /// Default detail tab: 0=underlying, 1=derivative.
    pub default_tag: i32,
    /// Query start time (unix timestamp string).
    pub start: *const c_char,
    /// Query end time (unix timestamp string).
    pub end: *const c_char,
    /// Query start date string.
    pub start_date: *const c_char,
    /// Query end date string.
    pub end_date: *const c_char,
}

pub(crate) struct CProfitAnalysisDetailOwned {
    profit: CString,
    underlying_details: CProfitDetailsOwned,
    derivative_pnl_details: CProfitDetailsOwned,
    name: CString,
    updated_at: CString,
    updated_date: CString,
    currency: CString,
    default_tag: i32,
    start: CString,
    end: CString,
    start_date: CString,
    end_date: CString,
}

impl From<ProfitAnalysisDetail> for CProfitAnalysisDetailOwned {
    fn from(v: ProfitAnalysisDetail) -> Self {
        Self {
            profit: v.profit.map(|d| d.to_string()).unwrap_or_default().into(),
            underlying_details: v.underlying_details.into(),
            derivative_pnl_details: v.derivative_pnl_details.into(),
            name: v.name.into(),
            updated_at: v.updated_at.into(),
            updated_date: v.updated_date.into(),
            currency: v.currency.into(),
            default_tag: v.default_tag,
            start: v.start.into(),
            end: v.end.into(),
            start_date: v.start_date.into(),
            end_date: v.end_date.into(),
        }
    }
}

impl ToFFI for CProfitAnalysisDetailOwned {
    type FFIType = CProfitAnalysisDetail;
    fn to_ffi_type(&self) -> Self::FFIType {
        CProfitAnalysisDetail {
            profit: self.profit.to_ffi_type(),
            underlying_details: self.underlying_details.to_ffi_type(),
            derivative_pnl_details: self.derivative_pnl_details.to_ffi_type(),
            name: self.name.to_ffi_type(),
            updated_at: self.updated_at.to_ffi_type(),
            updated_date: self.updated_date.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            default_tag: self.default_tag,
            start: self.start.to_ffi_type(),
            end: self.end.to_ffi_type(),
            start_date: self.start_date.to_ffi_type(),
            end_date: self.end_date.to_ffi_type(),
        }
    }
}
