mod cancel_order;
mod estimate_max_purchase_quantity;
mod get_all_executions;
mod get_cash_flow;
mod get_fund_positions;
mod get_history_executions;
mod get_history_orders;
mod get_order_detail;
mod get_stock_positions;
mod get_today_executions;
mod get_today_orders;
mod replace_order;
mod submit_multileg;
mod submit_order;

pub use cancel_order::CancelOrderOptions;
pub use estimate_max_purchase_quantity::EstimateMaxPurchaseQuantityOptions;
pub use get_all_executions::GetAllExecutionsOptions;
pub use get_cash_flow::GetCashFlowOptions;
pub use get_fund_positions::GetFundPositionsOptions;
pub use get_history_executions::GetHistoryExecutionsOptions;
pub use get_history_orders::GetHistoryOrdersOptions;
pub use get_order_detail::GetOrderDetailOptions;
pub use get_stock_positions::GetStockPositionsOptions;
pub use get_today_executions::GetTodayExecutionsOptions;
pub use get_today_orders::GetTodayOrdersOptions;
pub use replace_order::{ReplaceAttachedParams, ReplaceOrderOptions};
pub use submit_multileg::{SubmitMultiLegOrderLeg, SubmitMultiLegOrderOptions};
pub use submit_order::{SubmitAttachedParams, SubmitOrderOptions};

/// Wire value for a time-condition submit time.
///
/// `0` and `1` are sentinels on the wire (`0` = no time condition, and on
/// replace `1` = clear it), so a time that maps to either would silently be
/// read as one of them. Any time that early is in the past, which the server
/// rejects, so clamp it to `2` to get that rejection instead.
pub(crate) fn delayed_until_timestamp(delayed_until: time::OffsetDateTime) -> i64 {
    delayed_until.unix_timestamp().max(2)
}
