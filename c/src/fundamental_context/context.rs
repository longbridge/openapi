use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{FundamentalContext, fundamental::types::*};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    fundamental_context::{enum_types::*, types::*},
    types::{CCow, cstr_to_rust},
};

pub(crate) struct CFundamentalContext {
    ctx: FundamentalContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_new(
    config: *const CConfig,
) -> *const CFundamentalContext {
    let config = Arc::new((*config).0.clone());
    Arc::into_raw(Arc::new(CFundamentalContext {
        ctx: FundamentalContext::new(config),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_retain(ctx: *const CFundamentalContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_release(ctx: *const CFundamentalContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get financial reports — returns `CFinancialReports` (list_json is JSON
/// string)
///
/// @param kind   report kind enum value
/// @param period 0=af, 1=saf, 2=q1, 3=q2, 4=q3, 5=qf, -1=none
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_financial_report(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    kind: CFinancialReportKind,
    period: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let k: FinancialReportKind = kind.into();
    let p = match period {
        0 => Some(FinancialReportPeriod::Annual),
        1 => Some(FinancialReportPeriod::SemiAnnual),
        2 => Some(FinancialReportPeriod::Q1),
        3 => Some(FinancialReportPeriod::Q2),
        4 => Some(FinancialReportPeriod::Q3),
        5 => Some(FinancialReportPeriod::QuarterlyFull),
        _ => None,
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFinancialReportsOwned> = CCow::new(CFinancialReportsOwned::from(
            ctx_inner.financial_report(symbol, k, p).await?,
        ));
        Ok(resp)
    });
}

/// Get analyst ratings. Returns `CInstitutionRating`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_institution_rating(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let r: CCow<CInstitutionRatingOwned> = CCow::new(CInstitutionRatingOwned::from(
            ctx_inner.institution_rating(symbol).await?,
        ));
        Ok(r)
    });
}

/// Get analyst rating detail. Returns `CInstitutionRatingDetail`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_institution_rating_detail(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CInstitutionRatingDetailOwned> = CCow::new(
            CInstitutionRatingDetailOwned::from(ctx_inner.institution_rating_detail(symbol).await?),
        );
        Ok(_r)
    });
}

/// Get dividend history. Returns `CDividendList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_dividend(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CDividendListOwned> =
            CCow::new(CDividendListOwned::from(ctx_inner.dividend(symbol).await?));
        Ok(_r)
    });
}

/// Get detailed dividend information. Returns `CDividendList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_dividend_detail(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CDividendListOwned> = CCow::new(CDividendListOwned::from(
            ctx_inner.dividend_detail(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get EPS forecasts. Returns `CForecastEps`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_forecast_eps(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CForecastEpsOwned> = CCow::new(CForecastEpsOwned::from(
            ctx_inner.forecast_eps(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get valuation metrics. Returns `CValuationData`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_valuation(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CValuationDataOwned> = CCow::new(CValuationDataOwned::from(
            ctx_inner.valuation(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get historical valuation data. Returns `CValuationHistoryResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_valuation_history(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CValuationHistoryResponseOwned> = CCow::new(
            CValuationHistoryResponseOwned::from(ctx_inner.valuation_history(symbol).await?),
        );
        Ok(_r)
    });
}

/// Get company overview. Returns `CCompanyOverview`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_company(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CCompanyOverviewOwned> = CCow::new(CCompanyOverviewOwned::from(
            ctx_inner.company(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get major shareholders. Returns `CShareholderList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_shareholder(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CShareholderListOwned> = CCow::new(CShareholderListOwned::from(
            ctx_inner.shareholder(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get fund and ETF holders. Returns `CFundHolders`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_fund_holder(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CFundHoldersOwned> = CCow::new(CFundHoldersOwned::from(
            ctx_inner.fund_holder(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get corporate actions. Returns `CCorpActions`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_corp_action(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CCorpActionsOwned> = CCow::new(CCorpActionsOwned::from(
            ctx_inner.corp_action(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get investor relations data. Returns `CInvestRelations`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_invest_relation(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<CInvestRelationsOwned> = CCow::new(CInvestRelationsOwned::from(
            ctx_inner.invest_relation(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get operating metrics. Returns `COperatingList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_operating(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let _r: CCow<COperatingListOwned> = CCow::new(COperatingListOwned::from(
            ctx_inner.operating(symbol).await?,
        ));
        Ok(_r)
    });
}

/// Get consensus estimates. Returns `CFinancialConsensus`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_consensus(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFinancialConsensusOwned> = CCow::new(CFinancialConsensusOwned::from(
            ctx_inner.consensus(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get industry valuation. Returns `CIndustryValuationList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_industry_valuation(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CIndustryValuationListOwned> = CCow::new(CIndustryValuationListOwned::from(
            ctx_inner.industry_valuation(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get industry valuation distribution. Returns `CIndustryValuationDist`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_industry_valuation_dist(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CIndustryValuationDistOwned> = CCow::new(CIndustryValuationDistOwned::from(
            ctx_inner.industry_valuation_dist(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get executive info. Returns `CExecutiveList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_executive(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CExecutiveListOwned> = CCow::new(CExecutiveListOwned::from(
            ctx_inner.executive(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get buyback data. Returns `CBuybackData`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_buyback(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBuybackDataOwned> =
            CCow::new(CBuybackDataOwned::from(ctx_inner.buyback(symbol).await?));
        Ok(resp)
    });
}

/// Get stock ratings. Returns `CStockRatings`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_ratings(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CStockRatingsOwned> =
            CCow::new(CStockRatingsOwned::from(ctx_inner.ratings(symbol).await?));
        Ok(resp)
    });
}

/// Get business segment breakdowns. Returns `CBusinessSegments`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_business_segments(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBusinessSegmentsOwned> = CCow::new(CBusinessSegmentsOwned::from(
            ctx_inner.business_segments(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get historical business segment breakdowns. Returns
/// `CBusinessSegmentsHistory`.
///
/// Pass `NULL` for `report` or `cate` to omit those parameters.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_business_segments_history(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    report: *const c_char,
    cate: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let report_str = if report.is_null() {
        None
    } else {
        Some(cstr_to_rust(report))
    };
    let cate_opt = if cate.is_null() {
        None
    } else {
        Some(cstr_to_rust(cate))
    };
    let report_static: Option<&'static str> = match report_str.as_deref() {
        Some("qf") => Some("qf"),
        Some("saf") => Some("saf"),
        Some("af") => Some("af"),
        _ => None,
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBusinessSegmentsHistoryOwned> =
            CCow::new(CBusinessSegmentsHistoryOwned::from(
                ctx_inner
                    .business_segments_history(symbol, report_static, cate_opt)
                    .await?,
            ));
        Ok(resp)
    });
}

/// Get historical institutional rating views. Returns
/// `CInstitutionRatingViews`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_institution_rating_views(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CInstitutionRatingViewsOwned> = CCow::new(
            CInstitutionRatingViewsOwned::from(ctx_inner.institution_rating_views(symbol).await?),
        );
        Ok(resp)
    });
}

/// Get industry rank for a market. Returns `CIndustryRankResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_industry_rank(
    ctx: *const CFundamentalContext,
    market: *const c_char,
    indicator: *const c_char,
    sort_type: *const c_char,
    limit: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = cstr_to_rust(market);
    let indicator = cstr_to_rust(indicator);
    let sort_type = cstr_to_rust(sort_type);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CIndustryRankResponseOwned> = CCow::new(CIndustryRankResponseOwned::from(
            ctx_inner
                .industry_rank(market, indicator, sort_type, limit)
                .await?,
        ));
        Ok(resp)
    });
}

/// Get industry peer chain. Returns `CIndustryPeersResponse`.
///
/// Pass `NULL` for `industry_id` to omit it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_industry_peers(
    ctx: *const CFundamentalContext,
    counter_id: *const c_char,
    market: *const c_char,
    industry_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let counter_id = cstr_to_rust(counter_id);
    let market = cstr_to_rust(market);
    let industry_id_opt = if industry_id.is_null() {
        None
    } else {
        Some(cstr_to_rust(industry_id))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CIndustryPeersResponseOwned> = CCow::new(CIndustryPeersResponseOwned::from(
            ctx_inner
                .industry_peers(counter_id, market, industry_id_opt)
                .await?,
        ));
        Ok(resp)
    });
}

/// Get financial report snapshot. Returns `CFinancialReportSnapshot`.
///
/// Pass `NULL` for optional parameters to omit them.
/// `fiscal_year` is ignored when 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_financial_report_snapshot(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    report: *const c_char,
    fiscal_year: i32,
    fiscal_period: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let report_str = if report.is_null() {
        None
    } else {
        Some(cstr_to_rust(report))
    };
    let fiscal_year_opt = if fiscal_year == 0 {
        None
    } else {
        Some(fiscal_year)
    };
    let fiscal_period_str = if fiscal_period.is_null() {
        None
    } else {
        Some(cstr_to_rust(fiscal_period))
    };
    let report_static: Option<&'static str> = match report_str.as_deref() {
        Some("qf") => Some("qf"),
        Some("saf") => Some("saf"),
        Some("af") => Some("af"),
        _ => None,
    };
    let fiscal_period_static: Option<&'static str> = match fiscal_period_str.as_deref() {
        Some("q1") => Some("q1"),
        Some("q2") => Some("q2"),
        Some("q3") => Some("q3"),
        Some("q4") => Some("q4"),
        Some("fy") => Some("fy"),
        Some("h1") => Some("h1"),
        Some("h2") => Some("h2"),
        _ => None,
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFinancialReportSnapshotOwned> =
            CCow::new(CFinancialReportSnapshotOwned::from(
                ctx_inner
                    .financial_report_snapshot(
                        symbol,
                        report_static,
                        fiscal_year_opt,
                        fiscal_period_static,
                    )
                    .await?,
            ));
        Ok(resp)
    });
}
