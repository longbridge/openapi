use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{AlertContext, Config, alert::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{ObjectArray, get_field},
};

/// Read a Java `AlertItem` object into a Rust [`longbridge::alert::AlertItem`].
fn read_alert_item(
    env: &mut JNIEnv,
    item: &JObject,
) -> jni::errors::Result<longbridge::alert::AlertItem> {
    let id: String = get_field(env, item, "id")?;
    let indicator_id: String = get_field(env, item, "indicatorId")?;
    let enabled: bool = get_field(env, item, "enabled")?;
    let frequency: i32 = get_field(env, item, "frequency")?;
    let scope: i32 = get_field(env, item, "scope")?;
    let text: String = get_field(env, item, "text")?;
    // state: int[] — read as a Java int array
    let state = unsafe {
        let state_obj = env.get_field(item, "state", "[I")?.l()?;
        if state_obj.is_null() {
            Vec::new()
        } else {
            let arr = jni::objects::JIntArray::from(state_obj);
            let elements = env
                .get_array_elements::<jni::sys::jint>(&arr, jni::objects::ReleaseMode::CopyBack)?;
            std::slice::from_raw_parts(elements.as_ptr(), elements.len()).to_vec()
        }
    };
    // valueMap: JSON string
    let value_map_str: String = get_field(env, item, "valueMap")?;
    let value_map = serde_json::from_str(&value_map_str).unwrap_or(serde_json::Value::Null);
    Ok(longbridge::alert::AlertItem {
        id,
        indicator_id,
        enabled,
        frequency,
        scope,
        text,
        state,
        value_map,
    })
}

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
        let condition: Option<AlertCondition> = get_field(env, &opts, "condition")?;
        let condition = condition.unwrap_or(AlertCondition::PriceRise);
        let trigger_value: String = get_field(env, &opts, "triggerValue")?;
        let frequency: Option<AlertFrequency> = get_field(env, &opts, "frequency")?;
        let frequency = frequency.unwrap_or(AlertFrequency::Once);
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
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_alertContextUpdate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    item: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let alert_item = read_alert_item(env, &item)?;
        async_util::execute(env, callback, async move {
            context.ctx.update(&alert_item).await?;
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
