use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::{Path, PathBuf},
    sync::Arc,
};

use http::Method;
pub(crate) use http::{HeaderValue, Request, header};
use longport_httpcli::{HttpClient, HttpClientConfig, Json, is_cn};
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tracing::{Level, Subscriber, subscriber::NoSubscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt};

use crate::error::Result;

const DEFAULT_QUOTE_WS_URL: &str = "wss://openapi-quote.longportapp.com/v2";
const DEFAULT_TRADE_WS_URL: &str = "wss://openapi-trade.longportapp.com/v2";
const DEFAULT_QUOTE_WS_URL_CN: &str = "wss://openapi-quote.longportapp.cn/v2";
const DEFAULT_TRADE_WS_URL_CN: &str = "wss://openapi-trade.longportapp.cn/v2";

/// Language identifier
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum Language {
    /// zh-CN
    ZH_CN = 0,
    /// zh-HK
    ZH_HK = 2,
    /// en
    #[default]
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
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum PushCandlestickMode {
    /// Realtime mode
    #[default]
    Realtime,
    /// Confirmed mode
    Confirmed,
}

/// Configuration options for LongPort sdk
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) http_cli_config: HttpClientConfig,
    pub(crate) quote_ws_url: Option<String>,
    pub(crate) trade_ws_url: Option<String>,
    pub(crate) enable_overnight: Option<bool>,
    pub(crate) push_candlestick_mode: Option<PushCandlestickMode>,
    pub(crate) enable_print_quote_packages: bool,
    pub(crate) language: Option<Language>,
    pub(crate) log_path: Option<PathBuf>,
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
            quote_ws_url: None,
            trade_ws_url: None,
            language: None,
            enable_overnight: None,
            push_candlestick_mode: None,
            enable_print_quote_packages: true,
            log_path: None,
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
    /// - `LONGPORT_LOG_PATH` - Set the path of the log files (Default: `no
    ///   logs`)
    pub fn from_env() -> Result<Self> {
        let _ = dotenv::dotenv();

        let http_cli_config = HttpClientConfig::from_env()?;
        let quote_ws_url = std::env::var("LONGPORT_QUOTE_WS_URL").ok();
        let trade_ws_url = std::env::var("LONGPORT_TRADE_WS_URL").ok();
        let enable_overnight = std::env::var("LONGPORT_ENABLE_OVERNIGHT")
            .map(|value| value == "true")
            .ok();
        let push_candlestick_mode = std::env::var("LONGPORT_PUSH_CANDLESTICK_MODE")
            .map(|value| match value.as_str() {
                "confirmed" => PushCandlestickMode::Confirmed,
                _ => PushCandlestickMode::Realtime,
            })
            .ok();
        let enable_print_quote_packages = std::env::var("LONGPORT_PRINT_QUOTE_PACKAGES")
            .as_deref()
            .unwrap_or("true")
            == "true";
        let log_path = std::env::var("LONGPORT_LOG_PATH").ok().map(PathBuf::from);

        Ok(Config {
            http_cli_config,
            quote_ws_url,
            trade_ws_url,
            language: None,
            enable_overnight,
            push_candlestick_mode,
            enable_print_quote_packages,
            log_path,
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
            quote_ws_url: Some(url.into()),
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
            trade_ws_url: Some(url.into()),
            ..self
        }
    }

    /// Specifies the language
    ///
    /// Default: `Language::EN`
    pub fn language(self, language: Language) -> Self {
        Self {
            language: Some(language),
            ..self
        }
    }

    /// Enable overnight quote
    ///
    /// Default: `false`
    pub fn enable_overnight(self) -> Self {
        Self {
            enable_overnight: Some(true),
            ..self
        }
    }

    /// Specifies the push candlestick mode
    ///
    /// Default: `PushCandlestickMode::Realtime`
    pub fn push_candlestick_mode(self, mode: PushCandlestickMode) -> Self {
        Self {
            push_candlestick_mode: Some(mode),
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
        metadata.insert(
            "accept-language".to_string(),
            self.language.unwrap_or_default().to_string(),
        );
        if self.enable_overnight.unwrap_or_default() {
            metadata.insert("need_over_night_quote".to_string(), "true".to_string());
        }
        metadata
    }

    #[inline]
    pub(crate) fn create_http_client(&self) -> HttpClient {
        HttpClient::new(self.http_cli_config.clone()).header(
            header::ACCEPT_LANGUAGE,
            self.language.unwrap_or_default().as_str(),
        )
    }

    fn create_ws_request(&self, url: &str) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        let mut request = url.into_client_request()?;
        request.headers_mut().append(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_str(self.language.unwrap_or_default().as_str()).unwrap(),
        );
        Ok(request)
    }

    pub(crate) async fn create_quote_ws_request(
        &self,
    ) -> (&str, tokio_tungstenite::tungstenite::Result<Request<()>>) {
        match self.quote_ws_url.as_deref() {
            Some(url) => (url, self.create_ws_request(url)),
            None => {
                let url = if is_cn().await {
                    DEFAULT_QUOTE_WS_URL_CN
                } else {
                    DEFAULT_QUOTE_WS_URL
                };
                (url, self.create_ws_request(url))
            }
        }
    }

    pub(crate) async fn create_trade_ws_request(
        &self,
    ) -> (&str, tokio_tungstenite::tungstenite::Result<Request<()>>) {
        match self.trade_ws_url.as_deref() {
            Some(url) => (url, self.create_ws_request(url)),
            None => {
                let url = if is_cn().await {
                    DEFAULT_TRADE_WS_URL_CN
                } else {
                    DEFAULT_TRADE_WS_URL
                };
                (url, self.create_ws_request(url))
            }
        }
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

    /// Specifies the path of the log file
    ///
    /// Default: `None`
    pub fn log_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.log_path = Some(path.into());
        self
    }

    pub(crate) fn create_log_subscriber(
        &self,
        path: impl AsRef<Path>,
    ) -> Arc<dyn Subscriber + Send + Sync> {
        fn internal_create_log_subscriber(
            config: &Config,
            path: impl AsRef<Path>,
        ) -> Option<Arc<dyn Subscriber + Send + Sync>> {
            let log_path = config.log_path.as_ref()?;
            let appender = RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_suffix("log")
                .build(log_path.join(path))
                .ok()?;
            Some(Arc::new(
                tracing_subscriber::fmt()
                    .with_writer(appender)
                    .with_ansi(false)
                    .finish()
                    .with(Targets::new().with_targets([("longport", Level::INFO)])),
            ))
        }

        internal_create_log_subscriber(self, path).unwrap_or_else(|| Arc::new(NoSubscriber::new()))
    }
}
