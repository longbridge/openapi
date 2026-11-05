use longbridge::oauth::{OAuth as CoreOAuth, OAuthBuilder as CoreOAuthBuilder};
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

/// OAuth 2.0 client handle for Longbridge OpenAPI
///
/// Obtain an instance via `OAuth.build(...)`.
/// Pass it to `Config.fromOAuth(...)` or `HttpClient.fromOAuth(...)`.
///
/// @example
/// ```javascript
/// const { OAuth, Config } = require('longbridge');
///
/// const oauth = await OAuth.build('your-client-id', (_, url) => {
///   console.log('Open:', url);
/// });
/// const config = Config.fromOAuth(oauth);
/// ```
#[napi_derive::napi]
pub struct OAuth(pub(crate) CoreOAuth);

#[napi_derive::napi]
impl OAuth {
    /// Build an OAuth 2.0 client.
    ///
    /// If a valid token is already cached on disk
    /// (`~/.longbridge/openapi/tokens/<clientId>`) it is reused; otherwise
    /// the browser authorization flow is started and `onOpenUrl` is called
    /// with the authorization URL.
    ///
    /// @param clientId      OAuth 2.0 client ID from the Longbridge developer
    /// portal @param onOpenUrl     Called with the authorization URL; open
    /// it in a browser or print it however you like
    /// @param callbackPort  TCP port for the local callback server
    ///                      (default: 60355). Must match one of the redirect
    ///                      URIs registered for the client.
    /// @returns OAuth handle that can be passed to `Config.fromOAuth` or
    ///          `HttpClient.fromOAuth`
    #[napi(factory)]
    pub async fn build(
        client_id: String,
        on_open_url: ThreadsafeFunction<String, ()>,
        callback_port: Option<u16>,
    ) -> Result<OAuth> {
        let mut builder = CoreOAuthBuilder::new(client_id);
        if let Some(port) = callback_port {
            builder = builder.callback_port(port);
        }
        let oauth = builder
            .build(move |url| {
                on_open_url.call(Ok(url.to_string()), ThreadsafeFunctionCallMode::NonBlocking);
            })
            .await
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(OAuth(oauth))
    }

    /// Revoke the current token on the server (RFC 7009).
    ///
    /// The refresh token is revoked when present, otherwise the access token.
    /// Only the server is contacted; the cached token on disk is left
    /// untouched (see `clear`).
    ///
    /// @example
    /// ```javascript
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Open:', url));
    /// await oauth.revoke();
    /// ```
    #[napi]
    pub async fn revoke(&self) -> Result<()> {
        self.0
            .revoke()
            .await
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Clear the token locally.
    ///
    /// Drops the in-memory token and deletes the cached token file
    /// (`~/.longbridge/openapi/tokens/<clientId>`). The server is not
    /// contacted (see `revoke`).
    ///
    /// @example
    /// ```javascript
    /// await oauth.clear();
    /// ```
    #[napi]
    pub async fn clear(&self) -> Result<()> {
        self.0
            .clear()
            .await
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}
