use std::sync::Arc;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JClass, JObject, JValueOwned},
};
use longbridge::{Config, content::ContentContext};

use crate::{
    async_util,
    error::jni_result,
    init::CONTENT_CONTEXT_CLASS,
    types::{FromJValue, IntoJValue, ObjectArray, set_field},
};

struct ContextObj {
    ctx: ContentContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newContentContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
    callback: JObject,
) {
    struct ContextObjRef(i64);

    impl IntoJValue for ContextObjRef {
        fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
            let ctx_obj = env.new_object(CONTENT_CONTEXT_CLASS.get().unwrap(), "()V", &[])?;
            set_field(env, &ctx_obj, "raw", self.0)?;
            Ok(JValueOwned::from(ctx_obj))
        }
    }

    jni_result(&mut env, (), |env| {
        let config = Arc::new((*(config as *const Config)).clone());

        async_util::execute(env, callback, async move {
            let ctx = ContentContext::try_new(config)?;
            Ok(ContextObjRef(
                Box::into_raw(Box::new(ContextObj { ctx })) as i64
            ))
        })?;

        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeContentContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_contentContextTopics(
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
            Ok(ObjectArray(context.ctx.topics(symbol).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_contentContextNews(
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
            Ok(ObjectArray(context.ctx.news(symbol).await?))
        })?;
        Ok(())
    })
}
