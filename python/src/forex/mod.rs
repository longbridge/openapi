mod context;
pub(crate) mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<ForexOrderStatus>()?;
    parent.add_class::<ForexQuote>()?;
    parent.add_class::<ForexOrderDetail>()?;
    parent.add_class::<context::ForexContext>()?;
    Ok(())
}
