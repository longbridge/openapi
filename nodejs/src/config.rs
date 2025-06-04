use chrono::{DateTime, Utc};
use napi::Result;

use crate::{
    error::ErrorNewType,
    types::{Language, PushCandlestickMode},
    utils::from_datetime,
};

/// Configuration parameters
#[napi_derive::napi(object)]
pub struct ConfigParams {
    /// App Key
    pub app_key: String,
    /// App Secret
    pub app_secret: String,
    /// Access Token
    pub access_token: String,
    /// HTTP API url (default: "https://openapi.longportapp.com")
    pub http_url: Option<String>,
    /// Websocket url for quote API (default:
    /// "wss://openapi-quote.longportapp.com/v2")
    pub quote_ws_url: Option<String>,
    /// Websocket url for trade API (default:
    /// "wss://openapi-trade.longportapp.com/v2")
    pub trade_ws_url: Option<String>,
    /// Language identifier (default: Language.EN)
    pub language: Option<Language>,
    /// Enable overnight (default: false)
    pub enable_overnight: Option<bool>,
    /// Push candlesticks mode (default: PushCandlestickMode.Realtime)
    pub push_candlestick_mode: Option<PushCandlestickMode>,
    /// Enable printing the opened quote packages when connected to the server
    /// (default: true)
    pub enable_print_quote_packages: bool,
    /// Set the path of the log files (Default: `no logs`)
    pub log_path: Option<String>,
}

/// Configuration for LongPort sdk
#[napi_derive::napi]
pub struct Config(pub(crate) longport::Config);

#[napi_derive::napi]
impl Config {
    /// Create a new `Config`
    #[napi(constructor)]
    pub fn new(params: ConfigParams) -> Self {
        let mut config =
            longport::Config::new(params.app_key, params.app_secret, params.access_token);

        if let Some(http_url) = params.http_url {
            config = config.http_url(http_url);
        }

        if let Some(quote_ws_url) = params.quote_ws_url {
            config = config.quote_ws_url(quote_ws_url);
        }

        if let Some(trade_ws_url) = params.trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }

        if let Some(language) = params.language {
            config = config.language(language.into());
        }

        if let Some(true) = params.enable_overnight {
            config = config.enable_overnight();
        }

        if let Some(mode) = params.push_candlestick_mode {
            config = config.push_candlestick_mode(mode.into());
        }

        if !params.enable_print_quote_packages {
            config = config.dont_print_quote_packages();
        }

        if let Some(log_path) = params.log_path {
            config = config.log_path(log_path);
        }

        Self(config)
    }

    /// Create a new `Config` from the given environment variables
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGPORT_LANGUAGE` - Language identifier, `zh-CN`, `zh-HK` or `en`
    ///   (Default: `en`)
    /// - `LONGPORT_APP_KEY` - App key
    /// - `LONGPORT_APP_SECRET` - App secret
    /// - `LONGPORT_ACCESS_TOKEN` - Access token
    /// - `LONGPORT_HTTP_URL` - HTTP endpoint url
    /// - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url
    /// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url
    /// - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or
    ///   `false` (Default: `false`)
    /// - `LONGPORT_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed` (Default:
    ///   `realtime`)
    /// - `LONGPORT_PRINT_QUOTE_PACKAGES` - Print quote packages when connected,
    ///   `true` or `false` (Default: `true`)
    #[napi(factory)]
    pub fn from_env() -> Result<Self> {
        Ok(Self(longport::Config::from_env().map_err(ErrorNewType)?))
    }

    /// Gets a new `access_token`
    ///
    /// `expired_at` - The expiration time of the access token, defaults to `90`
    /// days.
    #[napi]
    pub async fn refresh_access_token(&self, expired_at: Option<DateTime<Utc>>) -> Result<String> {
        Ok(self
            .0
            .refresh_access_token(expired_at.map(from_datetime))
            .await
            .map_err(ErrorNewType)?)
    }
}
