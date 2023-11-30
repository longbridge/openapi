use pyo3::{prelude::*, types::PyType};

use crate::{error::ErrorNewType, time::PyOffsetDateTimeWrapper, types::Language};

#[pyclass(name = "Config")]
pub(crate) struct Config(pub(crate) longport::Config);

#[pymethods]
impl Config {
    #[new]
    #[pyo3(signature = (
        app_key,
        app_secret,
        access_token,
        http_url = None,
        quote_ws_url = None,
        trade_ws_url = None,
        language = None,
    ))]
    fn py_new(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: Option<String>,
        quote_ws_url: Option<String>,
        trade_ws_url: Option<String>,
        language: Option<Language>,
    ) -> Self {
        let mut config = longport::Config::new(app_key, app_secret, access_token);
        if let Some(http_url) = http_url {
            config = config.http_url(http_url);
        }
        if let Some(quote_ws_url) = quote_ws_url {
            config = config.quote_ws_url(quote_ws_url);
        }
        if let Some(trade_ws_url) = trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }
        if let Some(language) = language {
            config = config.language(language.into());
        }
        Self(config)
    }

    #[classmethod]
    fn from_env(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(longport::Config::from_env().map_err(ErrorNewType)?))
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
