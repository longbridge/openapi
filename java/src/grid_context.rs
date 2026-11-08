use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::jobjectArray,
};
use longbridge::{
    Config, Market,
    grid::{
        GetGridOrderDetailOptions, GetGridOrdersByIdsOptions, GetGridOrdersOptions,
        GetGridTriggerHistoryOptions, GridContext, GridTradeRule, ReplaceGridOrderOptions,
        SubmitGridOrderOptions,
    },
};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, JavaLong, ObjectArray, get_field},
};

struct ContextObj {
    ctx: GridContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newGridContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: GridContext::new(Arc::new((*(config as *const Config)).clone())),
        })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeGridContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

/// Read a `GridTradeRule` from a Java `com.longbridge.grid.GridTradeRule`
/// object. Every field is optional (nullable box type on the Java side).
fn read_grid_trade_rule(
    env: &mut JNIEnv<'_>,
    obj: &JObject<'_>,
) -> jni::errors::Result<GridTradeRule> {
    fn read_opt_bool(
        env: &mut JNIEnv<'_>,
        obj: &JObject<'_>,
        name: &str,
    ) -> jni::errors::Result<Option<bool>> {
        let field = env.get_field(obj, name, "Ljava/lang/Boolean;")?.l()?;
        if field.is_null() {
            Ok(None)
        } else {
            Ok(Some(
                env.call_method(&field, "booleanValue", "()Z", &[])?.z()?,
            ))
        }
    }

    let mut rule = GridTradeRule::default();
    rule.submitted_base_price = get_field(env, obj, "submittedBasePrice")?;
    rule.upper_limit_price = get_field(env, obj, "upperLimitPrice")?;
    rule.lower_limit_price = get_field(env, obj, "lowerLimitPrice")?;
    rule.trigger_price_type = get_field::<_, _, Option<longbridge::grid::TriggerPriceType>>(
        env,
        obj,
        "triggerPriceType",
    )?;
    rule.trigger_spread_up = get_field(env, obj, "triggerSpreadUp")?;
    rule.trigger_spread_down = get_field(env, obj, "triggerSpreadDown")?;
    rule.trigger_percent_up = get_field(env, obj, "triggerPercentUp")?;
    rule.trigger_percent_down = get_field(env, obj, "triggerPercentDown")?;
    rule.multiple_trigger = read_opt_bool(env, obj, "multipleTrigger")?;
    rule.time_in_force =
        get_field::<_, _, Option<longbridge::grid::GridTimeInForce>>(env, obj, "timeInForce")?;
    rule.upper_limit_quantity = get_field(env, obj, "upperLimitQuantity")?;
    rule.lower_limit_quantity = get_field(env, obj, "lowerLimitQuantity")?;
    rule.expire_time = get_field::<_, _, Option<JavaLong>>(env, obj, "expireTime")?.map(i64::from);
    rule.upper_limit_event =
        get_field::<_, _, Option<longbridge::grid::GridLimitEvent>>(env, obj, "upperLimitEvent")?;
    rule.lower_limit_event =
        get_field::<_, _, Option<longbridge::grid::GridLimitEvent>>(env, obj, "lowerLimitEvent")?;
    rule.trigger_sell_depth =
        get_field::<_, _, Option<JavaInteger>>(env, obj, "triggerSellDepth")?.map(i32::from);
    rule.trigger_buy_depth =
        get_field::<_, _, Option<JavaInteger>>(env, obj, "triggerBuyDepth")?.map(i32::from);
    rule.trigger_quantity = get_field(env, obj, "triggerQuantity")?;
    rule.support_shortsell = read_opt_bool(env, obj, "supportShortsell")?;
    rule.rth = get_field::<_, _, Option<JavaInteger>>(env, obj, "rth")?.map(i32::from);
    rule.grid_order_type_up = get_field(env, obj, "gridOrderTypeUp")?;
    rule.grid_order_type_down = get_field(env, obj, "gridOrderTypeDown")?;
    Ok(rule)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextSubmit(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = get_field(env, &opts, "symbol")?;
        let settlement_currency: String = get_field(env, &opts, "settlementCurrency")?;
        let rule_obj = env
            .get_field(
                &opts,
                "gridTradingRule",
                "Lcom/longbridge/grid/GridTradeRule;",
            )?
            .l()?;
        let rule = read_grid_trade_rule(env, &rule_obj)?;
        let new_opts = SubmitGridOrderOptions::new(symbol, settlement_currency, rule);
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.submit(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextReplace(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let order_id: String = get_field(env, &opts, "orderId")?;
        let rule_obj = env
            .get_field(
                &opts,
                "gridTradingRule",
                "Lcom/longbridge/grid/GridTradeRule;",
            )?
            .l()?;
        let rule = read_grid_trade_rule(env, &rule_obj)?;
        let new_opts = ReplaceGridOrderOptions::new(order_id, rule);
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.replace(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextList(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let opts = if !opts.is_null() {
            let mut new_opts = GetGridOrdersOptions::new();
            if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "page")? {
                new_opts = new_opts.page(page.into());
            }
            if let Some(limit) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "limit")? {
                new_opts = new_opts.limit(limit.into());
            }
            if let Some(market) = get_field::<_, _, Option<Market>>(env, &opts, "market")? {
                new_opts = new_opts.market(market);
            }
            if let Some(status) = get_field::<_, _, Option<String>>(env, &opts, "status")? {
                new_opts = new_opts.status(status);
            }
            if let Some(symbol) = get_field::<_, _, Option<String>>(env, &opts, "symbol")? {
                new_opts = new_opts.symbol(symbol);
            }
            if let Some(sort_by) = get_field::<_, _, Option<String>>(env, &opts, "sortBy")? {
                new_opts = new_opts.sort_by(sort_by);
            }
            if let Some(sort_order) = get_field::<_, _, Option<String>>(env, &opts, "sortOrder")? {
                new_opts = new_opts.sort_order(sort_order);
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            let resp = __owned_ctx.list(opts).await?;
            Ok(crate::types::GridOrdersResponse::new(
                resp.grid_order,
                resp.has_more,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextListByIds(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_ids: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let order_ids: ObjectArray<String> =
            FromJValue::from_jvalue(env, JObject::from_raw(order_ids).into())?;
        let new_opts = GetGridOrdersByIdsOptions::new(order_ids.0);
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.list_by_ids(new_opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let order_id: String = get_field(env, &opts, "orderId")?;
        let mut new_opts = GetGridOrderDetailOptions::new(order_id);
        if let Some(history_id) = get_field::<_, _, Option<String>>(env, &opts, "historyId")? {
            new_opts = new_opts.history_id(history_id);
        }
        if let Some(limit) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "limit")? {
            new_opts = new_opts.limit(limit.into());
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.detail(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextTriggerHistory(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let grid_order_id: String = get_field(env, &opts, "gridOrderId")?;
        let mut new_opts = GetGridTriggerHistoryOptions::new(grid_order_id);
        if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "page")? {
            new_opts = new_opts.page(page.into());
        }
        if let Some(limit) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "limit")? {
            new_opts = new_opts.limit(limit.into());
        }
        async_util::execute(env, callback, async move {
            let resp = __owned_ctx.trigger_history(new_opts).await?;
            Ok(crate::types::GridTriggerHistoryResponse::new(
                resp.trigger_orders,
                resp.has_more,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextCancel(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.cancel(order_id).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextSuspend(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.suspend(order_id).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextRestart(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let order_id: String = FromJValue::from_jvalue(env, order_id.into())?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.restart(order_id).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_gridContextSymbolInfo(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.symbol_info(symbol).await?)
        })?;
        Ok(())
    })
}
