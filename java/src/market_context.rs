use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, MarketContext, market::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, ObjectArray, get_field},
};

struct ContextObj {
    ctx: MarketContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newMarketContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = MarketContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeMarketContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextMarketStatus(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.market_status().await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextBrokerHolding(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let period: Option<BrokerHoldingPeriod> = get_field(env, &opts, "period")?;
        let period = period.unwrap_or(BrokerHoldingPeriod::Rct1);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.broker_holding(symbol, period).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextBrokerHoldingDaily(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let broker_id: String = get_field(env, &opts, "brokerId")?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.broker_holding_daily(symbol, broker_id).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextAhPremium(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let period: Option<AhPremiumPeriod> = get_field(env, &opts, "period")?;
        let period = period.unwrap_or(AhPremiumPeriod::Day);
        let count_val: Option<JavaInteger> = get_field(env, &opts, "count")?;
        let count = count_val.map(i32::from).unwrap_or(100) as u32;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.ah_premium(symbol, period, count).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

macro_rules! symbol_method {
    ($jni_name:ident, $method:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn $jni_name(
            mut env: JNIEnv,
            _class: JClass,
            context: i64,
            symbol: JObject,
            callback: JObject,
        ) {
            jni_result(&mut env, (), |env| {
                let context = &*(context as *const ContextObj);
                let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
                async_util::execute(env, callback, async move {
                    let resp = context.ctx.$method(symbol).await?;
                    Ok(resp)
                })?;
                Ok(())
            })
        }
    };
}

macro_rules! market_method {
    ($jni_name:ident, $method:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn $jni_name(
            mut env: JNIEnv,
            _class: JClass,
            context: i64,
            market: JObject,
            callback: JObject,
        ) {
            jni_result(&mut env, (), |env| {
                let context = &*(context as *const ContextObj);
                let market: String = FromJValue::from_jvalue(env, market.into())?;
                async_util::execute(env, callback, async move {
                    let resp = context.ctx.$method(market).await?;
                    Ok(resp)
                })?;
                Ok(())
            })
        }
    };
}

symbol_method!(
    Java_com_longbridge_SdkNative_marketContextBrokerHoldingDetail,
    broker_holding_detail
);
symbol_method!(
    Java_com_longbridge_SdkNative_marketContextAhPremiumIntraday,
    ah_premium_intraday
);
symbol_method!(
    Java_com_longbridge_SdkNative_marketContextTradeStats,
    trade_stats
);
symbol_method!(
    Java_com_longbridge_SdkNative_marketContextConstituent,
    constituent
);
market_method!(Java_com_longbridge_SdkNative_marketContextAnomaly, anomaly);

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextTopMovers(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let markets_raw: ObjectArray<String> = get_field(env, &opts, "markets")?;
        let markets: Vec<String> = markets_raw.0;
        let sort_opt: Option<JavaInteger> = get_field(env, &opts, "sort")?;
        let sort = sort_opt.map(i32::from).unwrap_or(0) as u32;
        let date: Option<String> = get_field(env, &opts, "date")?;
        let limit_opt: Option<JavaInteger> = get_field(env, &opts, "limit")?;
        let limit = limit_opt.map(i32::from).unwrap_or(20) as u32;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.top_movers(markets, sort, date, limit).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextRankCategories(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.rank_categories().await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_marketContextRankList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    key: JObject,
    need_article: bool,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let key: String = FromJValue::from_jvalue(env, key.into())?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.rank_list(key, need_article).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}
