use std::collections::HashMap;

use longbridge::httpclient::{HttpClient as LbHttpClient, HttpClientConfig, Json, Method};
use napi::{Error, Result};
use serde_json::Value;

use crate::oauth::OAuth;

#[napi_derive::napi]
pub struct HttpClient(LbHttpClient);

#[napi_derive::napi]
impl HttpClient {
    /// Create a new `HttpClient` using API Key authentication
    ///
    /// `LONGBRIDGE_HTTP_URL` is read from the environment automatically.
    /// Passing `httpUrl` overrides that value.
    ///
    /// @param appKey      App key
    /// @param appSecret   App secret
    /// @param accessToken Access token
    /// @param httpUrl     HTTP endpoint url override (reads
    /// `LONGBRIDGE_HTTP_URL`                    from env if omitted; falls
    /// back to                    `https://openapi.longbridge.com`)
    #[napi(factory, js_name = "fromApikey")]
    pub fn from_apikey(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: Option<String>,
    ) -> Self {
        let mut config = HttpClientConfig::from_apikey(app_key, app_secret, access_token);
        if let Some(url) = http_url {
            config = config.http_url(url);
        }
        Self(LbHttpClient::new(config))
    }

    /// Create a new `HttpClient` from environment variables (API Key mode)
    ///
    /// It first reads the `.env` file in the current directory.
    ///
    /// # Variables
    ///
    /// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url
    /// - `LONGBRIDGE_APP_KEY` - App key
    /// - `LONGBRIDGE_APP_SECRET` - App secret
    /// - `LONGBRIDGE_ACCESS_TOKEN` - Access token
    #[napi(factory, js_name = "fromApikeyEnv")]
    pub fn from_apikey_env() -> Result<Self> {
        Ok(Self(LbHttpClient::new(
            HttpClientConfig::from_apikey_env()
                .map_err(|err| Error::from_reason(err.to_string()))?,
        )))
    }

    /// Create a new `HttpClient` from an OAuth handle
    ///
    /// `LONGBRIDGE_HTTP_URL` is read from the environment automatically.
    /// Passing `httpUrl` overrides that value.
    ///
    /// @param oauth    OAuth handle obtained from `OAuthBuilder.build(...)`
    /// @param httpUrl  HTTP endpoint url override (reads `LONGBRIDGE_HTTP_URL`
    ///                 from env if omitted; falls back to
    ///                 `https://openapi.longbridge.com`)
    #[napi(factory, js_name = "fromOAuth")]
    pub fn from_oauth(oauth: &OAuth, http_url: Option<String>) -> Self {
        let mut config = HttpClientConfig::from_oauth(oauth.0.clone());
        if let Some(url) = http_url {
            config = config.http_url(url);
        }
        Self(LbHttpClient::new(config))
    }

    /// Performs a HTTP request
    #[napi]
    pub async fn request(
        &self,
        method: String,
        path: String,
        headers: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value> {
        let req = self.0.request(
            method
                .to_uppercase()
                .parse::<Method>()
                .map_err(|err| Error::from_reason(err.to_string()))?,
            path,
        );
        let req = headers
            .unwrap_or_default()
            .into_iter()
            .fold(req, |acc, (name, value)| acc.header(name, value));

        match body {
            Some(body) => {
                let resp = req
                    .body(Json(body))
                    .response::<Json<Value>>()
                    .send()
                    .await
                    .map_err(|err| Error::from_reason(err.to_string()))?;
                Ok(resp.0)
            }
            None => {
                let resp = req
                    .response::<Json<Value>>()
                    .send()
                    .await
                    .map_err(|err| Error::from_reason(err.to_string()))?;
                Ok(resp.0)
            }
        }
    }
}
