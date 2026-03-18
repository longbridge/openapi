mod context;
mod context_async;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    parent.add_class::<types::TopicType>()?;
    parent.add_class::<types::Execution>()?;
    parent.add_class::<types::OrderStatus>()?;
    parent.add_class::<types::OrderSide>()?;
    parent.add_class::<types::OrderType>()?;
    parent.add_class::<types::OrderTag>()?;
    parent.add_class::<types::TimeInForceType>()?;
    parent.add_class::<types::TriggerStatus>()?;
    parent.add_class::<types::OutsideRTH>()?;
    parent.add_class::<types::Order>()?;
    parent.add_class::<types::PushOrderChanged>()?;
    parent.add_class::<types::MarginRatio>()?;
    parent.add_class::<types::CommissionFreeStatus>()?;
    parent.add_class::<types::DeductionStatus>()?;
    parent.add_class::<types::ChargeCategoryCode>()?;
    parent.add_class::<types::OrderHistoryDetail>()?;
    parent.add_class::<types::OrderChargeFee>()?;
    parent.add_class::<types::OrderChargeItem>()?;
    parent.add_class::<types::OrderChargeDetail>()?;
    parent.add_class::<types::OrderDetail>()?;
    parent.add_class::<types::BalanceType>()?;
    parent.add_class::<types::EstimateMaxPurchaseQuantityResponse>()?;
    parent.add_class::<types::FrozenTransactionFee>()?;
    parent.add_class::<types::SubmitOrderResponse>()?;
    parent.add_class::<types::CashInfo>()?;
    parent.add_class::<types::AccountBalance>()?;
    parent.add_class::<types::CashFlowDirection>()?;
    parent.add_class::<types::CashFlow>()?;
    parent.add_class::<types::FundPositionsResponse>()?;
    parent.add_class::<types::FundPositionChannel>()?;
    parent.add_class::<types::FundPosition>()?;
    parent.add_class::<types::StockPositionsResponse>()?;
    parent.add_class::<types::StockPositionChannel>()?;
    parent.add_class::<types::StockPosition>()?;

    parent.add_class::<context::TradeContext>()?;
    parent.add_class::<context_async::AsyncTradeContext>()?;
    Ok(())
}
