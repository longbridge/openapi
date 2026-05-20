use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{MarketContext, market::types::*};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    market_context::{enum_types::*, types::*},
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
/// Returns `CBrokerHoldingTop`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_broker_holding(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    period: CBrokerHoldingPeriod,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let p: BrokerHoldingPeriod = period.into();
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
/// @param count   Number of K-lines
/// Returns `CAhPremiumKlines`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_ah_premium(
    ctx: *const CMarketContext,
    symbol: *const c_char,
    period: CAhPremiumPeriod,
    count: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let p: AhPremiumPeriod = period.into();
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

/// Get stock events across one or more markets.
/// Pass markets as a NULL-terminated array of C strings.
/// Returns `CStockEventsResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_stock_events(
    ctx: *const CMarketContext,
    markets: *const *const c_char,
    num_markets: usize,
    sort: u32,
    date: *const c_char,
    limit: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let markets: Vec<String> = (0..num_markets)
        .map(|i| cstr_to_rust(*markets.add(i)))
        .collect();
    let date = if date.is_null() {
        None
    } else {
        Some(cstr_to_rust(date))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CStockEventsResponseOwned> = CCow::new(CStockEventsResponseOwned::from(
            ctx_inner.stock_events(markets, sort, date, limit).await?,
        ));
        Ok(resp)
    });
}

/// Get all available rank category keys and labels.
/// Returns `CRankCategoriesResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_rank_categories(
    ctx: *const CMarketContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CRankCategoriesResponseOwned> = CCow::new(
            CRankCategoriesResponseOwned::from(ctx_inner.rank_categories().await?),
        );
        Ok(resp)
    });
}

/// Get a ranked list of securities for the given category key.
/// Returns `CRankListResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_market_context_rank_list(
    ctx: *const CMarketContext,
    key: *const c_char,
    need_article: bool,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let key = cstr_to_rust(key);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CRankListResponseOwned> = CCow::new(CRankListResponseOwned::from(
            ctx_inner.rank_list(key, need_article).await?,
        ));
        Ok(resp)
    });
}
