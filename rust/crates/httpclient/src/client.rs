use std::sync::Arc;

use longbridge_geo::DcRegion;
use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::Deserialize;

use crate::{
    AuthConfig, HttpClientConfig, HttpClientError, HttpClientResult, Json, RequestBuilder,
};

/// Longbridge HTTP client
#[derive(Clone)]
pub struct HttpClient {
    pub(crate) http_cli: Client,
    pub(crate) config: Arc<HttpClientConfig>,
    pub(crate) default_headers: HeaderMap,
}

impl HttpClient {
    /// Create a new `HttpClient`
    pub fn new(config: HttpClientConfig) -> Self {
        Self {
            http_cli: Client::new(),
            config: Arc::new(config),
            default_headers: HeaderMap::new(),
        }
    }

    /// Set the default header
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        let key = key.try_into();
        let value = value.try_into();
        if let (Ok(key), Ok(value)) = (key, value) {
            self.default_headers.insert(key, value);
        }
        self
    }

    /// The data-center region (`us`/`ap`) derived from this client's auth
    /// credentials. Used by non-HTTP call sites (e.g. WebSocket quote commands)
    /// to apply the same AP-only routing guard the HTTP path enforces.
    pub async fn dc_region(&self) -> DcRegion {
        match &self.config.auth {
            AuthConfig::ApiKey {
                app_key,
                app_secret,
                access_token,
            } => DcRegion::from_credentials(&[app_key, access_token, app_secret]),
            AuthConfig::OAuth(oauth) => oauth
                .access_token()
                .await
                .map(|token| DcRegion::from_credential(&token))
                .unwrap_or(DcRegion::Ap),
        }
    }

    /// Create a new request builder
    #[inline]
    pub fn request(
        &self,
        method: Method,
        path: impl Into<String>,
    ) -> RequestBuilder<'_, (), (), ()> {
        RequestBuilder::new(self, method, path)
    }

    /// Get the socket OTP(One Time Password)
    ///
    /// Reference: <https://open.longbridge.com/en/docs/socket-token-api>
    pub async fn get_otp(&self) -> HttpClientResult<String> {
        #[derive(Debug, Deserialize)]
        struct Response {
            otp: String,
            limit: i32,
            online: i32,
        }

        let resp = self
            .request(Method::GET, "/v1/socket/token")
            .response::<Json<Response>>()
            .send()
            .await?
            .0;
        tracing::info!(limit = resp.limit, online = resp.online, "create otp");

        if resp.online >= resp.limit {
            return Err(HttpClientError::ConnectionLimitExceeded {
                limit: resp.limit,
                online: resp.online,
            });
        }

        Ok(resp.otp)
    }
}
