use std::sync::Arc;

use jni::{
    JNIEnv, JavaVM,
    errors::Result,
    objects::{GlobalRef, JClass, JObject, JString},
    sys::{jboolean, jobjectArray},
};
use longbridge::{
    Config, Decimal, Market, TradeContext,
    trade::{
        BalanceType, EstimateMaxPurchaseQuantityOptions, GetCashFlowOptions,
        GetFundPositionsOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
        GetStockPositionsOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions, OrderSide,
        OrderStatus, OrderType, OutsideRTH, PushEvent, QueryUSOrdersOptions, ReplaceOrderOptions,
        SubmitOrderOptions, TimeInForceType, TopicType,
    },
};
use parking_lot::Mutex;
use time::{Date, OffsetDateTime};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, IntoJValue, JavaInteger, ObjectArray, get_field},
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
        let client_request_id: Option<String> = get_field(env, &opts, "clientRequestId")?;
        if let Some(id) = client_request_id {
            new_opts = new_opts.client_request_id(id);
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

// ── US-market JNI stubs ──────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextUsQueryOrders(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JObject,
    action: i64,
    start_at: i64,
    end_at: i64,
    query_type: i64,
    page: i64,
    limit: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol_str: String = FromJValue::from_jvalue(env, symbol.into())?;
        let us_opts = longbridge::trade::GetUSHistoryOrders {
            symbol: if symbol_str.is_empty() {
                None
            } else {
                Some(symbol_str)
            },
            side: match action {
                1 => longbridge::trade::OrderSide::Buy,
                2 => longbridge::trade::OrderSide::Sell,
                _ => longbridge::trade::OrderSide::Unknown,
            },
            start_at,
            end_at,
            query_type: query_type as i32,
            page: page as i32,
            limit: limit as i32,
        };
        async_util::execute(env, callback, async move {
            let resp = context.ctx.us_query_orders(us_opts).await?;
            Ok(serde_json::to_string(&resp).unwrap_or_default())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextUsOrderDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.us_order_detail(order_id).await?;
            Ok(serde_json::to_string(&resp).unwrap_or_default())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextUsAssetOverview(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(env, callback, async move {
            let resp = context.ctx.us_asset_overview().await?;
            Ok(serde_json::to_string(&resp).unwrap_or_default())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_tradeContextUsRealizedPl(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    currency: JObject,
    category: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let currency: String = FromJValue::from_jvalue(env, currency.into())?;
        let category: Option<String> = FromJValue::from_jvalue(env, category.into()).ok();
        async_util::execute(env, callback, async move {
            let resp = context.ctx.us_realized_pl(longbridge::trade::GetUSRealizedPLOptions { currency, category: category.unwrap_or_default() }).await?;
            Ok(serde_json::to_string(&resp).unwrap_or_default())
        })?;
        Ok(())
    })
}
