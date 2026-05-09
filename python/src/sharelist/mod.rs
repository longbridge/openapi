mod context;
pub(crate) mod types;
use pyo3::prelude::*;
pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<SharelistStock>()?;
    parent.add_class::<SharelistScopes>()?;
    parent.add_class::<SharelistInfo>()?;
    parent.add_class::<SharelistList>()?;
    parent.add_class::<SharelistDetail>()?;
    parent.add_class::<context::SharelistContext>()?;
    Ok(())
}
