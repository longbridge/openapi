use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{
    FundContext,
    fund::{
        FundNavRangeOptions, FundPageOptions, GetFundAnalysisOptions, GetFundHoldingsOptions,
        GetFundOrdersOptions, GetFundPositionDividendsOptions, GetFundPositionOptions,
        GetFundPositionProfitsOptions, GetFundPositionsOptions, GetFundStockHoldingsOptions,
        GetFundTransactionsOptions, GetFundsOptions, SubmitFundOrderOptions,
        ValidateFundOrderOptions,
    },
};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    fund_context::types::{
        CFundAnalysisDetailOwned, CFundAnalysisOwned, CFundAnnualReturnOwned, CFundBriefOwned,
        CFundDetailOwned, CFundDividendsOwned, CFundFiltersOwned, CFundHoldingsOwned,
        CFundNavRangeOptions, CFundNavValueOwned, CFundOrderDetailOwned, CFundOrderOwned,
        CFundOrderSubmitResponseOwned, CFundOrderValidationOwned, CFundPageOptions,
        CFundPerformanceComparisonOwned, CFundPerformanceOwned, CFundPositionDetailOwned,
        CFundPositionNavOwned, CFundPositionPerformanceOwned, CFundPositionProfitsOwned,
        CFundPositionsOwned, CFundQuarterlyReturnOwned, CFundStockHoldingOwned,
        CFundTransactionOwned, CFundTrendOwned, CGetFundAnalysisOptions, CGetFundHoldingsOptions,
        CGetFundOrdersOptions, CGetFundPositionDividendsOptions, CGetFundPositionOptions,
        CGetFundPositionProfitsOptions, CGetFundPositionsOptions, CGetFundStockHoldingsOptions,
        CGetFundTransactionsOptions, CGetFundsOptions, CHotFundOwned, CSubmitFundOrderOptions,
        CValidateFundOrderOptions,
    },
    types::{CCow, CVec, cstr_array_to_rust, cstr_to_rust, slice_from_raw_parts},
};

pub struct CFundContext {
    ctx: FundContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_new(config: *const CConfig) -> *const CFundContext {
    Arc::into_raw(Arc::new(CFundContext {
        ctx: FundContext::new(Arc::new((*config).0.clone())),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_retain(ctx: *const CFundContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_release(ctx: *const CFundContext) {
    let _ = Arc::from_raw(ctx);
}

// ── option builders ─────────────────────────────────────────────────────────

unsafe fn build_get_funds_options(opts: *const CGetFundsOptions) -> GetFundsOptions {
    let mut o = GetFundsOptions::new();
    if opts.is_null() {
        return o;
    }
    if !(*opts).filter.is_null()
        && let Ok(value) = serde_json::from_str(&cstr_to_rust((*opts).filter))
    {
        o = o.filter(value);
    }
    let quick_ids = slice_from_raw_parts((*opts).quick_ids, (*opts).num_quick_ids).to_vec();
    if !quick_ids.is_empty() {
        o = o.quick_ids(quick_ids);
    }
    let time_interval = cstr_array_to_rust((*opts).time_interval, (*opts).num_time_interval);
    if !time_interval.is_empty() {
        o = o.time_interval(time_interval);
    }
    o
}

unsafe fn build_analysis_options(opts: *const CGetFundAnalysisOptions) -> GetFundAnalysisOptions {
    let mut o = GetFundAnalysisOptions::new();
    if !opts.is_null() && !(*opts).period.is_null() {
        o = o.period(*(*opts).period);
    }
    o
}

unsafe fn build_page_options(opts: *const CFundPageOptions) -> FundPageOptions {
    let mut o = FundPageOptions::new();
    if !opts.is_null() {
        if !(*opts).page.is_null() {
            o = o.page(*(*opts).page);
        }
        if !(*opts).size.is_null() {
            o = o.size(*(*opts).size);
        }
    }
    o
}

unsafe fn build_nav_range_options(opts: *const CFundNavRangeOptions) -> FundNavRangeOptions {
    let mut o = FundNavRangeOptions::new();
    if !opts.is_null() {
        if !(*opts).month_before.is_null() {
            o = o.month_before(*(*opts).month_before);
        }
        if !(*opts).year_before.is_null() {
            o = o.year_before(*(*opts).year_before);
        }
    }
    o
}

unsafe fn build_holdings_options(opts: *const CGetFundHoldingsOptions) -> GetFundHoldingsOptions {
    let mut o = GetFundHoldingsOptions::new();
    if !opts.is_null() && !(*opts).scene.is_null() {
        o = o.scene(*(*opts).scene);
    }
    o
}

unsafe fn build_stock_holdings_options(
    opts: *const CGetFundStockHoldingsOptions,
) -> GetFundStockHoldingsOptions {
    let mut o = GetFundStockHoldingsOptions::new();
    if !opts.is_null() && !(*opts).limit.is_null() {
        o = o.limit(*(*opts).limit);
    }
    o
}

unsafe fn build_positions_options(
    opts: *const CGetFundPositionsOptions,
) -> GetFundPositionsOptions {
    let mut o = GetFundPositionsOptions::new();
    if !opts.is_null() {
        if !(*opts).account_channel.is_null() {
            o = o.account_channel(cstr_to_rust((*opts).account_channel));
        }
        if !(*opts).aaid.is_null() {
            o = o.aaid(*(*opts).aaid);
        }
    }
    o
}

unsafe fn build_position_options(opts: *const CGetFundPositionOptions) -> GetFundPositionOptions {
    let mut o = GetFundPositionOptions::new();
    if !opts.is_null() {
        if !(*opts).account_channel.is_null() {
            o = o.account_channel(cstr_to_rust((*opts).account_channel));
        }
        if !(*opts).aaid.is_null() {
            o = o.aaid(*(*opts).aaid);
        }
        if !(*opts).start.is_null() {
            o = o.start(cstr_to_rust((*opts).start));
        }
        if !(*opts).end.is_null() {
            o = o.end(cstr_to_rust((*opts).end));
        }
    }
    o
}

unsafe fn build_position_profits_options(
    opts: *const CGetFundPositionProfitsOptions,
) -> GetFundPositionProfitsOptions {
    let mut o = GetFundPositionProfitsOptions::new();
    if !opts.is_null() {
        if !(*opts).account_channel.is_null() {
            o = o.account_channel(cstr_to_rust((*opts).account_channel));
        }
        if !(*opts).aaid.is_null() {
            o = o.aaid(*(*opts).aaid);
        }
        if !(*opts).start.is_null() {
            o = o.start(cstr_to_rust((*opts).start));
        }
        if !(*opts).end.is_null() {
            o = o.end(cstr_to_rust((*opts).end));
        }
        if !(*opts).page.is_null() {
            o = o.page(*(*opts).page);
        }
        if !(*opts).size.is_null() {
            o = o.size(*(*opts).size);
        }
    }
    o
}

unsafe fn build_position_dividends_options(
    opts: *const CGetFundPositionDividendsOptions,
) -> GetFundPositionDividendsOptions {
    let mut o = GetFundPositionDividendsOptions::new();
    if !opts.is_null() {
        if !(*opts).account_channel.is_null() {
            o = o.account_channel(cstr_to_rust((*opts).account_channel));
        }
        if !(*opts).aaid.is_null() {
            o = o.aaid(*(*opts).aaid);
        }
        if !(*opts).currency.is_null() {
            o = o.currency(cstr_to_rust((*opts).currency));
        }
        if !(*opts).start.is_null() {
            o = o.start(*(*opts).start);
        }
        if !(*opts).end.is_null() {
            o = o.end(*(*opts).end);
        }
        if !(*opts).page.is_null() {
            o = o.page(*(*opts).page);
        }
        if !(*opts).size.is_null() {
            o = o.size(*(*opts).size);
        }
    }
    o
}

unsafe fn build_orders_options(opts: *const CGetFundOrdersOptions) -> GetFundOrdersOptions {
    let mut o = GetFundOrdersOptions::new();
    if opts.is_null() {
        return o;
    }
    let symbols = cstr_array_to_rust((*opts).symbols, (*opts).num_symbols);
    if !symbols.is_empty() {
        o = o.symbols(symbols);
    }
    if !(*opts).actions.is_null() {
        o = o.actions(cstr_to_rust((*opts).actions));
    }
    if !(*opts).states.is_null() {
        o = o.states(cstr_to_rust((*opts).states));
    }
    if !(*opts).currency.is_null() {
        o = o.currency(cstr_to_rust((*opts).currency));
    }
    if !(*opts).start.is_null() {
        o = o.start(*(*opts).start);
    }
    if !(*opts).end.is_null() {
        o = o.end(*(*opts).end);
    }
    if !(*opts).page.is_null() {
        o = o.page(*(*opts).page);
    }
    if !(*opts).size.is_null() {
        o = o.size(*(*opts).size);
    }
    o
}

unsafe fn build_transactions_options(
    opts: *const CGetFundTransactionsOptions,
) -> GetFundTransactionsOptions {
    let mut o = GetFundTransactionsOptions::new();
    if opts.is_null() {
        return o;
    }
    if !(*opts).account_channel.is_null() {
        o = o.account_channel(cstr_to_rust((*opts).account_channel));
    }
    if !(*opts).business_type.is_null() {
        o = o.business_type(cstr_to_rust((*opts).business_type));
    }
    if !(*opts).category.is_null() {
        o = o.category(cstr_to_rust((*opts).category));
    }
    if !(*opts).currencies.is_null() {
        o = o.currencies(cstr_to_rust((*opts).currencies));
    }
    if !(*opts).start.is_null() {
        o = o.start(*(*opts).start);
    }
    if !(*opts).end.is_null() {
        o = o.end(*(*opts).end);
    }
    if !(*opts).page.is_null() {
        o = o.page(*(*opts).page);
    }
    if !(*opts).size.is_null() {
        o = o.size(*(*opts).size);
    }
    o
}

unsafe fn build_validate_order_options(
    opts: *const CValidateFundOrderOptions,
) -> ValidateFundOrderOptions {
    let mut o = ValidateFundOrderOptions::new(
        cstr_to_rust((*opts).symbol),
        cstr_to_rust((*opts).action),
        cstr_to_rust((*opts).currency),
    );
    if !(*opts).amount.is_null() {
        o = o.amount(cstr_to_rust((*opts).amount));
    }
    if !(*opts).units.is_null() {
        o = o.units(cstr_to_rust((*opts).units));
    }
    if !(*opts).dividend_option.is_null() {
        o = o.dividend_option(*(*opts).dividend_option);
    }
    if !(*opts).fund_source.is_null() {
        o = o.fund_source(*(*opts).fund_source);
    }
    if !(*opts).account_channel.is_null() {
        o = o.account_channel(cstr_to_rust((*opts).account_channel));
    }
    o
}

unsafe fn build_submit_order_options(
    opts: *const CSubmitFundOrderOptions,
) -> SubmitFundOrderOptions {
    let mut o = SubmitFundOrderOptions::new(
        cstr_to_rust((*opts).symbol),
        cstr_to_rust((*opts).action),
        cstr_to_rust((*opts).currency),
    );
    if !(*opts).amount.is_null() {
        o = o.amount(cstr_to_rust((*opts).amount));
    }
    if !(*opts).units.is_null() {
        o = o.units(cstr_to_rust((*opts).units));
    }
    if !(*opts).dividend_option.is_null() {
        o = o.dividend_option(*(*opts).dividend_option);
    }
    if !(*opts).fee.is_null() {
        o = o.fee(cstr_to_rust((*opts).fee));
    }
    if !(*opts).is_sell_all.is_null() {
        o = o.is_sell_all(*(*opts).is_sell_all);
    }
    if !(*opts).remark.is_null() {
        o = o.remark(cstr_to_rust((*opts).remark));
    }
    if !(*opts).trade_method.is_null() {
        o = o.trade_method(*(*opts).trade_method);
    }
    o
}

// ── catalog / market data ───────────────────────────────────────────────────

/// Get the hot-selling fund list
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_hot_funds(
    ctx: *const CFundContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CHotFundOwned> = ctx_inner.hot_funds().await?.into();
        Ok(rows)
    });
}

/// Get the fund list
///
/// @param[in] opts Options for get funds request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_funds(
    ctx: *const CFundContext,
    opts: *const CGetFundsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts = build_get_funds_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundBriefOwned> = ctx_inner.funds(opts).await?.into();
        Ok(rows)
    });
}

/// Get the fund list filter options
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_filters(
    ctx: *const CFundContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundFiltersOwned> = CCow::new(ctx_inner.filters().await?);
        Ok(resp)
    });
}

/// Get fund detail
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_detail(
    ctx: *const CFundContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundDetailOwned> = CCow::new(ctx_inner.detail(symbol).await?);
        Ok(resp)
    });
}

/// Get fund analysis (level 1)
///
/// @param[in] opts Options for the analysis request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_analysis(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundAnalysisOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_analysis_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundAnalysisOwned> = CCow::new(ctx_inner.analysis(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get fund analysis detail (level 2)
///
/// @param[in] opts Options for the analysis request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_analysis_detail(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundAnalysisOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_analysis_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundAnalysisDetailOwned> =
            CCow::new(ctx_inner.analysis_detail(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get fund trend chart
///
/// @param[in] opts Options for the trend request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_trend(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundAnalysisOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_analysis_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundTrendOwned> = CCow::new(ctx_inner.trend(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get fund annual returns
///
/// @param[in] opts Paging options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_annual_returns(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CFundPageOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_page_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundAnnualReturnOwned> =
            ctx_inner.annual_returns(symbol, opts).await?.into();
        Ok(rows)
    });
}

/// Get fund quarterly returns
///
/// @param[in] opts Paging options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_quarterly_returns(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CFundPageOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_page_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundQuarterlyReturnOwned> =
            ctx_inner.quarterly_returns(symbol, opts).await?.into();
        Ok(rows)
    });
}

/// Get fund performance figures
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_performance(
    ctx: *const CFundContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundPerformanceOwned> = ctx_inner.performance(symbol).await?.into();
        Ok(rows)
    });
}

/// Get fund performance comparison
///
/// @param[in] opts Options for the comparison request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_performance_comparison(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundAnalysisOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_analysis_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundPerformanceComparisonOwned> =
            CCow::new(ctx_inner.performance_comparison(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get fund latest net value
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_nav(
    ctx: *const CFundContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundNavValueOwned> = ctx_inner.nav(symbol).await?.into();
        Ok(rows)
    });
}

/// Get fund historical net value (paged)
///
/// @param[in] opts Paging options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_nav_history(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CFundPageOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_page_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundNavValueOwned> = ctx_inner.nav_history(symbol, opts).await?.into();
        Ok(rows)
    });
}

/// Get fund historical net value by relative time range
///
/// @param[in] opts Net-value range options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_nav_range(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CFundNavRangeOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_nav_range_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundNavValueOwned> = ctx_inner.nav_range(symbol, opts).await?.into();
        Ok(rows)
    });
}

/// Get a fund's top-10 holdings
///
/// @param[in] opts Options for the holdings request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_holdings(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundHoldingsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_holdings_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundHoldingsOwned> = CCow::new(ctx_inner.holdings(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get the stocks held by a fund (reverse lookup)
///
/// @param[in] opts Options for the stock-holdings request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_stock_holdings(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundStockHoldingsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_stock_holdings_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundStockHoldingOwned> =
            ctx_inner.stock_holdings(symbol, opts).await?.into();
        Ok(rows)
    });
}

// ── user fund positions ─────────────────────────────────────────────────────

/// Get the user's fund positions overview
///
/// @param[in] opts Options for the positions request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_positions(
    ctx: *const CFundContext,
    opts: *const CGetFundPositionsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts = build_positions_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundPositionsOwned> = CCow::new(ctx_inner.positions(opts).await?);
        Ok(resp)
    });
}

/// Get the user's single fund position detail
///
/// @param[in] opts Options for the position request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_position(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundPositionOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_position_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundPositionDetailOwned> =
            CCow::new(ctx_inner.position(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get the performance figures of a held fund
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_position_performance(
    ctx: *const CFundContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundPositionPerformanceOwned> =
            ctx_inner.position_performance(symbol).await?.into();
        Ok(rows)
    });
}

/// Get the cumulative-profit series of a held fund
///
/// @param[in] opts Options for the profits request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_position_profits(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundPositionProfitsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_position_profits_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundPositionProfitsOwned> =
            CCow::new(ctx_inner.position_profits(symbol, opts).await?);
        Ok(resp)
    });
}

/// Get the net-value history of a held fund
///
/// @param[in] opts Net-value range options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_position_nav(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CFundNavRangeOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_nav_range_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundPositionNavOwned> = ctx_inner.position_nav(symbol, opts).await?.into();
        Ok(rows)
    });
}

/// Get the dividend records of a held fund
///
/// @param[in] opts Options for the dividends request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_position_dividends(
    ctx: *const CFundContext,
    symbol: *const c_char,
    opts: *const CGetFundPositionDividendsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let opts = build_position_dividends_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundDividendsOwned> =
            CCow::new(ctx_inner.position_dividends(symbol, opts).await?);
        Ok(resp)
    });
}

// ── fund orders & trading ───────────────────────────────────────────────────

/// Get the user's fund orders
///
/// @param[in] opts Options for the orders request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_orders(
    ctx: *const CFundContext,
    opts: *const CGetFundOrdersOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts = build_orders_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundOrderOwned> = ctx_inner.orders(opts).await?.into();
        Ok(rows)
    });
}

/// Get a fund order detail
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_order(
    ctx: *const CFundContext,
    order_id: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundOrderDetailOwned> = CCow::new(ctx_inner.order(order_id).await?);
        Ok(resp)
    });
}

/// Get the user's fund transactions (cash-flow records)
///
/// @param[in] opts Options for the transactions request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_transactions(
    ctx: *const CFundContext,
    opts: *const CGetFundTransactionsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts = build_transactions_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CFundTransactionOwned> = ctx_inner.transactions(opts).await?.into();
        Ok(rows)
    });
}

/// Validate a fund order before submitting
///
/// @param[in] opts Options for the validate request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_validate_order(
    ctx: *const CFundContext,
    opts: *const CValidateFundOrderOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts = build_validate_order_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundOrderValidationOwned> =
            CCow::new(ctx_inner.validate_order(opts).await?);
        Ok(resp)
    });
}

/// Submit a fund order (buy / sell)
///
/// @param[in] opts Options for the submit request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_submit_order(
    ctx: *const CFundContext,
    opts: *const CSubmitFundOrderOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts = build_submit_order_options(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundOrderSubmitResponseOwned> =
            CCow::new(ctx_inner.submit_order(opts).await?);
        Ok(resp)
    });
}

/// Cancel (withdraw) a fund order
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_fund_context_cancel_order(
    ctx: *const CFundContext,
    order_id: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.cancel_order(order_id).await?;
        Ok(())
    });
}
