use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::SharelistContext;

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    sharelist_context::types::*,
    types::{CCow, cstr_to_rust},
};

pub struct CSharelistContext {
    ctx: SharelistContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_new(
    config: *const CConfig,
) -> *const CSharelistContext {
    Arc::into_raw(Arc::new(CSharelistContext {
        ctx: SharelistContext::new(Arc::new((*config).0.clone())),
    }))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_retain(ctx: *const CSharelistContext) {
    Arc::increment_strong_count(ctx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_release(ctx: *const CSharelistContext) {
    let _ = Arc::from_raw(ctx);
}

/// List user's sharelists. Returns `CSharelistList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_list(
    ctx: *const CSharelistContext,
    count: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSharelistListOwned> =
            CCow::new(CSharelistListOwned::from(ctx_inner.list(count).await?));
        Ok(resp)
    });
}

/// Get sharelist detail. Returns `CSharelistDetail`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_detail(
    ctx: *const CSharelistContext,
    id: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSharelistDetailOwned> =
            CCow::new(CSharelistDetailOwned::from(ctx_inner.detail(id).await?));
        Ok(resp)
    });
}

/// Get popular sharelists. Returns `CSharelistList`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_popular(
    ctx: *const CSharelistContext,
    count: u32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSharelistListOwned> =
            CCow::new(CSharelistListOwned::from(ctx_inner.popular(count).await?));
        Ok(resp)
    });
}

/// Add securities to a sharelist.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_add_securities(
    ctx: *const CSharelistContext,
    id: i64,
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
        ctx_inner.add_securities(id, syms).await?;
        Ok(())
    });
}

/// Remove securities from a sharelist.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_remove_securities(
    ctx: *const CSharelistContext,
    id: i64,
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
        ctx_inner.remove_securities(id, syms).await?;
        Ok(())
    });
}

/// Create a new sharelist. Returns no data (empty response).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_create(
    ctx: *const CSharelistContext,
    name: *const c_char,
    description: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let n = cstr_to_rust(name);
    let desc = if description.is_null() {
        None
    } else {
        Some(cstr_to_rust(description))
    };
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.create(n, desc).await?;
        Ok(())
    });
}

/// Delete a sharelist.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_delete(
    ctx: *const CSharelistContext,
    id: i64,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.delete(id).await?;
        Ok(())
    });
}

/// Reorder securities in a sharelist.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_sharelist_context_sort_securities(
    ctx: *const CSharelistContext,
    id: i64,
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
        ctx_inner.sort_securities(id, syms).await?;
        Ok(())
    });
}
