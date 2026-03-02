use std::{collections::HashMap, sync::Arc};

use longport::httpclient::{
    HttpClient as LbHttpClient, HttpClientConfig, HttpClientError, Json, Method,
};
use pyo3::{conversion::IntoPyObject, exceptions::PyRuntimeError, prelude::*, types::PyType};
use serde_json::Value;

use crate::error::ErrorNewType;

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
    #[new]
    fn new(
        http_url: String,
        app_key: String,
        app_secret: String,
        access_token: String,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(LbHttpClient::new(
            HttpClientConfig::new(app_key, app_secret, access_token).http_url(http_url),
        ))))
    }

    #[classmethod]
    fn from_env(_cls: Bound<PyType>) -> PyResult<Self> {
        Ok(Self(Arc::new(LbHttpClient::from_env().map_err(|err| {
            ErrorNewType(longport::Error::HttpClient(err))
        })?)))
    }

    #[classmethod]
    fn from_oauth(_cls: Bound<PyType>, client_id: String, access_token: String) -> Self {
        Self(Arc::new(LbHttpClient::new(HttpClientConfig::from_oauth(
            client_id,
            access_token,
        ))))
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
                ErrorNewType(longport::Error::HttpClient(
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
                    .map_err(|err| ErrorNewType(longport::Error::HttpClient(err)))?;
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
                    .map_err(|err| ErrorNewType(longport::Error::HttpClient(err)))?;
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
            ErrorNewType(longport::Error::HttpClient(
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
                    .map_err(|e| ErrorNewType(longport::Error::HttpClient(e)))?,
                None => req
                    .response::<Json<Value>>()
                    .send()
                    .await
                    .map_err(|e| ErrorNewType(longport::Error::HttpClient(e)))?,
            };
            Ok(JsonResponse(resp.0))
        })
        .map(|b| b.unbind())
    }
}
