use longport::oauth::{OAuth as CoreOAuth, OAuthToken as CoreOAuthToken};
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

use crate::error::ErrorNewType;

/// OAuth 2.0 access token
#[napi_derive::napi]
pub struct OAuthToken(CoreOAuthToken);

#[napi_derive::napi]
impl OAuthToken {
    /// The access token for API authentication
    #[napi(getter)]
    pub fn access_token(&self) -> &str {
        &self.0.access_token
    }

    /// Refresh token, or `null` if not provided by the server
    #[napi(getter)]
    pub fn refresh_token(&self) -> Option<&str> {
        self.0.refresh_token.as_deref()
    }

    /// Unix timestamp (seconds) when the token expires
    #[napi(getter)]
    pub fn expires_at(&self) -> u32 {
        self.0.expires_at as u32
    }

    /// Returns `true` if the token has expired
    #[napi]
    pub fn is_expired(&self) -> bool {
        self.0.is_expired()
    }

    /// Returns `true` if the token will expire within 1 hour
    #[napi]
    pub fn expires_soon(&self) -> bool {
        self.0.expires_soon()
    }
}

/// OAuth 2.0 client for LongPort OpenAPI
#[napi_derive::napi]
pub struct OAuth {
    inner: CoreOAuth,
}

#[napi_derive::napi]
impl OAuth {
    /// Create a new OAuth 2.0 client
    ///
    /// @param clientId  OAuth 2.0 client ID from the LongPort developer portal
    ///
    /// @example
    /// ```javascript
    /// const { OAuth } = require('longport');
    ///
    /// const oauth = new OAuth('your-client-id');
    /// const token = await oauth.authorize((url) => {
    ///   console.log('Open:', url);
    /// });
    /// console.log(token.accessToken);
    /// ```
    #[napi(constructor)]
    pub fn new(client_id: String) -> Self {
        Self {
            inner: CoreOAuth::new(client_id),
        }
    }

    /// Start the OAuth 2.0 authorization flow
    ///
    /// Starts a local HTTP server, calls `onOpenUrl` with the authorization URL,
    /// then waits for the redirect and exchanges the code for a token.
    ///
    /// @param onOpenUrl  Called with the authorization URL; open it in a browser
    ///                   or print it however you like
    /// @returns OAuthToken containing `accessToken`, `refreshToken`, and `expiresAt`
    #[napi]
    pub async fn authorize(
        &self,
        on_open_url: ThreadsafeFunction<String, ()>,
    ) -> Result<OAuthToken> {
        let client_id = self.inner.client_id().to_string();

        let token = CoreOAuth::new(client_id)
            .authorize(move |url| {
                on_open_url.call(Ok(url.to_string()), ThreadsafeFunctionCallMode::NonBlocking);
            })
            .await
            .map_err(ErrorNewType)?;

        Ok(OAuthToken(token))
    }

    /// Refresh an access token using a refresh token
    ///
    /// @param refreshToken  Refresh token from a previous authorization
    /// @returns New OAuthToken with a fresh access token
    #[napi]
    pub async fn refresh(&self, refresh_token: String) -> Result<OAuthToken> {
        let client_id = self.inner.client_id().to_string();
        let token = CoreOAuth::new(client_id)
            .refresh(&refresh_token)
            .await
            .map_err(ErrorNewType)?;
        Ok(OAuthToken(token))
    }
}
