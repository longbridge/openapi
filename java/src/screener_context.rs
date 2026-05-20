use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, ScreenerContext};

use crate::{
    async_util,
    error::jni_result,
    types::{JavaInteger, get_field},
};

struct ContextObj {
    ctx: ScreenerContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newScreenerContext(
    _env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    let config = &*(config as *const Config);
    let ctx = ScreenerContext::new(Arc::new(config.clone()));
    Box::into_raw(Box::new(ContextObj { ctx })) as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeScreenerContext(
    _env: JNIEnv,
    _class: JClass,
    context: i64,
) {
    let _ = Box::from_raw(context as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_screenerContextRecommendStrategies(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_recommend_strategies().await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_screenerContextUserStrategies(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_user_strategies().await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_screenerContextStrategy(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    id: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_strategy(id).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_screenerContextSearch(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: String = get_field(env, &opts, "market")?;
        let strategy_id: Option<i64> = get_field(env, &opts, "strategyId")?;
        let page_opt: Option<JavaInteger> = get_field(env, &opts, "page")?;
        let page = page_opt.map(i32::from).unwrap_or(1) as u32;
        let size_opt: Option<JavaInteger> = get_field(env, &opts, "size")?;
        let size = size_opt.map(i32::from).unwrap_or(20) as u32;
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .screener_search(market, strategy_id, page, size)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_screenerContextIndicators(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_indicators().await?;
            Ok(resp)
        })?;
        Ok(())
    })
}
