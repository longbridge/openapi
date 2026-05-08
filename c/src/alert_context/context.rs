use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{AlertContext, alert::types::*};

use crate::{
    alert_context::types::*,
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

/// Add a price alert. condition: 1=PriceRise,2=PriceFall,3=PctRise,4=PctFall;
/// frequency: 1=Daily,2=EveryTime,3=Once
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_add(
    ctx: *const CAlertContext,
    symbol: *const c_char,
    condition: i32,
    trigger_value: *const c_char,
    frequency: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let trigger_value = cstr_to_rust(trigger_value);
    let cond = match condition {
        2 => AlertCondition::PriceFall,
        3 => AlertCondition::PercentRise,
        4 => AlertCondition::PercentFall,
        _ => AlertCondition::PriceRise,
    };
    let freq = match frequency {
        1 => AlertFrequency::Daily,
        2 => AlertFrequency::EveryTime,
        _ => AlertFrequency::Once,
    };
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.add(symbol, cond, trigger_value, freq).await?;
        Ok(())
    });
}

/// Enable a price alert.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_enable(
    ctx: *const CAlertContext,
    alert_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = cstr_to_rust(alert_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.enable(id).await?;
        Ok(())
    });
}

/// Disable a price alert.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_alert_context_disable(
    ctx: *const CAlertContext,
    alert_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let id = cstr_to_rust(alert_id);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.disable(id).await?;
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
