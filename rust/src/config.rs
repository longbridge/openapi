use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use http::Method;
pub(crate) use http::{header, HeaderValue, Request};
use longport_httpcli::{is_cn, HttpClient, HttpClientConfig, Json};
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use crate::error::Result;

const QUOTE_WS_URL: &str = "wss://openapi-quote.longportapp.com/v2";
const TRADE_WS_URL: &str = "wss://openapi-trade.longportapp.com/v2";
const CN_QUOTE_WS_URL: &str = "wss://openapi-quote.longportapp.cn/v2";
const CN_TRADE_WS_URL: &str = "wss://openapi-trade.longportapp.cn/v2";

/// Language identifier
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum Language {
    /// zh-CN
    ZH_CN = 0,
    /// zh-HK
    ZH_HK = 2,
    /// en
    EN = 1,
}

impl Language {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Language::ZH_CN => "zh-CN",
            Language::ZH_HK => "zh-HK",
            Language::EN => "en",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Push mode for candlestick
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PushCandlestickMode {
    /// Realtime mode
    Realtime,
    /// Confirmed mode
    Confirmed,
}

/// Configuration options for LongPort sdk
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) http_cli_config: HttpClientConfig,
    pub(crate) quote_ws_url: String,
    pub(crate) trade_ws_url: String,
    pub(crate) language: Language,
    pub(crate) enable_overnight: bool,
    pub(crate) push_candlestick_mode: PushCandlestickMode,
    pub(crate) enable_print_quote_packages: bool,
}

impl Config {
    /// Create a new `Config`
    pub fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self {
            http_cli_config: HttpClientConfig::new(app_key, app_secret, access_token),
            quote_ws_url: if is_cn() {
                CN_QUOTE_WS_URL
            } else {
                QUOTE_WS_URL
            }
            .to_string(),
            trade_ws_url: if is_cn() {
                CN_TRADE_WS_URL
            } else {
                TRADE_WS_URL
            }
            .to_string(),
            language: Language::EN,
            enable_overnight: false,
            push_candlestick_mode: PushCandlestickMode::Realtime,
            enable_print_quote_packages: true,
        }
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
    /// - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longportapp.com`)
    /// - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
    ///   `wss://openapi-quote.longportapp.com/v2`)
    /// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
    ///   `wss://openapi-trade.longportapp.com/v2`)
    /// - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or
    ///   `false` (Default: `false`)
    /// - `LONGPORT_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed` (Default:
    ///   `realtime`)
    /// - `LONGPORT_PRINT_QUOTE_PACKAGES` - Print quote packages when connected,
    ///   `true` or `false` (Default: `true`)
    pub fn from_env() -> Result<Self> {
        let _ = dotenv::dotenv();

        let http_cli_config = HttpClientConfig::from_env()?;
        let quote_ws_url = std::env::var("LONGBRIDGE_QUOTE_WS_URL")
            .or_else(|_| std::env::var("LONGPORT_QUOTE_WS_URL"))
            .unwrap_or_else(|_| {
                if is_cn() {
                    CN_QUOTE_WS_URL
                } else {
                    QUOTE_WS_URL
                }
                .to_string()
            });
        let trade_ws_url = std::env::var("LONGBRIDGE_TRADE_WS_URL")
            .or_else(|_| std::env::var("LONGPORT_TRADE_WS_URL"))
            .unwrap_or_else(|_| {
                if is_cn() {
                    CN_TRADE_WS_URL
                } else {
                    TRADE_WS_URL
                }
                .to_string()
            });
        let enable_overnight = std::env::var("LONGPORT_ENABLE_OVERNIGHT")
            .ok()
            .map(|var| var == "true")
            .unwrap_or_default();
        let push_candlestick_mode =
            if std::env::var("LONGPORT_PUSH_CANDLESTICK_MODE").as_deref() == Ok("confirmed") {
                PushCandlestickMode::Confirmed
            } else {
                PushCandlestickMode::Realtime
            };
        let enable_print_quote_packages = std::env::var("LONGPORT_PRINT_QUOTE_PACKAGES")
            .as_deref()
            .unwrap_or("true")
            == "true";

        Ok(Config {
            http_cli_config,
            quote_ws_url,
            trade_ws_url,
            language: Language::EN,
            enable_overnight,
            push_candlestick_mode,
            enable_print_quote_packages,
        })
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: `https://openapi.longportapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn http_url(mut self, url: impl Into<String>) -> Self {
        self.http_cli_config = self.http_cli_config.http_url(url);
        self
    }

    /// Specifies the url of the OpenAPI quote websocket server.
    ///
    /// Default: `wss://openapi-quote.longportapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn quote_ws_url(self, url: impl Into<String>) -> Self {
        Self {
            quote_ws_url: url.into(),
            ..self
        }
    }

    /// Specifies the url of the OpenAPI trade websocket server.
    ///
    /// Default: `wss://openapi-trade.longportapp.com/v2`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn trade_ws_url(self, url: impl Into<String>) -> Self {
        Self {
            trade_ws_url: url.into(),
            ..self
        }
    }

    /// Specifies the language
    ///
    /// Default: `Language::EN`
    pub fn language(self, language: Language) -> Self {
        Self { language, ..self }
    }

    /// Enable overnight quote
    ///
    /// Default: `false`
    pub fn enable_overnight(self) -> Self {
        Self {
            enable_overnight: true,
            ..self
        }
    }

    /// Specifies the push candlestick mode
    ///
    /// Default: `PushCandlestickMode::Realtime`
    pub fn push_candlestick_mode(self, mode: PushCandlestickMode) -> Self {
        Self {
            push_candlestick_mode: mode,
            ..self
        }
    }

    /// Disable printing the opened quote packages when connected to the server.
    pub fn dont_print_quote_packages(self) -> Self {
        Self {
            enable_print_quote_packages: false,
            ..self
        }
    }

    /// Create metadata for auth/reconnect request
    pub fn create_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("accept-language".to_string(), self.language.to_string());
        if self.enable_overnight {
            metadata.insert("need_over_night_quote".to_string(), "true".to_string());
        }
        metadata
    }

    #[inline]
    pub(crate) fn create_http_client(&self) -> HttpClient {
        HttpClient::new(self.http_cli_config.clone())
            .header(header::ACCEPT_LANGUAGE, self.language.as_str())
    }

    fn create_ws_request(&self, url: &str) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        let mut request = url.into_client_request()?;
        request.headers_mut().append(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_str(self.language.as_str()).unwrap(),
        );
        Ok(request)
    }

    #[inline]
    pub(crate) fn create_quote_ws_request(
        &self,
    ) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        self.create_ws_request(&self.quote_ws_url)
    }

    #[inline]
    pub(crate) fn create_trade_ws_request(
        &self,
    ) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        self.create_ws_request(&self.trade_ws_url)
    }

    /// Gets a new `access_token`
    ///
    /// `expired_at` - The expiration time of the access token, defaults to `90`
    /// days.
    ///
    /// Reference: <https://open.longportapp.com/en/docs/refresh-token-api>
    pub async fn refresh_access_token(&self, expired_at: Option<OffsetDateTime>) -> Result<String> {
        #[derive(Debug, Serialize)]
        struct Request {
            expired_at: String,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            token: String,
        }

        let request = Request {
            expired_at: expired_at
                .unwrap_or_else(|| OffsetDateTime::now_utc() + time::Duration::days(90))
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        };

        let new_token = self
            .create_http_client()
            .request(Method::GET, "/v1/token/refresh")
            .query_params(request)
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .token;
        Ok(new_token)
    }

    /// Gets a new `access_token`, and also replaces the `access_token` in
    /// `Config`.
    ///
    /// `expired_at` - The expiration time of the access token, defaults to `90`
    /// days.
    ///
    /// Reference: <https://open.longportapp.com/en/docs/refresh-token-api>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn refresh_access_token_blocking(
        &self,
        expired_at: Option<OffsetDateTime>,
    ) -> Result<String> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("create tokio runtime")
            .block_on(self.refresh_access_token(expired_at))
    }
}
