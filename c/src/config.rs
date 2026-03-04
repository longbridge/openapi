use std::{ffi::CStr, os::raw::c_char, ptr};

use longbridge::Config;

use crate::{
    error::{CError, set_error},
    oauth::COAuth,
    types::{CLanguage, CPushCandlestickMode},
};

/// Configuration options for Longbridge SDK
pub struct CConfig(pub(crate) Config);

/// Create a new `Config` using API Key authentication
///
/// Optional environment variables are read automatically:
/// `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
/// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
/// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
/// `LONGBRIDGE_LOG_PATH`.  Use the corresponding `lb_config_set_*` functions
/// to override any of these values after construction.
///
/// @param app_key       App key
/// @param app_secret    App secret
/// @param access_token  Access token
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_from_apikey(
    app_key: *const c_char,
    app_secret: *const c_char,
    access_token: *const c_char,
) -> *mut CConfig {
    let app_key = CStr::from_ptr(app_key).to_str().expect("invalid app key");
    let app_secret = CStr::from_ptr(app_secret)
        .to_str()
        .expect("invalid app secret");
    let access_token = CStr::from_ptr(access_token)
        .to_str()
        .expect("invalid access token");
    let config = Config::from_apikey(app_key, app_secret, access_token);
    Box::into_raw(Box::new(CConfig(config)))
}

/// Create a new `Config` from environment variables (API Key mode)
///
/// It first reads the `.env` file in the current directory.
///
/// Variables: `LONGBRIDGE_APP_KEY`, `LONGBRIDGE_APP_SECRET`,
/// `LONGBRIDGE_ACCESS_TOKEN`, `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_QUOTE_WS_URL`,
/// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_LANGUAGE`,
/// `LONGBRIDGE_ENABLE_OVERNIGHT`, `LONGBRIDGE_PUSH_CANDLESTICK_MODE`,
/// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`, `LONGBRIDGE_LOG_PATH`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_from_apikey_env(error: *mut *mut CError) -> *mut CConfig {
    match Config::from_apikey_env() {
        Ok(config) => {
            set_error(error, None);
            Box::into_raw(Box::new(CConfig(config)))
        }
        Err(err) => {
            set_error(error, Some(err));
            ptr::null_mut()
        }
    }
}

/// Create a new `Config` for OAuth 2.0 authentication
///
/// Optional environment variables are read automatically:
/// `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
/// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
/// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
/// `LONGBRIDGE_LOG_PATH`.  Use the corresponding `lb_config_set_*` functions
/// to override any of these values after construction.
///
/// Does **not** take ownership of `oauth`. The caller must free `oauth` with
/// `lb_oauth_free` after this call returns.
///
/// @param oauth  OAuth 2.0 client obtained from `lb_oauth_new`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_from_oauth(oauth: *const COAuth) -> *mut CConfig {
    let config = Config::from_oauth((*oauth).inner.clone());
    Box::into_raw(Box::new(CConfig(config)))
}

// ── Config setters
// ────────────────────────────────────────────────────────────
//
// Each setter obtains a `&mut Config` via `&mut (*config).0` and calls the
// corresponding `set_*` method.  No ownership transfer, no ptr::read/write.

/// Set the HTTP endpoint URL
///
/// @param config   Config object
/// @param http_url HTTP endpoint URL (e.g. `https://openapi.longbridge.com`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_set_http_url(config: *mut CConfig, http_url: *const c_char) {
    let url = CStr::from_ptr(http_url).to_str().expect("invalid http_url");
    (*config).0.set_http_url(url);
}

/// Set the Quote WebSocket endpoint URL
///
/// @param config        Config object
/// @param quote_ws_url  Quote WebSocket URL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_set_quote_ws_url(
    config: *mut CConfig,
    quote_ws_url: *const c_char,
) {
    let url = CStr::from_ptr(quote_ws_url)
        .to_str()
        .expect("invalid quote_ws_url");
    (*config).0.set_quote_ws_url(url);
}

/// Set the Trade WebSocket endpoint URL
///
/// @param config        Config object
/// @param trade_ws_url  Trade WebSocket URL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_set_trade_ws_url(
    config: *mut CConfig,
    trade_ws_url: *const c_char,
) {
    let url = CStr::from_ptr(trade_ws_url)
        .to_str()
        .expect("invalid trade_ws_url");
    (*config).0.set_trade_ws_url(url);
}

/// Set the language identifier
///
/// @param config    Config object
/// @param language  Language identifier
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_set_language(config: *mut CConfig, language: CLanguage) {
    (*config).0.set_language(language.into());
}

/// Enable overnight quote
///
/// @param config  Config object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_enable_overnight(config: *mut CConfig) {
    (*config).0.set_enable_overnight();
}

/// Set the push candlestick mode
///
/// @param config  Config object
/// @param mode    Push candlestick mode
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_set_push_candlestick_mode(
    config: *mut CConfig,
    mode: CPushCandlestickMode,
) {
    (*config).0.set_push_candlestick_mode(mode.into());
}

/// Disable printing of quote packages on connection
///
/// @param config  Config object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_disable_print_quote_packages(config: *mut CConfig) {
    (*config).0.set_dont_print_quote_packages();
}

/// Set the log file path
///
/// @param config    Config object
/// @param log_path  Path for log files
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_set_log_path(config: *mut CConfig, log_path: *const c_char) {
    let path = CStr::from_ptr(log_path).to_str().expect("invalid log_path");
    (*config).0.set_log_path(path);
}

/// Free the config object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_free(config: *mut CConfig) {
    let _ = Box::from_raw(config);
}
