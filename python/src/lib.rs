mod alert;
mod asset;
mod async_callback;
mod calendar;
mod config;
mod content;
mod dca;
mod decimal;
mod error;
mod fundamental;
mod http_client;
mod market;
mod oauth;
mod portfolio;
mod quote;
mod screener;
mod sharelist;
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
    asset::register_types(&openapi)?;
    alert::register_types(&openapi)?;
    dca::register_types(&openapi)?;
    sharelist::register_types(&openapi)?;
    calendar::register_types(&openapi)?;
    fundamental::register_types(&openapi)?;
    market::register_types(&openapi)?;
    portfolio::register_types(&openapi)?;
    quote::register_types(&openapi)?;
    screener::register_types(&openapi)?;
    trade::register_types(&openapi)?;
    content::register_types(&openapi)?;

    m.add_submodule(&openapi)?;
    Ok(())
}
