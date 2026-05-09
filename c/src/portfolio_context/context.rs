use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::PortfolioContext;

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    portfolio_context::types::*,
    types::{CCow, cstr_to_rust},
};

pub struct CPortfolioContext {
    ctx: PortfolioContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_new(
    config: *const CConfig,
) -> *const CPortfolioContext {
    let config = Arc::new((*config).0.clone());
    Arc::into_raw(Arc::new(CPortfolioContext {
        ctx: PortfolioContext::new(config),
    }))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_retain(ctx: *const CPortfolioContext) {
    Arc::increment_strong_count(ctx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_release(ctx: *const CPortfolioContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get exchange rates. Returns CExchangeRates.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_exchange_rate(
    ctx: *const CPortfolioContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CExchangeRatesOwned> =
            CCow::new(CExchangeRatesOwned::from(ctx_inner.exchange_rate().await?));
        Ok(resp)
    });
}

/// Get portfolio P&L analysis. Returns `CProfitAnalysis`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_profit_analysis(
    ctx: *const CPortfolioContext,
    start: *const c_char,
    end: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let start = if start.is_null() {
        None
    } else {
        Some(cstr_to_rust(start))
    };
    let end = if end.is_null() {
        None
    } else {
        Some(cstr_to_rust(end))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CProfitAnalysisOwned> = CCow::new(CProfitAnalysisOwned::from(
            ctx_inner.profit_analysis(start, end).await?,
        ));
        Ok(resp)
    });
}

/// Get P&L by market. Returns `CProfitAnalysisByMarket`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_profit_analysis_by_market(
    ctx: *const CPortfolioContext,
    market: *const c_char,
    start: *const c_char,
    end: *const c_char,
    currency: *const c_char,
    page: i32,
    size: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = if market.is_null() {
        None
    } else {
        Some(cstr_to_rust(market))
    };
    let start = if start.is_null() {
        None
    } else {
        Some(cstr_to_rust(start))
    };
    let end = if end.is_null() {
        None
    } else {
        Some(cstr_to_rust(end))
    };
    let currency = if currency.is_null() {
        None
    } else {
        Some(cstr_to_rust(currency))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CProfitAnalysisByMarketOwned> =
            CCow::new(CProfitAnalysisByMarketOwned::from(
                ctx_inner
                    .profit_analysis_by_market(
                        market,
                        start,
                        end,
                        currency,
                        page as u32,
                        size as u32,
                    )
                    .await?,
            ));
        Ok(resp)
    });
}

/// Get P&L flow records for a security. Returns `CProfitAnalysisFlows`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_profit_analysis_flows(
    ctx: *const CPortfolioContext,
    symbol: *const c_char,
    page: i32,
    size: i32,
    derivative: bool,
    start: *const c_char,
    end: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let start = if start.is_null() {
        None
    } else {
        Some(cstr_to_rust(start))
    };
    let end = if end.is_null() {
        None
    } else {
        Some(cstr_to_rust(end))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CProfitAnalysisFlowsOwned> = CCow::new(CProfitAnalysisFlowsOwned::from(
            ctx_inner
                .profit_analysis_flows(symbol, page as u32, size as u32, derivative, start, end)
                .await?,
        ));
        Ok(resp)
    });
}

/// Get P&L detail for a security. Returns `CProfitAnalysisDetail`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_portfolio_context_profit_analysis_detail(
    ctx: *const CPortfolioContext,
    symbol: *const c_char,
    start: *const c_char,
    end: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let start = if start.is_null() {
        None
    } else {
        Some(cstr_to_rust(start))
    };
    let end = if end.is_null() {
        None
    } else {
        Some(cstr_to_rust(end))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CProfitAnalysisDetailOwned> = CCow::new(CProfitAnalysisDetailOwned::from(
            ctx_inner.profit_analysis_detail(symbol, start, end).await?,
        ));
        Ok(resp)
    });
}
