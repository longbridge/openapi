use std::{
    ffi::{CStr, c_void},
    os::raw::c_char,
    sync::Arc,
};

use longport::Config;
use time::OffsetDateTime;

use crate::{
    async_call::{CAsyncCallback, execute_async},
    error::{CError, set_error},
    types::{CLanguage, CPushCandlestickMode, CString},
};

/// Configuration options for LongPort sdk
pub struct CConfig(pub(crate) Arc<Config>);

/// Create a new `Config` from the given environment variables
///
/// It first gets the environment variables from the `.env` file in the
/// current directory.
///
/// # Variables
///
/// - `LONGPORT_APP_KEY` - App key
/// - `LONGPORT_APP_SECRET` - App secret
/// - `LONGPORT_ACCESS_TOKEN` - Access token
/// - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longportapp.com`)
/// - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
///   `wss://openapi-quote.longportapp.com/v2`)
/// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
///   `wss://openapi-trade.longportapp.com/v2`)
/// - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or `false`
///   (Default: `false`)
/// - `LONGPORT_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed` (Default:
///   `realtime`)
/// - `LONGPORT_PRINT_QUOTE_PACKAGES` - Print quote packages when connected,
///   `true` or `false` (Default: `true`)
/// - `LONGPORT_LOG_PATH` - Set the path of the log files (Default: `no logs`)
#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_new(
    app_key: *const c_char,
    app_secret: *const c_char,
    access_token: *const c_char,
    http_url: *const c_char,
    quote_ws_url: *const c_char,
    trade_ws_url: *const c_char,
    language: *const CLanguage,
    enable_overight: bool,
    push_candlestick_mode: *const CPushCandlestickMode,
    enable_print_quote_packages: bool,
    log_path: *const c_char,
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

    if enable_overight {
        config = config.enable_overnight();
    }

    if !push_candlestick_mode.is_null() {
        config = config.push_candlestick_mode((*push_candlestick_mode).into());
    }

    if !enable_print_quote_packages {
        config = config.dont_print_quote_packages();
    }

    if !log_path.is_null() {
        config = config.log_path(CStr::from_ptr(log_path).to_str().expect("invalid log path"));
    }

    Box::into_raw(Box::new(CConfig(Arc::new(config))))
}

/// Free the config object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_config_free(config: *mut CConfig) {
    let _ = Box::from_raw(config);
}

/// Gets a new `access_token`
#[unsafe(no_mangle)]
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
