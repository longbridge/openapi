use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{
    GridContext,
    grid::{
        GetGridOrderDetailOptions, GetGridOrdersByIdsOptions, GetGridOrdersOptions,
        GetGridTriggerHistoryOptions, GridLimitEvent, GridTimeInForce, GridTradeRule,
        ReplaceGridOrderOptions, SubmitGridOrderOptions, TriggerPriceType,
    },
};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    grid_context::types::{
        CGetGridOrderDetailOptions, CGetGridOrdersByIdsOptions, CGetGridOrdersOptions,
        CGetGridTriggerHistoryOptions, CGridOrderDetailOwned, CGridOrderOwned,
        CGridOrdersResponseOwned, CGridSymbolInfoOwned, CGridTradeRule,
        CGridTriggerHistoryResponseOwned, CReplaceGridOrderOptions, CSubmitGridOrderOptions,
        CSubmitGridOrderResponseOwned,
    },
    types::{CCow, CVec, ToFFI, cstr_array_to_rust, cstr_to_rust},
};

pub struct CGridContext {
    ctx: GridContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_new(config: *const CConfig) -> *const CGridContext {
    Arc::into_raw(Arc::new(CGridContext {
        ctx: GridContext::new(Arc::new((*config).0.clone())),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_retain(ctx: *const CGridContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_release(ctx: *const CGridContext) {
    let _ = Arc::from_raw(ctx);
}

/// Build a Rust `GridTradeRule` from the C representation.
unsafe fn build_grid_trade_rule(rule: &CGridTradeRule) -> GridTradeRule {
    let mut r = GridTradeRule::default();
    if !rule.submitted_base_price.is_null() {
        r.submitted_base_price = Some((*rule.submitted_base_price).value);
    }
    if !rule.upper_limit_price.is_null() {
        r.upper_limit_price = Some((*rule.upper_limit_price).value);
    }
    if !rule.lower_limit_price.is_null() {
        r.lower_limit_price = Some((*rule.lower_limit_price).value);
    }
    if !rule.trigger_price_type.is_null() {
        r.trigger_price_type = Some(TriggerPriceType::from(*rule.trigger_price_type));
    }
    if !rule.trigger_spread_up.is_null() {
        r.trigger_spread_up = Some((*rule.trigger_spread_up).value);
    }
    if !rule.trigger_spread_down.is_null() {
        r.trigger_spread_down = Some((*rule.trigger_spread_down).value);
    }
    if !rule.trigger_percent_up.is_null() {
        r.trigger_percent_up = Some((*rule.trigger_percent_up).value);
    }
    if !rule.trigger_percent_down.is_null() {
        r.trigger_percent_down = Some((*rule.trigger_percent_down).value);
    }
    if !rule.multiple_trigger.is_null() {
        r.multiple_trigger = Some(*rule.multiple_trigger);
    }
    if !rule.time_in_force.is_null() {
        r.time_in_force = Some(GridTimeInForce::from(*rule.time_in_force));
    }
    if !rule.upper_limit_quantity.is_null() {
        r.upper_limit_quantity = Some((*rule.upper_limit_quantity).value);
    }
    if !rule.lower_limit_quantity.is_null() {
        r.lower_limit_quantity = Some((*rule.lower_limit_quantity).value);
    }
    if !rule.expire_time.is_null() {
        r.expire_time = Some(*rule.expire_time);
    }
    if !rule.upper_limit_event.is_null() {
        r.upper_limit_event = Some(GridLimitEvent::from(*rule.upper_limit_event));
    }
    if !rule.lower_limit_event.is_null() {
        r.lower_limit_event = Some(GridLimitEvent::from(*rule.lower_limit_event));
    }
    if !rule.trigger_sell_depth.is_null() {
        r.trigger_sell_depth = Some(*rule.trigger_sell_depth);
    }
    if !rule.trigger_buy_depth.is_null() {
        r.trigger_buy_depth = Some(*rule.trigger_buy_depth);
    }
    if !rule.trigger_quantity.is_null() {
        r.trigger_quantity = Some((*rule.trigger_quantity).value);
    }
    if !rule.support_shortsell.is_null() {
        r.support_shortsell = Some(*rule.support_shortsell);
    }
    if !rule.rth.is_null() {
        r.rth = Some(*rule.rth);
    }
    if !rule.grid_order_type_up.is_null() {
        r.grid_order_type_up = Some(cstr_to_rust(rule.grid_order_type_up));
    }
    if !rule.grid_order_type_down.is_null() {
        r.grid_order_type_down = Some(cstr_to_rust(rule.grid_order_type_down));
    }
    r
}

/// Submit a grid trading order
///
/// @param[in] opts Options for submit grid order request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_submit(
    ctx: *const CGridContext,
    opts: *const CSubmitGridOrderOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust((*opts).symbol);
    let settlement_currency = cstr_to_rust((*opts).settlement_currency);
    let rule = build_grid_trade_rule(&(*opts).grid_trading_rule);
    let opts2 = SubmitGridOrderOptions::new(symbol, settlement_currency, rule);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSubmitGridOrderResponseOwned> = CCow::new(ctx_inner.submit(opts2).await?);
        Ok(resp)
    });
}

/// Replace (modify) a grid trading order
///
/// @param[in] opts Options for replace grid order request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_replace(
    ctx: *const CGridContext,
    opts: *const CReplaceGridOrderOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust((*opts).order_id);
    let rule = build_grid_trade_rule(&(*opts).grid_trading_rule);
    let opts2 = ReplaceGridOrderOptions::new(order_id, rule);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.replace(opts2).await?;
        Ok(())
    });
}

/// Get grid trading orders (paged list)
///
/// @param[in] opts Options for get grid orders request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_list(
    ctx: *const CGridContext,
    opts: *const CGetGridOrdersOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetGridOrdersOptions::new();
    if !opts.is_null() {
        if !(*opts).page.is_null() {
            opts2 = opts2.page(*(*opts).page);
        }
        if !(*opts).limit.is_null() {
            opts2 = opts2.limit(*(*opts).limit);
        }
        if !(*opts).market.is_null() {
            opts2 = opts2.market((*(*opts).market).into());
        }
        if !(*opts).status.is_null() {
            opts2 = opts2.status(cstr_to_rust((*opts).status));
        }
        if !(*opts).symbol.is_null() {
            opts2 = opts2.symbol(cstr_to_rust((*opts).symbol));
        }
        if !(*opts).sort_by.is_null() {
            opts2 = opts2.sort_by(cstr_to_rust((*opts).sort_by));
        }
        if !(*opts).sort_order.is_null() {
            opts2 = opts2.sort_order(cstr_to_rust((*opts).sort_order));
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let resp = ctx_inner.list(opts2).await?;
        let owned: CCow<CGridOrdersResponseOwned> = CCow::new(CGridOrdersResponseOwned::new(
            resp.grid_order,
            resp.has_more,
        ));
        Ok(owned)
    });
}

/// Query grid trading orders by IDs
///
/// @param[in] opts Options for get grid orders by ids request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_list_by_ids(
    ctx: *const CGridContext,
    opts: *const CGetGridOrdersByIdsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_ids = cstr_array_to_rust((*opts).order_ids, (*opts).num_order_ids);
    let opts2 = GetGridOrdersByIdsOptions::new(order_ids);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CGridOrderOwned> = ctx_inner.list_by_ids(opts2).await?.into();
        Ok(rows)
    });
}

/// Get grid trading order detail (and paged history)
///
/// @param[in] opts Options for get grid order detail request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_detail(
    ctx: *const CGridContext,
    opts: *const CGetGridOrderDetailOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust((*opts).order_id);
    let mut opts2 = GetGridOrderDetailOptions::new(order_id);
    if !(*opts).history_id.is_null() {
        opts2 = opts2.history_id(cstr_to_rust((*opts).history_id));
    }
    if !(*opts).limit.is_null() {
        opts2 = opts2.limit(*(*opts).limit);
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CGridOrderDetailOwned> = CCow::new(ctx_inner.detail(opts2).await?);
        Ok(resp)
    });
}

/// Get grid trading trigger history
///
/// @param[in] opts Options for get grid trigger history request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_trigger_history(
    ctx: *const CGridContext,
    opts: *const CGetGridTriggerHistoryOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let grid_order_id = cstr_to_rust((*opts).grid_order_id);
    let mut opts2 = GetGridTriggerHistoryOptions::new(grid_order_id);
    if !(*opts).page.is_null() {
        opts2 = opts2.page(*(*opts).page);
    }
    if !(*opts).limit.is_null() {
        opts2 = opts2.limit(*(*opts).limit);
    }
    execute_async(callback, ctx, userdata, async move {
        let resp = ctx_inner.trigger_history(opts2).await?;
        let owned: CCow<CGridTriggerHistoryResponseOwned> = CCow::new(
            CGridTriggerHistoryResponseOwned::new(resp.trigger_orders, resp.has_more),
        );
        Ok(owned)
    });
}

/// Cancel a grid trading order
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_cancel(
    ctx: *const CGridContext,
    order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust(order_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.cancel(order_id).await?;
        Ok(())
    });
}

/// Suspend a grid trading order
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_suspend(
    ctx: *const CGridContext,
    order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust(order_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.suspend(order_id).await?;
        Ok(())
    });
}

/// Restart a grid trading order
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_restart(
    ctx: *const CGridContext,
    order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let order_id = cstr_to_rust(order_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.restart(order_id).await?;
        Ok(())
    });
}

/// Get the security (symbol) info used to build a grid order (lot size,
/// authorization flag, settlement currency, etc.).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_grid_context_symbol_info(
    ctx: *const CGridContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CGridSymbolInfoOwned> = CCow::new(ctx_inner.symbol_info(symbol).await?);
        Ok(resp)
    });
}
