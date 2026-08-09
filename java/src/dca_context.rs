use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, DCAContext, dca::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, ObjectArray, get_field},
};

struct ContextObj {
    ctx: DCAContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newDcaContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: DCAContext::new(Arc::new((*(config as *const Config)).clone())),
        })) as i64)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeDcaContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let status: Option<DCAStatus> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "status")?
        };
        let symbol: Option<String> = if opts.is_null() {
            None
        } else {
            get_field(env, &opts, "symbol")?
        };
        async_util::execute(env, callback, async move {
            Ok(ctx.ctx.list(status, symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextStats(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let sym: Option<String> = if symbol.is_null() {
            None
        } else {
            Some(FromJValue::from_jvalue(env, symbol.into())?)
        };
        async_util::execute(env, callback, async move { Ok(ctx.ctx.stats(sym).await?) })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextCheckSupport(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let syms: ObjectArray<String> = FromJValue::from_jvalue(env, symbols.into())?;
        async_util::execute(env, callback, async move {
            Ok(ctx.ctx.check_support(syms.0).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextHistory(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let plan_id: String = get_field(env, &opts, "planId")?;
        let page: Option<JavaInteger> = get_field(env, &opts, "page")?;
        let limit: Option<JavaInteger> = get_field(env, &opts, "limit")?;
        async_util::execute(env, callback, async move {
            Ok(ctx
                .ctx
                .history(
                    plan_id,
                    page.map(i32::from).unwrap_or(1),
                    limit.map(i32::from).unwrap_or(20),
                )
                .await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextPause(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    plan_id: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let id: String = FromJValue::from_jvalue(env, plan_id.into())?;
        async_util::execute(env, callback, async move { Ok(ctx.ctx.pause(id).await?) })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextResume(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    plan_id: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let id: String = FromJValue::from_jvalue(env, plan_id.into())?;
        async_util::execute(env, callback, async move { Ok(ctx.ctx.resume(id).await?) })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextStop(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    plan_id: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let id: String = FromJValue::from_jvalue(env, plan_id.into())?;
        async_util::execute(env, callback, async move { Ok(ctx.ctx.stop(id).await?) })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextCalcDate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let frequency: Option<DCAFrequency> = get_field(env, &opts, "frequency")?;
        let frequency = frequency.unwrap_or(DCAFrequency::Monthly);
        let day_of_week: Option<String> = get_field(env, &opts, "dayOfWeek")?;
        let day_of_month_i: Option<JavaInteger> = get_field(env, &opts, "dayOfMonth")?;
        let day_of_month: Option<u32> = day_of_month_i.map(|v| i32::from(v) as u32);
        async_util::execute(env, callback, async move {
            Ok(ctx
                .ctx
                .calc_date(symbol, frequency, day_of_week, day_of_month)
                .await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextSetReminder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    hours: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let h: String = FromJValue::from_jvalue(env, hours.into())?;
        async_util::execute(
            env,
            callback,
            async move { Ok(ctx.ctx.set_reminder(h).await?) },
        )?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextCreate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let amount: String = get_field(env, &opts, "amount")?;
        let freq_v: Option<DCAFrequency> = get_field(env, &opts, "frequency")?;
        let frequency = freq_v.unwrap_or(DCAFrequency::Monthly);
        let day_of_week: Option<String> = get_field(env, &opts, "dayOfWeek")?;
        let day_of_month_s: Option<String> = get_field(env, &opts, "dayOfMonth")?;
        let day_of_month: Option<u32> = day_of_month_s.and_then(|s| s.parse().ok());

        let allow_margin: bool = get_field(env, &opts, "allowMargin")?;
        async_util::execute(env, callback, async move {
            Ok(ctx
                .ctx
                .create(
                    symbol,
                    amount,
                    frequency,
                    day_of_week,
                    day_of_month,
                    allow_margin,
                )
                .await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_dcaContextUpdate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let plan_id: String = get_field(env, &opts, "planId")?;
        let amount: Option<String> = get_field(env, &opts, "amount")?;
        let frequency: Option<DCAFrequency> = get_field(env, &opts, "frequency")?;
        let day_of_week: Option<String> = get_field(env, &opts, "dayOfWeek")?;
        let day_of_month_s: Option<String> = get_field(env, &opts, "dayOfMonth")?;
        let day_of_month: Option<u32> = day_of_month_s.and_then(|s| s.parse().ok());

        let allow_margin: bool = get_field(env, &opts, "allowMargin")?;
        async_util::execute(env, callback, async move {
            Ok(ctx
                .ctx
                .update(
                    plan_id,
                    amount,
                    frequency,
                    day_of_week,
                    day_of_month,
                    Some(allow_margin),
                )
                .await?)
        })?;
        Ok(())
    })
}
