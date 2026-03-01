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
    /// OAuth 2.0 mode uses Bearer token authentication and does not require app_secret.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth 2.0 client ID (used as app_key)
    /// * `access_token` - OAuth 2.0 access token (should start with "Bearer ")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use longport_httpcli::HttpClientConfig;
    ///
    /// let config = HttpClientConfig::from_oauth(
    ///     "your-client-id",
    ///     "Bearer your-access-token"
    /// );
    /// ```
    pub fn from_oauth(client_id: impl Into<String>, access_token: impl Into<String>) -> Self {
        let access_token = access_token.into();
        // Ensure Bearer prefix
        let bearer_token = if access_token.starts_with("Bearer ") {
            access_token
        } else {
            format!("Bearer {}", access_token)
        };

        Self {
            http_url: None,
            app_key: client_id.into(),
            app_secret: String::new(), // Not used in OAuth 2.0 mode
            access_token: bearer_token,
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
    /// For OAuth 2.0 authentication, use [`from_oauth`](HttpClientConfig::from_oauth) instead.
    /// OAuth tokens should not be stored in environment variables for security reasons.
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
