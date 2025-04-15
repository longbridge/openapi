use std::{
    convert::Infallible,
    error::Error,
    fmt::Debug,
    marker::PhantomData,
    time::{Duration, Instant},
};

use reqwest::{
    Method, StatusCode,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    HttpClient, HttpClientError, HttpClientResult, is_cn,
    signature::{SignatureParams, signature},
    timestamp::Timestamp,
};

const HTTP_URL: &str = "https://openapi.longportapp.com";
const HTTP_URL_CN: &str = "https://openapi.longportapp.cn";

const USER_AGENT: &str = "openapi-sdk";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const RETRY_COUNT: usize = 5;
const RETRY_INITIAL_DELAY: Duration = Duration::from_millis(100);
const RETRY_FACTOR: f32 = 2.0;

/// A JSON payload
#[derive(Debug)]
pub struct Json<T>(pub T);

/// Represents a type that can parse from payload
pub trait FromPayload: Sized + Send + Sync + 'static {
    /// A error type
    type Err: Error;

    /// Parse the payload to this object
    fn parse_from_bytes(data: &[u8]) -> Result<Self, Self::Err>;
}

/// Represents a type that can convert to payload
pub trait ToPayload: Debug + Sized + Send + Sync + 'static {
    /// A error type
    type Err: Error;

    /// Convert this object to the payload
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Err>;
}

impl<T> FromPayload for Json<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    type Err = serde_json::Error;

    #[inline]
    fn parse_from_bytes(data: &[u8]) -> Result<Self, Self::Err> {
        Ok(Json(serde_json::from_slice(data)?))
    }
}

impl<T> ToPayload for Json<T>
where
    T: Debug + Serialize + Send + Sync + 'static,
{
    type Err = serde_json::Error;

    #[inline]
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Err> {
        serde_json::to_vec(&self.0)
    }
}

impl FromPayload for String {
    type Err = std::string::FromUtf8Error;

    #[inline]
    fn parse_from_bytes(data: &[u8]) -> Result<Self, Self::Err> {
        String::from_utf8(data.to_vec())
    }
}

impl ToPayload for String {
    type Err = std::string::FromUtf8Error;

    #[inline]
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(self.clone().into_bytes())
    }
}

impl FromPayload for () {
    type Err = Infallible;

    #[inline]
    fn parse_from_bytes(_data: &[u8]) -> Result<Self, Self::Err> {
        Ok(())
    }
}

impl ToPayload for () {
    type Err = Infallible;

    #[inline]
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(vec![])
    }
}

#[derive(Deserialize)]
struct OpenApiResponse {
    code: i32,
    message: String,
    data: Option<Box<serde_json::value::RawValue>>,
}

/// A request builder
pub struct RequestBuilder<'a, T, Q, R> {
    client: &'a HttpClient,
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Option<T>,
    query_params: Option<Q>,
    mark_resp: PhantomData<R>,
}

impl<'a> RequestBuilder<'a, (), (), ()> {
    pub(crate) fn new(client: &'a HttpClient, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            headers: Default::default(),
            body: None,
            query_params: None,
            mark_resp: PhantomData,
        }
    }
}

impl<'a, T, Q, R> RequestBuilder<'a, T, Q, R> {
    /// Set the request body
    #[must_use]
    pub fn body<T2>(self, body: T2) -> RequestBuilder<'a, T2, Q, R>
    where
        T2: ToPayload,
    {
        RequestBuilder {
            client: self.client,
            method: self.method,
            path: self.path,
            headers: self.headers,
            body: Some(body),
            query_params: self.query_params,
            mark_resp: self.mark_resp,
        }
    }

    /// Set the header
    #[must_use]
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        let key = key.try_into();
        let value = value.try_into();
        if let (Ok(key), Ok(value)) = (key, value) {
            self.headers.insert(key, value);
        }
        self
    }

    /// Set the query string
    #[must_use]
    pub fn query_params<Q2>(self, params: Q2) -> RequestBuilder<'a, T, Q2, R>
    where
        Q2: Serialize + Send + Sync,
    {
        RequestBuilder {
            client: self.client,
            method: self.method,
            path: self.path,
            headers: self.headers,
            body: self.body,
            query_params: Some(params),
            mark_resp: self.mark_resp,
        }
    }

    /// Set the response body type
    #[must_use]
    pub fn response<R2>(self) -> RequestBuilder<'a, T, Q, R2>
    where
        R2: FromPayload,
    {
        RequestBuilder {
            client: self.client,
            method: self.method,
            path: self.path,
            headers: self.headers,
            body: self.body,
            query_params: self.query_params,
            mark_resp: PhantomData,
        }
    }
}

impl<T, Q, R> RequestBuilder<'_, T, Q, R>
where
    T: ToPayload,
    Q: Serialize + Send,
    R: FromPayload,
{
    async fn http_url(&self) -> &str {
        if let Some(url) = self.client.config.http_url.as_deref() {
            return url;
        }

        if is_cn().await { HTTP_URL_CN } else { HTTP_URL }
    }

    async fn do_send(&self) -> HttpClientResult<R> {
        let HttpClient {
            http_cli,
            config,
            default_headers,
        } = &self.client;
        let timestamp = self
            .headers
            .get("X-Timestamp")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(Timestamp::now);
        let app_key_value =
            HeaderValue::from_str(&config.app_key).map_err(|_| HttpClientError::InvalidApiKey)?;
        let access_token_value = HeaderValue::from_str(&config.access_token)
            .map_err(|_| HttpClientError::InvalidAccessToken)?;

        let url = self.http_url().await;
        let mut request_builder = http_cli
            .request(self.method.clone(), format!("{}{}", url, self.path))
            .headers(default_headers.clone())
            .headers(self.headers.clone())
            .header("User-Agent", USER_AGENT)
            .header("X-Api-Key", app_key_value)
            .header("Authorization", access_token_value)
            .header("X-Timestamp", timestamp.to_string())
            .header("Content-Type", "application/json; charset=utf-8");

        // set the request body
        if let Some(body) = &self.body {
            let body = body
                .to_bytes()
                .map_err(|err| HttpClientError::SerializeRequestBody(err.to_string()))?;
            request_builder = request_builder.body(body);
        }

        let mut request = request_builder.build().expect("invalid request");

        // set the query string
        if let Some(query_params) = &self.query_params {
            let query_string = crate::qs::to_string(&query_params)?;
            request.url_mut().set_query(Some(&query_string));
        }

        // signature the request
        let sign = signature(SignatureParams {
            request: &request,
            app_key: &config.app_key,
            access_token: Some(&config.access_token),
            app_secret: &config.app_secret,
            timestamp,
        });
        request.headers_mut().insert(
            "X-Api-Signature",
            HeaderValue::from_maybe_shared(sign).expect("valid signature"),
        );

        if let Some(body) = &self.body {
            tracing::info!(method = %request.method(), url = %request.url(), body = ?body, "http request");
        } else {
            tracing::info!(method = %request.method(), url = %request.url(), "http request");
        }

        let s = Instant::now();

        // send request
        let (status, trace_id, text) = tokio::time::timeout(REQUEST_TIMEOUT, async move {
            let resp = http_cli
                .execute(request)
                .await
                .map_err(|err| HttpClientError::Http(err.into()))?;
            let status = resp.status();
            let trace_id = resp
                .headers()
                .get("x-trace-id")
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string();
            let text = resp
                .text()
                .await
                .map_err(|err| HttpClientError::Http(err.into()))?;
            Ok::<_, HttpClientError>((status, trace_id, text))
        })
        .await
        .map_err(|_| HttpClientError::RequestTimeout)??;

        tracing::info!(duration = ?s.elapsed(), body = %text.as_str(), "http response");

        let resp = match serde_json::from_str::<OpenApiResponse>(&text) {
            Ok(resp) if resp.code == 0 => resp.data.ok_or(HttpClientError::UnexpectedResponse),
            Ok(resp) => Err(HttpClientError::OpenApi {
                code: resp.code,
                message: resp.message,
                trace_id,
            }),
            Err(err) if status == StatusCode::OK => {
                Err(HttpClientError::DeserializeResponseBody(err.to_string()))
            }
            Err(_) => Err(HttpClientError::BadStatus(status)),
        }?;

        R::parse_from_bytes(resp.get().as_bytes())
            .map_err(|err| HttpClientError::DeserializeResponseBody(err.to_string()))
    }

    /// Send request and get the response
    pub async fn send(self) -> HttpClientResult<R> {
        match self.do_send().await {
            Ok(resp) => Ok(resp),
            Err(HttpClientError::BadStatus(StatusCode::TOO_MANY_REQUESTS)) => {
                let mut retry_delay = RETRY_INITIAL_DELAY;

                for _ in 0..RETRY_COUNT {
                    tokio::time::sleep(retry_delay).await;

                    match self.do_send().await {
                        Ok(resp) => return Ok(resp),
                        Err(HttpClientError::BadStatus(StatusCode::TOO_MANY_REQUESTS)) => {
                            retry_delay =
                                Duration::from_secs_f32(retry_delay.as_secs_f32() * RETRY_FACTOR);
                            continue;
                        }
                        Err(err) => return Err(err),
                    }
                }

                Err(HttpClientError::BadStatus(StatusCode::TOO_MANY_REQUESTS))
            }
            Err(err) => Err(err),
        }
    }
}
