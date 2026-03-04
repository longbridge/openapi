use longport_oauth::OAuth;

use crate::HttpClientError;

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
    /// Create a new `HttpClientConfig` using API Key authentication
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
        Self {
            http_url: None,
            auth: AuthConfig::ApiKey {
                app_key: app_key.into(),
                app_secret: app_secret.into(),
                access_token: access_token.into(),
            },
        }
    }

    /// Create a new `HttpClientConfig` for OAuth 2.0 authentication
    ///
    /// The [`OAuth`] client handles token lifecycle automatically, including
    /// expiry checks and token refresh.
    ///
    /// # Arguments
    ///
    /// * `oauth` - An [`OAuth`] client obtained from
    ///   [`longport_oauth::OAuthBuilder`]
    pub fn from_oauth(oauth: OAuth) -> Self {
        Self {
            http_url: None,
            auth: AuthConfig::OAuth(oauth),
        }
    }

    /// Create a new `HttpClientConfig` from environment variables (API Key
    /// mode)
    ///
    /// # Variables
    ///
    /// - `LONGPORT_APP_KEY` - App key
    /// - `LONGPORT_APP_SECRET` - App secret
    /// - `LONGPORT_ACCESS_TOKEN` - Access token
    /// - `LONGPORT_HTTP_URL` - (Optional) HTTP endpoint URL
    ///
    /// # Note
    ///
    /// For OAuth 2.0 authentication, use
    /// [`from_oauth`](HttpClientConfig::from_oauth) instead. OAuth tokens
    /// should not be stored in environment variables for security reasons.
    pub fn from_apikey_env() -> Result<Self, HttpClientError> {
        let _ = dotenv::dotenv();

        let app_key =
            std::env::var("LONGPORT_APP_KEY").map_err(|_| HttpClientError::MissingEnvVar {
                name: "LONGPORT_APP_KEY",
            })?;
        let app_secret =
            std::env::var("LONGPORT_APP_SECRET").map_err(|_| HttpClientError::MissingEnvVar {
                name: "LONGPORT_APP_SECRET",
            })?;
        let access_token =
            std::env::var("LONGPORT_ACCESS_TOKEN").map_err(|_| HttpClientError::MissingEnvVar {
                name: "LONGPORT_ACCESS_TOKEN",
            })?;

        let mut config = Self::from_apikey(app_key, app_secret, access_token);
        config.http_url = std::env::var("LONGPORT_HTTP_URL").ok();
        Ok(config)
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: <https://openapi.longportapp.com>
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
