use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, FundamentalContext, fundamental::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, ObjectArray, get_field},
};

struct ContextObj {
    ctx: FundamentalContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newFundamentalContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = FundamentalContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeFundamentalContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

// ── financial_report ─────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextFinancialReport(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let kind: Option<FinancialReportKind> = get_field(env, &opts, "kind")?;
        let kind = kind.unwrap_or(FinancialReportKind::All);
        let period: Option<FinancialReportPeriod> = get_field(env, &opts, "period")?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.financial_report(symbol, kind, period).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

// ── simple symbol-only methods ────────────────────────────────────

macro_rules! symbol_method {
    ($jni_name:ident, $method:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn $jni_name(
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
                    let resp = context.ctx.$method(symbol).await?;
                    Ok(resp)
                })?;
                Ok(())
            })
        }
    };
}

symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextInstitutionRating,
    institution_rating
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextInstitutionRatingDetail,
    institution_rating_detail
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextDividend,
    dividend
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextDividendDetail,
    dividend_detail
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextForecastEps,
    forecast_eps
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextConsensus,
    consensus
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextValuation,
    valuation
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextValuationHistory,
    valuation_history
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextIndustryValuation,
    industry_valuation
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextIndustryValuationDist,
    industry_valuation_dist
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextCompany,
    company
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextExecutive,
    executive
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextShareholder,
    shareholder
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextFundHolder,
    fund_holder
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextCorpAction,
    corp_action
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextInvestRelation,
    invest_relation
);
symbol_method!(
    Java_com_longbridge_SdkNative_fundamentalContextOperating,
    operating
);
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetBuyback(
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
            let resp = context.ctx.buyback(symbol).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetRatings(
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
            let resp = context.ctx.ratings(symbol).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextShareholderTop(
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
            let resp = context.ctx.shareholder_top(symbol).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextShareholderDetail(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JObject,
    object_id: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        async_util::execute(env, callback, async move {
            let resp = context.ctx.shareholder_detail(symbol, object_id).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextValuationComparison(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JObject,
    currency: JObject,
    comparison_symbols: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = FromJValue::from_jvalue(env, symbol.into())?;
        let currency: String = FromJValue::from_jvalue(env, currency.into())?;
        let comparison_syms: Option<Vec<String>> = if comparison_symbols.is_null() {
            None
        } else {
            let arr: ObjectArray<String> = FromJValue::from_jvalue(env, comparison_symbols.into())?;
            Some(arr.0)
        };
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .valuation_comparison(symbol, currency, comparison_syms)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextMacrodataIndicators(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    offset: JObject,
    limit: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let offset: Option<i32> = FromJValue::from_jvalue(env, offset.into())?;
        let limit: Option<i32> = FromJValue::from_jvalue(env, limit.into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.macrodata_indicators(offset, limit).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextMacrodata(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    indicator_code: JObject,
    start_time: JObject,
    end_time: JObject,
    limit: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let indicator_code: String = FromJValue::from_jvalue(env, indicator_code.into())?;
        let start_date: Option<String> = FromJValue::from_jvalue(env, start_time.into())?;
        let end_date: Option<String> = FromJValue::from_jvalue(env, end_time.into())?;
        let limit: Option<i32> = FromJValue::from_jvalue(env, limit.into())?;
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .macrodata(indicator_code, start_date, end_date, limit)
                .await?)
        })?;
        Ok(())
    })
}
