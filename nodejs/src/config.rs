use chrono::{DateTime, Utc};
use napi::Result;

use crate::{error::ErrorNewType, types::Language, utils::from_datetime};

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

        Self(config)
    }

    /// Create a new `Config` from the given environment variables
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGPORT_APP_KEY` - App key
    /// - `LONGPORT_APP_SECRET` - App secret
    /// - `LONGPORT_ACCESS_TOKEN` - Access token
    /// - `LONGPORT_HTTP_URL` - HTTP endpoint url
    /// - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url
    /// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url
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
