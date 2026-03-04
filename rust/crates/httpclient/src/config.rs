use longbridge_oauth::OAuth;

use crate::HttpClientError;

/// Reads an env var by trying `LONGBRIDGE_<suffix>` first, then falling back
/// to the legacy `LONGBRIDGE_<suffix>` name.  Returns `None` if neither is set.
fn env_var(suffix: &str) -> Option<String> {
    std::env::var(format!("LONGBRIDGE_{suffix}"))
        .ok()
        .or_else(|| std::env::var(format!("LONGBRIDGE_{suffix}")).ok())
}

/// Like [`env_var`] but returns an error (using the new `LONGBRIDGE_` name)
/// when neither variable is set.
fn env_var_required(suffix: &str) -> Result<String, HttpClientError> {
    env_var(suffix).ok_or_else(|| HttpClientError::MissingEnvVar {
        name: format!("LONGBRIDGE_{suffix}"),
    })
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub enum AuthConfig {
    /// Legacy API Key mode: HMAC-SHA256 signed requests
    ApiKey {
        /// App key
        app_key: String,
        /// App secret (used for HMAC-SHA256 signing)
        app_secret: String,
        /// Static access token
        access_token: String,
    },
    /// OAuth 2.0 mode: Bearer token, auto-refreshed via the [`OAuth`] client
    OAuth(OAuth),
}

/// Configuration options for Http client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// HTTP API url
    pub(crate) http_url: Option<String>,
    /// Authentication configuration
    pub(crate) auth: AuthConfig,
}

impl HttpClientConfig {
    /// Create a new `HttpClientConfig` using API Key authentication.
    ///
    /// `LONGBRIDGE_HTTP_URL` is read from the environment (or `.env` file) and
    /// applied automatically if set.
    ///
    /// # Arguments
    ///
    /// * `app_key` - Application key
    /// * `app_secret` - Application secret (used for request signing)
    /// * `access_token` - Access token
    pub fn from_apikey(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        let _ = dotenv::dotenv();
        Self {
            http_url: env_var("HTTP_URL"),
            auth: AuthConfig::ApiKey {
                app_key: app_key.into(),
                app_secret: app_secret.into(),
                access_token: access_token.into(),
            },
        }
    }

    /// Create a new `HttpClientConfig` for OAuth 2.0 authentication.
    ///
    /// `LONGBRIDGE_HTTP_URL` is read from the environment (or `.env` file) and
    /// applied automatically if set.
    ///
    /// The [`OAuth`] client handles token lifecycle automatically, including
    /// expiry checks and token refresh.
    ///
    /// # Arguments
    ///
    /// * `oauth` - An [`OAuth`] client obtained from
    ///   [`longbridge_oauth::OAuthBuilder`]
    pub fn from_oauth(oauth: OAuth) -> Self {
        let _ = dotenv::dotenv();
        Self {
            http_url: env_var("HTTP_URL"),
            auth: AuthConfig::OAuth(oauth),
        }
    }

    /// Create a new `HttpClientConfig` from environment variables (API Key
    /// mode).
    ///
    /// # Variables
    ///
    /// - `LONGBRIDGE_APP_KEY` - App key (required)
    /// - `LONGBRIDGE_APP_SECRET` - App secret (required)
    /// - `LONGBRIDGE_ACCESS_TOKEN` - Access token (required)
    /// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint URL (optional)
    ///
    /// # Note
    ///
    /// For OAuth 2.0 authentication, use
    /// [`from_oauth`](HttpClientConfig::from_oauth) instead.
    pub fn from_apikey_env() -> Result<Self, HttpClientError> {
        let _ = dotenv::dotenv();

        let app_key = env_var_required("APP_KEY")?;
        let app_secret = env_var_required("APP_SECRET")?;
        let access_token = env_var_required("ACCESS_TOKEN")?;

        Ok(Self {
            http_url: env_var("HTTP_URL"),
            auth: AuthConfig::ApiKey {
                app_key,
                app_secret,
                access_token,
            },
        })
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: <https://openapi.longbridge.com>
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn http_url(self, url: impl Into<String>) -> Self {
        Self {
            http_url: Some(url.into()),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_httpclient_config_new() {
        let config = HttpClientConfig::from_apikey("app-key", "app-secret", "access-token");

        match &config.auth {
            AuthConfig::ApiKey {
                app_key,
                app_secret,
                access_token,
            } => {
                assert_eq!(app_key, "app-key");
                assert_eq!(app_secret, "app-secret");
                assert_eq!(access_token, "access-token");
            }
            _ => panic!("Expected ApiKey auth config"),
        }
        assert_eq!(config.http_url, None);
    }

    #[test]
    fn test_httpclient_config_http_url() {
        let config = HttpClientConfig::from_apikey("app-key", "app-secret", "access-token")
            .http_url("https://custom.example.com");

        assert_eq!(
            config.http_url,
            Some("https://custom.example.com".to_string())
        );
    }
}
