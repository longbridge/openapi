use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{Config, FundamentalContext, fundamental::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, get_field},
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
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetBusinessSegments(
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
            let resp = context.ctx.business_segments(symbol).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetBusinessSegmentsHistory(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let report: Option<String> = get_field(env, &opts, "report")?;
        let cate: Option<String> = get_field(env, &opts, "cate")?;
        let report_static: Option<&'static str> = match report.as_deref() {
            Some("qf") => Some("qf"),
            Some("saf") => Some("saf"),
            Some("af") => Some("af"),
            _ => None,
        };
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .business_segments_history(symbol, report_static, cate)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetInstitutionRatingViews(
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
            let resp = context.ctx.institution_rating_views(symbol).await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetIndustryRank(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let market: String = get_field(env, &opts, "market")?;
        let indicator: String = get_field(env, &opts, "indicator")?;
        let sort_type: String = get_field(env, &opts, "sortType")?;
        let limit: i32 = get_field(env, &opts, "limit")?;
        let limit = limit.max(0) as u32;
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .industry_rank(market, indicator, sort_type, limit)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetIndustryPeers(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let counter_id: String = get_field(env, &opts, "counterId")?;
        let market: String = get_field(env, &opts, "market")?;
        let industry_id: Option<String> = get_field(env, &opts, "industryId")?;
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .industry_peers(counter_id, market, industry_id)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_fundamentalContextGetFinancialReportSnapshot(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let symbol: String = get_field(env, &opts, "symbol")?;
        let report: Option<String> = get_field(env, &opts, "report")?;
        let fiscal_year: Option<i32> = get_field(env, &opts, "fiscalYear")?;
        let fiscal_period: Option<String> = get_field(env, &opts, "fiscalPeriod")?;
        let report_static: Option<&'static str> = match report.as_deref() {
            Some("qf") => Some("qf"),
            Some("saf") => Some("saf"),
            Some("af") => Some("af"),
            _ => None,
        };
        let fiscal_period_static: Option<&'static str> = match fiscal_period.as_deref() {
            Some("q1") => Some("q1"),
            Some("q2") => Some("q2"),
            Some("q3") => Some("q3"),
            Some("q4") => Some("q4"),
            Some("fy") => Some("fy"),
            Some("h1") => Some("h1"),
            Some("h2") => Some("h2"),
            _ => None,
        };
        async_util::execute(env, callback, async move {
            let resp = context
                .ctx
                .financial_report_snapshot(symbol, report_static, fiscal_year, fiscal_period_static)
                .await?;
            Ok(resp)
        })?;
        Ok(())
    })
}
