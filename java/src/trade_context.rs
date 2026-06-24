use std::sync::Arc;

use jni::{
    JNIEnv, JavaVM,
    errors::Result,
    objects::{GlobalRef, JClass, JObject, JString},
    sys::jobjectArray,
};
use longbridge::{
    Config, Decimal, Market, TradeContext,
    trade::{
        AttachedOrderType, BalanceType, EstimateMaxPurchaseQuantityOptions, GetCashFlowOptions,
        GetFundPositionsOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
        GetOrderDetailOptions, GetStockPositionsOptions, GetTodayExecutionsOptions,
        GetTodayOrdersOptions, OrderSide, OrderStatus, OrderType, OutsideRTH, PushEvent,
        ReplaceAttachedParams, ReplaceOrderOptions, SubmitAttachedParams, SubmitOrderOptions,
        TimeInForceType, TopicType,
    },
};
use parking_lot::Mutex;
use time::{Date, OffsetDateTime};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, IntoJValue, JavaInteger, JavaLong, ObjectArray, get_field},
};

#[derive(Default)]
struct Callbacks {
    order_changed: Option<GlobalRef>,
}

struct ContextObj {
    ctx: TradeContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

fn send_push_event(jvm: &JavaVM, callbacks: &Callbacks, event: PushEvent) -> Result<()> {
    let mut env = jvm.attach_current_thread().unwrap();

    match event {
        PushEvent::OrderChanged(order_changed) => {
            if let Some(handler) = &callbacks.order_changed {
                let event = order_changed.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onOrderChanged",
                    "(Lcom/longbridge/trade/PushOrderChanged;)V",
                    &[event.borrow()],
                )?;
            }
        }
    }

    Ok(())
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newTradeContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let jvm = env.get_java_vm()?;

        let (ctx, mut receiver) = TradeContext::new(config);
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));

        longbridge::runtime_handle().spawn({
            let callbacks = callbacks.clone();
            async move {
                while let Some(event) = receiver.recv().await {
                    let callbacks = callbacks.lock();
                    let _ = send_push_event(&jvm, &callbacks, event);
                }
            }
        });

        Ok(Box::into_raw(Box::new(ContextObj { ctx, callbacks })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeTradeContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextSetOnOrderChanged(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().order_changed = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().order_changed = None;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextSubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    topics: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let topics: ObjectArray<TopicType> =
            FromJValue::from_jvalue(env, JObject::from_raw(topics).into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.subscribe(topics.0).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextUnsubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    topics: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let topics: ObjectArray<TopicType> =
            FromJValue::from_jvalue(env, JObject::from_raw(topics).into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.unsubscribe(topics.0).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextHistoryExecutions(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let opts = if !opts.is_null() {
            let mut new_opts = GetHistoryExecutionsOptions::new();
            let symbol: Option<String> = get_field(env, &opts, "symbol")?;
            if let Some(symbol) = symbol {
                new_opts = new_opts.symbol(symbol);
            }
            let start_at: Option<OffsetDateTime> = get_field(env, &opts, "startAt")?;
            if let Some(start_at) = start_at {
                new_opts = new_opts.start_at(start_at);
            }
            let end_at: Option<OffsetDateTime> = get_field(env, &opts, "endAt")?;
            if let Some(end_at) = end_at {
                new_opts = new_opts.end_at(end_at);
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.history_executions(opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextTodayExecutions(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let opts = if !opts.is_null() {
            let mut new_opts = GetTodayExecutionsOptions::new();
            let symbol: Option<String> = get_field(env, &opts, "symbol")?;
            if let Some(symbol) = symbol {
                new_opts = new_opts.symbol(symbol);
            }
            let order_id: Option<String> = get_field(env, &opts, "orderId")?;
            if let Some(order_id) = order_id {
                new_opts = new_opts.order_id(order_id);
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.today_executions(opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextHistoryOrders(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let opts = if !opts.is_null() {
            let mut new_opts = GetHistoryOrdersOptions::new();
            let symbol: Option<String> = get_field(env, &opts, "symbol")?;
            if let Some(symbol) = symbol {
                new_opts = new_opts.symbol(symbol);
            }
            let status: ObjectArray<OrderStatus> = get_field(env, &opts, "status")?;
            new_opts = new_opts.status(status.0);
            let side: Option<OrderSide> = get_field(env, &opts, "side")?;
            if let Some(side) = side {
                new_opts = new_opts.side(side);
            }
            let market: Option<Market> = get_field(env, &opts, "market")?;
            if let Some(market) = market {
                new_opts = new_opts.market(market);
            }
            let start_at: Option<OffsetDateTime> = get_field(env, &opts, "startAt")?;
            if let Some(start_at) = start_at {
                new_opts = new_opts.start_at(start_at);
            }
            let end_at: Option<OffsetDateTime> = get_field(env, &opts, "endAt")?;
            if let Some(end_at) = end_at {
                new_opts = new_opts.end_at(end_at);
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.history_orders(opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextTodayOrders(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let opts = if !opts.is_null() {
            let mut new_opts = GetTodayOrdersOptions::new();
            let symbol: Option<String> = get_field(env, &opts, "symbol")?;
            if let Some(symbol) = symbol {
                new_opts = new_opts.symbol(symbol);
            }
            let status: ObjectArray<OrderStatus> = get_field(env, &opts, "status")?;
            new_opts = new_opts.status(status.0);
            let side: Option<OrderSide> = get_field(env, &opts, "side")?;
            if let Some(side) = side {
                new_opts = new_opts.side(side);
            }
            let market: Option<Market> = get_field(env, &opts, "market")?;
            if let Some(market) = market {
                new_opts = new_opts.market(market);
            }
            let order_id: Option<String> = get_field(env, &opts, "orderId")?;
            if let Some(order_id) = order_id {
                new_opts = new_opts.order_id(order_id);
            }
            let is_attached_obj = env.get_field(&opts, "isAttached", "Ljava/lang/Boolean;")?;
            if !is_attached_obj.l()?.is_null() {
                new_opts = new_opts.is_attached();
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.today_orders(opts).await?))
        })?;
        Ok(())
    })
}

fn read_submit_attached_params(
    env: &mut JNIEnv<'_>,
    obj: &JObject<'_>,
) -> jni::errors::Result<SubmitAttachedParams> {
    let attached_order_type: AttachedOrderType = get_field(env, obj, "attachedOrderType")?;
    let mut params = SubmitAttachedParams::new(attached_order_type);
    let profit_taker_price: Option<Decimal> = get_field(env, obj, "profitTakerPrice")?;
    if let Some(v) = profit_taker_price {
        params = params.profit_taker_price(v);
    }
    let stop_loss_price: Option<Decimal> = get_field(env, obj, "stopLossPrice")?;
    if let Some(v) = stop_loss_price {
        params = params.stop_loss_price(v);
    }
    let time_in_force: Option<TimeInForceType> = get_field(env, obj, "timeInForce")?;
    if let Some(v) = time_in_force {
        params = params.time_in_force(v);
    }
    let expire_time_obj = env.get_field(obj, "expireTime", "Ljava/lang/Long;")?;
    if !expire_time_obj.l()?.is_null() {
        let expire_time: JavaLong = get_field(env, obj, "expireTime")?;
        params = params.expire_time(i64::from(expire_time));
    }
    let activate_order_type: Option<OrderType> = get_field(env, obj, "activateOrderType")?;
    if let Some(v) = activate_order_type {
        params = params.activate_order_type(v);
    }
    let profit_taker_submit_price: Option<Decimal> = get_field(env, obj, "profitTakerSubmitPrice")?;
    if let Some(v) = profit_taker_submit_price {
        params = params.profit_taker_submit_price(v);
    }
    let stop_loss_submit_price: Option<Decimal> = get_field(env, obj, "stopLossSubmitPrice")?;
    if let Some(v) = stop_loss_submit_price {
        params = params.stop_loss_submit_price(v);
    }
    let activate_rth: Option<OutsideRTH> = get_field(env, obj, "activateRth")?;
    if let Some(v) = activate_rth {
        params = params.activate_rth(v);
    }
    Ok(params)
}

fn read_replace_attached_params(
    env: &mut JNIEnv<'_>,
    obj: &JObject<'_>,
) -> jni::errors::Result<ReplaceAttachedParams> {
    let attached_order_type: AttachedOrderType = get_field(env, obj, "attachedOrderType")?;
    let mut params = ReplaceAttachedParams::new(attached_order_type);
    let profit_taker_price: Option<Decimal> = get_field(env, obj, "profitTakerPrice")?;
    if let Some(v) = profit_taker_price {
        params = params.profit_taker_price(v);
    }
    let stop_loss_price: Option<Decimal> = get_field(env, obj, "stopLossPrice")?;
    if let Some(v) = stop_loss_price {
        params = params.stop_loss_price(v);
    }
    let time_in_force: Option<TimeInForceType> = get_field(env, obj, "timeInForce")?;
    if let Some(v) = time_in_force {
        params = params.time_in_force(v);
    }
    let expire_time_obj = env.get_field(obj, "expireTime", "Ljava/lang/Long;")?;
    if !expire_time_obj.l()?.is_null() {
        let expire_time: JavaLong = get_field(env, obj, "expireTime")?;
        params = params.expire_time(i64::from(expire_time));
    }
    let activate_order_type: Option<OrderType> = get_field(env, obj, "activateOrderType")?;
    if let Some(v) = activate_order_type {
        params = params.activate_order_type(v);
    }
    let profit_taker_submit_price: Option<Decimal> = get_field(env, obj, "profitTakerSubmitPrice")?;
    if let Some(v) = profit_taker_submit_price {
        params = params.profit_taker_submit_price(v);
    }
    let stop_loss_submit_price: Option<Decimal> = get_field(env, obj, "stopLossSubmitPrice")?;
    if let Some(v) = stop_loss_submit_price {
        params = params.stop_loss_submit_price(v);
    }
    let activate_rth: Option<OutsideRTH> = get_field(env, obj, "activateRth")?;
    if let Some(v) = activate_rth {
        params = params.activate_rth(v);
    }
    let profit_taker_id_obj = env.get_field(obj, "profitTakerId", "Ljava/lang/Long;")?;
    if !profit_taker_id_obj.l()?.is_null() {
        let profit_taker_id: JavaLong = get_field(env, obj, "profitTakerId")?;
        params = params.profit_taker_id(i64::from(profit_taker_id));
    }
    let stop_loss_id_obj = env.get_field(obj, "stopLossId", "Ljava/lang/Long;")?;
    if !stop_loss_id_obj.l()?.is_null() {
        let stop_loss_id: JavaLong = get_field(env, obj, "stopLossId")?;
        params = params.stop_loss_id(i64::from(stop_loss_id));
    }
    let cancel_all_obj = env.get_field(obj, "cancelAllAttached", "Ljava/lang/Boolean;")?;
    if !cancel_all_obj.l()?.is_null() {
        params = params.cancel_all_attached();
    }
    let main_id_obj = env.get_field(obj, "mainId", "Ljava/lang/Long;")?;
    if !main_id_obj.l()?.is_null() {
        let main_id: JavaLong = get_field(env, obj, "mainId")?;
        params = params.main_id(i64::from(main_id));
    }
    let quantity: Option<Decimal> = get_field(env, obj, "quantity")?;
    if let Some(v) = quantity {
        params = params.quantity(v);
    }
    let market_price: Option<Decimal> = get_field(env, obj, "marketPrice")?;
    if let Some(v) = market_price {
        params = params.market_price(v);
    }
    Ok(params)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextReplaceOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let order_id: String = get_field(env, &opts, "orderId")?;
        let quantity: Decimal = get_field(env, &opts, "quantity")?;
        let mut new_opts = ReplaceOrderOptions::new(order_id, quantity);
        let price: Option<Decimal> = get_field(env, &opts, "price")?;
        if let Some(price) = price {
            new_opts = new_opts.price(price);
        }
        let trigger_price: Option<Decimal> = get_field(env, &opts, "triggerPrice")?;
        if let Some(trigger_price) = trigger_price {
            new_opts = new_opts.trigger_price(trigger_price);
        }
        let limit_offset: Option<Decimal> = get_field(env, &opts, "limitOffset")?;
        if let Some(limit_offset) = limit_offset {
            new_opts = new_opts.limit_offset(limit_offset);
        }
        let trailing_amount: Option<Decimal> = get_field(env, &opts, "trailingAmount")?;
        if let Some(trailing_amount) = trailing_amount {
            new_opts = new_opts.trailing_amount(trailing_amount);
        }
        let trailing_percent: Option<Decimal> = get_field(env, &opts, "trailingPercent")?;
        if let Some(trailing_percent) = trailing_percent {
            new_opts = new_opts.trailing_percent(trailing_percent);
        }
        let limit_depth_level: Option<JavaInteger> = get_field(env, &opts, "limitDepthLevel")?;
        if let Some(limit_depth_level) = limit_depth_level {
            new_opts = new_opts.limit_depth_level(limit_depth_level.into());
        }
        let trigger_count: Option<JavaInteger> = get_field(env, &opts, "triggerCount")?;
        if let Some(trigger_count) = trigger_count {
            new_opts = new_opts.trigger_count(trigger_count.into());
        }
        let monitor_price: Option<Decimal> = get_field(env, &opts, "monitorPrice")?;
        if let Some(monitor_price) = monitor_price {
            new_opts = new_opts.monitor_price(monitor_price);
        }
        let remark: Option<String> = get_field(env, &opts, "remark")?;
        if let Some(remark) = remark {
            new_opts = new_opts.remark(remark);
        }
        let attached_params_obj = env.get_field(
            &opts,
            "attachedParams",
            "Lcom/longbridge/trade/ReplaceAttachedParams;",
        )?;
        if !attached_params_obj.l()?.is_null() {
            let attached_params_java = env
                .get_field(
                    &opts,
                    "attachedParams",
                    "Lcom/longbridge/trade/ReplaceAttachedParams;",
                )?
                .l()?;
            let ap = read_replace_attached_params(env, &attached_params_java)?;
            new_opts = new_opts.attached_params(ap);
        }

        async_util::execute(env, callback, async move {
            Ok(context.ctx.replace_order(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextSubmitOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let quantity: OrderType = get_field(env, &opts, "orderType")?;
        let side: OrderSide = get_field(env, &opts, "side")?;
        let submitted_quantity: Decimal = get_field(env, &opts, "submittedQuantity")?;
        let time_in_force: TimeInForceType = get_field(env, &opts, "timeInForce")?;
        let mut new_opts =
            SubmitOrderOptions::new(symbol, quantity, side, submitted_quantity, time_in_force);
        let submitted_price: Option<Decimal> = get_field(env, &opts, "submittedPrice")?;
        if let Some(submitted_price) = submitted_price {
            new_opts = new_opts.submitted_price(submitted_price);
        }
        let trigger_price: Option<Decimal> = get_field(env, &opts, "triggerPrice")?;
        if let Some(trigger_price) = trigger_price {
            new_opts = new_opts.trigger_price(trigger_price);
        }
        let limit_offset: Option<Decimal> = get_field(env, &opts, "limitOffset")?;
        if let Some(limit_offset) = limit_offset {
            new_opts = new_opts.limit_offset(limit_offset);
        }
        let trailing_amount: Option<Decimal> = get_field(env, &opts, "trailingAmount")?;
        if let Some(trailing_amount) = trailing_amount {
            new_opts = new_opts.trailing_amount(trailing_amount);
        }
        let trailing_percent: Option<Decimal> = get_field(env, &opts, "trailingPercent")?;
        if let Some(trailing_percent) = trailing_percent {
            new_opts = new_opts.trailing_percent(trailing_percent);
        }
        let expire_date: Option<Date> = get_field(env, &opts, "expireDate")?;
        if let Some(expire_date) = expire_date {
            new_opts = new_opts.expire_date(expire_date);
        }
        let outside_rth: Option<OutsideRTH> = get_field(env, &opts, "outsideRth")?;
        if let Some(outside_rth) = outside_rth {
            new_opts = new_opts.outside_rth(outside_rth);
        }
        let limit_depth_level: Option<JavaInteger> = get_field(env, &opts, "limitDepthLevel")?;
        if let Some(limit_depth_level) = limit_depth_level {
            new_opts = new_opts.limit_depth_level(limit_depth_level.into());
        }
        let trigger_count: Option<JavaInteger> = get_field(env, &opts, "triggerCount")?;
        if let Some(trigger_count) = trigger_count {
            new_opts = new_opts.trigger_count(trigger_count.into());
        }
        let monitor_price: Option<Decimal> = get_field(env, &opts, "monitorPrice")?;
        if let Some(monitor_price) = monitor_price {
            new_opts = new_opts.monitor_price(monitor_price);
        }
        let remark: Option<String> = get_field(env, &opts, "remark")?;
        if let Some(remark) = remark {
            new_opts = new_opts.remark(remark);
        }
        let attached_params_obj = env.get_field(
            &opts,
            "attachedParams",
            "Lcom/longbridge/trade/SubmitAttachedParams;",
        )?;
        if !attached_params_obj.l()?.is_null() {
            let attached_params_java = env
                .get_field(
                    &opts,
                    "attachedParams",
                    "Lcom/longbridge/trade/SubmitAttachedParams;",
                )?
                .l()?;
            let ap = read_submit_attached_params(env, &attached_params_java)?;
            new_opts = new_opts.attached_params(ap);
        }

        async_util::execute(env, callback, async move {
            Ok(context.ctx.submit_order(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextCancelOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.cancel_order(order_id).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextAccountBalance(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    currency: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let currency: Option<String> = FromJValue::from_jvalue(env, currency.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                context.ctx.account_balance(currency.as_deref()).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextCashFlow(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let start_at: OffsetDateTime = get_field(env, &opts, "startAt")?;
        let end_at: OffsetDateTime = get_field(env, &opts, "endAt")?;
        let mut new_opts = GetCashFlowOptions::new(start_at, end_at);
        let business_type: Option<BalanceType> = get_field(env, &opts, "businessType")?;
        let symbol: Option<String> = get_field(env, &opts, "symbol")?;

        if let Some(business_type) = business_type {
            new_opts = new_opts.business_type(business_type);
        }
        if let Some(symbol) = symbol {
            new_opts = new_opts.symbol(symbol);
        }
        let page: i32 = get_field(env, &opts, "page")?;
        if page > 0 {
            new_opts = new_opts.page(page as usize);
        }
        let size: i32 = get_field(env, &opts, "size")?;
        if size > 0 {
            new_opts = new_opts.size(size as usize);
        }

        async_util::execute(env, callback, async move {
            Ok(ObjectArray(context.ctx.cash_flow(new_opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextFundPositions(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let opts = if !opts.is_null() {
            let mut new_opts = GetFundPositionsOptions::new();
            let symbols: ObjectArray<String> = get_field(env, opts, "symbols")?;
            new_opts = new_opts.symbols(symbols.0);
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(context.ctx.fund_positions(opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextStockPositions(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let opts = if !opts.is_null() {
            let mut new_opts = GetStockPositionsOptions::new();
            let symbols: ObjectArray<String> = get_field(env, opts, "symbols")?;
            new_opts = new_opts.symbols(symbols.0);
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(crate::types::StockPositionsResponse::from(
                context.ctx.stock_positions(opts).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextMarginRatio(
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
            Ok(context.ctx.margin_ratio(symbol).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextOrderDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.order_detail(order_id).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextEstimateMaxPurchaseQuantity(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let order_type: OrderType = get_field(env, &opts, "orderType")?;
        let side: OrderSide = get_field(env, &opts, "side")?;
        let mut new_opts = EstimateMaxPurchaseQuantityOptions::new(symbol, order_type, side);
        let price: Option<Decimal> = get_field(env, &opts, "price")?;
        if let Some(price) = price {
            new_opts = new_opts.price(price);
        }
        let currency: Option<String> = get_field(env, &opts, "currency")?;
        if let Some(currency) = currency {
            new_opts = new_opts.currency(currency);
        }
        let order_id: Option<String> = get_field(env, &opts, "orderId")?;
        if let Some(order_id) = order_id {
            new_opts = new_opts.order_id(order_id);
        }
        let fractional_shares: bool = get_field(env, &opts, "fractionalShares")?;
        if fractional_shares {
            new_opts = new_opts.fractional_shares();
        }
        async_util::execute(env, callback, async move {
            Ok(context.ctx.estimate_max_purchase_quantity(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextOrderDetailAttached(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .order_detail(GetOrderDetailOptions::new(order_id).is_attached())
                .await?)
        })?;
        Ok(())
    })
}
