use std::{
    ffi::{c_void, CStr},
    os::raw::c_char,
    sync::Arc,
};

use longbridge::Config;
use time::OffsetDateTime;

use crate::{
    async_call::{execute_async, CAsyncCallback},
    error::{set_error, CError},
    types::{CLanguage, CString},
};

/// Configuration options for Longbridge sdk
pub struct CConfig(pub(crate) Arc<Config>);

/// Create a new `Config` from the given environment variables
///
/// It first gets the environment variables from the `.env` file in the
/// current directory.
///
/// # Variables
///
/// - `LONGBRIDGE_APP_KEY` - App key
/// - `LONGBRIDGE_APP_SECRET` - App secret
/// - `LONGBRIDGE_ACCESS_TOKEN` - Access token
/// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longbridgeapp.com`)
/// - `LONGBRIDGE_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
///   `wss://openapi-quote.longbridgeapp.com`)
/// - `LONGBRIDGE_TRADE_WS_URL` - Trade websocket endpoint url (Default:
///   `wss://openapi-trade.longbridgeapp.com`)
#[no_mangle]
pub unsafe extern "C" fn lb_config_from_env(error: *mut *mut CError) -> *mut CConfig {
    match Config::from_env() {
        Ok(config) => {
            set_error(error, None);
            Box::into_raw(Box::new(CConfig(Arc::new(config))))
        }
        Err(err) => {
            set_error(error, Some(err));
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lb_config_new(
    app_key: *const c_char,
    app_secret: *const c_char,
    access_token: *const c_char,
    http_url: *const c_char,
    quote_ws_url: *const c_char,
    trade_ws_url: *const c_char,
    language: *const CLanguage,
) -> *mut CConfig {
    let app_key = CStr::from_ptr(app_key).to_str().expect("invalid app key");
    let app_secret = CStr::from_ptr(app_secret)
        .to_str()
        .expect("invalid app secret");
    let access_token = CStr::from_ptr(access_token)
        .to_str()
        .expect("invalid access token");
    let mut config = Config::new(app_key, app_secret, access_token);

    if !http_url.is_null() {
        config = config.http_url(CStr::from_ptr(http_url).to_str().expect("invalid http url"));
    }
    if !quote_ws_url.is_null() {
        config = config.quote_ws_url(
            CStr::from_ptr(quote_ws_url)
                .to_str()
                .expect("invalid quote websocket url"),
        );
    }
    if !trade_ws_url.is_null() {
        config = config.trade_ws_url(
            CStr::from_ptr(trade_ws_url)
                .to_str()
                .expect("invalid trade websocket url"),
        );
    }
    if !language.is_null() {
        config = config.language((*language).into());
    }

    Box::into_raw(Box::new(CConfig(Arc::new(config))))
}

/// Free the config object
#[no_mangle]
pub unsafe extern "C" fn lb_config_free(config: *mut CConfig) {
    let _ = Box::from_raw(config);
}

/// Gets a new `access_token`
#[no_mangle]
pub unsafe extern "C" fn lb_config_refresh_access_token(
    config: *mut CConfig,
    expired_at: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let config = &mut (*config).0;
    execute_async::<c_void, _, _>(callback, std::ptr::null(), userdata, async move {
        let token: CString = config
            .refresh_access_token(if expired_at == 0 {
                None
            } else {
                Some(OffsetDateTime::from_unix_timestamp(expired_at).unwrap())
            })
            .await?
            .into();
        Ok(token)
    });
}
