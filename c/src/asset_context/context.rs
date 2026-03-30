use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::asset::{
    AssetContext, GetStatementListOptions, GetStatementOptions, StatementType,
};

use crate::{
    asset_context::types::CStatementItemOwned,
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    types::{CString, CVec, cstr_to_rust},
};

/// Asset context
pub struct CAssetContext {
    ctx: AssetContext,
}

/// Create a new `AssetContext`
///
/// @param config  Config object
/// @return A new asset context
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_new(config: *const CConfig) -> *const CAssetContext {
    let config = Arc::new((*config).0.clone());
    let ctx = AssetContext::new(config);
    Arc::into_raw(Arc::new(CAssetContext { ctx }))
}

/// Retain the asset context (increment reference count)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_retain(ctx: *const CAssetContext) {
    Arc::increment_strong_count(ctx);
}

/// Release the asset context (decrement reference count)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_release(ctx: *const CAssetContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get statement data list
///
/// @param ctx             Asset context
/// @param statement_type  1 = daily, 2 = monthly
/// @param start_date      Start date for pagination (0 = default)
/// @param limit           Number of results (0 = default 20)
/// @param callback        Async callback
/// @param userdata        User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_statements(
    ctx: *const CAssetContext,
    statement_type: i32,
    start_date: i32,
    limit: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let st = if statement_type == 2 {
        StatementType::Monthly
    } else {
        StatementType::Daily
    };
    let mut opts = GetStatementListOptions::new(st);
    if start_date > 0 {
        opts = opts.page(start_date);
    }
    if limit > 0 {
        opts = opts.page_size(limit);
    }
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CStatementItemOwned> = ctx_inner.statements(opts).await?.list.into();
        Ok(rows)
    });
}

/// Get statement data download URL
///
/// @param ctx       Asset context
/// @param file_key  File key from the list response
/// @param callback  Async callback
/// @param userdata  User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_download_url(
    ctx: *const CAssetContext,
    file_key: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let file_key = cstr_to_rust(file_key);
    let opts = GetStatementOptions::new(file_key);
    execute_async(callback, ctx, userdata, async move {
        let url: CString = ctx_inner.statement_download_url(opts).await?.url.into();
        Ok(url)
    });
}
