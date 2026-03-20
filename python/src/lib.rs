mod async_callback;
mod config;
mod content;
mod decimal;
mod error;
mod http_client;
mod oauth;
mod quote;
mod time;
mod trade;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn longbridge(py: Python<'_>, m: Bound<PyModule>) -> PyResult<()> {
    let openapi = PyModule::new(py, "openapi")?;

    openapi.add_class::<config::Config>()?;
    openapi.add_class::<oauth::OAuth>()?;
    openapi.add_class::<oauth::OAuthBuilder>()?;
    openapi.add_class::<types::Language>()?;
    openapi.add_class::<types::Market>()?;
    openapi.add_class::<types::PushCandlestickMode>()?;
    openapi.add_class::<http_client::HttpClient>()?;
    openapi.add_class::<error::ErrorKind>()?;
    quote::register_types(&openapi)?;
    trade::register_types(&openapi)?;
    content::register_types(&openapi)?;

    m.add_submodule(&openapi)?;
    Ok(())
}
