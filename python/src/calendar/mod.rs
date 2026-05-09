mod context;
mod context_async;
pub(crate) mod types;
use pyo3::prelude::*;
pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<CalendarDataKv>()?;
    parent.add_class::<CalendarEventInfo>()?;
    parent.add_class::<CalendarDateGroup>()?;
    parent.add_class::<CalendarEventsResponse>()?;
    parent.add_class::<CalendarCategory>()?;
    parent.add_class::<context::CalendarContext>()?;
    parent.add_class::<context_async::AsyncCalendarContext>()?;
    Ok(())
}
