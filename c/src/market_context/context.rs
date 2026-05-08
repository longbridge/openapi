use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{MarketContext, market::types::*};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    market_context::types::*,
    types::{CCow, cstr_to_rust},
};

/// Market data context
pub struct CMarketContext {
    ctx: MarketContext,
}

/// Create a new `MarketContext`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_new(config: *const CConfig) -> *const CMarketContext {
    let config = Arc::new((*config).0.clone());
    Arc::into_raw(Arc::new(CMarketContext {
        ctx: MarketContext::new(config),
    }))
}

/// Retain the market context
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_retain(ctx: *const CMarketContext) {
    Arc::increment_strong_count(ctx);
}

/// Release the market context
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_release(ctx: *const CMarketContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get market trading status
///
/// Returns `CMarketStatusResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_market_status(
    ctx: *const CMarketContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CMarketStatusResponseOwned> = CCow::new(CMarketStatusResponseOwned::from(
            ctx_inner.market_status().await?,
        ));
        Ok(resp)
    });
}

/// Get top broker holdings
///
/// @param period  0=rct_1, 1=rct_5, 2=rct_20, 3=rct_60
/// Returns `CBrokerHoldingTop`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_broker_holding(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    period: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let p = match period {
        1 => BrokerHoldingPeriod::Rct5,
        2 => BrokerHoldingPeriod::Rct20,
        3 => BrokerHoldingPeriod::Rct60,
        _ => BrokerHoldingPeriod::Rct1,
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBrokerHoldingTopOwned> = CCow::new(CBrokerHoldingTopOwned::from(
            ctx_inner.broker_holding(symbol, p).await?,
        ));
        Ok(resp)
    });
}

/// Get full broker holding details
/// Returns `CBrokerHoldingDetail`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_broker_holding_detail(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBrokerHoldingDetailOwned> = CCow::new(CBrokerHoldingDetailOwned::from(
            ctx_inner.broker_holding_detail(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get daily broker holding history
/// Returns `CBrokerHoldingDailyHistory`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_broker_holding_daily(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    broker_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let broker_id = cstr_to_rust(broker_id);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CBrokerHoldingDailyHistoryOwned> =
            CCow::new(CBrokerHoldingDailyHistoryOwned::from(
                ctx_inner.broker_holding_daily(symbol, broker_id).await?,
            ));
        Ok(resp)
    });
}

/// Get A/H premium K-lines
///
/// @param period  0=1m,1=5m,2=15m,3=30m,4=60m,5=day,6=week,7=month,8=year
/// @param count   Number of K-lines
/// Returns `CAhPremiumKlines`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_ah_premium(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    period: i32,
    count: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let p = match period {
        0 => AhPremiumPeriod::Min1,
        1 => AhPremiumPeriod::Min5,
        2 => AhPremiumPeriod::Min15,
        3 => AhPremiumPeriod::Min30,
        4 => AhPremiumPeriod::Min60,
        6 => AhPremiumPeriod::Week,
        7 => AhPremiumPeriod::Month,
        8 => AhPremiumPeriod::Year,
        _ => AhPremiumPeriod::Day,
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CAhPremiumKlinesOwned> = CCow::new(CAhPremiumKlinesOwned::from(
            ctx_inner.ah_premium(symbol, p, count).await?,
        ));
        Ok(resp)
    });
}

/// Get A/H premium intraday data
/// Returns `CAhPremiumIntraday`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_ah_premium_intraday(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CAhPremiumIntradayOwned> = CCow::new(CAhPremiumIntradayOwned::from(
            ctx_inner.ah_premium_intraday(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get trade statistics
/// Returns `CTradeStatsResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_trade_stats(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CTradeStatsResponseOwned> = CCow::new(CTradeStatsResponseOwned::from(
            ctx_inner.trade_stats(symbol).await?,
        ));
        Ok(resp)
    });
}

/// Get market anomaly alerts
/// Returns `CAnomalyResponse`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_anomaly(
    ctx: *const CMarketContext,
    market: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = cstr_to_rust(market);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CAnomalyResponseOwned> = CCow::new(CAnomalyResponseOwned::from(
            ctx_inner.anomaly(market).await?,
        ));
        Ok(resp)
    });
}

/// Get index constituent stocks
/// Returns `CIndexConstituents`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_constituent(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CIndexConstituentsOwned> = CCow::new(CIndexConstituentsOwned::from(
            ctx_inner.constituent(symbol).await?,
        ));
        Ok(resp)
    });
}
