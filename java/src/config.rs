use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::{jboolean, jlong},
};
use longbridge::{Config, Language, PushCandlestickMode};

use crate::{error::jni_result, types::FromJValue};

// ── Constructors
// ──────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longbridge_SdkNative_newConfigFromApikey(
    mut env: JNIEnv,
    _class: JClass,
    app_key: JString,
    app_secret: JString,
    access_token: JString,
) -> jlong {
    jni_result(&mut env, 0, |env| {
        let app_key = String::from_jvalue(env, app_key.into())?;
        let app_secret = String::from_jvalue(env, app_secret.into())?;
        let access_token = String::from_jvalue(env, access_token.into())?;
        let config = Config::from_apikey(app_key, app_secret, access_token);
        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longbridge_SdkNative_newConfigFromApikeyEnv(
    mut env: JNIEnv,
    _class: JClass,
) -> jlong {
    jni_result(&mut env, 0, |_env| {
        let config = Config::from_apikey_env()?;
        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newConfigFromOauth(
    mut env: JNIEnv,
    _class: JClass,
    oauth: jlong,
) -> jlong {
    jni_result(&mut env, 0, |_env| {
        let oauth = &*(oauth as *const longbridge::oauth::OAuth);
        let config = Config::from_oauth(oauth.clone());
        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

// ── Setters
// ───────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetHttpUrl(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    http_url: JString,
) -> jlong {
    jni_result(&mut env, config, |env| {
        let url = String::from_jvalue(env, http_url.into())?;
        (*(config as *mut Config)).set_http_url(url);
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetQuoteWsUrl(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    quote_ws_url: JString,
) -> jlong {
    jni_result(&mut env, config, |env| {
        let url = String::from_jvalue(env, quote_ws_url.into())?;
        (*(config as *mut Config)).set_quote_ws_url(url);
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetTradeWsUrl(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    trade_ws_url: JString,
) -> jlong {
    jni_result(&mut env, config, |env| {
        let url = String::from_jvalue(env, trade_ws_url.into())?;
        (*(config as *mut Config)).set_trade_ws_url(url);
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetLanguage(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    language: JObject,
) -> jlong {
    jni_result(&mut env, config, |env| {
        let lang = Language::from_jvalue(env, language.into())?;
        (*(config as *mut Config)).set_language(lang);
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetEnableOvernight(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
) -> jlong {
    jni_result(&mut env, config, |_env| {
        (*(config as *mut Config)).set_enable_overnight();
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetPushCandlestickMode(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    mode: JObject,
) -> jlong {
    jni_result(&mut env, config, |env| {
        let mode = PushCandlestickMode::from_jvalue(env, mode.into())?;
        (*(config as *mut Config)).set_push_candlestick_mode(mode);
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetEnablePrintQuotePackages(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    enable: jboolean,
) -> jlong {
    jni_result(&mut env, config, |_env| {
        if enable == 0 {
            (*(config as *mut Config)).set_dont_print_quote_packages();
        }
        Ok(config)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_configSetLogPath(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    log_path: JString,
) -> jlong {
    jni_result(&mut env, config, |env| {
        let path = String::from_jvalue(env, log_path.into())?;
        (*(config as *mut Config)).set_log_path(path);
        Ok(config)
    })
}

// ── Free ──────────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeConfig(
    _env: JNIEnv,
    _class: JClass,
    config: jlong,
) {
    let _ = Box::from_raw(config as *mut Config);
}
