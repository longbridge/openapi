use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
};
use longbridge::{
    Config,
    fund::{
        FundContext, FundNavRangeOptions, FundPageOptions, GetFundAnalysisOptions,
        GetFundHoldingsOptions, GetFundOrdersOptions, GetFundPositionDividendsOptions,
        GetFundPositionOptions, GetFundPositionProfitsOptions, GetFundPositionsOptions,
        GetFundStockHoldingsOptions, GetFundTransactionsOptions, GetFundsOptions,
        SubmitFundOrderOptions, ValidateFundOrderOptions,
    },
};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JavaInteger, JavaLong, ObjectArray, PrimaryArray, get_field},
};

struct ContextObj {
    ctx: FundContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newFundContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: FundContext::new(Arc::new((*(config as *const Config)).clone())),
        })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeFundContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

// ----- fund catalog / market data -----

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextHotFunds(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.hot_funds().await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextFunds(
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
            let mut new_opts = GetFundsOptions::new();
            if let Some(filter) = get_field::<_, _, Option<String>>(env, &opts, "filter")?
                && let Ok(value) = serde_json::from_str::<serde_json::Value>(&filter)
            {
                new_opts = new_opts.filter(value);
            }
            let quick_ids: PrimaryArray<i64> = get_field(env, &opts, "quickIds")?;
            if !quick_ids.0.is_empty() {
                new_opts = new_opts.quick_ids(quick_ids.0);
            }
            let time_interval: ObjectArray<String> = get_field(env, &opts, "timeInterval")?;
            if !time_interval.0.is_empty() {
                new_opts = new_opts.time_interval(time_interval.0);
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.funds(opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextFilters(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        async_util::execute(
            env,
            callback,
            async move { Ok(__owned_ctx.filters().await?) },
        )?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextDetail(
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
            Ok(__owned_ctx.detail(symbol).await?)
        })?;
        Ok(())
    })
}

/// Read a nullable Java `Boolean` field as `Option<bool>`.
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

/// Read a [`GetFundAnalysisOptions`] from a nullable Java options object.
fn read_analysis_opts(
    env: &mut JNIEnv<'_>,
    opts: &JObject<'_>,
) -> jni::errors::Result<Option<GetFundAnalysisOptions>> {
    if opts.is_null() {
        return Ok(None);
    }
    let mut new_opts = GetFundAnalysisOptions::new();
    if let Some(period) = get_field::<_, _, Option<JavaInteger>>(env, opts, "period")? {
        new_opts = new_opts.period(period.into());
    }
    Ok(Some(new_opts))
}

/// Read a [`FundPageOptions`] from a nullable Java options object.
fn read_page_opts(
    env: &mut JNIEnv<'_>,
    opts: &JObject<'_>,
) -> jni::errors::Result<Option<FundPageOptions>> {
    if opts.is_null() {
        return Ok(None);
    }
    let mut new_opts = FundPageOptions::new();
    if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, opts, "page")? {
        new_opts = new_opts.page(page.into());
    }
    if let Some(size) = get_field::<_, _, Option<JavaInteger>>(env, opts, "size")? {
        new_opts = new_opts.size(size.into());
    }
    Ok(Some(new_opts))
}

/// Read a [`FundNavRangeOptions`] from a nullable Java options object.
fn read_nav_range_opts(
    env: &mut JNIEnv<'_>,
    opts: &JObject<'_>,
) -> jni::errors::Result<Option<FundNavRangeOptions>> {
    if opts.is_null() {
        return Ok(None);
    }
    let mut new_opts = FundNavRangeOptions::new();
    if let Some(month_before) = get_field::<_, _, Option<JavaInteger>>(env, opts, "monthBefore")? {
        new_opts = new_opts.month_before(month_before.into());
    }
    if let Some(year_before) = get_field::<_, _, Option<JavaInteger>>(env, opts, "yearBefore")? {
        new_opts = new_opts.year_before(year_before.into());
    }
    Ok(Some(new_opts))
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextAnalysis(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_analysis_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.analysis(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextAnalysisDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_analysis_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.analysis_detail(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextTrend(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_analysis_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.trend(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextAnnualReturns(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_page_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.annual_returns(symbol, opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextQuarterlyReturns(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_page_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                __owned_ctx.quarterly_returns(symbol, opts).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPerformance(
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
            Ok(ObjectArray(__owned_ctx.performance(symbol).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPerformanceComparison(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_analysis_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.performance_comparison(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextNav(
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
            Ok(ObjectArray(__owned_ctx.nav(symbol).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextNavHistory(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_page_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.nav_history(symbol, opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextNavRange(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_nav_range_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.nav_range(symbol, opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextHoldings(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = if !opts.is_null() {
            let mut new_opts = GetFundHoldingsOptions::new();
            if let Some(scene) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "scene")? {
                new_opts = new_opts.scene(scene.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.holdings(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextStockHoldings(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = if !opts.is_null() {
            let mut new_opts = GetFundStockHoldingsOptions::new();
            if let Some(limit) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "limit")? {
                new_opts = new_opts.limit(limit.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.stock_holdings(symbol, opts).await?))
        })?;
        Ok(())
    })
}

// ----- user fund positions -----

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPositions(
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
            let mut new_opts = GetFundPositionsOptions::new();
            if let Some(account_channel) =
                get_field::<_, _, Option<String>>(env, &opts, "accountChannel")?
            {
                new_opts = new_opts.account_channel(account_channel);
            }
            if let Some(aaid) = get_field::<_, _, Option<JavaLong>>(env, &opts, "aaid")? {
                new_opts = new_opts.aaid(aaid.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.positions(opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPosition(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = if !opts.is_null() {
            let mut new_opts = GetFundPositionOptions::new();
            if let Some(account_channel) =
                get_field::<_, _, Option<String>>(env, &opts, "accountChannel")?
            {
                new_opts = new_opts.account_channel(account_channel);
            }
            if let Some(aaid) = get_field::<_, _, Option<JavaLong>>(env, &opts, "aaid")? {
                new_opts = new_opts.aaid(aaid.into());
            }
            if let Some(start) = get_field::<_, _, Option<String>>(env, &opts, "start")? {
                new_opts = new_opts.start(start);
            }
            if let Some(end) = get_field::<_, _, Option<String>>(env, &opts, "end")? {
                new_opts = new_opts.end(end);
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.position(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPositionPerformance(
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
            Ok(ObjectArray(__owned_ctx.position_performance(symbol).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPositionProfits(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = if !opts.is_null() {
            let mut new_opts = GetFundPositionProfitsOptions::new();
            if let Some(account_channel) =
                get_field::<_, _, Option<String>>(env, &opts, "accountChannel")?
            {
                new_opts = new_opts.account_channel(account_channel);
            }
            if let Some(aaid) = get_field::<_, _, Option<JavaLong>>(env, &opts, "aaid")? {
                new_opts = new_opts.aaid(aaid.into());
            }
            if let Some(start) = get_field::<_, _, Option<String>>(env, &opts, "start")? {
                new_opts = new_opts.start(start);
            }
            if let Some(end) = get_field::<_, _, Option<String>>(env, &opts, "end")? {
                new_opts = new_opts.end(end);
            }
            if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "page")? {
                new_opts = new_opts.page(page.into());
            }
            if let Some(size) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "size")? {
                new_opts = new_opts.size(size.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.position_profits(symbol, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPositionNav(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = read_nav_range_opts(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.position_nav(symbol, opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextPositionDividends(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let opts = if !opts.is_null() {
            let mut new_opts = GetFundPositionDividendsOptions::new();
            if let Some(account_channel) =
                get_field::<_, _, Option<String>>(env, &opts, "accountChannel")?
            {
                new_opts = new_opts.account_channel(account_channel);
            }
            if let Some(aaid) = get_field::<_, _, Option<JavaLong>>(env, &opts, "aaid")? {
                new_opts = new_opts.aaid(aaid.into());
            }
            if let Some(currency) = get_field::<_, _, Option<String>>(env, &opts, "currency")? {
                new_opts = new_opts.currency(currency);
            }
            if let Some(start) = get_field::<_, _, Option<JavaLong>>(env, &opts, "start")? {
                new_opts = new_opts.start(start.into());
            }
            if let Some(end) = get_field::<_, _, Option<JavaLong>>(env, &opts, "end")? {
                new_opts = new_opts.end(end.into());
            }
            if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "page")? {
                new_opts = new_opts.page(page.into());
            }
            if let Some(size) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "size")? {
                new_opts = new_opts.size(size.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.position_dividends(symbol, opts).await?)
        })?;
        Ok(())
    })
}

// ----- fund orders & trading -----

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextOrders(
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
            let mut new_opts = GetFundOrdersOptions::new();
            let symbols: ObjectArray<String> = get_field(env, &opts, "symbols")?;
            if !symbols.0.is_empty() {
                new_opts = new_opts.symbols(symbols.0);
            }
            if let Some(actions) = get_field::<_, _, Option<String>>(env, &opts, "actions")? {
                new_opts = new_opts.actions(actions);
            }
            if let Some(states) = get_field::<_, _, Option<String>>(env, &opts, "states")? {
                new_opts = new_opts.states(states);
            }
            if let Some(currency) = get_field::<_, _, Option<String>>(env, &opts, "currency")? {
                new_opts = new_opts.currency(currency);
            }
            if let Some(start) = get_field::<_, _, Option<JavaLong>>(env, &opts, "start")? {
                new_opts = new_opts.start(start.into());
            }
            if let Some(end) = get_field::<_, _, Option<JavaLong>>(env, &opts, "end")? {
                new_opts = new_opts.end(end.into());
            }
            if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "page")? {
                new_opts = new_opts.page(page.into());
            }
            if let Some(size) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "size")? {
                new_opts = new_opts.size(size.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.orders(opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.order(order_id).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextTransactions(
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
            let mut new_opts = GetFundTransactionsOptions::new();
            if let Some(account_channel) =
                get_field::<_, _, Option<String>>(env, &opts, "accountChannel")?
            {
                new_opts = new_opts.account_channel(account_channel);
            }
            if let Some(business_type) =
                get_field::<_, _, Option<String>>(env, &opts, "businessType")?
            {
                new_opts = new_opts.business_type(business_type);
            }
            if let Some(category) = get_field::<_, _, Option<String>>(env, &opts, "category")? {
                new_opts = new_opts.category(category);
            }
            if let Some(currencies) = get_field::<_, _, Option<String>>(env, &opts, "currencies")? {
                new_opts = new_opts.currencies(currencies);
            }
            if let Some(start) = get_field::<_, _, Option<JavaLong>>(env, &opts, "start")? {
                new_opts = new_opts.start(start.into());
            }
            if let Some(end) = get_field::<_, _, Option<JavaLong>>(env, &opts, "end")? {
                new_opts = new_opts.end(end.into());
            }
            if let Some(page) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "page")? {
                new_opts = new_opts.page(page.into());
            }
            if let Some(size) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "size")? {
                new_opts = new_opts.size(size.into());
            }
            Some(new_opts)
        } else {
            None
        };
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.transactions(opts).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextValidateOrder(
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
        let action: String = get_field(env, &opts, "action")?;
        let currency: String = get_field(env, &opts, "currency")?;
        let mut new_opts = ValidateFundOrderOptions::new(symbol, action, currency);
        if let Some(amount) = get_field::<_, _, Option<String>>(env, &opts, "amount")? {
            new_opts = new_opts.amount(amount);
        }
        if let Some(units) = get_field::<_, _, Option<String>>(env, &opts, "units")? {
            new_opts = new_opts.units(units);
        }
        if let Some(dividend_option) =
            get_field::<_, _, Option<JavaInteger>>(env, &opts, "dividendOption")?
        {
            new_opts = new_opts.dividend_option(dividend_option.into());
        }
        if let Some(fund_source) = get_field::<_, _, Option<JavaInteger>>(env, &opts, "fundSource")?
        {
            new_opts = new_opts.fund_source(fund_source.into());
        }
        if let Some(account_channel) =
            get_field::<_, _, Option<String>>(env, &opts, "accountChannel")?
        {
            new_opts = new_opts.account_channel(account_channel);
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.validate_order(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextSubmitOrder(
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
        let action: String = get_field(env, &opts, "action")?;
        let currency: String = get_field(env, &opts, "currency")?;
        let mut new_opts = SubmitFundOrderOptions::new(symbol, action, currency);
        if let Some(amount) = get_field::<_, _, Option<String>>(env, &opts, "amount")? {
            new_opts = new_opts.amount(amount);
        }
        if let Some(units) = get_field::<_, _, Option<String>>(env, &opts, "units")? {
            new_opts = new_opts.units(units);
        }
        if let Some(dividend_option) =
            get_field::<_, _, Option<JavaInteger>>(env, &opts, "dividendOption")?
        {
            new_opts = new_opts.dividend_option(dividend_option.into());
        }
        if let Some(fee) = get_field::<_, _, Option<String>>(env, &opts, "fee")? {
            new_opts = new_opts.fee(fee);
        }
        if let Some(is_sell_all) = read_opt_bool(env, &opts, "isSellAll")? {
            new_opts = new_opts.is_sell_all(is_sell_all);
        }
        if let Some(remark) = get_field::<_, _, Option<String>>(env, &opts, "remark")? {
            new_opts = new_opts.remark(remark);
        }
        if let Some(trade_method) =
            get_field::<_, _, Option<JavaInteger>>(env, &opts, "tradeMethod")?
        {
            new_opts = new_opts.trade_method(trade_method.into());
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.submit_order(new_opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundContextCancelOrder(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    order_id: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.cancel_order(order_id).await?)
        })?;
        Ok(())
    })
}
