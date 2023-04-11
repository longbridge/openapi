mod context;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &PyModule) -> PyResult<()> {
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
    parent.add_class::<types::EstimateMaxPurchaseQuantityResponse>()?;

    parent.add_class::<context::TradeContext>()?;
    Ok(())
}
