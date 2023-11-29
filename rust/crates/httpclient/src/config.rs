use crate::{is_cn, HttpClientError};

const HTTP_URL: &str = "https://openapi.longportapp.com";
const CN_HTTP_URL: &str = "https://openapi.longportapp.cn";

/// Configuration options for Http client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// HTTP API url
    pub(crate) http_url: String,
    /// App key
    pub(crate) app_key: String,
    /// App secret
    pub(crate) app_secret: String,
    /// Access token
    pub(crate) access_token: String,
}

impl HttpClientConfig {
    /// Create a new `HttpClientConfig`
    pub fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        let http_url = if is_cn() { CN_HTTP_URL } else { HTTP_URL };
        Self {
            http_url: http_url.to_string(),
            app_key: app_key.into(),
            app_secret: app_secret.into(),
            access_token: access_token.into(),
        }
    }

    /// Create a new `HttpClientConfig` from the given environment variables
    ///
    /// # Variables
    ///
    /// - LONGPORT_APP_KEY
    /// - LONGPORT_APP_SECRET
    /// - LONGPORT_ACCESS_TOKEN
    /// - LONGPORT_HTTP_URL
    pub fn from_env() -> Result<Self, HttpClientError> {
        let _ = dotenv::dotenv();

        let app_key = std::env::var("LONGBRIDGE_APP_KEY")
            .or_else(|_| std::env::var("LONGPORT_APP_KEY"))
            .map_err(|_| HttpClientError::MissingEnvVar {
                name: "LONGPORT_APP_KEY",
            })?;
        let app_secret = std::env::var("LONGBRIDGE_APP_SECRET")
            .or_else(|_| std::env::var("LONGPORT_APP_SECRET"))
            .map_err(|_| HttpClientError::MissingEnvVar {
                name: "LONGPORT_APP_SECRET",
            })?;
        let access_token = std::env::var("LONGBRIDGE_ACCESS_TOKEN")
            .or_else(|_| std::env::var("LONGPORT_ACCESS_TOKEN"))
            .map_err(|_| HttpClientError::MissingEnvVar {
                name: "LONGPORT_ACCESS_TOKEN",
            })?;

        let mut config = Self::new(app_key, app_secret, access_token);
        if let Ok(http_url) =
            std::env::var("LONGBRIDGE_HTTP_URL").or_else(|_| std::env::var("LONGPORT_HTTP_URL"))
        {
            config.http_url = http_url;
        }
        Ok(config)
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: <https://openapi.longportapp.com>
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn http_url(self, url: impl Into<String>) -> Self {
        Self {
            http_url: url.into(),
            ..self
        }
    }
}
