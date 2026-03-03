use std::sync::Arc;

use jni::{
    JNIEnv, JavaVM,
    errors::Result,
    objects::{GlobalRef, JClass, JObject, JString, JValueOwned},
    sys::{jboolean, jobjectArray},
};
use longport::{
    Config, Market, QuoteContext,
    quote::{
        AdjustType, CalcIndex, FilterWarrantExpiryDate, FilterWarrantInOutBoundsType, Period,
        PushEvent, PushEventDetail, RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup,
        SecuritiesUpdateMode, SecurityListCategory, SortOrderType, SubFlags, TradeSessions,
        WarrantSortBy, WarrantStatus, WarrantType,
    },
};
use parking_lot::Mutex;
use time::{Date, PrimitiveDateTime};

use crate::{
    async_util,
    error::jni_result,
    init::QUOTE_CONTEXT_CLASS,
    types::{
        CreateWatchlistGroupResponse, FromJValue, IntoJValue, ObjectArray, PrimaryArray, get_field,
        set_field,
    },
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
    let mut env = jvm.attach_current_thread().unwrap();

    match event.detail {
        PushEventDetail::Quote(push_quote) => {
            if let Some(handler) = &callbacks.quote {
                let symbol = event.symbol.into_jvalue(&mut env)?;
                let event = push_quote.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onQuote",
                    "(Ljava/lang/String;Lcom/longport/quote/PushQuote;)V",
                    &[symbol.borrow(), event.borrow()],
                )?;
            }
        }
        PushEventDetail::Depth(push_depth) => {
            if let Some(handler) = &callbacks.depth {
                let symbol = event.symbol.into_jvalue(&mut env)?;
                let event = push_depth.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onDepth",
                    "(Ljava/lang/String;Lcom/longport/quote/PushDepth;)V",
                    &[symbol.borrow(), event.borrow()],
                )?;
            }
        }
        PushEventDetail::Brokers(push_brokers) => {
            if let Some(handler) = &callbacks.brokers {
                let symbol = event.symbol.into_jvalue(&mut env)?;
                let event = push_brokers.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onBrokers",
                    "(Ljava/lang/String;Lcom/longport/quote/PushBrokers;)V",
                    &[symbol.borrow(), event.borrow()],
                )?;
            }
        }
        PushEventDetail::Trade(push_trades) => {
            if let Some(handler) = &callbacks.trades {
                let symbol = event.symbol.into_jvalue(&mut env)?;
                let event = push_trades.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onTrades",
                    "(Ljava/lang/String;Lcom/longport/quote/PushTrades;)V",
                    &[symbol.borrow(), event.borrow()],
                )?;
            }
        }
        PushEventDetail::Candlestick(push_candlestick) => {
            if let Some(handler) = &callbacks.candlestick {
                let symbol = event.symbol.into_jvalue(&mut env)?;
                let event = push_candlestick.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onCandlestick",
                    "(Ljava/lang/String;Lcom/longport/quote/PushCandlestick;)V",
                    &[symbol.borrow(), event.borrow()],
                )?;
            }
        }
    }

    Ok(())
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_newQuoteContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
    callback: JObject,
) {
    struct ContextObjRef(i64);

    impl IntoJValue for ContextObjRef {
        fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
            let ctx_obj = env.new_object(QUOTE_CONTEXT_CLASS.get().unwrap(), "()V", &[])?;
            set_field(env, &ctx_obj, "raw", self.0)?;
            Ok(JValueOwned::from(ctx_obj))
        }
    }

    jni_result(&mut env, (), |env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let jvm = env.get_java_vm()?;

        async_util::execute(env, callback, async move {
            let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
            let callbacks = Arc::new(Mutex::new(Callbacks::default()));

            tokio::spawn({
                let callbacks = callbacks.clone();
                async move {
                    while let Some(event) = receiver.recv().await {
                        let callbacks = callbacks.lock();
                        let _ = send_push_event(&jvm, &callbacks, event);
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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_freeQuoteContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextGetMemberId(
    mut _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) -> i64 {
    let context = &*(ctx as *const ContextObj);
    context.ctx.member_id()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextGetQuoteLevel<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    ctx: i64,
) -> JObject<'a> {
    let context = &*(ctx as *const ContextObj);
    context
        .ctx
        .quote_level()
        .to_string()
        .into_jvalue(&mut env)
        .unwrap()
        .l()
        .unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextGetQuotePackageDetails<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    ctx: i64,
) -> JObject<'a> {
    let context = &*(ctx as *const ContextObj);
    ObjectArray(context.ctx.quote_package_details().to_vec())
        .into_jvalue(&mut env)
        .unwrap()
        .l()
        .unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSetOnQuote(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().quote = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().quote = None;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSetOnDepth(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().depth = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().depth = None;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSetOnBrokers(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().brokers = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().brokers = None;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSetOnTrades(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().trades = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().trades = None;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSetOnCandlestick(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().candlestick = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().candlestick = None;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    flags: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        let sub_flags = SubFlags::from_bits(flags as u8).unwrap_or(SubFlags::empty());
        async_util::execute(env, callback, async move {
            Ok(context.ctx.subscribe(symbols.0, sub_flags).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextUnsubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    flags: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        let sub_flags = SubFlags::from_bits(flags as u8).unwrap_or(SubFlags::empty());
        async_util::execute(env, callback, async move {
            Ok(context.ctx.unsubscribe(symbols.0, sub_flags).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSubscribeCandlesticks(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    trade_sessions: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(env, period.into())?;
        let trade_sessions: TradeSessions = FromJValue::from_jvalue(env, trade_sessions.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .subscribe_candlesticks(symbol, period, trade_sessions)
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextUnsubscribeCandlesticks(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(env, period.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.unsubscribe_candlesticks(symbol, period).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSubscriptions(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let list = context.ctx.subscriptions().await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextStaticInfo(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        async_util::execute(env, callback, async move {
            let list = context.ctx.static_info(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextQuote(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        async_util::execute(env, callback, async move {
            let list = context.ctx.quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextOptionQuote(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        async_util::execute(env, callback, async move {
            let list = context.ctx.option_quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextWarrantQuote(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        async_util::execute(env, callback, async move {
            let list = context.ctx.warrant_quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextDepth(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.depth(symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextBrokers(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.brokers(symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextParticipants(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.participants().await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextTrades(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    count: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context.ctx.trades(symbol, count.max(0) as usize).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextIntraday(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    trade_sessions: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let trade_sessions: TradeSessions = FromJValue::from_jvalue(env, trade_sessions.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context.ctx.intraday(symbol, trade_sessions).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextCandlesticks(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    count: i32,
    adjust_type: JObject,
    trade_sessions: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(env, period.into())?;
        let adjust_type: AdjustType = FromJValue::from_jvalue(env, adjust_type.into())?;
        let trade_sessions: TradeSessions = FromJValue::from_jvalue(env, trade_sessions.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .candlesticks(
                        symbol,
                        period,
                        count.max(0) as usize,
                        adjust_type,
                        trade_sessions,
                    )
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextHistoryCandlesticksByOffset(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    adjust_type: JObject,
    forward: jboolean,
    datetime: JObject,
    count: i32,
    trade_sessions: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(env, period.into())?;
        let adjust_type: AdjustType = FromJValue::from_jvalue(env, adjust_type.into())?;
        let trade_sessions: TradeSessions = FromJValue::from_jvalue(env, trade_sessions.into())?;
        let datetime: Option<PrimitiveDateTime> = FromJValue::from_jvalue(env, datetime.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .history_candlesticks_by_offset(
                        symbol,
                        period,
                        adjust_type,
                        forward > 0,
                        datetime,
                        count as usize,
                        trade_sessions,
                    )
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextHistoryCandlesticksByDate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    adjust_type: JObject,
    start: JObject,
    end: JObject,
    trade_sessions: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(env, period.into())?;
        let adjust_type: AdjustType = FromJValue::from_jvalue(env, adjust_type.into())?;
        let start: Option<Date> = FromJValue::from_jvalue(env, start.into())?;
        let end: Option<Date> = FromJValue::from_jvalue(env, end.into())?;
        let trade_sessions: TradeSessions = FromJValue::from_jvalue(env, trade_sessions.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .history_candlesticks_by_date(
                        symbol,
                        period,
                        adjust_type,
                        start,
                        end,
                        trade_sessions,
                    )
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextOptionChainExpiryDateList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context.ctx.option_chain_expiry_date_list(symbol).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextOptionChainInfoByDate(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    expiry_date: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let expiry_date: Date = FromJValue::from_jvalue(env, expiry_date.into())?;
        async_util::execute(env, callback, async move {
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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextWarrantIssuers(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.warrant_issuers().await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextWarrantList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let sort_by: WarrantSortBy = get_field(env, &opts, "sortBy")?;
        let sort_type: SortOrderType = get_field(env, &opts, "sortType")?;
        let warrant_type: ObjectArray<WarrantType> = get_field(env, &opts, "warrantType")?;
        let issuer: PrimaryArray<i32> = get_field(env, &opts, "issuer")?;
        let expiry_date: ObjectArray<FilterWarrantExpiryDate> =
            get_field(env, &opts, "expiryDate")?;
        let price_type: ObjectArray<FilterWarrantInOutBoundsType> =
            get_field(env, &opts, "priceType")?;
        let status: ObjectArray<WarrantStatus> = get_field(env, &opts, "status")?;

        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .warrant_list(
                        symbol,
                        sort_by,
                        sort_type,
                        Some(&warrant_type.0),
                        Some(&issuer.0),
                        Some(&expiry_date.0),
                        Some(&price_type.0),
                        Some(&status.0),
                    )
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextTradingSession(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.trading_session().await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextTradingDays(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    market: JObject,
    begin: JObject,
    end: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: Market = FromJValue::from_jvalue(env, market.into())?;
        let begin: Date = FromJValue::from_jvalue(env, begin.into())?;
        let end: Date = FromJValue::from_jvalue(env, end.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.trading_days(market, begin, end).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextCapitalFlow(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.capital_flow(symbol).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextCapitalDistribution(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.capital_distribution(symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextCalcIndexes(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    indexes: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(symbols).into())?;
        let indexes: ObjectArray<CalcIndex> =
            FromJValue::from_jvalue(env, JObject::from_raw(indexes).into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context
                    .ctx
                    .calc_indexes(symbols.0, indexes.0)
                    .await?
                    .into_iter()
                    .map(crate::types::SecurityCalcIndex::from)
                    .collect(),
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextWatchlist(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.watchlist().await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextCreateWatchlistGroup(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    req: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let name: String = get_field(env, &req, "name")?;
        let securities: Option<ObjectArray<String>> = get_field(env, &req, "securities")?;
        async_util::execute(env, callback, async move {
            let id = context
                .ctx
                .create_watchlist_group(RequestCreateWatchlistGroup {
                    name,
                    securities: securities.map(|securities| securities.0),
                })
                .await?;
            Ok(CreateWatchlistGroupResponse { id })
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextDeleteWatchlistGroup(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    req: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let id: i64 = get_field(env, &req, "id")?;
        let purge: bool = get_field(env, &req, "purge")?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.delete_watchlist_group(id, purge).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextUpdateWatchlistGroup(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    req: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let id: i64 = get_field(env, &req, "id")?;
        let name: Option<String> = get_field(env, &req, "name")?;
        let securities: Option<ObjectArray<String>> = get_field(env, &req, "securities")?;
        let mode: Option<SecuritiesUpdateMode> = get_field(env, &req, "mode")?;
        let mode = mode.unwrap_or_default();
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .update_watchlist_group(RequestUpdateWatchlistGroup {
                    id,
                    name,
                    securities: securities.map(|securities| securities.0),
                    mode,
                })
                .await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextRealtimeQuote(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(env, symbols.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.realtime_quote(symbols.0).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextRealtimeDepth(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.realtime_depth(symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextRealtimeBrokers(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.realtime_brokers(symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextRealtimeTrades(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    count: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextRealtimeCandlesticks(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    period: JObject,
    count: i32,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let period: Period = FromJValue::from_jvalue(env, period.into())?;
        async_util::execute(env, callback, async move {
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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextSecurityList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    market: JObject,
    category: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: Market = FromJValue::from_jvalue(env, market.into())?;
        let category: Option<SecurityListCategory> = FromJValue::from_jvalue(env, category.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context.ctx.security_list(market, category).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextMarketTemperature(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    market: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: Market = FromJValue::from_jvalue(env, market.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.market_temperature(market).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_quoteContextHistoryMarketTemperature(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    market: JObject,
    start: JObject,
    end: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: Market = FromJValue::from_jvalue(env, market.into())?;
        let start: Date = FromJValue::from_jvalue(env, start.into())?;
        let end: Date = FromJValue::from_jvalue(env, end.into())?;
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .history_market_temperature(market, start, end)
                .await?)
        })?;
        Ok(())
    })
}
