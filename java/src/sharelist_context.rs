use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, SharelistContext};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, ObjectArray, get_field},
};

struct ContextObj {
    ctx: SharelistContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newSharelistContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: SharelistContext::new(Arc::new((*(config as *const Config)).clone())),
        })) as i64)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeSharelistContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    count: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(ctx.ctx.list(count as u32).await?)
        })?;
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    id: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move { Ok(ctx.ctx.detail(id).await?) })?;
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextPopular(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    count: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(ctx.ctx.popular(count as u32).await?)
        })?;
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextCreate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let name: String = get_field(env, &opts, "name")?;
        let description: Option<String> = get_field(env, &opts, "description")?;
        async_util::execute(env, callback, async move {
            Ok(ctx.ctx.create(name, description).await?)
        })?;
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextAddSecurities(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    id: i64,
    symbols: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let syms: ObjectArray<String> = FromJValue::from_jvalue(env, symbols.into())?;
        async_util::execute(env, callback, async move {
            ctx.ctx.add_securities(id, syms.0).await?;
            Ok(())
        })?;
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextRemoveSecurities(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    id: i64,
    symbols: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let syms: ObjectArray<String> = FromJValue::from_jvalue(env, symbols.into())?;
        async_util::execute(env, callback, async move {
            ctx.ctx.remove_securities(id, syms.0).await?;
            Ok(())
        })?;
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextSortSecurities(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    id: i64,
    symbols: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        let syms: ObjectArray<String> = FromJValue::from_jvalue(env, symbols.into())?;
        async_util::execute(env, callback, async move {
            ctx.ctx.sort_securities(id, syms.0).await?;
            Ok(())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_sharelistContextDelete(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    id: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let ctx = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            ctx.ctx.delete(id).await?;
            Ok(())
        })?;
        Ok(())
    })
}
