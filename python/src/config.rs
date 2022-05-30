use pyo3::{prelude::*, types::PyType};

use crate::error::ErrorNewType;

#[pyclass(name = "Config")]
pub(crate) struct Config(pub(crate) longbridge::Config);

#[pymethods]
impl Config {
    #[new]
    #[args(
        http_url = "\"https://openapi.longbridgeapp.com\"",
        quote_ws_url = "\"wss://openapi-quote.longbridgeapp.com\"",
        trade_ws_url = "\"wss://openapi-trade.longbridgeapp.com\""
    )]
    fn py_new(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: &str,
        quote_ws_url: &str,
        trade_ws_url: &str,
    ) -> Self {
        Self(
            longbridge::Config::new(app_key, app_secret, access_token)
                .http_url(http_url)
                .quote_ws_url(quote_ws_url)
                .trade_ws_url(trade_ws_url),
        )
    }

    #[classmethod]
    fn from_env(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(longbridge::Config::from_env().map_err(ErrorNewType)?))
    }
}
