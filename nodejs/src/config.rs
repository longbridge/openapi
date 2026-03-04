use napi::Result;

use crate::{
    error::ErrorNewType,
    oauth::OAuth,
    types::{Language, PushCandlestickMode},
};

/// Optional extra parameters shared by `Config.fromApikey` and
/// `Config.fromOAuth`.  All fields are optional.
#[napi_derive::napi(object)]
pub struct ExtraConfigParams {
    /// HTTP API url (default: "https://openapi.longbridge.com")
    pub http_url: Option<String>,
    /// Websocket url for quote API (default:
    /// "wss://openapi-quote.longbridge.com/v2")
    pub quote_ws_url: Option<String>,
    /// Websocket url for trade API (default:
    /// "wss://openapi-trade.longbridge.com/v2")
    pub trade_ws_url: Option<String>,
    /// Language identifier (default: Language.EN)
    pub language: Option<Language>,
    /// Enable overnight (default: false)
    pub enable_overnight: Option<bool>,
    /// Push candlesticks mode (default: PushCandlestickMode.Realtime)
    pub push_candlestick_mode: Option<PushCandlestickMode>,
    /// Enable printing the opened quote packages when connected to the server
    /// (default: true).  Set to `false` to suppress the output.
    pub enable_print_quote_packages: Option<bool>,
    /// Set the path of the log files (Default: `no logs`)
    pub log_path: Option<String>,
}

fn apply_extra(
    mut config: longbridge::Config,
    extra: Option<ExtraConfigParams>,
) -> longbridge::Config {
    if let Some(extra) = extra {
        if let Some(http_url) = extra.http_url {
            config.set_http_url(http_url);
        }
        if let Some(quote_ws_url) = extra.quote_ws_url {
            config.set_quote_ws_url(quote_ws_url);
        }
        if let Some(trade_ws_url) = extra.trade_ws_url {
            config.set_trade_ws_url(trade_ws_url);
        }
        if let Some(language) = extra.language {
            config.set_language(language.into());
        }
        if let Some(true) = extra.enable_overnight {
            config.set_enable_overnight();
        }
        if let Some(mode) = extra.push_candlestick_mode {
            config.set_push_candlestick_mode(mode.into());
        }
        if let Some(false) = extra.enable_print_quote_packages {
            config.set_dont_print_quote_packages();
        }
        if let Some(log_path) = extra.log_path {
            config.set_log_path(log_path);
        }
    }
    config
}

/// Configuration for Longbridge SDK
#[napi_derive::napi]
pub struct Config(pub(crate) longbridge::Config);

#[napi_derive::napi]
impl Config {
    /// Create a new `Config` using API Key authentication
    ///
    /// Optional environment variables are read automatically
    /// (`LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`,
    /// `LONGBRIDGE_QUOTE_WS_URL`, `LONGBRIDGE_TRADE_WS_URL`,
    /// `LONGBRIDGE_ENABLE_OVERNIGHT`, `LONGBRIDGE_PUSH_CANDLESTICK_MODE`,
    /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`, `LONGBRIDGE_LOG_PATH`).  Fields
    /// set in `extra` override the corresponding environment variables.
    ///
    /// @param appKey       Application key
    /// @param appSecret    Application secret
    /// @param accessToken  Access token
    /// @param extra        Optional extra parameters (override env variables)
    ///
    /// @example
    /// ```javascript
    /// const { Config } = require('longbridge');
    ///
    /// const config = Config.fromApikey(
    ///   process.env.LONGBRIDGE_APP_KEY,
    ///   process.env.LONGBRIDGE_APP_SECRET,
    ///   process.env.LONGBRIDGE_ACCESS_TOKEN,
    /// );
    /// ```
    #[napi(factory, js_name = "fromApikey")]
    pub fn from_apikey(
        app_key: String,
        app_secret: String,
        access_token: String,
        extra: Option<ExtraConfigParams>,
    ) -> Self {
        let config = longbridge::Config::from_apikey(app_key, app_secret, access_token);
        Self(apply_extra(config, extra))
    }

    /// Create a new `Config` from the environment (API Key authentication)
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGBRIDGE_LANGUAGE` - Language identifier, `zh-CN`, `zh-HK` or `en`
    ///   (Default: `en`)
    /// - `LONGBRIDGE_APP_KEY` - App key
    /// - `LONGBRIDGE_APP_SECRET` - App secret
    /// - `LONGBRIDGE_ACCESS_TOKEN` - Access token
    /// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url
    /// - `LONGBRIDGE_QUOTE_WS_URL` - Quote websocket endpoint url
    /// - `LONGBRIDGE_TRADE_WS_URL` - Trade websocket endpoint url
    /// - `LONGBRIDGE_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or
    ///   `false` (Default: `false`)
    /// - `LONGBRIDGE_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed`
    ///   (Default: `realtime`)
    /// - `LONGBRIDGE_PRINT_QUOTE_PACKAGES` - Print quote packages when
    ///   connected, `true` or `false` (Default: `true`)
    /// - `LONGBRIDGE_LOG_PATH` - Log file directory (Default: no logs)
    #[napi(factory, js_name = "fromApikeyEnv")]
    pub fn from_apikey_env() -> Result<Self> {
        Ok(Self(
            longbridge::Config::from_apikey_env().map_err(ErrorNewType)?,
        ))
    }

    /// Create a new `Config` for OAuth 2.0 authentication
    ///
    /// OAuth 2.0 is the recommended authentication method that uses Bearer
    /// tokens and does not require app_secret or HMAC signatures.
    ///
    /// Optional environment variables are read automatically
    /// (`LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`,
    /// `LONGBRIDGE_QUOTE_WS_URL`, `LONGBRIDGE_TRADE_WS_URL`,
    /// `LONGBRIDGE_ENABLE_OVERNIGHT`, `LONGBRIDGE_PUSH_CANDLESTICK_MODE`,
    /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`, `LONGBRIDGE_LOG_PATH`).  Fields
    /// set in `extra` override the corresponding environment variables.
    ///
    /// @param oauth  OAuth handle obtained from `OAuthBuilder.build(...)`
    /// @param extra  Optional extra parameters (override env variables)
    ///
    /// @example
    /// ```javascript
    /// const { OAuthBuilder, Config } = require('longbridge');
    ///
    /// const oauth = await OAuthBuilder.build('your-client-id', (url) => {
    ///   console.log('Open:', url);
    /// });
    /// const config = Config.fromOAuth(oauth);
    /// ```
    #[napi(factory, js_name = "fromOAuth")]
    pub fn from_oauth(oauth: &OAuth, extra: Option<ExtraConfigParams>) -> Self {
        let config = longbridge::Config::from_oauth(oauth.0.clone());
        Self(apply_extra(config, extra))
    }
}
