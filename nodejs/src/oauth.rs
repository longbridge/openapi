use longport::oauth::{OAuth as CoreOAuth, OAuthToken as CoreOAuthToken};
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

/// OAuth 2.0 access token
#[napi_derive::napi]
pub struct OAuthToken(pub(crate) CoreOAuthToken);

#[napi_derive::napi]
impl OAuthToken {
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
    /// Starts a local HTTP server, calls `onOpenUrl` with the authorization
    /// URL, then waits for the redirect and exchanges the code for a token.
    ///
    /// @param onOpenUrl  Called with the authorization URL; open it in a
    /// browser                   or print it however you like
    /// @returns OAuthToken that can be passed to `Config.fromOAuth` or
    /// `HttpClient.fromOAuth`
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
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(OAuthToken(token))
    }

    /// Refresh an access token using an existing OAuthToken
    ///
    /// @param token  Existing OAuthToken whose refresh token is used
    /// @returns New OAuthToken with a fresh access token
    #[napi]
    pub async fn refresh(&self, token: &OAuthToken) -> Result<OAuthToken> {
        let client_id = self.inner.client_id().to_string();
        let new_token = CoreOAuth::new(client_id)
            .refresh(&token.0)
            .await
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(OAuthToken(new_token))
    }
}
