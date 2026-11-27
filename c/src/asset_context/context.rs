use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::asset::{
    AssetContext, CouponOrderType, GetCouponsOptions, GetDepositsOptions, GetRewardRecordsOptions,
    GetStatementListOptions, GetStatementOptions, GetStockTransfersOptions, GetWithdrawalsOptions,
    StatementType,
};
use time::OffsetDateTime;

use crate::{
    asset_context::{
        enum_types::CCouponOrderType,
        types::{
            CBankCardOwned, CCouponOrderOwned, CCouponsResponseOwned, CDepositsResponseOwned,
            CEddaApplicationOwned, CGetCouponsOptions, CGetDepositsOptions,
            CGetRewardRecordsOptions, CGetStockTransfersInOptions, CGetStockTransfersOutOptions,
            CGetWithdrawalsOptions, CRewardRecordsResponseOwned, CStatementItemOwned,
            CStockTransferInResponseOwned, CStockTransferOutOwned, CWithdrawalsResponseOwned,
        },
    },
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    types::{CCow, CString, CVec, cstr_array_to_rust, cstr_to_rust},
};

/// Converts a C unix-timestamp pointer to a time (null or out of range →
/// `None`).
unsafe fn opt_time(value: *const i64) -> Option<OffsetDateTime> {
    if value.is_null() {
        None
    } else {
        OffsetDateTime::from_unix_timestamp(*value).ok()
    }
}

macro_rules! stock_transfers_options {
    ($opts:expr) => {{
        let opts = $opts;
        let mut opts2 = GetStockTransfersOptions::new();
        if !opts.is_null() {
            let opts = &*opts;
            if !opts.market.is_null() {
                opts2 = opts2.market(cstr_to_rust(opts.market));
            }
            if !opts.state.is_null() {
                opts2 = opts2.state((*opts.state).into());
            }
            if let Some(start) = opt_time(opts.start) {
                opts2 = opts2.start(start);
            }
            if let Some(end) = opt_time(opts.end) {
                opts2 = opts2.end(end);
            }
            if !opts.page.is_null() {
                opts2 = opts2.page(*opts.page);
            }
            if !opts.size.is_null() {
                opts2 = opts2.size(*opts.size);
            }
        }
        opts2
    }};
}

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

/// Get the linked bank cards, with their verification, binding and
/// authorization (eDDA, bank securities account, FPS) status
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is an array of `lb_bank_card_t`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_bank_cards(
    ctx: *const CAssetContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CBankCardOwned> = ctx_inner.bank_cards().await?.into();
        Ok(rows)
    });
}

/// Get the eDDA authorization applications of a bank card in one currency
///
/// The card id must be all digits; any other value fails with an error before
/// anything is sent.
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is an array of `lb_edda_application_t`.
///
/// @param card_id   Bank card ID (`lb_bank_card_t.id`)
/// @param currency  Currency, e.g. `HKD` or `USD`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_edda_applications(
    ctx: *const CAssetContext,
    card_id: *const c_char,
    currency: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let card_id = cstr_to_rust(card_id);
    let currency = cstr_to_rust(currency);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CEddaApplicationOwned> =
            ctx_inner.edda_applications(card_id, currency).await?.into();
        Ok(rows)
    });
}

/// Get stock transfer-in applications (another broker → this account)
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is a `lb_stock_transfer_in_response_t`.
///
/// @param[in] opts Options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_stock_transfers_in(
    ctx: *const CAssetContext,
    opts: *const CGetStockTransfersInOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts2 = stock_transfers_options!(opts);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CStockTransferInResponseOwned> =
            CCow::new(ctx_inner.stock_transfers_in(opts2).await?);
        Ok(resp)
    });
}

/// Get stock transfer-out applications (this account → another broker)
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is an array of `lb_stock_transfer_out_t`.
///
/// @param[in] opts Options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_stock_transfers_out(
    ctx: *const CAssetContext,
    opts: *const CGetStockTransfersOutOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let opts2 = stock_transfers_options!(opts);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CStockTransferOutOwned> = ctx_inner.stock_transfers_out(opts2).await?.into();
        Ok(rows)
    });
}

/// Get reward records from the reward center
///
/// The route has no date filter: to find rewards in a time range, page
/// through the records and filter on their times.
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is a `lb_reward_records_response_t`.
///
/// @param[in] opts Options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_reward_records(
    ctx: *const CAssetContext,
    opts: *const CGetRewardRecordsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetRewardRecordsOptions::new();
    if !opts.is_null() {
        let opts = &*opts;
        if !opts.page.is_null() {
            opts2 = opts2.page(*opts.page);
        }
        if !opts.page_size.is_null() {
            opts2 = opts2.page_size(*opts.page_size);
        }
        if !opts.reward_type.is_null() {
            opts2 = opts2.reward_type((*opts.reward_type).into());
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CRewardRecordsResponseOwned> =
            CCow::new(ctx_inner.reward_records(opts2).await?);
        Ok(resp)
    });
}

/// Get coupons (reward cards)
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is a `lb_coupons_response_t`.
///
/// @param[in] opts Options (can be null: coupons that have not expired)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_coupons(
    ctx: *const CAssetContext,
    opts: *const CGetCouponsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetCouponsOptions::new();
    if !opts.is_null() {
        let opts = &*opts;
        if !opts.page.is_null() {
            opts2 = opts2.page(*opts.page);
        }
        if !opts.page_size.is_null() {
            opts2 = opts2.page_size(*opts.page_size);
        }
        opts2 = opts2.expired(opts.expired);
        if !opts.card_type.is_null() {
            opts2 = opts2.card_type(cstr_to_rust(opts.card_type));
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CCouponsResponseOwned> = CCow::new(ctx_inner.coupons(opts2).await?);
        Ok(resp)
    });
}

/// Get the orders a coupon was applied to
///
/// The card id must be 32 hex characters; any other value fails with an error
/// before anything is sent.
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is an array of `lb_coupon_order_t`.
///
/// @param card_id     Coupon ID (`lb_coupon_t.card_id`, 32 hex characters)
/// @param order_type  Locked (in progress) or settled orders
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_coupon_orders(
    ctx: *const CAssetContext,
    card_id: *const c_char,
    order_type: CCouponOrderType,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let card_id = cstr_to_rust(card_id);
    let order_type: CouponOrderType = order_type.into();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCouponOrderOwned> =
            ctx_inner.coupon_orders(card_id, order_type).await?.into();
        Ok(rows)
    });
}

/// Get one page of deposit records
///
/// The server's total follows the page size rather than the record count, so
/// it is not returned; `has_more` reports a full page instead. The page size
/// is always sent: at most 20, default 20.
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is a `lb_deposits_response_t`.
///
/// @param[in] opts Options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_deposits(
    ctx: *const CAssetContext,
    opts: *const CGetDepositsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetDepositsOptions::new();
    if !opts.is_null() {
        let opts = &*opts;
        if !opts.page.is_null() {
            opts2 = opts2.page(*opts.page);
        }
        if !opts.size.is_null() {
            opts2 = opts2.size(*opts.size);
        }
        if !opts.states.is_null() && opts.num_states > 0 {
            let states = std::slice::from_raw_parts(opts.states, opts.num_states);
            opts2 = opts2.states(states.iter().map(|state| (*state).into()));
        }
        if !opts.currencies.is_null() {
            opts2 = opts2.currencies(cstr_array_to_rust(opts.currencies, opts.num_currencies));
        }
        if let Some(start) = opt_time(opts.start) {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opt_time(opts.end) {
            opts2 = opts2.end(end);
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CDepositsResponseOwned> = CCow::new(ctx_inner.deposits(opts2).await?);
        Ok(resp)
    });
}

/// Get withdrawal records
///
/// The response's `has_more` reports a full page of the size sent (default 20,
/// at most 50).
///
/// AP accounts only: US data-center accounts get a region error.
///
/// The callback data is a `lb_withdrawals_response_t`.
///
/// @param[in] opts Options (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_asset_context_withdrawals(
    ctx: *const CAssetContext,
    opts: *const CGetWithdrawalsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let mut opts2 = GetWithdrawalsOptions::new();
    if !opts.is_null() {
        let opts = &*opts;
        if !opts.page.is_null() {
            opts2 = opts2.page(*opts.page);
        }
        if !opts.size.is_null() {
            opts2 = opts2.size(*opts.size);
        }
        if let Some(start) = opt_time(opts.start) {
            opts2 = opts2.start(start);
        }
        if let Some(end) = opt_time(opts.end) {
            opts2 = opts2.end(end);
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CWithdrawalsResponseOwned> = CCow::new(ctx_inner.withdrawals(opts2).await?);
        Ok(resp)
    });
}
