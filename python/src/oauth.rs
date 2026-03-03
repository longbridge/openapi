use longport::oauth::{OAuth as CoreOAuth, OAuthToken as CoreOAuthToken};
use pyo3::{exceptions::PyRuntimeError, prelude::*};

/// OAuth 2.0 access token
#[pyclass(name = "OAuthToken")]
pub(crate) struct OAuthToken(pub(crate) CoreOAuthToken);

#[pymethods]
impl OAuthToken {
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
    #[new]
    fn py_new(client_id: String) -> Self {
        Self {
            inner: CoreOAuth::new(client_id),
        }
    }

    /// Set the callback port
    ///
    /// Args:
    ///     callback_port: TCP port for the local callback server (default
    ///         60355). Must match one of the redirect URIs registered for
    ///         the client.
    fn set_callback_port(&mut self, callback_port: u16) {
        self.inner.set_callback_port(callback_port);
    }

    /// Start the OAuth 2.0 authorization flow
    ///
    /// Starts a local HTTP server, calls `on_open_url` with the authorization
    /// URL, then waits for the redirect and exchanges the authorization code
    /// for a token.
    ///
    /// Args:
    ///     on_open_url: Callable that receives the authorization URL as a
    ///         string. Use it to open the URL in a browser or print it.
    ///
    /// Returns:
    ///     OAuthToken that can be passed to Config.from_oauth or
    ///     HttpClient.from_oauth
    fn authorize<'py>(
        &self,
        py: Python<'py>,
        on_open_url: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let token = inner
                .authorize(move |url| {
                    Python::attach(|py| {
                        let _ = on_open_url.call1(py, (url,));
                    });
                })
                .await
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            Ok(OAuthToken(token))
        })
    }

    /// Refresh an access token using an existing OAuthToken
    ///
    /// Args:
    ///     token: Existing OAuthToken whose refresh token is used
    ///
    /// Returns:
    ///     New OAuthToken with a fresh access token
    fn refresh<'py>(&self, py: Python<'py>, token: &OAuthToken) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let inner_token = token.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let new_token = inner
                .refresh(&inner_token)
                .await
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            Ok(OAuthToken(new_token))
        })
    }
}
