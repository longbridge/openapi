mod context;
mod context_async;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    parent.add_class::<types::TopicItem>()?;
    parent.add_class::<types::NewsItem>()?;
    parent.add_class::<context::ContentContext>()?;
    parent.add_class::<context_async::AsyncContentContext>()?;
    Ok(())
}
