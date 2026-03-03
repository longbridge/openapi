use longport_oauth::OAuthToken;

use crate::HttpClientError;

/// Configuration options for Http client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// HTTP API url
    pub(crate) http_url: Option<String>,
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
        Self {
            http_url: None,
            app_key: app_key.into(),
            app_secret: app_secret.into(),
            access_token: access_token.into(),
        }
    }

    /// Create a new `HttpClientConfig` for OAuth 2.0
    ///
    /// OAuth 2.0 mode uses Bearer token authentication and does not require
    /// app_secret.
    ///
    /// # Arguments
    ///
    /// * `token` - OAuth 2.0 token obtained from
    ///   [`longport_oauth::OAuth::authorize`]
    pub fn from_oauth(token: &OAuthToken) -> Self {
        Self {
            http_url: None,
            app_key: token.client_id.clone(),
            app_secret: String::new(), // Not used in OAuth 2.0 mode
            access_token: format!("Bearer {}", token.access_token),
        }
    }

    /// Check if this config is using OAuth 2.0 mode
    ///
    /// OAuth 2.0 mode is detected when:
    /// 1. access_token starts with "Bearer "
    /// 2. app_secret is empty
    pub fn is_oauth2(&self) -> bool {
        self.access_token.starts_with("Bearer ") || self.app_secret.is_empty()
    }

    /// Create a new `HttpClientConfig` from the given environment variables
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
    pub fn from_env() -> Result<Self, HttpClientError> {
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

        let mut config = Self::new(app_key, app_secret, access_token);
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
    fn test_httpclient_config_from_oauth() {
        let token = longport_oauth::OAuthToken {
            client_id: "test-client-id".to_string(),
            access_token: "test-access-token".to_string(),
            refresh_token: None,
            expires_at: u64::MAX,
        };
        let config = HttpClientConfig::from_oauth(&token);

        assert_eq!(config.app_key, "test-client-id");
        assert_eq!(config.access_token, "Bearer test-access-token");
        assert_eq!(config.app_secret, "");
        assert!(config.is_oauth2());
    }

    #[test]
    fn test_httpclient_config_from_oauth_adds_bearer_prefix() {
        let token = longport_oauth::OAuthToken {
            client_id: "test-client-id".to_string(),
            access_token: "my-token".to_string(),
            refresh_token: None,
            expires_at: u64::MAX,
        };
        let config = HttpClientConfig::from_oauth(&token);

        assert_eq!(config.access_token, "Bearer my-token");
        assert!(config.is_oauth2());
    }

    #[test]
    fn test_httpclient_config_is_oauth2_with_bearer_token() {
        let config = HttpClientConfig {
            http_url: None,
            app_key: "client-id".to_string(),
            app_secret: String::new(),
            access_token: "Bearer token123".to_string(),
        };

        assert!(config.is_oauth2());
    }

    #[test]
    fn test_httpclient_config_is_oauth2_with_empty_secret() {
        let config = HttpClientConfig {
            http_url: None,
            app_key: "app-key".to_string(),
            app_secret: String::new(),
            access_token: "regular-token".to_string(),
        };

        assert!(config.is_oauth2());
    }

    #[test]
    fn test_httpclient_config_is_not_oauth2_legacy_mode() {
        let config = HttpClientConfig {
            http_url: None,
            app_key: "app-key".to_string(),
            app_secret: "app-secret".to_string(),
            access_token: "access-token".to_string(),
        };

        assert!(!config.is_oauth2());
    }

    #[test]
    fn test_httpclient_config_new() {
        let config = HttpClientConfig::new("app-key", "app-secret", "access-token");

        assert_eq!(config.app_key, "app-key");
        assert_eq!(config.app_secret, "app-secret");
        assert_eq!(config.access_token, "access-token");
        assert_eq!(config.http_url, None);
    }
}
