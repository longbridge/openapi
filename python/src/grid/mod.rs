mod context;
pub(crate) mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<TriggerPriceType>()?;
    parent.add_class::<GridTimeInForce>()?;
    parent.add_class::<GridLimitEvent>()?;
    parent.add_class::<GridTradeRule>()?;
    parent.add_class::<SubmitGridOrderResponse>()?;
    parent.add_class::<GridOrder>()?;
    parent.add_class::<GridOrderSubOrder>()?;
    parent.add_class::<GridOrderHistory>()?;
    parent.add_class::<GridOrderDetail>()?;
    parent.add_class::<TriggerOrder>()?;
    parent.add_class::<GridBidSize>()?;
    parent.add_class::<GridChannelInfo>()?;
    parent.add_class::<GridOrderInfo>()?;
    parent.add_class::<GridOrdersResponse>()?;
    parent.add_class::<GridTriggerHistoryResponse>()?;
    parent.add_class::<context::GridContext>()?;
    Ok(())
}
