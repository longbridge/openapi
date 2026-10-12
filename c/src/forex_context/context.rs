use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::forex::{ForexContext, GetForexQuoteOptions, SubmitForexOrderOptions};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    forex_context::types::{CForexOrderDetailOwned, CForexQuoteOwned},
    types::{CCow, CDecimal, cstr_to_rust},
};

pub struct CForexContext {
    ctx: ForexContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_forex_context_new(config: *const CConfig) -> *const CForexContext {
    Arc::into_raw(Arc::new(CForexContext {
        ctx: ForexContext::new(Arc::new((*config).0.clone())),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_forex_context_retain(ctx: *const CForexContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_forex_context_release(ctx: *const CForexContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get a forex quote.
///
/// @param[in] from Convert-out currency (ISO 4217, e.g. `USD`)
/// @param[in] to Convert-in currency (e.g. `HKD`)
/// @param[in] amount Convert-out amount (can be null)
/// @param[in] target_amount Convert-in amount (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_forex_context_quote(
    ctx: *const CForexContext,
    from: *const c_char,
    to: *const c_char,
    amount: *const CDecimal,
    target_amount: *const CDecimal,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let from = cstr_to_rust(from);
    let to = cstr_to_rust(to);
    let mut options = GetForexQuoteOptions::new();
    if !amount.is_null() {
        options = options.amount((*amount).value);
    }
    if !target_amount.is_null() {
        options = options.target_amount((*target_amount).value);
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CForexQuoteOwned> = CCow::new(ctx_inner.quote(from, to, options).await?);
        Ok(resp)
    });
}

/// Submit a forex order.
///
/// @param[in] quote_id Quote id returned by the quote endpoint
/// @param[in] client_order_id Caller-supplied unique order id
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_forex_context_submit_order(
    ctx: *const CForexContext,
    quote_id: *const c_char,
    client_order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let options =
        SubmitForexOrderOptions::new(cstr_to_rust(quote_id), cstr_to_rust(client_order_id));
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.submit_order(options).await?;
        Ok(())
    });
}

/// Query a forex order by `client_order_id`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_forex_context_order(
    ctx: *const CForexContext,
    client_order_id: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let client_order_id = cstr_to_rust(client_order_id);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CForexOrderDetailOwned> = CCow::new(ctx_inner.order(client_order_id).await?);
        Ok(resp)
    });
}
