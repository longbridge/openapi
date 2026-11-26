use chrono::{DateTime, Utc};
use napi::Result;

use crate::{
    error::ErrorNewType,
    oauth::OAuth,
    types::{Language, PushCandlestickMode, QuoteTransport},
    utils::from_datetime,
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
    /// Transport for quote pull (request/response) APIs (default:
    /// QuoteTransport.WebSocket)
    ///
    /// Subscriptions and push events always use the WebSocket connection; this
    /// only selects how request/response quote APIs reach the server.
    pub quote_transport: Option<QuoteTransport>,
    /// Enable printing the opened quote packages when connected to the server
    /// (default: true).  Set to `false` to suppress the output.
    pub enable_print_quote_packages: Option<bool>,
    /// Set the path of the log files (Default: `no logs`)
    pub log_path: Option<String>,
    /// Enable paper trading mode (default: `false`).
    ///
    /// When `true`, every request carries `x-papertrading: true`, which asks
    /// the server to serve the paper trading (simulation) account and to
    /// reject a token that belongs to a real-money account.
    ///
    /// This is a server-side check, not a client-side guarantee. Two limits
    /// apply today, and neither is visible in the response:
    ///
    /// - The US data center (LBUS) has no paper trading environment. There is
    ///   no paper trading account to serve, and the server answers with the
    ///   real-money account instead of rejecting the request. The SDK therefore
    ///   refuses every order-placing call (submit, replace, multi-leg, grid
    ///   submit and replace, fund subscription) locally; reads are not refused
    ///   and return real-money data.
    /// - Under OAuth authentication the server does not apply the check at all,
    ///   in any region: a real-money token is accepted and reads return
    ///   real-money data without an error.
    ///
    /// Do not rely on this option as the only thing standing between your
    /// strategy and your real-money account.
    ///
    /// When `false` (the default) the server imposes no restrictions — both
    /// paper trading and real-money accounts are accepted.
    pub enable_papertrading: Option<bool>,
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
        if let Some(quote_transport) = extra.quote_transport {
            config.set_quote_transport(quote_transport.into());
        }
        if let Some(false) = extra.enable_print_quote_packages {
            config.set_dont_print_quote_packages();
        }
        if let Some(log_path) = extra.log_path {
            config.set_log_path(log_path);
        }
        if let Some(true) = extra.enable_papertrading {
            config.set_enable_papertrading();
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
    /// `LONGBRIDGE_QUOTE_TRANSPORT`, `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
    /// `LONGBRIDGE_LOG_PATH`).  Fields
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
    /// - `LONGBRIDGE_QUOTE_TRANSPORT` - Transport for quote pull APIs, `ws` or
    ///   `http` (Default: `ws`)
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
    /// `LONGBRIDGE_QUOTE_TRANSPORT`, `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
    /// `LONGBRIDGE_LOG_PATH`).  Fields
    /// set in `extra` override the corresponding environment variables.
    ///
    /// @param oauth  OAuth handle obtained from `OAuth.build(...)`
    /// @param extra  Optional extra parameters (override env variables)
    ///
    /// @example
    /// ```javascript
    /// const { OAuth, Config } = require('longbridge');
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => {
    ///   console.log('Open:', url);
    /// });
    /// const config = Config.fromOAuth(oauth);
    /// ```
    #[napi(factory, js_name = "fromOAuth")]
    pub fn from_oauth(oauth: &OAuth, extra: Option<ExtraConfigParams>) -> Self {
        let config = longbridge::Config::from_oauth(oauth.0.clone());
        Self(apply_extra(config, extra))
    }

    /// Gets a new `access_token`
    ///
    /// This method is only available when using **Legacy API Key**
    /// authentication (i.e. `Config.fromApikey`). It is not supported for
    /// OAuth 2.0 mode.
    ///
    /// @param expiredAt - The expiration time of the access token, defaults to
    ///   90 days from now.
    ///
    /// @see https://open.longportapp.com/en/docs/refresh-token-api
    #[napi]
    pub async fn refresh_access_token(&self, expired_at: Option<DateTime<Utc>>) -> Result<String> {
        Ok(self
            .0
            .refresh_access_token(expired_at.map(from_datetime))
            .await
            .map_err(ErrorNewType)?)
    }
}
