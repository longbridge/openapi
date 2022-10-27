use std::sync::Arc;

use jni::{
    errors::Result,
    objects::{GlobalRef, JClass, JObject, JString, JValue},
    sys::{jboolean, jobjectArray},
    JNIEnv, JavaVM,
};
use longbridge::{
    quote::{AdjustType, Period, PushEvent, PushEventDetail, SubFlags},
    Config, Market, QuoteContext,
};
use parking_lot::Mutex;
use time::Date;

use crate::{
    async_util,
    error::jni_result,
    init::QUOTE_CONTEXT_CLASS,
    types::{set_field, FromJValue, IntoJValue, ObjectArray},
};

#[derive(Default)]
struct Callbacks {
    quote: Option<GlobalRef>,
    depth: Option<GlobalRef>,
    brokers: Option<GlobalRef>,
    trades: Option<GlobalRef>,
    candlestick: Option<GlobalRef>,
}

struct ContextObj {
    ctx: QuoteContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

fn send_push_event(jvm: &JavaVM, callbacks: &Callbacks, event: PushEvent) -> Result<()> {
    let env = jvm.attach_current_thread().unwrap();

    match event.detail {
        PushEventDetail::Quote(push_quote) => {
            if let Some(handler) = &callbacks.quote {
                env.call_method(
                    handler,
                    "onQuote",
                    "(Ljava/lang/String;Lcom/longbridge/quote/PushQuote;)V",
                    &[
                        event.symbol.into_jvalue(&env)?,
                        push_quote.into_jvalue(&env)?,
                    ],
                )?;
            }
        }
        PushEventDetail::Depth(push_depth) => {
            if let Some(handler) = &callbacks.depth {
                env.call_method(
                    handler,
                    "onDepth",
                    "(Ljava/lang/String;Lcom/longbridge/quote/PushDepth;)V",
                    &[
                        event.symbol.into_jvalue(&env)?,
                        push_depth.into_jvalue(&env)?,
                    ],
                )?;
            }
        }
        PushEventDetail::Brokers(push_brokers) => {
            if let Some(handler) = &callbacks.brokers {
                env.call_method(
                    handler,
                    "onBrokers",
                    "(Ljava/lang/String;Lcom/longbridge/quote/PushBrokers;)V",
                    &[
                        event.symbol.into_jvalue(&env)?,
                        push_brokers.into_jvalue(&env)?,
                    ],
                )?;
            }
        }
        PushEventDetail::Trade(push_trades) => {
            if let Some(handler) = &callbacks.trades {
                env.call_method(
                    handler,
                    "onTrades",
                    "(Ljava/lang/String;Lcom/longbridge/quote/PushTrades;)V",
                    &[
                        event.symbol.into_jvalue(&env)?,
                        push_trades.into_jvalue(&env)?,
                    ],
                )?;
            }
        }
        PushEventDetail::Candlestick(push_candlestick) => {
            if let Some(handler) = &callbacks.candlestick {
                env.call_method(
                    handler,
                    "onCandlestick",
                    "(Ljava/lang/String;Lcom/longbridge/quote/PushCandlestick;)V",
                    &[
                        event.symbol.into_jvalue(&env)?,
                        push_candlestick.into_jvalue(&env)?,
                    ],
                )?;
            }
        }
    }

    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newQuoteContext(
    env: JNIEnv,
    _class: JClass,
    config: i64,
    callback: JObject,
) {
    struct ContextObjRef(i64);

    impl IntoJValue for ContextObjRef {
        fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
            let ctx_obj = env.new_object(QUOTE_CONTEXT_CLASS.get().unwrap(), "()V", &[])?;
            set_field(env, ctx_obj, "raw", self.0)?;
            Ok(JValue::from(ctx_obj))
        }
    }

    jni_result(&env, (), || {
        let config = Arc::new((*(config as *const Config)).clone());
        let jvm = env.get_java_vm()?;

        async_util::execute(&env, callback, async move {
            let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
            let callbacks = Arc::new(Mutex::new(Callbacks::default()));

            tokio::spawn({
                let callbacks = callbacks.clone();
                async move {
                    while let Some(event) = receiver.recv().await {
                        let callbacks = callbacks.lock();
                        let _ = send_push_event(&jvm, &*callbacks, event);
                    }
                }
            });

            Ok(ContextObjRef(
                Box::into_raw(Box::new(ContextObj { ctx, callbacks })) as i64,
            ))
        })?;

        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeQuoteContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSetOnQuote(
    env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&env, (), || {
        if !handler.is_null() {
            context.callbacks.lock().quote = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().quote = None;
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSetOnDepth(
    env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&env, (), || {
        if !handler.is_null() {
            context.callbacks.lock().depth = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().depth = None;
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSetOnBrokers(
    env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&env, (), || {
        if !handler.is_null() {
            context.callbacks.lock().brokers = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().brokers = None;
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSetOnTrades(
    env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&env, (), || {
        if !handler.is_null() {
            context.callbacks.lock().trades = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().trades = None;
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSetOnCandlestick(
    env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&env, (), || {
        if !handler.is_null() {
            context.callbacks.lock().candlestick = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().candlestick = None;
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSubscribe(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    flags: i32,
    is_first_push: jboolean,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        let sub_flags = SubFlags::from_bits(flags as u8).unwrap_or(SubFlags::empty());
        async_util::execute(&env, callback, async move {
            Ok(context
                .ctx
                .subscribe(symbols.0, sub_flags, is_first_push > 0)
                .await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextUnsubscribe(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    flags: i32,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        let sub_flags = SubFlags::from_bits(flags as u8).unwrap_or(SubFlags::empty());
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.unsubscribe(symbols.0, sub_flags).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSubscribeCandlesticks(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(&env, period.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.subscribe_candlesticks(symbol, period).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextUnsubscribeCandlesticks(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(&env, period.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.unsubscribe_candlesticks(symbol, period).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSubscriptions(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        async_util::execute(&env, callback, async move {
            let list = context.ctx.subscriptions().await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextStaticInfo(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.ctx.static_info(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.ctx.quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextOptionQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.ctx.option_quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextWarrantQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.ctx.warrant_quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextDepth(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.depth(symbol).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextBrokers(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.brokers(symbol).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextParticipants(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.participants().await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextTrades(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    count: i32,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context.ctx.trades(symbol, count.max(0) as usize).await?,
            ))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextIntraday(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.intraday(symbol).await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextCandlesticks(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    count: i32,
    adjust_type: JObject,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(&env, period.into())?;
        let adjust_type: AdjustType = FromJValue::from_jvalue(&env, adjust_type.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .candlesticks(symbol, period, count.max(0) as usize, adjust_type)
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextOptionChainExpiryDateList(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context.ctx.option_chain_expiry_date_list(symbol).await?,
            ))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextOptionChainInfoByDate(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    expiry_date: JObject,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        let expiry_date: Date = FromJValue::from_jvalue(&env, expiry_date.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .option_chain_info_by_date(symbol, expiry_date)
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextWarrantIssuers(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.warrant_issuers().await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextTradingSession(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.trading_session().await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextTradingDays(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    market: JObject,
    begin: JObject,
    end: JObject,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let market: Market = FromJValue::from_jvalue(&env, market.into())?;
        let begin: Date = FromJValue::from_jvalue(&env, begin.into())?;
        let end: Date = FromJValue::from_jvalue(&env, end.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.trading_days(market, begin, end).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextCapitalFlow(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.capital_flow(symbol).await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextCapitalDistribution(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.capital_distribution(symbol).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextWatchList(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.watch_list().await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextRealtimeQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: JObject,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.ctx.realtime_quote(symbols.0).await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextRealtimeDepth(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.realtime_depth(symbol).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextRealtimeBrokers(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(context.ctx.realtime_brokers(symbol).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextRealtimeTrades(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    count: i32,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .realtime_trades(symbol, count.max(0) as usize)
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextRealtimeCandlesticks(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    count: i32,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(&env, period.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .realtime_candlesticks(symbol, period, count.max(0) as usize)
                    .await?,
            ))
        })?;
        Ok(())
    })
}
