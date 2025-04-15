use std::{
    collections::HashMap,
    ffi::{CStr, CString, c_char, c_void},
};

use longport::{
    Error,
    httpclient::{HttpClient, HttpClientConfig, HttpClientError},
};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    error::{CError, set_error},
};

/// A HTTP client for LongPort OpenApi
pub struct CHttpClient(HttpClient);

/// Create a HTTP client
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_http_client_new(
    http_url: *const c_char,
    app_key: *const c_char,
    app_secret: *const c_char,
    access_token: *const c_char,
) -> *mut CHttpClient {
    let http_url = CStr::from_ptr(http_url).to_str().expect("invalid http url");
    let app_key = CStr::from_ptr(app_key).to_str().expect("invalid app key");
    let app_secret = CStr::from_ptr(app_secret)
        .to_str()
        .expect("invalid app secret");
    let access_token = CStr::from_ptr(access_token)
        .to_str()
        .expect("invalid access token");

    Box::leak(Box::new(CHttpClient(HttpClient::new(
        HttpClientConfig::new(app_key, app_secret, access_token).http_url(http_url),
    ))))
}

/// Free the http client
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_http_client_free(http_client: *mut CHttpClient) {
    let _ = Box::from_raw(http_client);
}

/// Create a new `HttpClient` from the given environment variables
///
/// It first gets the environment variables from the `.env` file in the
/// current directory.
///
/// # Variables
///
/// - `LONGPORT_HTTP_URL` - HTTP endpoint url
/// - `LONGPORT_APP_KEY` - App key
/// - `LONGPORT_APP_SECRET` - App secret
/// - `LONGPORT_ACCESS_TOKEN` - Access token
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_http_client_from_env(error: *mut *mut CError) -> *mut CHttpClient {
    match HttpClient::from_env() {
        Ok(http_client) => {
            set_error(error, None);
            Box::into_raw(Box::new(CHttpClient(http_client)))
        }
        Err(err) => {
            set_error(error, Some(Error::HttpClient(err)));
            std::ptr::null_mut()
        }
    }
}

pub struct CHttpResult {
    response_body: CString,
}

/// HTTP Header
#[repr(C)]
pub struct CHeader {
    pub name: *const c_char,
    pub value: *const c_char,
}

/// Performs a HTTP request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_http_client_request(
    http_client: *mut CHttpClient,
    method: *const c_char,
    path: *const c_char,
    headers: *const CHeader,
    request_body: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let http_client = &mut (*http_client).0;
    let method = CStr::from_ptr(method)
        .to_str()
        .expect("invalid method")
        .to_string();
    let path = CStr::from_ptr(path)
        .to_str()
        .expect("invalid path")
        .to_string();
    let request_body = if !request_body.is_null() {
        Some(
            CStr::from_ptr(request_body)
                .to_str()
                .expect("invalid request body")
                .to_string(),
        )
    } else {
        None
    };
    let mut r_headers = HashMap::new();
    if !headers.is_null() {
        while !(*headers).name.is_null() {
            let name = CStr::from_ptr((*headers).name)
                .to_str()
                .expect("invalid header name")
                .to_string();
            let value = CStr::from_ptr((*headers).value)
                .to_str()
                .expect("invalid header name")
                .to_string();
            r_headers.insert(name, value);
        }
    }

    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let r = http_client.request(
            method
                .to_uppercase()
                .parse()
                .map_err(|_| Error::HttpClient(HttpClientError::InvalidRequestMethod))?,
            path,
        );
        let r = r_headers
            .into_iter()
            .fold(r, |acc, (name, value)| acc.header(name, value));

        let response_body = match request_body {
            Some(request_body) => r.body(request_body).response::<String>().send().await,
            None => r.response::<String>().send().await,
        }?;
        Ok(Box::into_raw(Box::new(CHttpResult {
            response_body: CString::from_vec_unchecked(response_body.into_bytes()),
        })))
    });
}

/// Free the HTTP result
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_http_result_free(http_result: *mut CHttpResult) {
    let _ = Box::from_raw(http_result);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_http_result_response_body(
    http_result: *const CHttpResult,
) -> *const c_char {
    (*http_result).response_body.as_ptr() as *const _
}
