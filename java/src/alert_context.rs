use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{AlertContext, Config, alert::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, ObjectArray, get_field},
};

struct ContextObj {
    ctx: AlertContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newAlertContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: AlertContext::new(config),
        })) as i64)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeAlertContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_alertContextList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move { Ok(context.ctx.list().await?) })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_alertContextAdd(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let condition_val: Option<JavaInteger> = get_field(env, &opts, "condition")?;
        let condition = match condition_val.map(i32::from).unwrap_or(1) {
            2 => AlertCondition::PriceFall,
            3 => AlertCondition::PercentRise,
            4 => AlertCondition::PercentFall,
            _ => AlertCondition::PriceRise,
        };
        let trigger_value: String = get_field(env, &opts, "triggerValue")?;
        let freq_val: Option<JavaInteger> = get_field(env, &opts, "frequency")?;
        let frequency = match freq_val.map(i32::from).unwrap_or(3) {
            1 => AlertFrequency::Daily,
            2 => AlertFrequency::EveryTime,
            _ => AlertFrequency::Once,
        };
        async_util::execute(env, callback, async move {
            context
                .ctx
                .add(symbol, condition, trigger_value, frequency)
                .await?;
            Ok(())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_alertContextEnable(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    alert_id: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let id: String = FromJValue::from_jvalue(env, alert_id.into())?;
        async_util::execute(env, callback, async move {
            context.ctx.enable(id).await?;
            Ok(())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_alertContextDisable(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    alert_id: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let id: String = FromJValue::from_jvalue(env, alert_id.into())?;
        async_util::execute(env, callback, async move {
            context.ctx.disable(id).await?;
            Ok(())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_alertContextDelete(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let ids_raw: ObjectArray<String> = get_field(env, &opts, "ids")?;
        let ids: Vec<String> = ids_raw.0;
        async_util::execute(env, callback, async move {
            context.ctx.delete(ids).await?;
            Ok(())
        })?;
        Ok(())
    })
}
