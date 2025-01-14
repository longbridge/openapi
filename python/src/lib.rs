mod config;
mod decimal;
mod error;
mod http_client;
mod quote;
mod time;
mod trade;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn longport(py: Python<'_>, m: Bound<PyModule>) -> PyResult<()> {
    let openapi = PyModule::new(py, "openapi")?;

    openapi.add_class::<config::Config>()?;
    openapi.add_class::<types::Language>()?;
    openapi.add_class::<types::Market>()?;
    openapi.add_class::<types::PushCandlestickMode>()?;
    openapi.add_class::<http_client::HttpClient>()?;
    quote::register_types(&openapi)?;
    trade::register_types(&openapi)?;

    m.add_submodule(&openapi)?;
    Ok(())
}
