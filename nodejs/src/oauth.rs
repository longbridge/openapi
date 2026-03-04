use longbridge::oauth::{OAuth as CoreOAuth, OAuthBuilder as CoreOAuthBuilder};
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

/// OAuth 2.0 client handle for Longbridge OpenAPI
///
/// Obtain an instance via `OAuthBuilder.build(...)`.
/// Pass it to `Config.fromOAuth(...)` or `HttpClient.fromOAuth(...)`.
#[napi_derive::napi]
pub struct OAuth(pub(crate) CoreOAuth);

/// Builder that performs the OAuth 2.0 authorization flow
///
/// @example
/// ```javascript
/// const { OAuthBuilder, Config } = require('longbridge');
///
/// const oauth = await OAuthBuilder.build('your-client-id', (url) => {
///   console.log('Open:', url);
/// });
/// const config = Config.fromOAuth(oauth);
/// ```
#[napi_derive::napi]
pub struct OAuthBuilder;

#[napi_derive::napi]
impl OAuthBuilder {
    /// Build an OAuth 2.0 client.
    ///
    /// If a valid token is already cached on disk
    /// (`~/.longbridge-openapi/tokens/<clientId>`) it is reused; otherwise
    /// the browser authorization flow is started and `onOpenUrl` is called
    /// with the authorization URL.
    ///
    /// @param clientId      OAuth 2.0 client ID from the Longbridge developer
    /// portal @param onOpenUrl     Called with the authorization URL; open
    /// it in a                      browser or print it however you like
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
}
