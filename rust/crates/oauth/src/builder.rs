//! OAuth client builder.

use std::sync::Arc;

use crate::{
    client::{DEFAULT_CALLBACK_PORT, OAuth, OAuthInner},
    error::OAuthResult,
    token::{OAuthToken, token_path_for_client_id},
};

/// Builder for constructing an [`OAuth`] client
///
/// `client_id` is the only required field.
///
/// The token is persisted at `~/.longbridge/openapi/tokens/<client_id>`.
pub struct OAuthBuilder {
    /// OAuth 2.0 client ID
    pub(crate) client_id: String,
    /// Local port for the callback server
    pub(crate) callback_port: u16,
}

impl OAuthBuilder {
    /// Create a new builder with the given client ID
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            callback_port: DEFAULT_CALLBACK_PORT,
        }
    }

    /// Set the local callback server port (default: `60355`)
    #[must_use]
    pub fn callback_port(mut self, port: u16) -> Self {
        self.callback_port = port;
        self
    }

    /// Synchronously build the [`OAuth`] client.
    ///
    /// This is the blocking equivalent of [`build`](OAuthBuilder::build).  It
    /// spins up a temporary single-threaded Tokio runtime internally so it can
    /// be called from a non-async context such as a blocking application or a
    /// doc-test `fn main()`.
    ///
    /// First tries to load an existing token from
    /// `~/.longbridge/openapi/tokens/<client_id>`.  If no valid token is found
    /// the full browser-based authorization flow is started and `open_url` is
    /// called with the authorization URL.  The resulting token is persisted for
    /// future use.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use longbridge_oauth::OAuthBuilder;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let oauth =
    ///         OAuthBuilder::new("your-client-id").build_blocking(|url| println!("Visit: {url}"))?;
    ///     println!("client_id: {}", oauth.client_id());
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "blocking")]
    pub fn build_blocking(self, open_url: impl Fn(&str)) -> OAuthResult<OAuth> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| crate::error::OAuthError::Other(e.to_string()))?
            .block_on(self.build(open_url))
    }

    /// Asynchronously build the [`OAuth`] client.
    ///
    /// First tries to load an existing token from
    /// `~/.longbridge/openapi/tokens/<client_id>`.  If no valid token is found
    /// the full browser-based authorization flow is started and `open_url` is
    /// called with the authorization URL.  The resulting token is persisted for
    /// future use.
    pub async fn build(self, open_url: impl Fn(&str)) -> OAuthResult<OAuth> {
        let token_path = token_path_for_client_id(&self.client_id)?;

        let inner = Arc::new(OAuthInner {
            client_id: self.client_id.clone(),
            callback_port: self.callback_port,
            token: tokio::sync::Mutex::new(None),
        });
        let oauth = OAuth(inner);

        let loaded = OAuthToken::load_from_path(&token_path).ok();

        let token = match loaded {
            Some(t) if !t.is_expired() => {
                tracing::debug!(path = %token_path.display(), expires_at = t.expires_at, "loaded valid token from disk");
                t
            }
            Some(t) => {
                tracing::debug!(
                    path = %token_path.display(),
                    "loaded expired token from disk, attempting refresh"
                );
                match oauth.refresh_token(&t).await {
                    Ok(refreshed) => {
                        refreshed.save_to_path(&token_path)?;
                        refreshed
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "token refresh failed, falling back to authorization flow");
                        let new_token = oauth.authorize_inner(open_url).await?;
                        new_token.save_to_path(&token_path)?;
                        new_token
                    }
                }
            }
            None => {
                tracing::debug!("no cached token found, starting authorization flow");
                let new_token = oauth.authorize_inner(open_url).await?;
                new_token.save_to_path(&token_path)?;
                new_token
            }
        };

        *oauth.0.token.lock().await = Some(token);
        Ok(oauth)
    }
}
