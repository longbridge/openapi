use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{FundamentalContext, fundamental::types::*};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    fundamental_context::{enum_types::*, types::*},
    types::{CCow, cstr_to_rust},
};

// Helper: convert a nullable C string to an Option<&'static str> by matching
// known enum-like values (e.g. report period codes).
#[inline]
unsafe fn cstr_to_static_opt(ptr: *const c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }
    let s = cstr_to_rust(ptr);
    // Match against all known period/report values used across APIs.
    match s.as_str() {
        "qf" => Some("qf"),
        "saf" => Some("saf"),
        "af" => Some("af"),
        "q1" => Some("q1"),
        "q2" => Some("q2"),
        "q3" => Some("q3"),
        "annual" => Some("annual"),
        "semi_annual" => Some("semi_annual"),
        "quarterly" => Some("quarterly"),
        _ => None,
    }
}

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

/// Get ranked list of top shareholders. Returns `CShareholderTopResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_shareholder_top(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CShareholderTopResponseOwned> = CCow::new(
            CShareholderTopResponseOwned::from(ctx_inner.shareholder_top(symbol).await?),
        );
        Ok(resp)
    });
}

/// Get holding history and detail for one shareholder. Returns
/// `CShareholderDetailResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_shareholder_detail(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    object_id: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CShareholderDetailResponseOwned> =
            CCow::new(CShareholderDetailResponseOwned::from(
                ctx_inner.shareholder_detail(symbol, object_id).await?,
            ));
        Ok(resp)
    });
}

/// Get current business segment breakdown for a security.
/// Returns `CBusinessSegments`.
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

/// Get historical business segment breakdowns for a security.
/// Returns `CBusinessSegmentsHistory`.
/// Pass NULL for `report` and/or `cate` to omit those filters.
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
    let report: Option<&'static str> = cstr_to_static_opt(report);
    let cate: Option<String> = if cate.is_null() {
        None
    } else {
        Some(cstr_to_rust(cate))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBusinessSegmentsHistoryOwned> =
            CCow::new(CBusinessSegmentsHistoryOwned::from(
                ctx_inner
                    .business_segments_history(symbol, report, cate)
                    .await?,
            ));
        Ok(resp)
    });
}

/// Get historical institutional rating views for a security.
/// Returns `CInstitutionRatingViews`.
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

/// Get industry rank for a market.
/// Returns `CIndustryRankResponse`.
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

/// Get the industry peer chain for a security or industry.
/// Returns `CIndustryPeersResponse`.
/// Pass NULL for `industry_id` to omit it.
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
    let industry_id: Option<String> = if industry_id.is_null() {
        None
    } else {
        Some(cstr_to_rust(industry_id))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CIndustryPeersResponseOwned> = CCow::new(CIndustryPeersResponseOwned::from(
            ctx_inner
                .industry_peers(counter_id, market, industry_id)
                .await?,
        ));
        Ok(resp)
    });
}

/// Get a financial report snapshot for a security.
/// Returns `CFinancialReportSnapshot`.
/// Pass NULL for `report`, `fiscal_year_str`, and/or `fiscal_period` to omit
/// them. `fiscal_year_str` should be a decimal integer string (e.g. `"2024"`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_financial_report_snapshot(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    report: *const c_char,
    fiscal_year_str: *const c_char,
    fiscal_period: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let report: Option<&'static str> = cstr_to_static_opt(report);
    let fiscal_year: Option<i32> = if fiscal_year_str.is_null() {
        None
    } else {
        cstr_to_rust(fiscal_year_str).parse::<i32>().ok()
    };
    let fiscal_period: Option<&'static str> = cstr_to_static_opt(fiscal_period);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFinancialReportSnapshotOwned> =
            CCow::new(CFinancialReportSnapshotOwned::from(
                ctx_inner
                    .financial_report_snapshot(symbol, report, fiscal_year, fiscal_period)
                    .await?,
            ));
        Ok(resp)
    });
}

/// Get valuation comparison between a security and optional peers.
/// Returns `CValuationComparisonResponse`.
/// Pass NULL for `comparison_symbols` to skip peer comparison.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_valuation_comparison(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    currency: *const c_char,
    comparison_symbols: *const *const c_char,
    num_comparison_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let currency = cstr_to_rust(currency);
    let comparison = if comparison_symbols.is_null() || num_comparison_symbols == 0 {
        None
    } else {
        let syms: Vec<String> = (0..num_comparison_symbols)
            .map(|i| cstr_to_rust(*comparison_symbols.add(i)))
            .collect();
        Some(syms)
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CValuationComparisonResponseOwned> =
            CCow::new(CValuationComparisonResponseOwned::from(
                ctx_inner
                    .valuation_comparison(symbol, currency, comparison)
                    .await?,
            ));
        Ok(resp)
    });
}

/// Get ETF asset allocation (holdings / regional / asset class / industry).
/// Returns `CAssetAllocationResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fundamental_context_etf_asset_allocation(
    ctx: *const CFundamentalContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CAssetAllocationResponseOwned> = CCow::new(
            CAssetAllocationResponseOwned::from(ctx_inner.etf_asset_allocation(symbol).await?),
        );
        Ok(resp)
    });
}
