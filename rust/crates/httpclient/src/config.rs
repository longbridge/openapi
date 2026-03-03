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
    /// `app_secret`.  The `access_token` field is stored with the `"Bearer "`
    /// prefix so that downstream components can distinguish OAuth 2.0 mode
    /// from the legacy HMAC-SHA256 mode.
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

    /// Returns `true` when this config is operating in OAuth 2.0 Bearer token
    /// mode.
    ///
    /// OAuth 2.0 mode is detected by the presence of the `"Bearer "` prefix in
    /// `access_token`.  This prefix is set unconditionally by
    /// [`HttpClientConfig::from_oauth`] and is not present in tokens
    /// constructed via [`HttpClientConfig::new`] or
    /// [`HttpClientConfig::from_env`].
    ///
    /// Do **not** use `app_secret.is_empty()` as a proxy for OAuth 2.0 mode:
    /// an empty secret may occur in misconfigured legacy setups and would
    /// produce false positives.
    pub fn is_oauth2(&self) -> bool {
        self.access_token.starts_with("Bearer ")
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

    // --- from_oauth ----------------------------------------------------------

    #[test]
    fn test_from_oauth_sets_app_key_to_client_id() {
        let token = make_token("test-client-id", "test-access-token");
        let config = HttpClientConfig::from_oauth(&token);
        assert_eq!(config.app_key, "test-client-id");
    }

    #[test]
    fn test_from_oauth_prefixes_access_token_with_bearer() {
        let token = make_token("client-id", "my-token");
        let config = HttpClientConfig::from_oauth(&token);
        assert_eq!(config.access_token, "Bearer my-token");
    }

    #[test]
    fn test_from_oauth_clears_app_secret() {
        let token = make_token("client-id", "my-token");
        let config = HttpClientConfig::from_oauth(&token);
        assert_eq!(config.app_secret, "");
    }

    #[test]
    fn test_from_oauth_is_oauth2_true() {
        let token = make_token("client-id", "my-token");
        let config = HttpClientConfig::from_oauth(&token);
        assert!(config.is_oauth2());
    }

    // --- is_oauth2 -----------------------------------------------------------

    #[test]
    fn test_is_oauth2_true_when_bearer_prefix_present() {
        let config = HttpClientConfig {
            http_url: None,
            app_key: "client-id".to_string(),
            app_secret: String::new(),
            access_token: "Bearer token123".to_string(),
        };
        assert!(config.is_oauth2());
    }

    #[test]
    fn test_is_oauth2_false_when_no_bearer_prefix_even_with_empty_secret() {
        // An empty app_secret alone must NOT indicate OAuth 2.0 mode.
        let config = HttpClientConfig {
            http_url: None,
            app_key: "app-key".to_string(),
            app_secret: String::new(),
            access_token: "regular-token".to_string(),
        };
        assert!(!config.is_oauth2());
    }

    #[test]
    fn test_is_oauth2_false_in_legacy_mode() {
        let config = HttpClientConfig {
            http_url: None,
            app_key: "app-key".to_string(),
            app_secret: "app-secret".to_string(),
            access_token: "access-token".to_string(),
        };
        assert!(!config.is_oauth2());
    }

    #[test]
    fn test_is_oauth2_false_for_lowercase_bearer_prefix() {
        // The prefix must be exactly "Bearer " (capital B) as written by
        // from_oauth.  Lowercase "bearer " is a different string.
        let config = HttpClientConfig {
            http_url: None,
            app_key: "client-id".to_string(),
            app_secret: String::new(),
            access_token: "bearer lowercase-token".to_string(),
        };
        assert!(!config.is_oauth2());
    }

    // --- new -----------------------------------------------------------------

    #[test]
    fn test_new_stores_all_fields() {
        let config = HttpClientConfig::new("app-key", "app-secret", "access-token");
        assert_eq!(config.app_key, "app-key");
        assert_eq!(config.app_secret, "app-secret");
        assert_eq!(config.access_token, "access-token");
        assert_eq!(config.http_url, None);
    }

    #[test]
    fn test_new_is_not_oauth2() {
        let config = HttpClientConfig::new("app-key", "app-secret", "access-token");
        assert!(!config.is_oauth2());
    }

    // --- http_url builder ----------------------------------------------------

    #[test]
    fn test_http_url_builder_sets_url() {
        let config = HttpClientConfig::new("k", "s", "t")
            .http_url("https://custom.api.example.com");
        assert_eq!(
            config.http_url.as_deref(),
            Some("https://custom.api.example.com")
        );
    }

    // --- helpers -------------------------------------------------------------

    fn make_token(client_id: &str, access_token: &str) -> OAuthToken {
        OAuthToken {
            client_id: client_id.to_string(),
            access_token: access_token.to_string(),
            refresh_token: None,
            expires_at: u64::MAX,
        }
    }
}
