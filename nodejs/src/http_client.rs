use std::collections::HashMap;

use longport::httpclient::{HttpClient as LbHttpClient, HttpClientConfig, Json, Method};
use napi::{Error, Result};
use serde_json::Value;

#[napi_derive::napi]
pub struct HttpClient(LbHttpClient);

#[napi_derive::napi]
impl HttpClient {
    #[napi(constructor)]
    pub fn new(
        http_url: String,
        app_key: String,
        app_secret: String,
        access_token: String,
    ) -> Result<Self> {
        Ok(Self(LbHttpClient::new(
            HttpClientConfig::new(app_key, app_secret, access_token).http_url(http_url),
        )))
    }

    /// Create a new `HttpClient` from the given environment variables
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGPORT_HTTP_URL` - HTTP endpoint url
    /// - `LONGPORT_APP_KEY` - App key
    /// - `LONGPORT_APP_SECRET` - App secret
    /// - `LONGPORT_ACCESS_TOKEN` - Access token
    #[napi(factory)]
    pub fn from_env() -> Result<Self> {
        Ok(Self(
            LbHttpClient::from_env().map_err(|err| Error::from_reason(err.to_string()))?,
        ))
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
