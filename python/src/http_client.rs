use std::collections::HashMap;

use longbridge::httpclient::{
    HttpClient as LbHttpClient, HttpClientConfig, HttpClientError, Json, Method,
};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyType};
use serde_json::Value;

use crate::error::ErrorNewType;

#[pyclass]
pub(crate) struct HttpClient(LbHttpClient);

#[pymethods]
impl HttpClient {
    #[new]
    fn new(
        http_url: String,
        app_key: String,
        app_secret: String,
        access_token: String,
    ) -> PyResult<Self> {
        Ok(Self(LbHttpClient::new(
            HttpClientConfig::new(app_key, app_secret, access_token).http_url(http_url),
        )))
    }

    #[classmethod]
    fn from_env(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(LbHttpClient::from_env().map_err(|err| {
            ErrorNewType(longbridge::Error::HttpClient(err))
        })?))
    }

    fn request(
        &self,
        method: String,
        path: String,
        headers: Option<HashMap<String, String>>,
        body: Option<&PyAny>,
    ) -> PyResult<PyObject> {
        let body = body
            .map(pythonize::depythonize::<Value>)
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
                Ok(Python::with_gil(|py| pythonize::pythonize(py, &resp.0))
                    .map_err(|err| PyRuntimeError::new_err(err.to_string()))?)
            }
            None => {
                let resp = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(req.response::<Json<Value>>().send())
                    .map_err(|err| ErrorNewType(longbridge::Error::HttpClient(err)))?;
                Ok(Python::with_gil(|py| pythonize::pythonize(py, &resp.0))
                    .map_err(|err| PyRuntimeError::new_err(err.to_string()))?)
            }
        }
    }
}
