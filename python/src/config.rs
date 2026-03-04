use pyo3::{prelude::*, types::PyType};

use crate::{
    error::ErrorNewType,
    oauth::OAuth,
    types::{Language, PushCandlestickMode},
};

#[pyclass(name = "Config")]
pub(crate) struct Config(pub(crate) longbridge::Config);

#[pymethods]
impl Config {
    /// Create a new ``Config`` using API Key authentication.
    ///
    /// Optional environment variables are read automatically
    /// (``LONGBRIDGE_HTTP_URL``, ``LONGBRIDGE_LANGUAGE``,
    /// ``LONGBRIDGE_QUOTE_WS_URL``, ``LONGBRIDGE_TRADE_WS_URL``,
    /// ``LONGBRIDGE_ENABLE_OVERNIGHT``, ``LONGBRIDGE_PUSH_CANDLESTICK_MODE``,
    /// ``LONGBRIDGE_PRINT_QUOTE_PACKAGES``, ``LONGBRIDGE_LOG_PATH``).
    /// Any explicit parameter passed to this method overrides the
    /// corresponding environment variable.
    ///
    /// Args:
    ///     app_key: App Key
    ///     app_secret: App Secret
    ///     access_token: Access Token
    ///     http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL`` from
    ///         env if omitted)
    ///     quote_ws_url: Websocket url for quote API override (reads
    ///         ``LONGBRIDGE_QUOTE_WS_URL`` from env if omitted)
    ///     trade_ws_url: Websocket url for trade API override (reads
    ///         ``LONGBRIDGE_TRADE_WS_URL`` from env if omitted)
    ///     language: Language identifier override (reads
    /// ``LONGBRIDGE_LANGUAGE``         from env if omitted; default:
    /// ``Language.EN``)     enable_overnight: Enable overnight quote
    /// (default: ``False``)     push_candlestick_mode: Push candlestick
    /// mode     enable_print_quote_packages: Print opened quote packages on
    /// connect         (default: ``True``)
    ///     log_path: Path for log files (default: no logs)
    #[staticmethod]
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
    fn from_apikey(
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
        let mut config = longbridge::Config::from_apikey(app_key, app_secret, access_token);

        if let Some(http_url) = http_url {
            config.set_http_url(http_url);
        }
        if let Some(quote_ws_url) = quote_ws_url {
            config.set_quote_ws_url(quote_ws_url);
        }
        if let Some(trade_ws_url) = trade_ws_url {
            config.set_trade_ws_url(trade_ws_url);
        }
        if let Some(language) = language {
            config.set_language(language.into());
        }
        if enable_overnight {
            config.set_enable_overnight();
        }
        if !enable_print_quote_packages {
            config.set_dont_print_quote_packages();
        }
        config.set_push_candlestick_mode(push_candlestick_mode.into());
        if let Some(log_path) = log_path {
            config.set_log_path(log_path);
        }

        Self(config)
    }

    /// Create a new ``Config`` from environment variables (API Key
    /// authentication).
    ///
    /// It first loads the environment variables from the ``.env`` file in the
    /// current directory.
    ///
    /// Variables:
    ///     - ``LONGBRIDGE_APP_KEY`` - App key
    ///     - ``LONGBRIDGE_APP_SECRET`` - App secret
    ///     - ``LONGBRIDGE_ACCESS_TOKEN`` - Access token
    ///     - ``LONGBRIDGE_LANGUAGE`` - Language identifier, ``zh-CN``,
    ///       ``zh-HK`` or ``en`` (Default: ``en``)
    ///     - ``LONGBRIDGE_HTTP_URL`` - HTTP endpoint url
    ///     - ``LONGBRIDGE_QUOTE_WS_URL`` - Quote websocket endpoint url
    ///     - ``LONGBRIDGE_TRADE_WS_URL`` - Trade websocket endpoint url
    ///     - ``LONGBRIDGE_ENABLE_OVERNIGHT`` - ``true`` or ``false`` (Default:
    ///       ``false``)
    ///     - ``LONGBRIDGE_PUSH_CANDLESTICK_MODE`` - ``realtime`` or
    ///       ``confirmed`` (Default: ``realtime``)
    ///     - ``LONGBRIDGE_PRINT_QUOTE_PACKAGES`` - ``true`` or ``false``
    ///       (Default: ``true``)
    ///     - ``LONGBRIDGE_LOG_PATH`` - Log file directory (Default: no logs)
    #[classmethod]
    fn from_apikey_env(_cls: Bound<PyType>) -> PyResult<Self> {
        Ok(Self(
            longbridge::Config::from_apikey_env().map_err(ErrorNewType)?,
        ))
    }

    /// Create a new ``Config`` for OAuth 2.0 authentication.
    ///
    /// OAuth 2.0 is the recommended authentication method — no app_secret or
    /// HMAC signatures required.
    ///
    /// Optional environment variables are read automatically
    /// (``LONGBRIDGE_HTTP_URL``, ``LONGBRIDGE_LANGUAGE``,
    /// ``LONGBRIDGE_QUOTE_WS_URL``, ``LONGBRIDGE_TRADE_WS_URL``,
    /// ``LONGBRIDGE_ENABLE_OVERNIGHT``, ``LONGBRIDGE_PUSH_CANDLESTICK_MODE``,
    /// ``LONGBRIDGE_PRINT_QUOTE_PACKAGES``, ``LONGBRIDGE_LOG_PATH``).
    /// Any explicit parameter passed to this method overrides the
    /// corresponding environment variable.
    ///
    /// Args:
    ///     oauth: :class:`OAuth` handle from :meth:`OAuthBuilder.build` or
    ///         :meth:`AsyncOAuthBuilder.build`
    ///     http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL`` from
    ///         env if omitted)
    ///     quote_ws_url: Quote WS url override (reads
    ///         ``LONGBRIDGE_QUOTE_WS_URL`` from env if omitted)
    ///     trade_ws_url: Trade WS url override (reads
    ///         ``LONGBRIDGE_TRADE_WS_URL`` from env if omitted)
    ///     language: Language identifier override (reads
    /// ``LONGBRIDGE_LANGUAGE``         from env if omitted)
    ///     enable_overnight: Enable overnight quote (optional)
    ///     push_candlestick_mode: Push candlestick mode (optional)
    ///     enable_print_quote_packages: Print opened quote packages on connect
    ///         (optional)
    ///     log_path: Path for log files (optional)
    ///
    /// Returns:
    ///     Config object
    #[classmethod]
    #[pyo3(signature = (
        oauth,
        http_url = None,
        quote_ws_url = None,
        trade_ws_url = None,
        language = None,
        enable_overnight = None,
        push_candlestick_mode = None,
        enable_print_quote_packages = None,
        log_path = None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn from_oauth(
        _cls: Bound<PyType>,
        oauth: &OAuth,
        http_url: Option<String>,
        quote_ws_url: Option<String>,
        trade_ws_url: Option<String>,
        language: Option<Language>,
        enable_overnight: Option<bool>,
        push_candlestick_mode: Option<PushCandlestickMode>,
        enable_print_quote_packages: Option<bool>,
        log_path: Option<String>,
    ) -> Self {
        let mut config = longbridge::Config::from_oauth(oauth.0.clone());

        if let Some(http_url) = http_url {
            config.set_http_url(http_url);
        }
        if let Some(quote_ws_url) = quote_ws_url {
            config.set_quote_ws_url(quote_ws_url);
        }
        if let Some(trade_ws_url) = trade_ws_url {
            config.set_trade_ws_url(trade_ws_url);
        }
        if let Some(language) = language {
            config.set_language(language.into());
        }
        if let Some(true) = enable_overnight {
            config.set_enable_overnight();
        }
        if let Some(mode) = push_candlestick_mode {
            config.set_push_candlestick_mode(mode.into());
        }
        if let Some(false) = enable_print_quote_packages {
            config.set_dont_print_quote_packages();
        }
        if let Some(log_path) = log_path {
            config.set_log_path(log_path);
        }

        Self(config)
    }
}
