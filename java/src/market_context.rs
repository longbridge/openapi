use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, MarketContext, market::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, get_field},
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
        let period_val: Option<JavaInteger> = get_field(env, &opts, "period")?;
        let period = match period_val.map(i32::from).unwrap_or(0) {
            1 => BrokerHoldingPeriod::Rct5,
            2 => BrokerHoldingPeriod::Rct20,
            3 => BrokerHoldingPeriod::Rct60,
            _ => BrokerHoldingPeriod::Rct1,
        };
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
        let period_val: Option<JavaInteger> = get_field(env, &opts, "period")?;
        let period = match period_val.map(i32::from).unwrap_or(5) {
            0 => AhPremiumPeriod::Min1,
            1 => AhPremiumPeriod::Min5,
            2 => AhPremiumPeriod::Min15,
            3 => AhPremiumPeriod::Min30,
            4 => AhPremiumPeriod::Min60,
            6 => AhPremiumPeriod::Week,
            7 => AhPremiumPeriod::Month,
            8 => AhPremiumPeriod::Year,
            _ => AhPremiumPeriod::Day,
        };
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
