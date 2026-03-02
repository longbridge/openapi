use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    time::{SystemTime, UNIX_EPOCH},
};

use longport::oauth::OAuth;

use crate::async_call::execute_async;

/// OAuth 2.0 access token (opaque handle)
///
/// Callers must never dereference or inspect the struct layout.
/// Always free with `lb_oauth_token_free`.
pub struct COAuthToken {
    access_token: CString,
    refresh_token: Option<CString>,
    expires_at: u64,
}

impl COAuthToken {
    fn from_token(token: longport::oauth::OAuthToken) -> Self {
        Self {
            access_token: CString::new(token.access_token).unwrap_or_default(),
            refresh_token: token
                .refresh_token
                .map(|s| CString::new(s).unwrap_or_default()),
            expires_at: token.expires_at,
        }
    }
}

fn into_token_ptr(token: longport::oauth::OAuthToken) -> *mut COAuthToken {
    Box::into_raw(Box::new(COAuthToken::from_token(token)))
}

/// OAuth 2.0 client — owns the Rust `OAuth` instance
pub struct COAuth {
    inner: OAuth,
}

/// Create a new OAuth 2.0 client
///
/// @param client_id  OAuth 2.0 client ID from the LongPort developer portal
/// @return Pointer to a new `lb_oauth_t`; free with `lb_oauth_free`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_new(client_id: *const c_char) -> *mut COAuth {
    let client_id = CStr::from_ptr(client_id)
        .to_str()
        .expect("invalid client_id")
        .to_string();
    Box::into_raw(Box::new(COAuth {
        inner: OAuth::new(client_id),
    }))
}

/// Free an OAuth 2.0 client object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_free(oauth: *mut COAuth) {
    drop(Box::from_raw(oauth));
}

/// Free a `lb_oauth_token_t` returned by `lb_oauth_authorize` or `lb_oauth_refresh`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_free(token: *mut COAuthToken) {
    if !token.is_null() {
        drop(Box::from_raw(token));
    }
}

/// Returns the access token string.
///
/// The returned pointer is valid until `lb_oauth_token_free` is called.
/// Do not free the returned pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_get_access_token(
    token: *const COAuthToken,
) -> *const c_char {
    (*token).access_token.as_ptr()
}

/// Returns the refresh token string, or null if not provided.
///
/// The returned pointer is valid until `lb_oauth_token_free` is called.
/// Do not free the returned pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_get_refresh_token(
    token: *const COAuthToken,
) -> *const c_char {
    match &(*token).refresh_token {
        Some(s) => s.as_ptr(),
        None => std::ptr::null(),
    }
}

/// Returns the Unix timestamp (seconds since epoch) when the token expires
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_get_expires_at(token: *const COAuthToken) -> u64 {
    (*token).expires_at
}

/// Returns true if the token has already expired
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_is_expired(token: *const COAuthToken) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now >= (*token).expires_at
}

/// Returns true if the token will expire within 1 hour
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_expires_soon(token: *const COAuthToken) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (*token).expires_at.saturating_sub(now) < 3600
}

/// Start the OAuth 2.0 authorization flow (async)
///
/// Starts a local HTTP server (OS-assigned port), calls `open_url_callback`
/// with the authorization URL so the caller can open it in a browser, then
/// waits for the redirect and exchanges the authorization code for a token.
///
/// @param oauth              OAuth client
/// @param open_url_callback  Called with the authorization URL and `open_url_userdata`
/// @param open_url_userdata  Opaque pointer forwarded to `open_url_callback`
/// @param callback           Async completion callback; `data` is `*mut lb_oauth_token_t` on success
/// @param userdata           Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_authorize(
    oauth: *const COAuth,
    open_url_callback: extern "C" fn(*const c_char, *mut c_void),
    open_url_userdata: *mut c_void,
    callback: crate::async_call::CAsyncCallback,
    userdata: *mut c_void,
) {
    let oauth = &*oauth;
    // Clone the client_id out of the held OAuth instance so the async block
    // can create its own borrow-free handle.  (OAuth itself is not Send.)
    let client_id = oauth.inner.client_id().to_string();
    let open_url_userdata_usize = open_url_userdata as usize;

    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let token = OAuth::new(client_id)
            .authorize(move |url| {
                let c_url = CString::new(url).unwrap_or_default();
                open_url_callback(c_url.as_ptr(), open_url_userdata_usize as *mut c_void);
            })
            .await?;
        Ok(into_token_ptr(token))
    });
}

/// Refresh an OAuth 2.0 access token (async)
///
/// @param oauth          OAuth client (provides client_id)
/// @param token          Existing token whose refresh token is used
/// @param callback       Async completion callback; `data` is `*mut lb_oauth_token_t` on success
/// @param userdata       Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_refresh(
    oauth: *const COAuth,
    token: *const COAuthToken,
    callback: crate::async_call::CAsyncCallback,
    userdata: *mut c_void,
) {
    let client_id = (*oauth).inner.client_id().to_string();
    let refresh_token = (*token)
        .refresh_token
        .as_ref()
        .expect("token has no refresh_token")
        .to_str()
        .expect("invalid refresh_token")
        .to_string();

    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let new_token = OAuth::new(client_id).refresh(&refresh_token).await?;
        Ok(into_token_ptr(new_token))
    });
}
