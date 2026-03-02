use longport::oauth::{OAuth as CoreOAuth, OAuthToken as CoreOAuthToken};
use pyo3::prelude::*;

use crate::error::ErrorNewType;

/// OAuth 2.0 access token
#[pyclass(name = "OAuthToken")]
pub(crate) struct OAuthToken(pub(crate) CoreOAuthToken);

#[pymethods]
impl OAuthToken {
    /// The access token for API authentication
    #[getter]
    fn access_token(&self) -> &str {
        &self.0.access_token
    }

    /// Refresh token, or `None` if not provided by the server
    #[getter]
    fn refresh_token(&self) -> Option<&str> {
        self.0.refresh_token.as_deref()
    }

    /// Unix timestamp when the token expires
    #[getter]
    fn expires_at(&self) -> u64 {
        self.0.expires_at
    }

    /// Returns `True` if the token has expired
    fn is_expired(&self) -> bool {
        self.0.is_expired()
    }

    /// Returns `True` if the token will expire within 1 hour
    fn expires_soon(&self) -> bool {
        self.0.expires_soon()
    }
}

/// OAuth 2.0 client for LongPort OpenAPI
#[pyclass(name = "OAuth")]
pub(crate) struct OAuth {
    inner: CoreOAuth,
}

#[pymethods]
impl OAuth {
    /// Create a new OAuth 2.0 client
    ///
    /// Args:
    ///     client_id: OAuth 2.0 client ID from the LongPort developer portal
    ///
    /// Example:
    ///
    /// ```python
    /// import asyncio
    /// from longport.openapi import OAuth
    ///
    /// async def main():
    ///     oauth = OAuth("your-client-id")
    ///     token = await oauth.authorize(lambda url: print(f"Open: {url}"))
    ///     print(token.access_token)
    ///
    /// asyncio.run(main())
    /// ```
    #[new]
    fn py_new(client_id: String) -> Self {
        Self {
            inner: CoreOAuth::new(client_id),
        }
    }

    /// Start the OAuth 2.0 authorization flow
    ///
    /// Starts a local HTTP server, calls `on_open_url` with the authorization
    /// URL, then waits for the redirect and exchanges the authorization code for
    /// a token.
    ///
    /// Args:
    ///     on_open_url: Callable that receives the authorization URL as a string.
    ///                  Use it to open the URL in a browser or print it.
    ///
    /// Returns:
    ///     OAuthToken with access_token, refresh_token (optional), and expires_at
    fn authorize<'py>(
        &self,
        py: Python<'py>,
        on_open_url: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client_id = self.inner.client_id().to_string();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let token = CoreOAuth::new(client_id)
                .authorize(move |url| {
                    Python::attach(|py| {
                        let _ = on_open_url.call1(py, (url,));
                    });
                })
                .await
                .map_err(ErrorNewType)?;
            Ok(OAuthToken(token))
        })
    }

    /// Refresh an access token using a refresh token
    ///
    /// Args:
    ///     refresh_token: Refresh token from a previous authorization
    ///
    /// Returns:
    ///     New OAuthToken with a fresh access token
    fn refresh<'py>(&self, py: Python<'py>, refresh_token: String) -> PyResult<Bound<'py, PyAny>> {
        let client_id = self.inner.client_id().to_string();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let token = CoreOAuth::new(client_id)
                .refresh(&refresh_token)
                .await
                .map_err(ErrorNewType)?;
            Ok(OAuthToken(token))
        })
    }
}
