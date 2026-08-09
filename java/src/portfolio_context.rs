use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, PortfolioContext};

use crate::{
    async_util,
    error::jni_result,
    types::{JavaInteger, get_field},
};

struct ContextObj {
    ctx: PortfolioContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newPortfolioContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = PortfolioContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freePortfolioContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_portfolioContextExchangeRate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(context.ctx.exchange_rate().await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_portfolioContextProfitAnalysis(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let start: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "start")?
        };
        let end: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "end")?
        };
        async_util::execute(env, callback, async move {
            Ok(context.ctx.profit_analysis(start, end).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_portfolioContextProfitAnalysisDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let start: Option<String> = get_field(env, &opts, "start")?;
        let end: Option<String> = get_field(env, &opts, "end")?;
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .profit_analysis_detail(symbol, start, end)
                .await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_portfolioContextProfitAnalysisByMarket(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "market")?
        };
        let start: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "start")?
        };
        let end: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "end")?
        };
        let currency: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "currency")?
        };
        let page_v: Option<JavaInteger> = get_field(env, &opts, "page")?;
        let page: u32 = page_v.map(|v| i32::from(v) as u32).unwrap_or(1);
        let size_v: Option<JavaInteger> = get_field(env, &opts, "size")?;
        let size: u32 = size_v.map(|v| i32::from(v) as u32).unwrap_or(20);
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .profit_analysis_by_market(market, start, end, currency, page, size)
                .await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_portfolioContextProfitAnalysisFlows(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let page_v: Option<JavaInteger> = get_field(env, &opts, "page")?;
        let page: u32 = page_v.map(|v| i32::from(v) as u32).unwrap_or(1);
        let size_v: Option<JavaInteger> = get_field(env, &opts, "size")?;
        let size: u32 = size_v.map(|v| i32::from(v) as u32).unwrap_or(20);
        let include_outside_rth: bool = get_field(env, &opts, "includeOutsideRth")?;
        let start: Option<String> = get_field(env, &opts, "start")?;
        let end: Option<String> = get_field(env, &opts, "end")?;
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .profit_analysis_flows(symbol, page, size, include_outside_rth, start, end)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}
