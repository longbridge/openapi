use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

pub(crate) use http::{HeaderValue, Request, header};
use longbridge_httpcli::{HttpClient, HttpClientConfig, is_cn};
use longbridge_oauth::OAuth;
use num_enum::IntoPrimitive;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tracing::{Level, Subscriber, subscriber::NoSubscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt};

use crate::error::Result;

const DEFAULT_QUOTE_WS_URL: &str = "wss://openapi-quote.longbridge.com/v2";
const DEFAULT_TRADE_WS_URL: &str = "wss://openapi-trade.longbridge.com/v2";
const DEFAULT_QUOTE_WS_URL_CN: &str = "wss://openapi-quote.longbridge.cn/v2";
const DEFAULT_TRADE_WS_URL_CN: &str = "wss://openapi-trade.longbridge.cn/v2";

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

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "zh-CN" => Ok(Language::ZH_CN),
            "zh-HK" => Ok(Language::ZH_HK),
            "en" => Ok(Language::EN),
            _ => Err(()),
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

/// Internal authentication mode (not part of the public API)
pub(crate) enum AuthMode {
    /// Legacy API Key mode (HMAC-SHA256 signed requests)
    ApiKey {
        app_key: String,
        app_secret: String,
        access_token: String,
    },
    /// OAuth 2.0 mode
    OAuth(OAuth),
}

impl Clone for AuthMode {
    fn clone(&self) -> Self {
        match self {
            AuthMode::ApiKey {
                app_key,
                app_secret,
                access_token,
            } => AuthMode::ApiKey {
                app_key: app_key.clone(),
                app_secret: app_secret.clone(),
                access_token: access_token.clone(),
            },
            AuthMode::OAuth(oauth) => AuthMode::OAuth(oauth.clone()),
        }
    }
}

impl fmt::Debug for AuthMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthMode::ApiKey { app_key, .. } => {
                f.debug_struct("ApiKey").field("app_key", app_key).finish()
            }
            AuthMode::OAuth(_) => f.debug_struct("OAuth").finish(),
        }
    }
}

/// Configuration options for Longbridge SDK
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) auth: AuthMode,
    pub(crate) http_url: Option<String>,
    pub(crate) quote_ws_url: Option<String>,
    pub(crate) trade_ws_url: Option<String>,
    pub(crate) enable_overnight: Option<bool>,
    pub(crate) push_candlestick_mode: Option<PushCandlestickMode>,
    pub(crate) enable_print_quote_packages: bool,
    pub(crate) language: Language,
    pub(crate) log_path: Option<PathBuf>,
}

/// Reads an env var by trying `LONGBRIDGE_<suffix>` first, then falling back
/// to the legacy `LONGBRIDGE_<suffix>` name.  Returns `None` if neither is set.
fn env_var(suffix: &str) -> Option<String> {
    std::env::var(format!("LONGBRIDGE_{suffix}"))
        .ok()
        .or_else(|| std::env::var(format!("LONGBRIDGE_{suffix}")).ok())
}

/// Non-credential environment variables shared by `from_apikey` and
/// `from_oauth`.  Callers must have already invoked `dotenv::dotenv()`.
struct ConfigExtras {
    http_url: Option<String>,
    quote_ws_url: Option<String>,
    trade_ws_url: Option<String>,
    language: Language,
    enable_overnight: Option<bool>,
    push_candlestick_mode: Option<PushCandlestickMode>,
    enable_print_quote_packages: bool,
    log_path: Option<PathBuf>,
}

impl ConfigExtras {
    fn from_env() -> Self {
        let language = env_var("LANGUAGE")
            .and_then(|v| v.parse::<Language>().ok())
            .unwrap_or(Language::EN);
        let enable_overnight = env_var("ENABLE_OVERNIGHT").map(|v| v == "true");
        let push_candlestick_mode = env_var("PUSH_CANDLESTICK_MODE").map(|v| match v.as_str() {
            "confirmed" => PushCandlestickMode::Confirmed,
            _ => PushCandlestickMode::Realtime,
        });
        let enable_print_quote_packages =
            env_var("PRINT_QUOTE_PACKAGES").as_deref().unwrap_or("true") == "true";
        Self {
            http_url: env_var("HTTP_URL"),
            quote_ws_url: env_var("QUOTE_WS_URL"),
            trade_ws_url: env_var("TRADE_WS_URL"),
            language,
            enable_overnight,
            push_candlestick_mode,
            enable_print_quote_packages,
            log_path: env_var("LOG_PATH").map(PathBuf::from),
        }
    }
}

impl Config {
    /// Create a new `Config` using API Key authentication.
    ///
    /// All optional environment variables (`LONGBRIDGE_HTTP_URL`,
    /// `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
    /// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
    /// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`,
    /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`, `LONGBRIDGE_LOG_PATH`) are read from
    /// the environment (or `.env` file) and applied automatically if set.
    ///
    /// For OAuth 2.0, use [`Config::from_oauth`] together with
    /// [`longbridge::oauth::OAuthBuilder`] instead.
    pub fn from_apikey(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        let _ = dotenv::dotenv();
        let extras = ConfigExtras::from_env();
        Self {
            auth: AuthMode::ApiKey {
                app_key: app_key.into(),
                app_secret: app_secret.into(),
                access_token: access_token.into(),
            },
            http_url: extras.http_url,
            quote_ws_url: extras.quote_ws_url,
            trade_ws_url: extras.trade_ws_url,
            language: extras.language,
            enable_overnight: extras.enable_overnight,
            push_candlestick_mode: extras.push_candlestick_mode,
            enable_print_quote_packages: extras.enable_print_quote_packages,
            log_path: extras.log_path,
        }
    }

    /// Create a new `Config` for OAuth 2.0 authentication.
    ///
    /// All optional environment variables (`LONGBRIDGE_HTTP_URL`,
    /// `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
    /// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
    /// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`,
    /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`, `LONGBRIDGE_LOG_PATH`) are read from
    /// the environment (or `.env` file) and applied automatically if set.
    ///
    /// # Arguments
    ///
    /// * `oauth` - An [`OAuth`] client obtained from
    ///   [`longbridge::oauth::OAuthBuilder`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let oauth = OAuthBuilder::new("your-client-id")
    ///         .build(|url| println!("Visit: {url}"))
    ///         .await?;
    ///     let config = Arc::new(Config::from_oauth(oauth));
    ///
    ///     let (ctx, receiver) = longbridge::quote::QuoteContext::try_new(config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from_oauth(oauth: OAuth) -> Self {
        let _ = dotenv::dotenv();
        let extras = ConfigExtras::from_env();
        Self {
            auth: AuthMode::OAuth(oauth),
            http_url: extras.http_url,
            quote_ws_url: extras.quote_ws_url,
            trade_ws_url: extras.trade_ws_url,
            language: extras.language,
            enable_overnight: extras.enable_overnight,
            push_candlestick_mode: extras.push_candlestick_mode,
            enable_print_quote_packages: extras.enable_print_quote_packages,
            log_path: extras.log_path,
        }
    }

    /// Create a new `Config` from environment variables (API Key
    /// authentication).
    ///
    /// It first loads the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGBRIDGE_APP_KEY` - App key
    /// - `LONGBRIDGE_APP_SECRET` - App secret
    /// - `LONGBRIDGE_ACCESS_TOKEN` - Access token
    /// - `LONGBRIDGE_LANGUAGE` - Language identifier, `zh-CN`, `zh-HK` or `en`
    ///   (Default: `en`)
    /// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longbridge.com`)
    /// - `LONGBRIDGE_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
    ///   `wss://openapi-quote.longbridge.com/v2`)
    /// - `LONGBRIDGE_TRADE_WS_URL` - Trade websocket endpoint url (Default:
    ///   `wss://openapi-trade.longbridge.com/v2`)
    /// - `LONGBRIDGE_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or
    ///   `false` (Default: `false`)
    /// - `LONGBRIDGE_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed`
    ///   (Default: `realtime`)
    /// - `LONGBRIDGE_PRINT_QUOTE_PACKAGES` - Print quote packages when
    ///   connected, `true` or `false` (Default: `true`)
    /// - `LONGBRIDGE_LOG_PATH` - Set the path of the log files (Default: `no
    ///   logs`)
    ///
    /// For OAuth 2.0 authentication use [`from_oauth`](Config::from_oauth)
    /// together with [`OAuthBuilder`](longbridge_oauth::OAuthBuilder).
    pub fn from_apikey_env() -> Result<Self> {
        let _ = dotenv::dotenv();

        let app_key = env_var("APP_KEY").ok_or_else(|| {
            longbridge_httpcli::HttpClientError::MissingEnvVar {
                name: "LONGBRIDGE_APP_KEY".to_string(),
            }
        })?;
        let app_secret = env_var("APP_SECRET").ok_or_else(|| {
            longbridge_httpcli::HttpClientError::MissingEnvVar {
                name: "LONGBRIDGE_APP_SECRET".to_string(),
            }
        })?;
        let access_token = env_var("ACCESS_TOKEN").ok_or_else(|| {
            longbridge_httpcli::HttpClientError::MissingEnvVar {
                name: "LONGBRIDGE_ACCESS_TOKEN".to_string(),
            }
        })?;
        let extras = ConfigExtras::from_env();

        Ok(Config {
            auth: AuthMode::ApiKey {
                app_key,
                app_secret,
                access_token,
            },
            http_url: extras.http_url,
            quote_ws_url: extras.quote_ws_url,
            trade_ws_url: extras.trade_ws_url,
            language: extras.language,
            enable_overnight: extras.enable_overnight,
            push_candlestick_mode: extras.push_candlestick_mode,
            enable_print_quote_packages: extras.enable_print_quote_packages,
            log_path: extras.log_path,
        })
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: `https://openapi.longbridge.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn http_url(mut self, url: impl Into<String>) -> Self {
        self.http_url = Some(url.into());
        self
    }

    /// Specifies the url of the OpenAPI quote websocket server.
    ///
    /// Default: `wss://openapi-quote.longbridge.com`
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
    /// Default: `wss://openapi-trade.longbridge.com/v2`
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
        Self { language, ..self }
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
        metadata.insert("accept-language".to_string(), self.language.to_string());
        if self.enable_overnight.unwrap_or_default() {
            metadata.insert("need_over_night_quote".to_string(), "true".to_string());
        }
        metadata
    }

    #[inline]
    pub(crate) fn create_http_client(&self) -> HttpClient {
        let mut config = match &self.auth {
            AuthMode::ApiKey {
                app_key,
                app_secret,
                access_token,
            } => HttpClientConfig::from_apikey(app_key, app_secret, access_token),
            AuthMode::OAuth(oauth) => HttpClientConfig::from_oauth(oauth.clone()),
        };
        if let Some(url) = &self.http_url {
            config = config.http_url(url.clone());
        }

        HttpClient::new(config).header(header::ACCEPT_LANGUAGE, self.language.as_str())
    }

    fn create_ws_request(&self, url: &str) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        let mut request = url.into_client_request()?;
        request.headers_mut().append(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_str(self.language.as_str()).unwrap(),
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

    /// Specifies the path of the log file
    ///
    /// Default: `None`
    pub fn log_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.log_path = Some(path.into());
        self
    }

    /// Set the HTTP endpoint URL in place.
    pub fn set_http_url(&mut self, url: impl Into<String>) {
        self.http_url = Some(url.into());
    }

    /// Set the quote websocket endpoint URL in place.
    pub fn set_quote_ws_url(&mut self, url: impl Into<String>) {
        self.quote_ws_url = Some(url.into());
    }

    /// Set the trade websocket endpoint URL in place.
    pub fn set_trade_ws_url(&mut self, url: impl Into<String>) {
        self.trade_ws_url = Some(url.into());
    }

    /// Set the language in place.
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    /// Enable overnight quote in place.
    pub fn set_enable_overnight(&mut self) {
        self.enable_overnight = Some(true);
    }

    /// Set the push candlestick mode in place.
    pub fn set_push_candlestick_mode(&mut self, mode: PushCandlestickMode) {
        self.push_candlestick_mode = Some(mode);
    }

    /// Disable printing quote packages in place.
    pub fn set_dont_print_quote_packages(&mut self) {
        self.enable_print_quote_packages = false;
    }

    /// Set the log path in place.
    pub fn set_log_path(&mut self, path: impl Into<PathBuf>) {
        self.log_path = Some(path.into());
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
                    .with(Targets::new().with_targets([("longbridge", Level::INFO)])),
            ))
        }

        internal_create_log_subscriber(self, path).unwrap_or_else(|| Arc::new(NoSubscriber::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_apikey() {
        let config = Config::from_apikey("app-key", "app-secret", "token");
        assert_eq!(config.language, Language::EN);
        match &config.auth {
            AuthMode::ApiKey {
                app_key,
                app_secret,
                access_token,
            } => {
                assert_eq!(app_key, "app-key");
                assert_eq!(app_secret, "app-secret");
                assert_eq!(access_token, "token");
            }
            _ => panic!("Expected ApiKey auth mode"),
        }
    }

    #[test]
    fn test_config_default_values() {
        let config = Config::from_apikey("key", "secret", "token");

        assert_eq!(config.language, Language::EN);
        assert_eq!(config.quote_ws_url, None);
        assert_eq!(config.trade_ws_url, None);
        assert_eq!(config.enable_overnight, None);
        assert_eq!(config.push_candlestick_mode, None);
        assert!(config.enable_print_quote_packages);
        assert_eq!(config.log_path, None);
    }
}
