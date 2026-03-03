use pyo3::{prelude::*, types::PyType};

use crate::{
    error::ErrorNewType,
    oauth::OAuthToken,
    time::PyOffsetDateTimeWrapper,
    types::{Language, PushCandlestickMode},
};

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
        enable_overnight = false,
        push_candlestick_mode = PushCandlestickMode::Realtime,
        enable_print_quote_packages = true,
        log_path = None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn py_new(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: Option<String>,
        quote_ws_url: Option<String>,
        trade_ws_url: Option<String>,
        language: Option<Language>,
        enable_overnight: bool,
        push_candlestick_mode: PushCandlestickMode,
        enable_print_quote_packages: bool,
        log_path: Option<String>,
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
        if enable_overnight {
            config = config.enable_overnight();
        }
        if !enable_print_quote_packages {
            config = config.dont_print_quote_packages();
        }

        config = config.push_candlestick_mode(push_candlestick_mode.into());

        if let Some(log_path) = log_path {
            config = config.log_path(log_path);
        }

        Self(config)
    }

    #[classmethod]
    fn from_env(_cls: Bound<PyType>) -> PyResult<Self> {
        Ok(Self(longport::Config::from_env().map_err(ErrorNewType)?))
    }

    /// Create a new `Config` for OAuth 2.0 authentication
    ///
    /// OAuth 2.0 is the recommended authentication method that uses Bearer
    /// tokens and does not require app_secret or HMAC signatures.
    ///
    /// # Example
    ///
    /// ```python
    /// from longport.openapi import Config, OAuth
    ///
    /// # Obtain a token via OAuth flow, then:
    /// # config = Config.from_oauth(token)
    /// ```
    #[classmethod]
    fn from_oauth(_cls: Bound<PyType>, token: &OAuthToken) -> Self {
        Self(longport::Config::from_oauth(&token.0))
    }

    /// Gets a new `access_token`.
    ///
    /// `expired_at` - The expiration time of the access token, defaults to `90`
    /// days.
    #[pyo3(signature = (expired_at = None))]
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
