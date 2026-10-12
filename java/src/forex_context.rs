use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
};
use longbridge::{
    Config,
    forex::{ForexContext, GetForexQuoteOptions, SubmitForexOrderOptions},
};

use crate::{async_util, error::jni_result, types::FromJValue};

struct ContextObj {
    ctx: ForexContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newForexContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: ForexContext::new(Arc::new((*(config as *const Config)).clone())),
        })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeForexContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_forexContextQuote(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    from: JString,
    to: JString,
    amount: JObject,
    target_amount: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let from: String = FromJValue::from_jvalue(env, from.into())?;
        let to: String = FromJValue::from_jvalue(env, to.into())?;
        let amount: Option<longbridge::Decimal> = FromJValue::from_jvalue(env, amount.into())?;
        let target_amount: Option<longbridge::Decimal> =
            FromJValue::from_jvalue(env, target_amount.into())?;
        let mut options = GetForexQuoteOptions::new();
        if let Some(amount) = amount {
            options = options.amount(amount);
        }
        if let Some(target_amount) = target_amount {
            options = options.target_amount(target_amount);
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.quote(from, to, options).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_forexContextSubmitOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    quote_id: JString,
    client_order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let quote_id: String = FromJValue::from_jvalue(env, quote_id.into())?;
        let client_order_id: String = FromJValue::from_jvalue(env, client_order_id.into())?;
        let options = SubmitForexOrderOptions::new(quote_id, client_order_id);
        async_util::execute(env, callback, async move {
            __owned_ctx.submit_order(options).await?;
            Ok(())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_forexContextOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    client_order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let client_order_id: String = FromJValue::from_jvalue(env, client_order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.order(client_order_id).await?)
        })?;
        Ok(())
    })
}
