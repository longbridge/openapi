use std::sync::Arc;

use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::Deserialize;

use crate::{HttpClientConfig, HttpClientError, HttpClientResult, Json, RequestBuilder};

/// LongPort HTTP client
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

    /// Create a new `HttpClient` from the given environment variables
    ///
    /// # Variables
    ///
    /// - LONGPORT_APP_KEY
    /// - LONGPORT_APP_SECRET
    /// - LONGPORT_ACCESS_TOKEN
    /// - LONGPORT_HTTP_URL
    pub fn from_env() -> Result<Self, HttpClientError> {
        Ok(Self::new(HttpClientConfig::from_env()?))
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
    /// Reference: <https://open.longportapp.com/en/docs/socket-token-api>
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
