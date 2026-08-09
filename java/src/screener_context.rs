use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
};
use longbridge::{Config, ScreenerContext};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, get_field},
};

struct ContextObj {
    ctx: ScreenerContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newScreenerContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0, |_env| {
        let config = crate::handles::get::<Config>(config)?;
        let ctx = ScreenerContext::new(config);
        Ok(crate::handles::insert(ContextObj { ctx }))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeScreenerContext(
    _env: JNIEnv,
    _class: JClass,
    context: i64,
) {
    crate::handles::remove(context);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_screenerContextRecommendStrategies(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    market: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = crate::handles::get::<ContextObj>(context)?;
        let market: String = FromJValue::from_jvalue(env, market.into())?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_recommend_strategies(market).await?;
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
    market: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = crate::handles::get::<ContextObj>(context)?;
        let market: String = FromJValue::from_jvalue(env, market.into())?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_user_strategies(market).await?;
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
        let context = crate::handles::get::<ContextObj>(context)?;
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
        let context = crate::handles::get::<ContextObj>(context)?;
        let market: String = get_field(env, &opts, "market")?;
        let strategy_id: Option<i64> = get_field(env, &opts, "strategyId")?;
        let page_opt: Option<JavaInteger> = get_field(env, &opts, "page")?;
        let page = page_opt.map(i32::from).unwrap_or(1) as u32;
        let size_opt: Option<JavaInteger> = get_field(env, &opts, "size")?;
        let size = size_opt.map(i32::from).unwrap_or(20) as u32;
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .screener_search(market, strategy_id, vec![], vec![], page, size)
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
        let context = crate::handles::get::<ContextObj>(context)?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.screener_indicators().await?;
            Ok(resp)
        })?;
        Ok(())
    })
}
