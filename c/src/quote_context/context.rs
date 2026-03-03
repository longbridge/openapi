use std::{
    ffi::{CString, c_void},
    os::raw::c_char,
    sync::{Arc, OnceLock},
    time::Instant,
};

use longport::{
    QuoteContext,
    quote::{
        PushEvent, PushEventDetail, RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup,
        SubFlags,
    },
};
use parking_lot::Mutex;

use crate::{
    async_call::{CAsyncCallback, CAsyncResult, execute_async},
    callback::{CFreeUserDataFunc, Callback},
    config::CConfig,
    quote_context::{
        enum_types::{
            CAdjustType, CCalcIndex, CFilterWarrantExpiryDate, CFilterWarrantInOutBoundsType,
            CPeriod, CSecurityListCategory, CSortOrderType, CTradeSessions, CWarrantSortBy,
            CWarrantStatus, CWarrantType,
        },
        types::{
            CCandlestickOwned, CCapitalDistributionResponseOwned, CCapitalFlowLineOwned,
            CCreateWatchlistGroup, CHistoryMarketTemperatureResponseOwned, CIntradayLineOwned,
            CIssuerInfoOwned, CMarketTemperatureOwned, CMarketTradingDaysOwned,
            CMarketTradingSessionOwned, COptionQuoteOwned, CParticipantInfoOwned, CPushBrokers,
            CPushBrokersOwned, CPushCandlestick, CPushCandlestickOwned, CPushDepth,
            CPushDepthOwned, CPushQuote, CPushQuoteOwned, CPushTrades, CPushTradesOwned,
            CQuotePackageDetailOwned, CRealtimeQuoteOwned, CSecurityBrokersOwned,
            CSecurityCalcIndexOwned, CSecurityDepthOwned, CSecurityOwned, CSecurityQuoteOwned,
            CSecurityStaticInfoOwned, CStrikePriceInfoOwned, CSubscriptionOwned, CTradeOwned,
            CUpdateWatchlistGroup, CWarrantInfoOwned, CWarrantQuoteOwned, CWatchlistGroupOwned,
            LB_WATCHLIST_GROUP_NAME, LB_WATCHLIST_GROUP_SECURITIES,
        },
    },
    types::{CCow, CDate, CDateTime, CMarket, CVec, ToFFI, cstr_array_to_rust, cstr_to_rust},
};

pub type COnQuoteCallback = extern "C" fn(*const CQuoteContext, *const CPushQuote, *mut c_void);

pub type COnDepthCallback = extern "C" fn(*const CQuoteContext, *const CPushDepth, *mut c_void);

pub type COnBrokersCallback = extern "C" fn(*const CQuoteContext, *const CPushBrokers, *mut c_void);

pub type COnTradesCallback = extern "C" fn(*const CQuoteContext, *const CPushTrades, *mut c_void);

pub type COnCandlestickCallback =
    extern "C" fn(*const CQuoteContext, *const CPushCandlestick, *mut c_void);

#[derive(Default)]
struct Callbacks {
    quote: Option<Callback<COnQuoteCallback>>,
    depth: Option<Callback<COnDepthCallback>>,
    brokers: Option<Callback<COnBrokersCallback>>,
    trades: Option<Callback<COnTradesCallback>>,
    candlestick: Option<Callback<COnCandlestickCallback>>,
}

pub struct CQuoteContextState {
    callbacks: Callbacks,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
}

unsafe impl Send for CQuoteContextState {}

/// Quote context
pub struct CQuoteContext {
    ctx: QuoteContext,
    quote_level: OnceLock<CString>,
    state: Mutex<CQuoteContextState>,
}

impl Drop for CQuoteContext {
    fn drop(&mut self) {
        let state = self.state.lock();
        if let Some(free_userdata) = state.free_userdata {
            free_userdata(state.userdata);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_new(
    config: *const CConfig,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let config = (*config).0.clone();
    let userdata_pointer = userdata as usize;

    execute_async(
        callback,
        std::ptr::null_mut::<c_void>(),
        userdata,
        async move {
            let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
            let state = Mutex::new(CQuoteContextState {
                userdata: std::ptr::null_mut(),
                callbacks: Callbacks::default(),
                free_userdata: None,
            });
            let arc_ctx = Arc::new(CQuoteContext {
                ctx,
                quote_level: OnceLock::new(),
                state,
            });
            let weak_ctx = Arc::downgrade(&arc_ctx);
            let ctx = Arc::into_raw(arc_ctx);

            tokio::spawn(async move {
                while let Some(event) = receiver.recv().await {
                    let ctx = match weak_ctx.upgrade() {
                        Some(ctx) => ctx,
                        None => return,
                    };

                    let state = ctx.state.lock();
                    match event {
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Quote(quote),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.quote {
                                let log_subscriber = ctx.ctx.log_subscriber();
                                let _guard =
                                    tracing::dispatcher::set_default(&log_subscriber.into());

                                let s = Instant::now();
                                tracing::info!("begin call on_quote callback");

                                let quote_owned: CPushQuoteOwned = (symbol, quote).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &quote_owned.to_ffi_type(),
                                    callback.userdata,
                                );

                                tracing::info!(
                                    duration = ?s.elapsed(),
                                    "after call on_quote callback"
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Depth(depth),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.depth {
                                let log_subscriber = ctx.ctx.log_subscriber();
                                let _guard =
                                    tracing::dispatcher::set_default(&log_subscriber.into());

                                let s = Instant::now();
                                tracing::info!("begin call on_depth callback");

                                let depth_owned: CPushDepthOwned = (symbol, depth).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &depth_owned.to_ffi_type(),
                                    callback.userdata,
                                );

                                tracing::info!(
                                    duration = ?s.elapsed(),
                                    "after call on_depth callback"
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Brokers(brokers),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.brokers {
                                let log_subscriber = ctx.ctx.log_subscriber();
                                let _guard =
                                    tracing::dispatcher::set_default(&log_subscriber.into());

                                let s = Instant::now();
                                tracing::info!("begin call on_brokers callback");

                                let brokers_owned: CPushBrokersOwned = (symbol, brokers).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &brokers_owned.to_ffi_type(),
                                    callback.userdata,
                                );

                                tracing::info!(
                                    duration = ?s.elapsed(),
                                    "after call on_brokers callback"
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Trade(trades),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.trades {
                                let log_subscriber = ctx.ctx.log_subscriber();
                                let _guard =
                                    tracing::dispatcher::set_default(&log_subscriber.into());

                                let s = Instant::now();
                                tracing::info!("begin call on_trades callback");

                                let trades_owned: CPushTradesOwned = (symbol, trades).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &trades_owned.to_ffi_type(),
                                    callback.userdata,
                                );

                                tracing::info!(
                                    duration = ?s.elapsed(),
                                    "after call on_trades callback"
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Candlestick(candlestick),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.candlestick {
                                let log_subscriber = ctx.ctx.log_subscriber();
                                let _guard =
                                    tracing::dispatcher::set_default(&log_subscriber.into());

                                let s = Instant::now();
                                tracing::info!("begin call on_candlestick callback");

                                let candlestick_owned: CPushCandlestickOwned =
                                    (symbol, candlestick).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &candlestick_owned.to_ffi_type(),
                                    callback.userdata,
                                );

                                tracing::info!(
                                    duration = ?s.elapsed(),
                                    "after call on_candlestick callback"
                                );
                            }
                        }
                    }
                }
            });

            Ok(CAsyncResult {
                ctx: ctx as *const c_void,
                error: std::ptr::null(),
                data: std::ptr::null_mut(),
                length: 0,
                userdata: userdata_pointer as *mut c_void,
            })
        },
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_retain(ctx: *const CQuoteContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_release(ctx: *const CQuoteContext) {
    let _ = Arc::from_raw(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_ref_count(ctx: *const CQuoteContext) -> usize {
    Arc::increment_strong_count(ctx);
    let ctx = Arc::from_raw(ctx);
    Arc::strong_count(&ctx)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_userdata(
    ctx: *const CQuoteContext,
    userdata: *mut c_void,
) {
    (*ctx).state.lock().userdata = userdata;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_userdata(ctx: *const CQuoteContext) -> *mut c_void {
    (*ctx).state.lock().userdata
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_free_userdata_func(
    ctx: *const CQuoteContext,
    f: CFreeUserDataFunc,
) {
    (*ctx).state.lock().free_userdata = f;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_member_id(ctx: *const CQuoteContext) -> i64 {
    (*ctx).ctx.member_id()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_quote_level(ctx: *const CQuoteContext) -> *const c_char {
    let quote_level = (*ctx)
        .quote_level
        .get_or_init(|| CString::new((*ctx).ctx.quote_level()).unwrap());
    quote_level.as_ptr() as *const _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_quote_package_details(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CQuotePackageDetailOwned> =
            ctx_inner.quote_package_details().to_vec().into();
        Ok(rows)
    });
}

/// Set quote callback, after receiving the quote data push, it will call back
/// to this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_on_quote(
    ctx: *const CQuoteContext,
    callback: COnQuoteCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.quote = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set depth callback, after receiving the depth data push, it will call
/// back to this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_on_depth(
    ctx: *const CQuoteContext,
    callback: COnDepthCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.depth = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set brokers callback, after receiving the brokers data push, it will
/// call back to this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_on_brokers(
    ctx: *const CQuoteContext,
    callback: COnBrokersCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.brokers = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set trades callback, after receiving the trades data push, it will call
/// back to this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_on_trades(
    ctx: *const CQuoteContext,
    callback: COnTradesCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.trades = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set candlestick callback, after receiving the trades data push, it will
/// call back to this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_set_on_candlestick(
    ctx: *const CQuoteContext,
    callback: COnCandlestickCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.candlestick = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_subscribe(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    sub_types: u8,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .subscribe(
                symbols,
                SubFlags::from_bits(sub_types).unwrap_or_else(SubFlags::empty),
            )
            .await
    });
}

/// Unsubscribe
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_unsubscribe(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    sub_types: u8,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .unsubscribe(
                symbols,
                SubFlags::from_bits(sub_types).unwrap_or_else(SubFlags::empty),
            )
            .await
    });
}

/// Subscribe security candlesticks
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_subscribe_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    trade_sessions: CTradeSessions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .subscribe_candlesticks(symbol, period.into(), trade_sessions.into())
            .await?
            .into();
        Ok(rows)
    });
}

/// Unsubscribe security candlesticks
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_unsubscribe_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .unsubscribe_candlesticks(symbol, period.into())
            .await
    });
}

/// Get subscription information
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_subscrptions(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSubscriptionOwned> = ctx_inner.subscriptions().await?.into();
        Ok(rows)
    });
}

/// Get basic information of securities
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_static_info(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSecurityStaticInfoOwned> = ctx_inner.static_info(symbols).await?.into();
        Ok(rows)
    });
}

/// Get quote of securities
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSecurityQuoteOwned> = ctx_inner.quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get quote of option securities
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_option_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<COptionQuoteOwned> = ctx_inner.option_quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get quote of warrant securities
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_warrant_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CWarrantQuoteOwned> = ctx_inner.warrant_quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get security depth
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_depth(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityDepthOwned> = CCow::new(
            ctx_inner
                .depth(symbol)
                .await
                .map(CSecurityDepthOwned::from)?,
        );
        Ok(resp)
    });
}

/// Get security brokers
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_brokers(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityBrokersOwned> = CCow::new(
            ctx_inner
                .brokers(symbol)
                .await
                .map(CSecurityBrokersOwned::from)?,
        );
        Ok(resp)
    });
}

/// Get participants
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_participants(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CParticipantInfoOwned> = ctx_inner.participants().await?.into();
        Ok(rows)
    });
}

/// Get security trades
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_trades(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    count: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CTradeOwned> = ctx_inner.trades(symbol, count).await?.into();
        Ok(rows)
    });
}

/// Get security intraday lines
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_intraday(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    trade_sessions: CTradeSessions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CIntradayLineOwned> = ctx_inner
            .intraday(symbol, trade_sessions.into())
            .await?
            .into();
        Ok(rows)
    });
}

/// Get security candlesticks
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    count: usize,
    adjust_type: CAdjustType,
    trade_sessions: CTradeSessions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .candlesticks(
                symbol,
                period.into(),
                count,
                adjust_type.into(),
                trade_sessions.into(),
            )
            .await?
            .into();
        Ok(rows)
    });
}

/// Get security history candlesticks by offset
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_history_candlesticks_by_offset(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    adjust_type: CAdjustType,
    forward: bool,
    time: *const CDateTime,
    count: usize,
    trade_sessions: CTradeSessions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let time = if !time.is_null() {
        Some((*time).into())
    } else {
        None
    };
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .history_candlesticks_by_offset(
                symbol,
                period.into(),
                adjust_type.into(),
                forward,
                time,
                count,
                trade_sessions.into(),
            )
            .await?
            .into();
        Ok(rows)
    });
}

/// Get security history candlesticks by date
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_history_candlesticks_by_date(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    adjust_type: CAdjustType,
    start: *const CDate,
    end: *const CDate,
    trade_sessions: CTradeSessions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let start = if start.is_null() {
        None
    } else {
        Some((*start).into())
    };
    let end = if end.is_null() {
        None
    } else {
        Some((*end).into())
    };
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .history_candlesticks_by_date(
                symbol,
                period.into(),
                adjust_type.into(),
                start,
                end,
                trade_sessions.into(),
            )
            .await?
            .into();
        Ok(rows)
    });
}

/// Get option chain expiry date list
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_option_chain_expiry_date_list(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CDate> = ctx_inner
            .option_chain_expiry_date_list(symbol)
            .await?
            .into();
        Ok(rows)
    });
}

/// Get option chain info by date
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_option_chain_info_by_date(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    expiry_date: *const CDate,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let expiry_date = (*expiry_date).into();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CStrikePriceInfoOwned> = ctx_inner
            .option_chain_info_by_date(symbol, expiry_date)
            .await?
            .into();
        Ok(rows)
    });
}

/// Get warrant issuers
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_warrant_issuers(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CIssuerInfoOwned> = ctx_inner.warrant_issuers().await?.into();
        Ok(rows)
    });
}

/// Query warrant list
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_warrant_list(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    sort_by: CWarrantSortBy,
    sort_order: CSortOrderType,
    warrant_type: *const CWarrantType,
    num_warrant_type: usize,
    issuer: *const i32,
    num_issuer: usize,
    expiry_date: *const CFilterWarrantExpiryDate,
    num_expiry_date: usize,
    price_type: *const CFilterWarrantInOutBoundsType,
    num_price_type: usize,
    status: *const CWarrantStatus,
    num_status: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let sort_by = sort_by.into();
    let sort_order = sort_order.into();
    let warrant_type = std::slice::from_raw_parts(warrant_type, num_warrant_type)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    let issuer = std::slice::from_raw_parts(issuer, num_issuer).to_vec();
    let expiry_date = std::slice::from_raw_parts(expiry_date, num_expiry_date)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    let price_type = std::slice::from_raw_parts(price_type, num_price_type)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    let status = std::slice::from_raw_parts(status, num_status)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CWarrantInfoOwned> = ctx_inner
            .warrant_list(
                symbol,
                sort_by,
                sort_order,
                Some(&warrant_type),
                Some(&issuer),
                Some(&expiry_date),
                Some(&price_type),
                Some(&status),
            )
            .await?
            .into();
        Ok(rows)
    });
}

/// Get trading session of the day
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_trading_session(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CMarketTradingSessionOwned> = ctx_inner.trading_session().await?.into();
        Ok(rows)
    });
}

/// Get market trading days
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_trading_days(
    ctx: *const CQuoteContext,
    market: CMarket,
    begin: *const CDate,
    end: *const CDate,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = market.into();
    let begin = *begin;
    let end = *end;
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CMarketTradingDaysOwned> = CCow::new(
            ctx_inner
                .trading_days(market, begin.into(), end.into())
                .await?,
        );
        Ok(resp)
    });
}

/// Get capital flow intraday
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_capital_flow(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCapitalFlowLineOwned> = ctx_inner.capital_flow(symbol).await?.into();
        Ok(rows)
    });
}

/// Get capital distribution
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_capital_distribution(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CCapitalDistributionResponseOwned> =
            CCow::new(ctx_inner.capital_distribution(symbol).await?);
        Ok(resp)
    });
}

/// Get calc indexes
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_calc_indexes(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    indexes: *const CCalcIndex,
    num_indexes: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    let indexes = std::slice::from_raw_parts(indexes, num_indexes)
        .iter()
        .map(|index| (*index).into())
        .collect::<Vec<_>>();
    execute_async(callback, ctx, userdata, async move {
        let resp: CVec<CSecurityCalcIndexOwned> =
            ctx_inner.calc_indexes(symbols, indexes).await?.into();
        Ok(resp)
    });
}

/// Get watchlist
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_watchlist(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CVec<CWatchlistGroupOwned> = ctx_inner.watchlist().await?.into();
        Ok(resp)
    });
}

/// Create watchlist group
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_create_watchlist_group(
    ctx: *const CQuoteContext,
    req: &CCreateWatchlistGroup,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let name = cstr_to_rust(req.name);
    let securities = std::slice::from_raw_parts(req.securities, req.num_securities);
    let securities = (req.num_securities > 0).then(|| {
        securities
            .iter()
            .map(|symbol| cstr_to_rust(*symbol))
            .collect::<Vec<_>>()
    });
    execute_async(callback, ctx, userdata, async move {
        let group_id = ctx_inner
            .create_watchlist_group(RequestCreateWatchlistGroup { name, securities })
            .await?;
        Ok(group_id)
    });
}

/// Delete watchlist group
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_delete_watchlist_group(
    ctx: *const CQuoteContext,
    id: i64,
    purge: bool,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.delete_watchlist_group(id, purge).await?;
        Ok(())
    });
}

/// Create watchlist group
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_update_watchlist_group(
    ctx: *const CQuoteContext,
    req: &CUpdateWatchlistGroup,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = req.id;
    let name = ((req.flags & LB_WATCHLIST_GROUP_NAME) != 0).then(|| cstr_to_rust(req.name));
    let securities = std::slice::from_raw_parts(req.securities, req.num_securities);
    let securities = ((req.flags & LB_WATCHLIST_GROUP_SECURITIES) != 0).then(|| {
        securities
            .iter()
            .map(|symbol| cstr_to_rust(*symbol))
            .collect::<Vec<_>>()
    });
    let mode = req.mode;
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .update_watchlist_group(RequestUpdateWatchlistGroup {
                id,
                name,
                securities,
                mode: mode.into(),
            })
            .await?;
        Ok(())
    });
}

/// Get quote of securities
///
/// Get real-time quotes of the subscribed symbols, it always returns the data
/// in the local storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_realtime_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CRealtimeQuoteOwned> = ctx_inner.realtime_quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get real-time depth
///
/// Get real-time depth of the subscribed symbols, it always returns the data in
/// the local storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_realtime_depth(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityDepthOwned> = CCow::new(ctx_inner.realtime_depth(symbol).await?);
        Ok(resp)
    });
}

/// Get real-time trades
///
/// Get real-time trades of the subscribed symbols, it always returns the data
/// in the local storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_realtime_trades(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    count: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CTradeOwned> = ctx_inner.realtime_trades(symbol, count).await?.into();
        Ok(rows)
    });
}

/// Get real-time broker queue
///
/// Get real-time broker queue of the subscribed symbols, it always returns the
/// data in the local storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_realtime_brokers(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityBrokersOwned> =
            CCow::new(ctx_inner.realtime_brokers(symbol).await?);
        Ok(resp)
    });
}

/// Get real-time candlesticks
///
/// Get real-time candlesticks of the subscribed symbols, it always returns the
/// data in the local storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_realtime_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    count: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .realtime_candlesticks(symbol, period.into(), count)
            .await?
            .into();
        Ok(rows)
    });
}

/// Get security list
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_security_list(
    ctx: *const CQuoteContext,
    market: CMarket,
    category: *const CSecurityListCategory,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let category = if !category.is_null() {
        Some((*category).into())
    } else {
        None
    };
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSecurityOwned> = ctx_inner
            .security_list(market.into(), category)
            .await?
            .into();
        Ok(rows)
    });
}

/// Get current market temperature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_market_temperature(
    ctx: *const CQuoteContext,
    market: CMarket,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CMarketTemperatureOwned> =
            CCow::new(ctx_inner.market_temperature(market.into()).await?);
        Ok(resp)
    });
}

/// Get historical market temperature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_quote_context_history_market_temperature(
    ctx: *const CQuoteContext,
    market: CMarket,
    start: *const CDate,
    end: *const CDate,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let start = (*start).into();
    let end = (*end).into();

    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CHistoryMarketTemperatureResponseOwned> = CCow::new(
            ctx_inner
                .history_market_temperature(market.into(), start, end)
                .await?,
        );
        Ok(resp)
    });
}
