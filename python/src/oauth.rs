use longbridge::oauth::{OAuthBuilder as CoreOAuthBuilder, OAuthError};
use pyo3::prelude::*;

use crate::error::ErrorNewType;

fn oauth_err(e: OAuthError) -> PyErr {
    ErrorNewType(longbridge::Error::OAuth(e.to_string())).into()
}

/// OAuth 2.0 client handle for Longbridge OpenAPI
///
/// Obtain an instance via :meth:`OAuthBuilder.build` (blocking) or
/// :meth:`OAuthBuilder.build_async` (async).  Pass it to
/// :meth:`Config.from_oauth` or :meth:`HttpClient.from_oauth`.
#[pyclass(name = "OAuth")]
pub(crate) struct OAuth(pub(crate) longbridge::oauth::OAuth);

/// Builder for the OAuth 2.0 authorization flow
///
/// Args:
///     client_id: OAuth 2.0 client ID from the Longbridge developer portal
///     callback_port: TCP port for the local callback server (default 60355).
///         Must match one of the redirect URIs registered for the client.
///
/// Example (blocking)::
///
///     from longbridge.openapi import OAuthBuilder, Config
///
///     oauth = OAuthBuilder("your-client-id").build(
///         lambda url: print("Open:", url)
///     )
///     config = Config.from_oauth(oauth)
///
/// Example (async)::
///
///     import asyncio
///     from longbridge.openapi import OAuthBuilder, Config
///
///     async def main():
///         oauth = await OAuthBuilder("your-client-id").build_async(
///             lambda url: print("Open:", url)
///         )
///         config = Config.from_oauth(oauth)
///
///     asyncio.run(main())
#[pyclass(name = "OAuthBuilder")]
pub(crate) struct OAuthBuilder {
    client_id: String,
    callback_port: Option<u16>,
}

#[pymethods]
impl OAuthBuilder {
    #[new]
    #[pyo3(signature = (client_id, callback_port = None))]
    fn py_new(client_id: String, callback_port: Option<u16>) -> Self {
        Self {
            client_id,
            callback_port,
        }
    }

    /// Build an OAuth 2.0 client (blocking).
    ///
    /// If a valid token is already cached on disk
    /// (``~/.longbridge-openapi/tokens/<client_id>``) it is reused;
    /// otherwise the browser authorization flow is started and
    /// ``on_open_url`` is called with the authorization URL.
    ///
    /// Args:
    ///     on_open_url: Callable that receives the authorization URL as a
    ///         string.
    ///
    /// Returns:
    ///     :class:`OAuth` handle
    fn build(&self, on_open_url: Py<PyAny>) -> PyResult<OAuth> {
        let mut builder = CoreOAuthBuilder::new(self.client_id.clone());
        if let Some(port) = self.callback_port {
            builder = builder.callback_port(port);
        }
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                builder
                    .build(move |url| {
                        Python::attach(|py| {
                            let _ = on_open_url.call1(py, (url,));
                        });
                    })
                    .await
                    .map(OAuth)
                    .map_err(oauth_err)
            })
    }

    /// Build an OAuth 2.0 client (async).
    ///
    /// If a valid token is already cached on disk
    /// (``~/.longbridge-openapi/tokens/<client_id>``) it is reused;
    /// otherwise the browser authorization flow is started and
    /// ``on_open_url`` is called with the authorization URL.
    ///
    /// Args:
    ///     on_open_url: Callable that receives the authorization URL as a
    ///         string.
    ///
    /// Returns:
    ///     Awaitable resolving to an :class:`OAuth` handle
    fn build_async<'py>(
        &self,
        py: Python<'py>,
        on_open_url: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut builder = CoreOAuthBuilder::new(self.client_id.clone());
        if let Some(port) = self.callback_port {
            builder = builder.callback_port(port);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            builder
                .build(move |url| {
                    Python::attach(|py| {
                        let _ = on_open_url.call1(py, (url,));
                    });
                })
                .await
                .map(OAuth)
                .map_err(oauth_err)
        })
    }
}
