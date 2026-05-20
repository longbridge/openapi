mod context;
pub(crate) mod types;
use pyo3::prelude::*;
pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<DcaPlan>()?;
    parent.add_class::<DcaList>()?;
    parent.add_class::<DcaStats>()?;
    parent.add_class::<DcaSupportInfo>()?;
    parent.add_class::<DcaSupportList>()?;
    parent.add_class::<DcaHistoryRecord>()?;
    parent.add_class::<DcaHistoryResponse>()?;
    parent.add_class::<DCAFrequency>()?;
    parent.add_class::<DCAStatus>()?;
    parent.add_class::<context::DCAContext>()?;
    Ok(())
}
