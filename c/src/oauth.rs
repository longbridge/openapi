use std::{ffi::c_void, os::raw::c_char};

use longport::oauth::OAuth;

use crate::async_call::execute_async;

/// OAuth 2.0 access token (opaque handle)
///
/// Callers must never dereference or inspect the struct layout.
/// Always free with `lb_oauth_token_free`.
pub struct COAuthToken(pub(crate) longport::oauth::OAuthToken);

fn into_token_ptr(token: longport::oauth::OAuthToken) -> *mut COAuthToken {
    Box::into_raw(Box::new(COAuthToken(token)))
}

/// OAuth 2.0 client — owns the Rust `OAuth` instance
pub struct COAuth {
    inner: OAuth,
}

/// Create a new OAuth 2.0 client with the default callback port (60355)
///
/// @param client_id  OAuth 2.0 client ID from the LongPort developer portal
/// @return Pointer to a new `lb_oauth_t`; free with `lb_oauth_free`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_new(client_id: *const c_char) -> *mut COAuth {
    let client_id = std::ffi::CStr::from_ptr(client_id)
        .to_str()
        .expect("invalid client_id")
        .to_string();
    Box::into_raw(Box::new(COAuth {
        inner: OAuth::new(client_id),
    }))
}

/// Set the callback port on an existing OAuth 2.0 client
///
/// @param oauth          OAuth client
/// @param callback_port  TCP port for the local callback server. Must match one
///                       of the redirect URIs registered for the client.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_set_callback_port(oauth: *mut COAuth, callback_port: u16) {
    (*oauth).inner.set_callback_port(callback_port);
}

/// Free an OAuth 2.0 client object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_free(oauth: *mut COAuth) {
    drop(Box::from_raw(oauth));
}

/// Free a `lb_oauth_token_t` returned by `lb_oauth_authorize` or
/// `lb_oauth_refresh`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_free(token: *mut COAuthToken) {
    if !token.is_null() {
        drop(Box::from_raw(token));
    }
}

/// Returns non-zero if the token has expired
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_is_expired(token: *const COAuthToken) -> i32 {
    (*token).0.is_expired() as i32
}

/// Returns non-zero if the token will expire within 1 hour
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_token_expires_soon(token: *const COAuthToken) -> i32 {
    (*token).0.expires_soon() as i32
}

/// Start the OAuth 2.0 authorization flow (async)
///
/// Starts a local HTTP server, calls `open_url_callback` with the
/// authorization URL so the caller can open it in a browser, then waits for
/// the redirect and exchanges the authorization code for a token.
///
/// @param oauth              OAuth client
/// @param open_url_callback  Called with the authorization URL and
///                           `open_url_userdata`
/// @param open_url_userdata  Opaque pointer forwarded to `open_url_callback`
/// @param callback           Async completion callback; `data` is
///                           `*mut lb_oauth_token_t` on success
/// @param userdata           Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_authorize(
    oauth: *const COAuth,
    open_url_callback: extern "C" fn(*const c_char, *mut c_void),
    open_url_userdata: *mut c_void,
    callback: crate::async_call::CAsyncCallback,
    userdata: *mut c_void,
) {
    let inner = (*oauth).inner.clone();
    let open_url_userdata_usize = open_url_userdata as usize;

    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let token = inner
            .authorize(move |url| {
                let c_url = std::ffi::CString::new(url).unwrap_or_default();
                open_url_callback(c_url.as_ptr(), open_url_userdata_usize as *mut c_void);
            })
            .await
            .map_err(|e| longport::Error::OAuth(e.to_string()))?;
        Ok(into_token_ptr(token))
    });
}

/// Refresh an OAuth 2.0 access token (async)
///
/// @param oauth          OAuth client
/// @param token          Existing token whose refresh token is used
/// @param callback       Async completion callback; `data` is
///                       `*mut lb_oauth_token_t` on success
/// @param userdata       Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_refresh(
    oauth: *const COAuth,
    token: *const COAuthToken,
    callback: crate::async_call::CAsyncCallback,
    userdata: *mut c_void,
) {
    let inner = (*oauth).inner.clone();
    let existing_token = (*token).0.clone();

    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let new_token = inner
            .refresh(&existing_token)
            .await
            .map_err(|e| longport::Error::OAuth(e.to_string()))?;
        Ok(into_token_ptr(new_token))
    });
}
