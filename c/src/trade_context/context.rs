use std::{ffi::c_void, os::raw::c_char, sync::Arc, time::Instant};

use longport::{
    TradeContext,
    trade::{
        EstimateMaxPurchaseQuantityOptions, GetCashFlowOptions, GetFundPositionsOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetStockPositionsOptions,
        GetTodayExecutionsOptions, GetTodayOrdersOptions, PushEvent, ReplaceOrderOptions,
        SubmitOrderOptions,
    },
};
use parking_lot::Mutex;
use time::OffsetDateTime;

use crate::{
    async_call::{CAsyncCallback, CAsyncResult, execute_async},
    callback::{CFreeUserDataFunc, Callback},
    config::CConfig,
    trade_context::{
        enum_types::CTopicType,
        types::{
            CAccountBalanceOwned, CCashFlowOwned, CEstimateMaxPurchaseQuantityOptions,
            CEstimateMaxPurchaseQuantityResponseOwned, CExecutionOwned,
            CFundPositionsResponseOwned, CGetCashFlowOptions, CGetFundPositionsOptions,
            CGetHistoryExecutionsOptions, CGetHistoryOrdersOptions, CGetStockPositionsOptions,
            CGetTodayExecutionsOptions, CGetTodayOrdersOptions, CMarginRatioOwned,
            COrderDetailOwned, COrderOwned, CPushOrderChanged, CPushOrderChangedOwned,
            CReplaceOrderOptions, CStockPositionsResponseOwned, CSubmitOrderOptions,
            CSubmitOrderResponseOwned,
        },
    },
    types::{CCow, CVec, ToFFI, cstr_array_to_rust, cstr_to_rust},
};

pub type COnOrderChangedCallback =
    extern "C" fn(*const CTradeContext, *const CPushOrderChanged, *mut c_void);

#[derive(Default)]
struct Callbacks {
    order_changed: Option<Callback<COnOrderChangedCallback>>,
}

pub struct CTradeContextState {
    callbacks: Callbacks,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
}

unsafe impl Send for CTradeContextState {}

/// Trade context
pub struct CTradeContext {
    ctx: TradeContext,
    state: Mutex<CTradeContextState>,
}

impl Drop for CTradeContext {
    fn drop(&mut self) {
        let state = self.state.lock();
        if let Some(free_userdata) = state.free_userdata {
            free_userdata(state.userdata);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_new(
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
            let (ctx, mut receiver) = TradeContext::try_new(config).await?;
            let state = Mutex::new(CTradeContextState {
                userdata: std::ptr::null_mut(),
                callbacks: Callbacks::default(),
                free_userdata: None,
            });
            let arc_ctx = Arc::new(CTradeContext { ctx, state });
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
                        PushEvent::OrderChanged(order_changed) => {
                            if let Some(callback) = &state.callbacks.order_changed {
                                let log_subscriber = ctx.ctx.log_subscriber();
                                let _guard =
                                    tracing::dispatcher::set_default(&log_subscriber.into());

                                let s = Instant::now();
                                tracing::info!("begin call on_order_changed callback");

                                let order_changed_owned: CPushOrderChangedOwned =
                                    order_changed.into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &order_changed_owned.to_ffi_type(),
                                    callback.userdata,
                                );

                                tracing::info!(
                                    duration = ?s.elapsed(),
                                    "after call on_order_changed callback"
                                );
                            }
                        }
                    }
                }
            });

            Ok(CAsyncResult {
                ctx: ctx as *const c_void,
                error: std::ptr::null_mut(),
                data: std::ptr::null_mut(),
                length: 0,
                userdata: userdata_pointer as *mut c_void,
            })
        },
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_retain(ctx: *const CTradeContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_release(ctx: *const CTradeContext) {
    let _ = Arc::from_raw(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_ref_count(ctx: *const CTradeContext) -> usize {
    Arc::increment_strong_count(ctx);
    let ctx = Arc::from_raw(ctx);
    Arc::strong_count(&ctx)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_set_userdata(
    ctx: *const CTradeContext,
    userdata: *mut c_void,
) {
    (*ctx).state.lock().userdata = userdata;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_userdata(ctx: *const CTradeContext) -> *mut c_void {
    (*ctx).state.lock().userdata
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_set_free_userdata_func(
    ctx: *const CTradeContext,
    f: CFreeUserDataFunc,
) {
    (*ctx).state.lock().free_userdata = f;
}

/// Set order changed callback, after receiving the order changed event, it will
/// call back to this function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_set_on_order_changed(
    ctx: *const CTradeContext,
    callback: COnOrderChangedCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.order_changed = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_subscribe(
    ctx: *const CTradeContext,
    topics: *const CTopicType,
    num_topics: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let topics = std::slice::from_raw_parts(topics, num_topics)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.subscribe(topics).await
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_unsubscribe(
    ctx: *const CTradeContext,
    topics: *const CTopicType,
    num_topics: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let topics = std::slice::from_raw_parts(topics, num_topics)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.unsubscribe(topics).await
    });
}

/// Get history executions
///
/// @param[in] opts Options for get history executions request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_history_executions(
    ctx: *const CTradeContext,
    opts: *const CGetHistoryExecutionsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetHistoryExecutionsOptions::new();
    if !opts.is_null() {
        if !(*opts).symbol.is_null() {
            opts2 = opts2.symbol(cstr_to_rust((*opts).symbol));
        }
        if !(*opts).start_at.is_null() {
            opts2 = opts2.start_at(
                OffsetDateTime::from_unix_timestamp(*(*opts).start_at).expect("invalid start at"),
            );
        }
        if !(*opts).end_at.is_null() {
            opts2 = opts2.end_at(
                OffsetDateTime::from_unix_timestamp(*(*opts).end_at).expect("invalid end at"),
            );
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CExecutionOwned> = ctx_inner.history_executions(opts2).await?.into();
        Ok(rows)
    });
}

/// Get today executions
///
/// @param[in] opts Options for get today executions request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_today_executions(
    ctx: *const CTradeContext,
    opts: *const CGetTodayExecutionsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetTodayExecutionsOptions::new();
    if !opts.is_null() {
        if !(*opts).symbol.is_null() {
            opts2 = opts2.symbol(cstr_to_rust((*opts).symbol));
        }
        if !(*opts).order_id.is_null() {
            opts2 = opts2.order_id(cstr_to_rust((*opts).order_id));
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CExecutionOwned> = ctx_inner.today_executions(opts2).await?.into();
        Ok(rows)
    });
}

/// Get history orders
///
/// @param[in] opts Options for get history orders request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_history_orders(
    ctx: *const CTradeContext,
    opts: *const CGetHistoryOrdersOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetHistoryOrdersOptions::new();
    if !opts.is_null() {
        if !(*opts).symbol.is_null() {
            opts2 = opts2.symbol(cstr_to_rust((*opts).symbol));
        }
        if !(*opts).status.is_null() {
            let status = std::slice::from_raw_parts((*opts).status, (*opts).num_status);
            opts2 = opts2.status(status.iter().copied().map(Into::into));
        }
        if !(*opts).side.is_null() {
            opts2 = opts2.side((*(*opts).side).into());
        }
        if !(*opts).market.is_null() {
            opts2 = opts2.market((*(*opts).market).into());
        }
        if !(*opts).start_at.is_null() {
            opts2 = opts2.start_at(
                OffsetDateTime::from_unix_timestamp(*(*opts).start_at).expect("invalid start at"),
            );
        }
        if !(*opts).end_at.is_null() {
            opts2 = opts2.end_at(
                OffsetDateTime::from_unix_timestamp(*(*opts).end_at).expect("invalid end at"),
            );
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<COrderOwned> = ctx_inner.history_orders(opts2).await?.into();
        Ok(rows)
    });
}

/// Get today orders
///
/// @param[in] opts Options for get today orders request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_today_orders(
    ctx: *const CTradeContext,
    opts: *const CGetTodayOrdersOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetTodayOrdersOptions::new();
    if !opts.is_null() {
        if !(*opts).symbol.is_null() {
            opts2 = opts2.symbol(cstr_to_rust((*opts).symbol));
        }
        if !(*opts).status.is_null() {
            let status = std::slice::from_raw_parts((*opts).status, (*opts).num_status);
            opts2 = opts2.status(status.iter().copied().map(Into::into));
        }
        if !(*opts).side.is_null() {
            opts2 = opts2.side((*(*opts).side).into());
        }
        if !(*opts).market.is_null() {
            opts2 = opts2.market((*(*opts).market).into());
        }
        if !(*opts).order_id.is_null() {
            opts2 = opts2.order_id(cstr_to_rust((*opts).order_id));
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<COrderOwned> = ctx_inner.today_orders(opts2).await?.into();
        Ok(rows)
    });
}

/// Replace order
///
/// @param[in] opts Options for replace order request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_replace_order(
    ctx: *const CTradeContext,
    opts: *const CReplaceOrderOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust((*opts).order_id);
    let quantity = (*opts).quantity;
    let mut opts2 = ReplaceOrderOptions::new(order_id, (*quantity).value);
    if !(*opts).price.is_null() {
        opts2 = opts2.price((*(*opts).price).value);
    }
    if !(*opts).trigger_price.is_null() {
        opts2 = opts2.trigger_price((*(*opts).trigger_price).value);
    }
    if !(*opts).limit_offset.is_null() {
        opts2 = opts2.limit_offset((*(*opts).limit_offset).value);
    }
    if !(*opts).trailing_amount.is_null() {
        opts2 = opts2.trailing_amount((*(*opts).trailing_amount).value);
    }
    if !(*opts).trailing_percent.is_null() {
        opts2 = opts2.trailing_percent((*(*opts).trailing_percent).value);
    }
    if !(*opts).limit_depth_level.is_null() {
        opts2 = opts2.limit_depth_level(*(*opts).limit_depth_level);
    }
    if !(*opts).trigger_count.is_null() {
        opts2 = opts2.trigger_count(*(*opts).trigger_count);
    }
    if !(*opts).monitor_price.is_null() {
        opts2 = opts2.monitor_price((*(*opts).monitor_price).value);
    }
    if !(*opts).remark.is_null() {
        opts2 = opts2.remark(cstr_to_rust((*opts).remark));
    }
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.replace_order(opts2).await?;
        Ok(())
    });
}

/// Submit order
///
/// @param[in] opts Options for submit order request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_submit_order(
    ctx: *const CTradeContext,
    opts: *const CSubmitOrderOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust((*opts).symbol);
    let order_type = (*opts).order_type.into();
    let side = (*opts).side.into();
    let submitted_quantity = (*opts).submitted_quantity;
    let time_in_force = (*opts).time_in_force.into();
    let mut opts2 = SubmitOrderOptions::new(
        symbol,
        order_type,
        side,
        (*submitted_quantity).value,
        time_in_force,
    );
    if !(*opts).submitted_price.is_null() {
        opts2 = opts2.submitted_price((*(*opts).submitted_price).value);
    }
    if !(*opts).trigger_price.is_null() {
        opts2 = opts2.trigger_price((*(*opts).trigger_price).value);
    }
    if !(*opts).limit_offset.is_null() {
        opts2 = opts2.limit_offset((*(*opts).limit_offset).value);
    }
    if !(*opts).trailing_amount.is_null() {
        opts2 = opts2.trailing_amount((*(*opts).trailing_amount).value);
    }
    if !(*opts).trailing_percent.is_null() {
        opts2 = opts2.trailing_percent((*(*opts).trailing_percent).value);
    }
    if !(*opts).expire_date.is_null() {
        opts2 = opts2.expire_date((*(*opts).expire_date).into());
    }
    if !(*opts).outside_rth.is_null() {
        opts2 = opts2.outside_rth((*(*opts).outside_rth).into());
    }
    if !(*opts).limit_depth_level.is_null() {
        opts2 = opts2.limit_depth_level(*(*opts).limit_depth_level);
    }
    if !(*opts).trigger_count.is_null() {
        opts2 = opts2.trigger_count(*(*opts).trigger_count);
    }
    if !(*opts).monitor_price.is_null() {
        opts2 = opts2.monitor_price((*(*opts).monitor_price).value);
    }
    if !(*opts).remark.is_null() {
        opts2 = opts2.remark(cstr_to_rust((*opts).remark));
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSubmitOrderResponseOwned> = CCow::new(ctx_inner.submit_order(opts2).await?);
        Ok(resp)
    });
}

/// Cancel order
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_cancel_order(
    ctx: *const CTradeContext,
    order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust(order_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.cancel_order(order_id).await?;
        Ok(())
    });
}

/// Get account balance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_account_balance(
    ctx: *const CTradeContext,
    currency: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let currency = (!currency.is_null()).then(|| cstr_to_rust(currency));
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CAccountBalanceOwned> =
            ctx_inner.account_balance(currency.as_deref()).await?.into();
        Ok(rows)
    });
}

/// Get cash flow
///
/// @param[in] opts Options for get cash flow request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_cash_flow(
    ctx: *const CTradeContext,
    opts: *const CGetCashFlowOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let start_at =
        OffsetDateTime::from_unix_timestamp((*opts).start_at).expect("invalid start time");
    let end_at = OffsetDateTime::from_unix_timestamp((*opts).end_at).expect("invalid end time");
    let mut opts2 = GetCashFlowOptions::new(start_at, end_at);
    if !(*opts).business_type.is_null() {
        opts2 = opts2.business_type((*(*opts).business_type).into());
    }
    if !(*opts).symbol.is_null() {
        opts2 = opts2.symbol(cstr_to_rust((*opts).symbol));
    }
    if !(*opts).page.is_null() {
        opts2 = opts2.page(*(*opts).page);
    }
    if !(*opts).size.is_null() {
        opts2 = opts2.size(*(*opts).size);
    }
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCashFlowOwned> = ctx_inner.cash_flow(opts2).await?.into();
        Ok(rows)
    });
}

/// Get fund positions
///
/// @param[in] opts Options for get fund positions request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_fund_positions(
    ctx: *const CTradeContext,
    opts: *const CGetFundPositionsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetFundPositionsOptions::new();
    if !opts.is_null() && !(*opts).symbols.is_null() {
        opts2 = opts2.symbols(cstr_array_to_rust((*opts).symbols, (*opts).num_symbols));
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CFundPositionsResponseOwned> =
            CCow::new(ctx_inner.fund_positions(opts2).await?);
        Ok(resp)
    });
}

/// Get stock positions
///
/// @param[in] opts Options for get stock positions request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_stock_positions(
    ctx: *const CTradeContext,
    opts: *const CGetStockPositionsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetStockPositionsOptions::new();
    if !opts.is_null() && !(*opts).symbols.is_null() {
        opts2 = opts2.symbols(cstr_array_to_rust((*opts).symbols, (*opts).num_symbols));
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CStockPositionsResponseOwned> =
            CCow::new(ctx_inner.stock_positions(opts2).await?);
        Ok(resp)
    });
}

/// Get margin ratio
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_margin_ratio(
    ctx: *const CTradeContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CMarginRatioOwned> = CCow::new(ctx_inner.margin_ratio(symbol).await?);
        Ok(resp)
    });
}

/// Get order detail
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_order_detail(
    ctx: *const CTradeContext,
    order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust(order_id);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<COrderDetailOwned> = CCow::new(ctx_inner.order_detail(order_id).await?);
        Ok(resp)
    });
}

/// Get order detail
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_trade_context_estimate_max_purchase_quantity(
    ctx: *const CTradeContext,
    opts: *const CEstimateMaxPurchaseQuantityOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust((*opts).symbol);
    let order_type = (*opts).order_type.into();
    let side = (*opts).side.into();
    let mut opts2 = EstimateMaxPurchaseQuantityOptions::new(symbol, order_type, side);
    if !(*opts).price.is_null() {
        opts2 = opts2.price((*(*opts).price).value);
    }
    if !(*opts).currency.is_null() {
        opts2 = opts2.currency(cstr_to_rust((*opts).currency));
    }
    if !(*opts).order_id.is_null() {
        opts2 = opts2.order_id(cstr_to_rust((*opts).order_id));
    }
    if (*opts).fractional_shares {
        opts2 = opts2.fractional_shares();
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CEstimateMaxPurchaseQuantityResponseOwned> =
            CCow::new(ctx_inner.estimate_max_purchase_quantity(opts2).await?);
        Ok(resp)
    });
}
