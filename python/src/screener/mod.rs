mod context;
mod context_async;
pub(crate) mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<ScreenerRecommendStrategiesResponse>()?;
    parent.add_class::<ScreenerUserStrategiesResponse>()?;
    parent.add_class::<ScreenerStrategyResponse>()?;
    parent.add_class::<ScreenerSearchResponse>()?;
    parent.add_class::<ScreenerIndicatorsResponse>()?;
    parent.add_class::<context::ScreenerContext>()?;
    parent.add_class::<context_async::AsyncScreenerContext>()?;
    Ok(())
}
