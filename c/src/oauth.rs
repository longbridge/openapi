use std::{ffi::c_void, os::raw::c_char};

use longbridge::oauth::{OAuth, OAuthBuilder};

use crate::async_call::{CAsyncCallback, execute_async};

/// OAuth 2.0 client — owns the Rust `OAuth` instance (opaque handle)
///
/// Callers must never dereference or inspect the struct layout.
/// Always free with `lb_oauth_free`.
pub struct COAuth {
    pub(crate) inner: OAuth,
}

/// Asynchronously build an OAuth 2.0 client.
///
/// Tries to load an existing token from
/// `~/.longbridge/openapi/tokens/<client_id>`. If the token is missing or
/// expired, starts a local callback server and calls `open_url_callback` so
/// the caller can open the authorization URL in a browser.
///
/// @param client_id          NUL-terminated OAuth 2.0 client ID
/// @param callback_port      Local callback server port; pass 0 to use the
///                           default (60355)
/// @param open_url_callback  Called with the authorization URL and
///                           `open_url_userdata` during the auth flow
/// @param open_url_userdata  Opaque pointer forwarded to `open_url_callback`
/// @param callback           Async completion callback; `data` is
///                           `*mut LbOAuth` on success (free with
///                           `lb_oauth_free`)
/// @param userdata           Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_new(
    client_id: *const c_char,
    callback_port: u16,
    open_url_callback: extern "C" fn(*const c_char, *mut c_void),
    open_url_userdata: *mut c_void,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let client_id = std::ffi::CStr::from_ptr(client_id)
        .to_str()
        .expect("invalid client_id")
        .to_string();
    let open_url_userdata_usize = open_url_userdata as usize;

    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let mut builder = OAuthBuilder::new(client_id);
        if callback_port != 0 {
            builder = builder.callback_port(callback_port);
        }
        let oauth = builder
            .build(move |url| {
                let c_url = std::ffi::CString::new(url).unwrap_or_default();
                open_url_callback(c_url.as_ptr(), open_url_userdata_usize as *mut c_void);
            })
            .await
            .map_err(|e| longbridge::Error::OAuth(e.to_string()))?;
        Ok(Box::into_raw(Box::new(COAuth { inner: oauth })))
    });
}

/// Clone an OAuth 2.0 client object
///
/// Increments the internal Arc reference count; the returned pointer must be
/// freed independently with `lb_oauth_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_clone(oauth: *const COAuth) -> *mut COAuth {
    Box::into_raw(Box::new(COAuth {
        inner: (*oauth).inner.clone(),
    }))
}

/// Free an OAuth 2.0 client object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_free(oauth: *mut COAuth) {
    if !oauth.is_null() {
        drop(Box::from_raw(oauth));
    }
}

/// Clone the inner client; `None` if the handle is NULL (e.g. a moved-from C++
/// wrapper).
unsafe fn clone_inner(oauth: *const COAuth) -> Option<OAuth> {
    oauth.as_ref().map(|oauth| oauth.inner.clone())
}

fn null_oauth_error() -> longbridge::Error {
    longbridge::Error::OAuth("OAuth client is NULL".to_string())
}

/// Asynchronously revoke the current token on the server (RFC 7009)
///
/// The refresh token is revoked when present, otherwise the access token.
/// Only the server is contacted; the cached token on disk is left untouched
/// (use `lb_oauth_clear` to remove it).
///
/// @param oauth     OAuth 2.0 client; if NULL, `callback` receives an error
/// @param callback  Async completion callback; `error` is set on failure
/// @param userdata  Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_revoke(
    oauth: *const COAuth,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let oauth = clone_inner(oauth);
    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        oauth
            .ok_or_else(null_oauth_error)?
            .revoke()
            .await
            .map_err(|e| longbridge::Error::OAuth(e.to_string()))?;
        Ok(())
    });
}

/// Asynchronously clear the token locally
///
/// Drops the in-memory token and deletes the cached token file
/// (`~/.longbridge/openapi/tokens/<client_id>`). The server is not contacted
/// (use `lb_oauth_revoke` for that).
///
/// @param oauth     OAuth 2.0 client; if NULL, `callback` receives an error
/// @param callback  Async completion callback; `error` is set on failure
/// @param userdata  Opaque pointer forwarded to `callback`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_oauth_clear(
    oauth: *const COAuth,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let oauth = clone_inner(oauth);
    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        oauth
            .ok_or_else(null_oauth_error)?
            .clear()
            .await
            .map_err(|e| longbridge::Error::OAuth(e.to_string()))?;
        Ok(())
    });
}
