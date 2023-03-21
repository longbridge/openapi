use pyo3::{prelude::*, types::PyType};

use crate::{error::ErrorNewType, time::PyOffsetDateTimeWrapper, types::Language};

#[pyclass(name = "Config")]
pub(crate) struct Config(pub(crate) longbridge::Config);

#[pymethods]
impl Config {
    #[new]
    #[pyo3(signature = (
        app_key,
        app_secret,
        access_token,
        http_url = "https://openapi.longbridgeapp.com",
        quote_ws_url = "wss://openapi-quote.longbridgeapp.com/v2",
        trade_ws_url = "wss://openapi-trade.longbridgeapp.com/v2",
        language = Language::EN,
    ))]
    fn py_new(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: &str,
        quote_ws_url: &str,
        trade_ws_url: &str,
        language: Language,
    ) -> Self {
        Self(
            longbridge::Config::new(app_key, app_secret, access_token)
                .http_url(http_url)
                .quote_ws_url(quote_ws_url)
                .trade_ws_url(trade_ws_url)
                .language(language.into()),
        )
    }

    #[classmethod]
    fn from_env(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(longbridge::Config::from_env().map_err(ErrorNewType)?))
    }

    /// Gets a new `access_token`.
    ///
    /// `expired_at` - The expiration time of the access token, defaults to `90`
    /// days.
    pub fn refresh_access_token(
        &self,
        expired_at: Option<PyOffsetDateTimeWrapper>,
    ) -> PyResult<String> {
        Ok(self
            .0
            .refresh_access_token_blocking(expired_at.map(|t| t.0))
            .map_err(ErrorNewType)?)
    }
}
