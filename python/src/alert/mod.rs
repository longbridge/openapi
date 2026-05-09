mod context;
pub(crate) mod types;
use pyo3::prelude::*;
pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<AlertItem>()?;
    parent.add_class::<AlertSymbolGroup>()?;
    parent.add_class::<AlertList>()?;
    parent.add_class::<AlertCondition>()?;
    parent.add_class::<AlertFrequency>()?;
    parent.add_class::<context::AlertContext>()?;
    Ok(())
}
