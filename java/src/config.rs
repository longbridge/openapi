use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::{jboolean, jlong},
};
use longport::{Config, Language, PushCandlestickMode};
use time::OffsetDateTime;

use crate::{async_util, error::jni_result, types::FromJValue};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longport_SdkNative_newConfig(
    mut env: JNIEnv,
    _class: JClass,
    app_key: JString,
    app_secret: JString,
    access_token: JString,
    http_url: JString,
    quote_ws_url: JString,
    trade_ws_url: JString,
    language: JObject,
    enable_overnight: jboolean,
    push_candlestick_mode: JObject,
    enable_print_quote_packages: jboolean,
    log_path: JString,
) -> jlong {
    jni_result(&mut env, 0, |env| {
        let app_key = String::from_jvalue(env, app_key.into())?;
        let app_secret = String::from_jvalue(env, app_secret.into())?;
        let access_token = String::from_jvalue(env, access_token.into())?;
        let http_url = <Option<String>>::from_jvalue(env, http_url.into())?;
        let quote_ws_url = <Option<String>>::from_jvalue(env, quote_ws_url.into())?;
        let trade_ws_url = <Option<String>>::from_jvalue(env, trade_ws_url.into())?;
        let language = <Option<Language>>::from_jvalue(env, language.into())?;
        let push_candlestick_mode =
            <Option<PushCandlestickMode>>::from_jvalue(env, push_candlestick_mode.into())?;
        let log_path = <Option<String>>::from_jvalue(env, log_path.into())?;

        let mut config = Config::new(app_key, app_secret, access_token);

        if let Some(http_url) = http_url {
            config = config.http_url(http_url);
        }
        if let Some(quote_ws_url) = quote_ws_url {
            config = config.quote_ws_url(quote_ws_url);
        }
        if let Some(trade_ws_url) = trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }
        if let Some(language) = language {
            config = config.language(language);
        }
        if enable_overnight > 0 {
            config = config.enable_overnight();
        }
        if let Some(mode) = push_candlestick_mode {
            config = config.push_candlestick_mode(mode);
        }
        if enable_print_quote_packages == 0 {
            config = config.dont_print_quote_packages();
        }
        if let Some(log_path) = log_path {
            config = config.log_path(log_path);
        }

        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longport_SdkNative_newConfigFromEnv(
    mut env: JNIEnv,
    _class: JClass,
) -> jlong {
    jni_result(&mut env, 0, |_env| {
        let config = Config::from_env()?;
        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_freeConfig(
    _env: JNIEnv,
    _class: JClass,
    config: jlong,
) {
    let _ = Box::from_raw(config as *mut Config);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_configRefreshAccessToken(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
    expired_at: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let config = &*(config as *const Config);
        let expired_at: Option<OffsetDateTime> = FromJValue::from_jvalue(env, expired_at.into())?;
        async_util::execute(env, callback, async move {
            let token = config.refresh_access_token(expired_at).await?;
            Ok(token)
        })?;
        Ok(())
    })
}
