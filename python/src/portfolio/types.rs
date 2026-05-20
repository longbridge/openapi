use longbridge::portfolio::types as lb;
use longbridge_python_macros::PyEnum;
use pyo3::prelude::*;

/// Trade flow direction
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::portfolio::types::FlowDirection")]
pub(crate) enum FlowDirection {
    /// Unknown
    Unknown,
    /// Buy
    Buy,
    /// Sell
    Sell,
}

/// Asset class category
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::portfolio::types::AssetType")]
pub(crate) enum AssetType {
    /// Unknown
    Unknown,
    /// Stock
    Stock,
    /// Fund
    Fund,
    /// Crypto
    Crypto,
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ExchangeRate {
    pub average_rate: f64,
    pub base_currency: String,
    pub bid_rate: f64,
    pub offer_rate: f64,
    pub other_currency: String,
}
impl From<lb::ExchangeRate> for ExchangeRate {
    fn from(v: lb::ExchangeRate) -> Self {
        Self {
            average_rate: v.average_rate,
            base_currency: v.base_currency,
            bid_rate: v.bid_rate,
            offer_rate: v.offer_rate,
            other_currency: v.other_currency,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ExchangeRates {
    pub exchanges: Vec<ExchangeRate>,
}
impl From<lb::ExchangeRates> for ExchangeRates {
    fn from(v: lb::ExchangeRates) -> Self {
        Self {
            exchanges: v.exchanges.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitSummaryInfo {
    pub asset_type: AssetType,
    pub profit_max: String,
    pub profit_max_name: String,
    pub loss_max: String,
    pub loss_max_name: String,
}
impl From<lb::ProfitSummaryInfo> for ProfitSummaryInfo {
    fn from(v: lb::ProfitSummaryInfo) -> Self {
        Self {
            asset_type: v.asset_type.into(),
            profit_max: v.profit_max,
            profit_max_name: v.profit_max_name,
            loss_max: v.loss_max,
            loss_max_name: v.loss_max_name,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitSummaryBreakdown {
    pub stock: Option<String>,
    pub fund: Option<String>,
    pub crypto: Option<String>,
    pub mmf: Option<String>,
    pub other: Option<String>,
    pub cumulative_transaction_amount: Option<String>,
    pub trade_order_num: String,
    pub trade_stock_num: String,
    pub ipo: Option<String>,
    pub ipo_hit: i32,
    pub ipo_subscription: i32,
    pub summary_info: Vec<ProfitSummaryInfo>,
}
impl From<lb::ProfitSummaryBreakdown> for ProfitSummaryBreakdown {
    fn from(v: lb::ProfitSummaryBreakdown) -> Self {
        Self {
            stock: v.stock.map(|d| d.to_string()),
            fund: v.fund.map(|d| d.to_string()),
            crypto: v.crypto.map(|d| d.to_string()),
            mmf: v.mmf.map(|d| d.to_string()),
            other: v.other.map(|d| d.to_string()),
            cumulative_transaction_amount: v.cumulative_transaction_amount.map(|d| d.to_string()),
            trade_order_num: v.trade_order_num,
            trade_stock_num: v.trade_stock_num,
            ipo: v.ipo.map(|d| d.to_string()),
            ipo_hit: v.ipo_hit,
            ipo_subscription: v.ipo_subscription,
            summary_info: v.summary_info.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisSummary {
    pub currency: String,
    pub current_total_asset: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub start_time: String,
    pub end_time: String,
    pub ending_asset_value: Option<String>,
    pub initial_asset_value: Option<String>,
    pub invest_amount: Option<String>,
    pub is_traded: bool,
    pub sum_profit: Option<String>,
    pub sum_profit_rate: Option<String>,
    pub profits: ProfitSummaryBreakdown,
}
impl From<lb::ProfitAnalysisSummary> for ProfitAnalysisSummary {
    fn from(v: lb::ProfitAnalysisSummary) -> Self {
        Self {
            currency: v.currency,
            current_total_asset: v.current_total_asset.map(|d| d.to_string()),
            start_date: v.start_date,
            end_date: v.end_date,
            start_time: v.start_time,
            end_time: v.end_time,
            ending_asset_value: v.ending_asset_value.map(|d| d.to_string()),
            initial_asset_value: v.initial_asset_value.map(|d| d.to_string()),
            invest_amount: v.invest_amount.map(|d| d.to_string()),
            is_traded: v.is_traded,
            sum_profit: v.sum_profit.map(|d| d.to_string()),
            sum_profit_rate: v.sum_profit_rate.map(|d| d.to_string()),
            profits: v.profits.into(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisItem {
    pub name: String,
    pub market: String,
    pub is_holding: bool,
    pub profit: Option<String>,
    pub profit_rate: Option<String>,
    pub clearance_times: i64,
    pub item_type: AssetType,
    pub currency: String,
    pub symbol: String,
    pub holding_period: String,
    pub security_code: String,
    pub isin: String,
    pub underlying_profit: Option<String>,
    pub derivatives_profit: Option<String>,
    pub order_profit: Option<String>,
}
impl From<lb::ProfitAnalysisItem> for ProfitAnalysisItem {
    fn from(v: lb::ProfitAnalysisItem) -> Self {
        Self {
            name: v.name,
            market: v.market,
            is_holding: v.is_holding,
            profit: v.profit.map(|d| d.to_string()),
            profit_rate: v.profit_rate.map(|d| d.to_string()),
            clearance_times: v.clearance_times,
            item_type: v.item_type.into(),
            currency: v.currency,
            symbol: v.symbol,
            holding_period: v.holding_period,
            security_code: v.security_code,
            isin: v.isin,
            underlying_profit: v.underlying_profit.map(|d| d.to_string()),
            derivatives_profit: v.derivatives_profit.map(|d| d.to_string()),
            order_profit: v.order_profit.map(|d| d.to_string()),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisSublist {
    pub start: String,
    pub end: String,
    pub start_date: String,
    pub end_date: String,
    pub updated_at: String,
    pub updated_date: String,
    pub items: Vec<ProfitAnalysisItem>,
}
impl From<lb::ProfitAnalysisSublist> for ProfitAnalysisSublist {
    fn from(v: lb::ProfitAnalysisSublist) -> Self {
        Self {
            start: v.start,
            end: v.end,
            start_date: v.start_date,
            end_date: v.end_date,
            updated_at: v.updated_at,
            updated_date: v.updated_date,
            items: v.items.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysis {
    pub summary: ProfitAnalysisSummary,
    pub sublist: ProfitAnalysisSublist,
}
impl From<lb::ProfitAnalysis> for ProfitAnalysis {
    fn from(v: lb::ProfitAnalysis) -> Self {
        Self {
            summary: v.summary.into(),
            sublist: v.sublist.into(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitDetailEntry {
    pub describe: String,
    pub amount: Option<String>,
}
impl From<lb::ProfitDetailEntry> for ProfitDetailEntry {
    fn from(v: lb::ProfitDetailEntry) -> Self {
        Self {
            describe: v.describe,
            amount: v.amount.map(|d| d.to_string()),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitDetails {
    pub holding_value: Option<String>,
    pub profit: Option<String>,
    pub cumulative_credited_amount: Option<String>,
    pub credited_details: Vec<ProfitDetailEntry>,
    pub cumulative_debited_amount: Option<String>,
    pub debited_details: Vec<ProfitDetailEntry>,
    pub cumulative_fee_amount: Option<String>,
    pub fee_details: Vec<ProfitDetailEntry>,
    pub short_holding_value: Option<String>,
    pub long_holding_value: Option<String>,
    pub holding_value_at_beginning: Option<String>,
    pub holding_value_at_ending: Option<String>,
}
impl From<lb::ProfitDetails> for ProfitDetails {
    fn from(v: lb::ProfitDetails) -> Self {
        Self {
            holding_value: v.holding_value.map(|d| d.to_string()),
            profit: v.profit.map(|d| d.to_string()),
            cumulative_credited_amount: v.cumulative_credited_amount.map(|d| d.to_string()),
            credited_details: v.credited_details.into_iter().map(Into::into).collect(),
            cumulative_debited_amount: v.cumulative_debited_amount.map(|d| d.to_string()),
            debited_details: v.debited_details.into_iter().map(Into::into).collect(),
            cumulative_fee_amount: v.cumulative_fee_amount.map(|d| d.to_string()),
            fee_details: v.fee_details.into_iter().map(Into::into).collect(),
            short_holding_value: v.short_holding_value.map(|d| d.to_string()),
            long_holding_value: v.long_holding_value.map(|d| d.to_string()),
            holding_value_at_beginning: v.holding_value_at_beginning.map(|d| d.to_string()),
            holding_value_at_ending: v.holding_value_at_ending.map(|d| d.to_string()),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisDetail {
    pub profit: Option<String>,
    pub underlying_details: ProfitDetails,
    pub derivative_pnl_details: ProfitDetails,
    pub name: String,
    pub updated_at: String,
    pub updated_date: String,
    pub currency: String,
    pub default_tag: i32,
    pub start: String,
    pub end: String,
    pub start_date: String,
    pub end_date: String,
}
impl From<lb::ProfitAnalysisDetail> for ProfitAnalysisDetail {
    fn from(v: lb::ProfitAnalysisDetail) -> Self {
        Self {
            profit: v.profit.map(|d| d.to_string()),
            underlying_details: v.underlying_details.into(),
            derivative_pnl_details: v.derivative_pnl_details.into(),
            name: v.name,
            updated_at: v.updated_at,
            updated_date: v.updated_date,
            currency: v.currency,
            default_tag: v.default_tag,
            start: v.start,
            end: v.end,
            start_date: v.start_date,
            end_date: v.end_date,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisByMarketItem {
    pub code: String,
    pub name: String,
    pub market: String,
    pub profit: Option<String>,
}
impl From<lb::ProfitAnalysisByMarketItem> for ProfitAnalysisByMarketItem {
    fn from(v: lb::ProfitAnalysisByMarketItem) -> Self {
        Self {
            code: v.code,
            name: v.name,
            market: v.market,
            profit: v.profit.map(|d| d.to_string()),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisByMarket {
    pub profit: Option<String>,
    pub has_more: bool,
    pub stock_items: Vec<ProfitAnalysisByMarketItem>,
}
impl From<lb::ProfitAnalysisByMarket> for ProfitAnalysisByMarket {
    fn from(v: lb::ProfitAnalysisByMarket) -> Self {
        Self {
            profit: v.profit.map(|d| d.to_string()),
            has_more: v.has_more,
            stock_items: v.stock_items.into_iter().map(Into::into).collect(),
        }
    }
}

// ── ProfitAnalysisFlows ───────────────────────────────────────────

/// One profit-analysis flow record
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct FlowItem {
    pub executed_date: String,
    /// Execution timestamp as a string representation
    pub executed_timestamp: String,
    pub code: String,
    pub direction: FlowDirection,
    pub executed_quantity: Option<String>,
    pub executed_price: Option<String>,
    pub executed_cost: Option<String>,
    pub describe: String,
}

impl From<lb::FlowItem> for FlowItem {
    fn from(v: lb::FlowItem) -> Self {
        Self {
            executed_date: v.executed_date,
            executed_timestamp: v.executed_timestamp.to_string(),
            code: v.code,
            direction: v.direction.into(),
            executed_quantity: v.executed_quantity.map(|d| d.to_string()),
            executed_price: v.executed_price.map(|d| d.to_string()),
            executed_cost: v.executed_cost.map(|d| d.to_string()),
            describe: v.describe,
        }
    }
}

/// Profit-analysis flows response
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ProfitAnalysisFlows {
    pub flows_list: Vec<FlowItem>,
    pub has_more: bool,
}

impl From<lb::ProfitAnalysisFlows> for ProfitAnalysisFlows {
    fn from(v: lb::ProfitAnalysisFlows) -> Self {
        Self {
            flows_list: v.flows_list.into_iter().map(Into::into).collect(),
            has_more: v.has_more,
        }
    }
}
