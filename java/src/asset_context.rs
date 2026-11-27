use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{
    Config,
    asset::{
        AssetContext, CouponOrderType, DepositState, GetCouponsOptions, GetDepositsOptions,
        GetRewardRecordsOptions, GetStatementListOptions, GetStatementOptions,
        GetStockTransfersOptions, GetWithdrawalsOptions, RewardType, StatementType,
        StockTransferInState, StockTransferOutState, StockTransferStateFilter,
    },
};
use time::OffsetDateTime;

use crate::{
    async_util,
    error::jni_result,
    types::{FromJValue, JSignature, JavaInteger, ObjectArray, get_field},
};

struct ContextObj {
    ctx: AssetContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newAssetContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let ctx = AssetContext::new(config);
        Ok(Box::into_raw(Box::new(ContextObj { ctx })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeAssetContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextStatements(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let statement_type: Option<JavaInteger> = get_field(env, &opts, "statementType")?;
        let start_date: Option<JavaInteger> = get_field(env, &opts, "startDate")?;
        let limit: Option<JavaInteger> = get_field(env, &opts, "limit")?;

        let st = match statement_type.map(i32::from).unwrap_or(1) {
            2 => StatementType::Monthly,
            _ => StatementType::Daily,
        };
        let mut options = GetStatementListOptions::new(st);
        if let Some(sd) = start_date {
            options = options.page(i32::from(sd));
        }
        if let Some(l) = limit {
            options = options.page_size(i32::from(l));
        }

        async_util::execute(env, callback, async move {
            let resp = __owned_ctx.statements(options).await?;
            Ok(serde_json::to_string(&resp).unwrap_or_default())
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextDownloadUrl(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    file_key: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let file_key: String = FromJValue::from_jvalue(env, file_key.into())?;
        let options = GetStatementOptions::new(file_key);

        async_util::execute(env, callback, async move {
            let resp = __owned_ctx.statement_download_url(options).await?;
            Ok(resp.url)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextBankCards(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.bank_cards().await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextEddaApplications(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    card_id: JObject,
    currency: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let card_id: String = FromJValue::from_jvalue(env, card_id.into())?;
        let currency: String = FromJValue::from_jvalue(env, currency.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                __owned_ctx.edda_applications(card_id, currency).await?,
            ))
        })?;
        Ok(())
    })
}

unsafe fn stock_transfers_options<S>(
    env: &mut JNIEnv,
    opts: &JObject,
) -> jni::errors::Result<GetStockTransfersOptions<S>>
where
    S: StockTransferStateFilter + FromJValue + JSignature,
{
    let mut options = GetStockTransfersOptions::new();
    if opts.is_null() {
        return Ok(options);
    }
    let market: Option<String> = get_field(env, opts, "market")?;
    let state: Option<S> = get_field(env, opts, "state")?;
    let start: Option<OffsetDateTime> = get_field(env, opts, "start")?;
    let end: Option<OffsetDateTime> = get_field(env, opts, "end")?;
    let page: Option<JavaInteger> = get_field(env, opts, "page")?;
    let size: Option<JavaInteger> = get_field(env, opts, "size")?;
    if let Some(market) = market {
        options = options.market(market);
    }
    if let Some(state) = state {
        options = options.state(state);
    }
    if let Some(start) = start {
        options = options.start(start);
    }
    if let Some(end) = end {
        options = options.end(end);
    }
    if let Some(page) = page {
        options = options.page(page.into());
    }
    if let Some(size) = size {
        options = options.size(size.into());
    }
    Ok(options)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextStockTransfersIn(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let options = stock_transfers_options::<StockTransferInState>(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.stock_transfers_in(options).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextStockTransfersOut(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let options = stock_transfers_options::<StockTransferOutState>(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(__owned_ctx.stock_transfers_out(options).await?))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextRewardRecords(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let mut options = GetRewardRecordsOptions::new();
        if !opts.is_null() {
            let page: Option<JavaInteger> = get_field(env, &opts, "page")?;
            let page_size: Option<JavaInteger> = get_field(env, &opts, "pageSize")?;
            let reward_type: Option<RewardType> = get_field(env, &opts, "rewardType")?;
            if let Some(page) = page {
                options = options.page(page.into());
            }
            if let Some(page_size) = page_size {
                options = options.page_size(page_size.into());
            }
            if let Some(reward_type) = reward_type {
                options = options.reward_type(reward_type);
            }
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.reward_records(options).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextCoupons(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let mut options = GetCouponsOptions::new();
        if !opts.is_null() {
            let page: Option<JavaInteger> = get_field(env, &opts, "page")?;
            let page_size: Option<JavaInteger> = get_field(env, &opts, "pageSize")?;
            let expired: bool = get_field(env, &opts, "expired")?;
            let card_type: Option<String> = get_field(env, &opts, "cardType")?;
            if let Some(page) = page {
                options = options.page(page.into());
            }
            if let Some(page_size) = page_size {
                options = options.page_size(page_size.into());
            }
            options = options.expired(expired);
            if let Some(card_type) = card_type {
                options = options.card_type(card_type);
            }
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.coupons(options).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextCouponOrders(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    card_id: JObject,
    order_type: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let card_id: String = FromJValue::from_jvalue(env, card_id.into())?;
        let order_type: CouponOrderType = FromJValue::from_jvalue(env, order_type.into())?;
        async_util::execute(env, callback, async move {
            Ok(ObjectArray(
                __owned_ctx.coupon_orders(card_id, order_type).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextDeposits(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let mut options = GetDepositsOptions::new();
        if !opts.is_null() {
            let page: Option<JavaInteger> = get_field(env, &opts, "page")?;
            let size: Option<JavaInteger> = get_field(env, &opts, "size")?;
            let states: Option<ObjectArray<DepositState>> = get_field(env, &opts, "states")?;
            let currencies: Option<ObjectArray<String>> = get_field(env, &opts, "currencies")?;
            let start: Option<OffsetDateTime> = get_field(env, &opts, "start")?;
            let end: Option<OffsetDateTime> = get_field(env, &opts, "end")?;
            if let Some(page) = page {
                options = options.page(page.into());
            }
            if let Some(size) = size {
                options = options.size(size.into());
            }
            if let Some(states) = states {
                options = options.states(states.0);
            }
            if let Some(currencies) = currencies {
                options = options.currencies(currencies.0);
            }
            if let Some(start) = start {
                options = options.start(start);
            }
            if let Some(end) = end {
                options = options.end(end);
            }
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.deposits(options).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_assetContextWithdrawals(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let __owned_ctx = context.ctx.clone();
        let mut options = GetWithdrawalsOptions::new();
        if !opts.is_null() {
            let page: Option<JavaInteger> = get_field(env, &opts, "page")?;
            let size: Option<JavaInteger> = get_field(env, &opts, "size")?;
            let start: Option<OffsetDateTime> = get_field(env, &opts, "start")?;
            let end: Option<OffsetDateTime> = get_field(env, &opts, "end")?;
            if let Some(page) = page {
                options = options.page(page.into());
            }
            if let Some(size) = size {
                options = options.size(size.into());
            }
            if let Some(start) = start {
                options = options.start(start);
            }
            if let Some(end) = end {
                options = options.end(end);
            }
        }
        async_util::execute(env, callback, async move {
            Ok(__owned_ctx.withdrawals(options).await?)
        })?;
        Ok(())
    })
}
