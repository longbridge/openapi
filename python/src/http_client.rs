use std::{collections::HashMap, sync::Arc};

use longbridge::httpclient::{
    HttpClient as LbHttpClient, HttpClientConfig, HttpClientError, Json, Method,
};
use pyo3::{conversion::IntoPyObject, exceptions::PyRuntimeError, prelude::*, types::PyType};
use serde_json::Value;

use crate::{error::ErrorNewType, oauth::OAuth};

/// Wrapper so we can return JSON from async and convert to Python via
/// IntoPyObject.
struct JsonResponse(Value);

impl<'py> IntoPyObject<'py> for JsonResponse {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

#[pyclass]
pub(crate) struct HttpClient(Arc<LbHttpClient>);

#[pymethods]
impl HttpClient {
    /// Create a new ``HttpClient`` using API Key authentication.
    ///
    /// ``LONGBRIDGE_HTTP_URL`` is read from the environment automatically.
    /// Passing ``http_url`` overrides that value.
    ///
    /// Args:
    ///     app_key: App Key
    ///     app_secret: App Secret
    ///     access_token: Access Token
    ///     http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL`` from
    ///         env if omitted; falls back to
    ///         ``https://openapi.longbridge.com``)
    #[staticmethod]
    #[pyo3(signature = (app_key, app_secret, access_token, http_url = None))]
    fn from_apikey(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: Option<String>,
    ) -> Self {
        let mut config = HttpClientConfig::from_apikey(app_key, app_secret, access_token);
        if let Some(url) = http_url {
            config = config.http_url(url);
        }
        Self(Arc::new(LbHttpClient::new(config)))
    }

    /// Create a new ``HttpClient`` from environment variables (API Key
    /// authentication).
    ///
    /// Variables: ``LONGBRIDGE_HTTP_URL``, ``LONGBRIDGE_APP_KEY``,
    /// ``LONGBRIDGE_APP_SECRET``, ``LONGBRIDGE_ACCESS_TOKEN``
    #[classmethod]
    fn from_apikey_env(_cls: Bound<PyType>) -> PyResult<Self> {
        Ok(Self(Arc::new(LbHttpClient::new(
            longbridge::httpclient::HttpClientConfig::from_apikey_env()
                .map_err(|err| ErrorNewType(longbridge::Error::HttpClient(err)))?,
        ))))
    }

    /// Create a new ``HttpClient`` from an OAuth handle.
    ///
    /// ``LONGBRIDGE_HTTP_URL`` is read from the environment automatically.
    /// Passing ``http_url`` overrides that value.
    ///
    /// Args:
    ///     oauth: :class:`OAuth` handle from :meth:`OAuthBuilder.build` or
    ///         :meth:`OAuthBuilder.build_async`
    ///     http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL`` from
    ///         env if omitted; falls back to
    ///         ``https://openapi.longbridge.com``)
    #[classmethod]
    #[pyo3(signature = (oauth, http_url = None))]
    fn from_oauth(_cls: Bound<PyType>, oauth: &OAuth, http_url: Option<String>) -> Self {
        let mut config = HttpClientConfig::from_oauth(oauth.0.clone());
        if let Some(url) = http_url {
            config = config.http_url(url);
        }
        Self(Arc::new(LbHttpClient::new(config)))
    }

    #[pyo3(signature = (method, path, headers=None, body=None))]
    fn request(
        &self,
        method: String,
        path: String,
        headers: Option<HashMap<String, String>>,
        body: Option<Bound<PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let body = body
            .map(|body| pythonize::depythonize::<Value>(&body))
            .transpose()
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
        let req = self.0.request(
            method.to_uppercase().parse::<Method>().map_err(|_| {
                ErrorNewType(longbridge::Error::HttpClient(
                    HttpClientError::InvalidRequestMethod,
                ))
            })?,
            path,
        );
        let req = headers
            .unwrap_or_default()
            .into_iter()
            .fold(req, |acc, (name, value)| acc.header(name, value));

        match body {
            Some(body) => {
                let resp = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(req.body(Json(body)).response::<Json<Value>>().send())
                    .map_err(|err| ErrorNewType(longbridge::Error::HttpClient(err)))?;
                Python::attach(|py| {
                    Ok(pythonize::pythonize(py, &resp.0)
                        .map_err(|err| PyRuntimeError::new_err(err.to_string()))?
                        .into())
                })
            }
            None => {
                let resp = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(req.response::<Json<Value>>().send())
                    .map_err(|err| ErrorNewType(longbridge::Error::HttpClient(err)))?;
                Python::attach(|py| {
                    Ok(pythonize::pythonize(py, &resp.0)
                        .map_err(|err| PyRuntimeError::new_err(err.to_string()))?
                        .into())
                })
            }
        }
    }

    /// Async request. Returns an awaitable; must be awaited inside asyncio.
    #[pyo3(signature = (method, path, headers=None, body=None))]
    fn request_async(
        &self,
        py: Python<'_>,
        method: String,
        path: String,
        headers: Option<HashMap<String, String>>,
        body: Option<Bound<PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let body_value: Option<Value> = body
            .map(|b| pythonize::depythonize::<Value>(&b))
            .transpose()
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
        let method_parsed = method.to_uppercase().parse::<Method>().map_err(|_| {
            ErrorNewType(longbridge::Error::HttpClient(
                HttpClientError::InvalidRequestMethod,
            ))
        })?;
        let headers = headers.unwrap_or_default();
        let client = Arc::clone(&self.0);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let req = client.request(method_parsed, path);
            let req = headers
                .into_iter()
                .fold(req, |acc, (name, value)| acc.header(name, value));
            let resp = match body_value {
                Some(b) => req
                    .body(Json(b))
                    .response::<Json<Value>>()
                    .send()
                    .await
                    .map_err(|e| ErrorNewType(longbridge::Error::HttpClient(e)))?,
                None => req
                    .response::<Json<Value>>()
                    .send()
                    .await
                    .map_err(|e| ErrorNewType(longbridge::Error::HttpClient(e)))?,
            };
            Ok(JsonResponse(resp.0))
        })
        .map(|b| b.unbind())
    }
}
