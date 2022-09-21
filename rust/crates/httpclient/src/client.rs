use std::sync::Arc;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Method,
};
use serde::Deserialize;

use crate::{HttpClientConfig, HttpClientResult, RequestBuilder};

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
            self.default_headers.append(key, value);
        }
        self
    }

    /// Create a new request builder
    #[inline]
    pub fn request(&self, method: Method, path: impl Into<String>) -> RequestBuilder<(), (), ()> {
        RequestBuilder::new(self.clone(), method, path)
    }

    /// Get the socket OTP(One Time Password)
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/socket-token-api>
    pub async fn get_otp(&self) -> HttpClientResult<String> {
        #[derive(Debug, Deserialize)]
        struct Response {
            otp: String,
        }

        Ok(self
            .request(Method::GET, "/v1/socket/token")
            .response::<Response>()
            .send()
            .await?
            .otp)
    }

    /// Get the socket OTP v2(One Time Password)
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/socket-token-api>
    pub async fn get_otp_v2(&self) -> HttpClientResult<String> {
        #[derive(Debug, Deserialize)]
        struct Response {
            otp: String,
        }

        Ok(self
            .request(Method::GET, "/v2/socket/token")
            .response::<Response>()
            .send()
            .await?
            .otp)
    }
}
