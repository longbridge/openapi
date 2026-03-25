use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::content::{ContentContext, CreateTopicOptions, ListMyTopicsOptions};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    content_context::types::{CNewsItemOwned, COwnedTopicOwned, CTopicItemOwned},
    types::{CString, CVec, cstr_array_to_rust, cstr_to_rust},
};

/// Content context
pub struct CContentContext {
    ctx: ContentContext,
}

/// Create a new `ContentContext`
///
/// @param config  Config object
/// @return A new content context
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_new(config: *const CConfig) -> *const CContentContext {
    let config = Arc::new((*config).0.clone());
    let ctx = ContentContext::new(config);
    Arc::into_raw(Arc::new(CContentContext { ctx }))
}

/// Retain the content context (increment reference count)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_retain(ctx: *const CContentContext) {
    Arc::increment_strong_count(ctx);
}

/// Release the content context (decrement reference count)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_release(ctx: *const CContentContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get topics created by the current authenticated user
///
/// @param ctx         Content context
/// @param page        Page number (0 = default 1)
/// @param size        Records per page, range 1~500 (0 = default 50)
/// @param topic_type  Filter by content type: "article" or "post" (NULL = all)
/// @param callback    Async callback
/// @param userdata    User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_topics_mine(
    ctx: *const CContentContext,
    page: i32,
    size: i32,
    topic_type: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let topic_type = if topic_type.is_null() {
        None
    } else {
        Some(cstr_to_rust(topic_type))
    };
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<COwnedTopicOwned> = ctx_inner
            .topics_mine(ListMyTopicsOptions {
                page: if page > 0 { Some(page) } else { None },
                size: if size > 0 { Some(size) } else { None },
                topic_type,
            })
            .await?
            .into();
        Ok(rows)
    });
}

/// Create a new topic
///
/// @param ctx          Content context
/// @param title        Topic title (required)
/// @param body         Topic body in Markdown format (required)
/// @param topic_type   Type: "article" or "post" (NULL = "post")
/// @param tickers      Related stock tickers array (NULL = none)
/// @param num_tickers  Number of tickers
/// @param hashtags     Hashtag names array (NULL = none)
/// @param num_hashtags Number of hashtags
/// @param license      0=none, 1=original, 2=non-original (-1 = default)
/// @param callback     Async callback
/// @param userdata     User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_create_topic(
    ctx: *const CContentContext,
    title: *const c_char,
    body: *const c_char,
    topic_type: *const c_char,
    tickers: *const *const c_char,
    num_tickers: usize,
    hashtags: *const *const c_char,
    num_hashtags: usize,
    license: i32,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let title = cstr_to_rust(title);
    let body = cstr_to_rust(body);
    let topic_type = if topic_type.is_null() {
        None
    } else {
        Some(cstr_to_rust(topic_type))
    };
    let tickers = if tickers.is_null() || num_tickers == 0 {
        None
    } else {
        Some(cstr_array_to_rust(tickers, num_tickers))
    };
    let hashtags = if hashtags.is_null() || num_hashtags == 0 {
        None
    } else {
        Some(cstr_array_to_rust(hashtags, num_hashtags))
    };
    let license = if license >= 0 { Some(license) } else { None };
    execute_async(callback, ctx, userdata, async move {
        let id = ctx_inner
            .create_topic(CreateTopicOptions {
                title,
                body,
                topic_type,
                tickers,
                hashtags,
                license,
            })
            .await?;
        Ok(CString::from(id))
    });
}

/// Get discussion topics list for a symbol
///
/// @param ctx       Content context
/// @param symbol    Security symbol (e.g. "700.HK")
/// @param callback  Async callback
/// @param userdata  User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_topics(
    ctx: *const CContentContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CTopicItemOwned> = ctx_inner.topics(symbol).await?.into();
        Ok(rows)
    });
}

/// Get news list for a symbol
///
/// @param ctx       Content context
/// @param symbol    Security symbol (e.g. "700.HK")
/// @param callback  Async callback
/// @param userdata  User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_news(
    ctx: *const CContentContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CNewsItemOwned> = ctx_inner.news(symbol).await?.into();
        Ok(rows)
    });
}
