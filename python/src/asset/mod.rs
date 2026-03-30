mod context;
mod context_async;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    parent.add_class::<types::StatementType>()?;
    parent.add_class::<types::StatementItem>()?;
    parent.add_class::<types::GetStatementListResponse>()?;
    parent.add_class::<types::GetStatementResponse>()?;
    parent.add_class::<context::AssetContext>()?;
    parent.add_class::<context_async::AsyncAssetContext>()?;
    Ok(())
}
