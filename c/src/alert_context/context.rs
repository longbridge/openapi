use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{AlertContext, alert::types::*};

use crate::{
    alert_context::{enum_types::*, types::*},
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    types::{CCow, cstr_to_rust},
};

pub struct CAlertContext {
    ctx: AlertContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_new(config: *const CConfig) -> *const CAlertContext {
    let config = Arc::new((*config).0.clone());
    Arc::into_raw(Arc::new(CAlertContext {
        ctx: AlertContext::new(config),
    }))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_retain(ctx: *const CAlertContext) {
    Arc::increment_strong_count(ctx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_release(ctx: *const CAlertContext) {
    let _ = Arc::from_raw(ctx);
}

/// List all price alerts. Returns `CAlertList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_list(
    ctx: *const CAlertContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CAlertListOwned> = CCow::new(CAlertListOwned::from(ctx_inner.list().await?));
        Ok(resp)
    });
}

/// Add a price alert.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_add(
    ctx: *const CAlertContext,
    symbol: *const c_char,
    condition: CAlertCondition,
    trigger_value: *const c_char,
    frequency: CAlertFrequency,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let trigger_value = cstr_to_rust(trigger_value);
    let cond: AlertCondition = condition.into();
    let freq: AlertFrequency = frequency.into();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.add(symbol, cond, trigger_value, freq).await?;
        Ok(())
    });
}

/// Update (enable or disable) a price alert.
///
/// `item` must point to a valid [`CAlertItem`] obtained from
/// [`lb_alert_context_list`]. Set `enabled` to `true` to re-enable or
/// `false` to disable. All fields of `item` are read before the function
/// returns, so the pointer only needs to be valid for the duration of
/// the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_update(
    ctx: *const CAlertContext,
    item: *const CAlertItem,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let alert_item = (*item).to_alert_item();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.update(&alert_item).await?;
        Ok(())
    });
}

/// Delete price alerts. alert_ids: array of alert ID strings, num_ids: count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_delete(
    ctx: *const CAlertContext,
    alert_ids: *const *const c_char,
    num_ids: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let ids: Vec<String> = (0..num_ids)
        .map(|i| cstr_to_rust(*alert_ids.add(i)))
        .collect();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.delete(ids).await?;
        Ok(())
    });
}
