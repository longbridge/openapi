use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{DCAContext, dca::types::*};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    dca_context::types::*,
    types::{CCow, cstr_to_rust},
};

pub struct CDCAContext {
    ctx: DCAContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_new(config: *const CConfig) -> *const CDCAContext {
    Arc::into_raw(Arc::new(CDCAContext {
        ctx: DCAContext::new(Arc::new((*config).0.clone())),
    }))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_retain(ctx: *const CDCAContext) {
    Arc::increment_strong_count(ctx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_release(ctx: *const CDCAContext) {
    let _ = Arc::from_raw(ctx);
}

/// List DCA plans (status: 0=Active,1=Suspended,2=Finished,-1=all).
/// Returns `CDcaList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_list(
    ctx: *const CDCAContext,
    status: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let s = match status {
        0 => Some(DCAStatus::Active),
        1 => Some(DCAStatus::Suspended),
        2 => Some(DCAStatus::Finished),
        _ => None,
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CDcaListOwned> =
            CCow::new(CDcaListOwned::from(ctx_inner.list(s, None).await?));
        Ok(resp)
    });
}

/// Get DCA stats. Returns `CDcaStats`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_stats(
    ctx: *const CDCAContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CDcaStatsOwned> =
            CCow::new(CDcaStatsOwned::from(ctx_inner.stats(None).await?));
        Ok(resp)
    });
}

/// Check which symbols support DCA. Returns `CDcaSupportList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_check_support(
    ctx: *const CDCAContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let syms: Vec<String> = (0..num_symbols)
        .map(|i| cstr_to_rust(*symbols.add(i)))
        .collect();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CDcaSupportListOwned> = CCow::new(CDcaSupportListOwned::from(
            ctx_inner.check_support(syms).await?,
        ));
        Ok(resp)
    });
}

/// Pause a DCA plan. Returns no data (empty response).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_pause(
    ctx: *const CDCAContext,
    plan_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = cstr_to_rust(plan_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.pause(id).await?;
        Ok(())
    });
}

/// Resume a DCA plan. Returns no data (empty response).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_resume(
    ctx: *const CDCAContext,
    plan_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = cstr_to_rust(plan_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.resume(id).await?;
        Ok(())
    });
}

/// Stop a DCA plan. Returns no data (empty response).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_stop(
    ctx: *const CDCAContext,
    plan_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = cstr_to_rust(plan_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.stop(id).await?;
        Ok(())
    });
}

/// Calculate next projected trade date. Returns `CDcaCalcDateResult`.
/// day_of_month: 0 = not set; 1–28 = day of month for monthly plans.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_calc_date(
    ctx: *const CDCAContext,
    symbol: *const c_char,
    frequency: i32,
    day_of_week: *const c_char,
    day_of_month: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let sym = cstr_to_rust(symbol);
    let freq = match frequency {
        0 => DCAFrequency::Daily,
        1 => DCAFrequency::Weekly,
        2 => DCAFrequency::Fortnightly,
        _ => DCAFrequency::Monthly,
    };
    let dow = if day_of_week.is_null() {
        None
    } else {
        Some(cstr_to_rust(day_of_week))
    };
    let dom = if day_of_month == 0 {
        None
    } else {
        Some(day_of_month)
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CDcaCalcDateResultOwned> = CCow::new(CDcaCalcDateResultOwned::from(
            ctx_inner.calc_date(sym, freq, dow, dom).await?,
        ));
        Ok(resp)
    });
}

/// Get DCA execution history for a plan. Returns `CDcaHistoryResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_history(
    ctx: *const CDCAContext,
    plan_id: *const c_char,
    page: i32,
    limit: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = cstr_to_rust(plan_id);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CDcaHistoryResponseOwned> = CCow::new(CDcaHistoryResponseOwned::from(
            ctx_inner.history(id, page, limit).await?,
        ));
        Ok(resp)
    });
}

/// Update advance reminder hours. `hours` must be `"1"`, `"6"`, or `"12"`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_set_reminder(
    ctx: *const CDCAContext,
    hours: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let h = cstr_to_rust(hours);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.set_reminder(h).await?;
        Ok(())
    });
}

/// Create a new DCA plan. Returns `CDcaCreateResult`.
/// frequency: 0=Daily, 1=Weekly, 2=Fortnightly, 3=Monthly
/// day_of_week: optional (e.g. "Mon"), pass NULL if not applicable
/// day_of_month: 0 = not set
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_create(
    ctx: *const CDCAContext,
    symbol: *const c_char,
    amount: *const c_char,
    frequency: i32,
    day_of_week: *const c_char,
    day_of_month: u32,
    allow_margin: bool,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let sym = cstr_to_rust(symbol);
    let amt = cstr_to_rust(amount);
    let freq = match frequency {
        0 => DCAFrequency::Daily,
        1 => DCAFrequency::Weekly,
        2 => DCAFrequency::Fortnightly,
        _ => DCAFrequency::Monthly,
    };
    let dow = if day_of_week.is_null() {
        None
    } else {
        Some(cstr_to_rust(day_of_week))
    };
    let dom = if day_of_month == 0 {
        None
    } else {
        Some(day_of_month)
    };
    execute_async(callback, ctx, userdata, async move {
        let result = ctx_inner
            .create(sym, amt, freq, dow, dom, allow_margin)
            .await?;
        let resp: CCow<CDcaCreateResultOwned> = CCow::new(CDcaCreateResultOwned::from(result));
        Ok(resp)
    });
}

/// Update an existing DCA plan. Returns `CDcaCreateResult`.
/// Pass -1 for frequency to leave unchanged; pass NULL for optional string
/// fields.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_dca_context_update(
    ctx: *const CDCAContext,
    plan_id: *const c_char,
    amount: *const c_char,
    frequency: i32,
    day_of_week: *const c_char,
    day_of_month: *const c_char,
    allow_margin: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let pid = cstr_to_rust(plan_id);
    let amt = if amount.is_null() {
        None
    } else {
        Some(cstr_to_rust(amount))
    };
    let freq = match frequency {
        0 => Some(DCAFrequency::Daily),
        1 => Some(DCAFrequency::Weekly),
        2 => Some(DCAFrequency::Fortnightly),
        3 => Some(DCAFrequency::Monthly),
        _ => None,
    };
    let dow = if day_of_week.is_null() {
        None
    } else {
        Some(cstr_to_rust(day_of_week))
    };
    let dom_s = if day_of_month.is_null() {
        None
    } else {
        let s = cstr_to_rust(day_of_month);
        s.parse::<u32>().ok()
    };
    let margin = match allow_margin {
        1 => Some(true),
        0 => Some(false),
        _ => None,
    };
    execute_async(callback, ctx, userdata, async move {
        let result = ctx_inner.update(pid, amt, freq, dow, dom_s, margin).await?;
        let resp: CCow<CDcaCreateResultOwned> = CCow::new(CDcaCreateResultOwned::from(result));
        Ok(resp)
    });
}
