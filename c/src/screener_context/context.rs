use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::ScreenerContext;

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    screener_context::types::*,
    types::{CCow, cstr_to_rust},
};

pub(crate) struct CScreenerContext {
    ctx: ScreenerContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_new(
    config: *const CConfig,
) -> *const CScreenerContext {
    let config = Arc::new((*config).0.clone());
    Arc::into_raw(Arc::new(CScreenerContext {
        ctx: ScreenerContext::new(config),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_retain(ctx: *const CScreenerContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_release(ctx: *const CScreenerContext) {
    Arc::decrement_strong_count(ctx);
}

/// Get recommended built-in screener strategies.
/// Returns `CScreenerRecommendStrategiesResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_recommend_strategies(
    ctx: *const CScreenerContext,
    market: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = cstr_to_rust(market);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CScreenerRecommendStrategiesResponseOwned> =
            CCow::new(CScreenerRecommendStrategiesResponseOwned::from(
                ctx_inner.screener_recommend_strategies(market).await?,
            ));
        Ok(resp)
    });
}

/// Get the current user's saved screener strategies.
/// Returns `CScreenerUserStrategiesResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_user_strategies(
    ctx: *const CScreenerContext,
    market: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = cstr_to_rust(market);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CScreenerUserStrategiesResponseOwned> =
            CCow::new(CScreenerUserStrategiesResponseOwned::from(
                ctx_inner.screener_user_strategies(market).await?,
            ));
        Ok(resp)
    });
}

/// Get detail for one screener strategy by ID.
/// Returns `CScreenerStrategyResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_strategy(
    ctx: *const CScreenerContext,
    id: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CScreenerStrategyResponseOwned> = CCow::new(
            CScreenerStrategyResponseOwned::from(ctx_inner.screener_strategy(id).await?),
        );
        Ok(resp)
    });
}

/// Search / screen securities using a strategy.
/// Returns `CScreenerSearchResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_search(
    ctx: *const CScreenerContext,
    market: *const c_char,
    strategy_id: i64,
    has_strategy_id: bool,
    page: u32,
    size: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = cstr_to_rust(market);
    let strategy_id = if has_strategy_id {
        Some(strategy_id)
    } else {
        None
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CScreenerSearchResponseOwned> =
            CCow::new(CScreenerSearchResponseOwned::from(
                ctx_inner
                    .screener_search(market, strategy_id, vec![], vec![], page, size)
                    .await?,
            ));
        Ok(resp)
    });
}

/// Get all available screener indicator definitions.
/// Returns `CScreenerIndicatorsResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_screener_context_indicators(
    ctx: *const CScreenerContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CScreenerIndicatorsResponseOwned> = CCow::new(
            CScreenerIndicatorsResponseOwned::from(ctx_inner.screener_indicators().await?),
        );
        Ok(resp)
    });
}
