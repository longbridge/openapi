use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{
    Config,
    content::{ContentContext, CreateTopicOptions, MyTopicsOptions},
};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, ObjectArray, get_field},
};

struct ContextObj {
    ctx: ContentContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newContentContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = ContentContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
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
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_contentContextMyTopics(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let page: Option<JavaInteger> = get_field(env, &opts, "page")?;
        let size: Option<JavaInteger> = get_field(env, &opts, "size")?;
        let topic_type: Option<String> = get_field(env, &opts, "topicType")?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .my_topics(MyTopicsOptions {
                        page: page.map(i32::from),
                        size: size.map(i32::from),
                        topic_type,
                    })
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_contentContextCreateTopic(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let title: String = get_field(env, &opts, "title")?;
        let body: String = get_field(env, &opts, "body")?;
        let topic_type: Option<String> = get_field(env, &opts, "topicType")?;
        let tickers: Option<ObjectArray<String>> = get_field(env, &opts, "tickers")?;
        let hashtags: Option<ObjectArray<String>> = get_field(env, &opts, "hashtags")?;
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .create_topic(CreateTopicOptions {
                    title,
                    body,
                    topic_type,
                    tickers: tickers.map(|a| a.0),
                    hashtags: hashtags.map(|a| a.0),
                })
                .await?)
        })?;
        Ok(())
    })
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
