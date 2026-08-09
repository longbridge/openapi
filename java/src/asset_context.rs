use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{
    Config,
    asset::{AssetContext, GetStatementListOptions, GetStatementOptions, StatementType},
};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, get_field},
};

struct ContextObj {
    ctx: AssetContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newAssetContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = AssetContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeAssetContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextStatements(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let statement_type: Option<JavaInteger> = get_field(env, &opts, "statementType")?;
        let start_date: Option<JavaInteger> = get_field(env, &opts, "startDate")?;
        let limit: Option<JavaInteger> = get_field(env, &opts, "limit")?;

        let st = match statement_type.map(i32::from).unwrap_or(1) {
            2 => StatementType::Monthly,
            _ => StatementType::Daily,
        };
        let mut options = GetStatementListOptions::new(st);
        if let Some(sd) = start_date {
            options = options.page(i32::from(sd));
        }
        if let Some(l) = limit {
            options = options.page_size(i32::from(l));
        }

        async_util::execute(env, callback, async move {
            let resp = context.ctx.statements(options).await?;
            Ok(serde_json::to_string(&resp).unwrap_or_default())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextDownloadUrl(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    file_key: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let file_key: String = FromJValue::from_jvalue(env, file_key.into())?;
        let options = GetStatementOptions::new(file_key);

        async_util::execute(env, callback, async move {
            let resp = context.ctx.statement_download_url(options).await?;
            Ok(resp.url)
        })?;
        Ok(())
    })
}
