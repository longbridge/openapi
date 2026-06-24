use napi::bindgen_prelude::ClassInstance;

use crate::{
    decimal::Decimal,
    trade::types::{AttachedOrderType, OrderType, OutsideRTH, TimeInForceType},
};

/// Parameters for replacing an attached order
#[napi_derive::napi(object)]
pub struct ReplaceAttachedParams<'env> {
    /// Attached order type
    pub attached_order_type: AttachedOrderType,
    /// Profit taker price
    pub profit_taker_price: Option<ClassInstance<'env, Decimal>>,
    /// Stop loss price
    pub stop_loss_price: Option<ClassInstance<'env, Decimal>>,
    /// Time in force type
    pub time_in_force: Option<TimeInForceType>,
    /// Expire time (unix timestamp)
    pub expire_time: Option<i64>,
    /// Activate order type
    pub activate_order_type: Option<OrderType>,
    /// Profit taker submit price
    pub profit_taker_submit_price: Option<ClassInstance<'env, Decimal>>,
    /// Stop loss submit price
    pub stop_loss_submit_price: Option<ClassInstance<'env, Decimal>>,
    /// Activate RTH
    pub activate_rth: Option<OutsideRTH>,
    /// Profit taker order ID
    pub profit_taker_id: Option<i64>,
    /// Stop loss order ID
    pub stop_loss_id: Option<i64>,
    /// Cancel all attached orders
    pub cancel_all_attached: Option<bool>,
    /// Main order ID
    pub main_id: Option<i64>,
    /// Quantity
    pub quantity: Option<ClassInstance<'env, Decimal>>,
    /// Market price
    pub market_price: Option<ClassInstance<'env, Decimal>>,
}

impl<'env> From<ReplaceAttachedParams<'env>> for longbridge::trade::ReplaceAttachedParams {
    fn from(p: ReplaceAttachedParams<'env>) -> Self {
        let mut opts = longbridge::trade::ReplaceAttachedParams::new(p.attached_order_type.into());
        if let Some(v) = p.profit_taker_price {
            opts = opts.profit_taker_price(v.0);
        }
        if let Some(v) = p.stop_loss_price {
            opts = opts.stop_loss_price(v.0);
        }
        if let Some(v) = p.time_in_force {
            opts = opts.time_in_force(v.into());
        }
        if let Some(v) = p.expire_time {
            opts = opts.expire_time(v);
        }
        if let Some(v) = p.activate_order_type {
            opts = opts.activate_order_type(v.into());
        }
        if let Some(v) = p.profit_taker_submit_price {
            opts = opts.profit_taker_submit_price(v.0);
        }
        if let Some(v) = p.stop_loss_submit_price {
            opts = opts.stop_loss_submit_price(v.0);
        }
        if let Some(v) = p.activate_rth {
            opts = opts.activate_rth(v.into());
        }
        if let Some(v) = p.profit_taker_id {
            opts = opts.profit_taker_id(v);
        }
        if let Some(v) = p.stop_loss_id {
            opts = opts.stop_loss_id(v);
        }
        if p.cancel_all_attached == Some(true) {
            opts = opts.cancel_all_attached();
        }
        if let Some(v) = p.main_id {
            opts = opts.main_id(v);
        }
        if let Some(v) = p.quantity {
            opts = opts.quantity(v.0);
        }
        if let Some(v) = p.market_price {
            opts = opts.market_price(v.0);
        }
        opts
    }
}

/// Options for replace order request
#[napi_derive::napi(object)]
pub struct ReplaceOrderOptions<'env> {
    /// Order id
    pub order_id: String,
    /// Replaced quantity
    pub quantity: ClassInstance<'env, Decimal>,
    /// Replaced price
    pub price: Option<ClassInstance<'env, Decimal>>,
    /// Trigger price (`LIT` / `MIT` Order Required)
    pub trigger_price: Option<ClassInstance<'env, Decimal>>,
    /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
    pub limit_offset: Option<ClassInstance<'env, Decimal>>,
    /// Trailing amount (`TSLPAMT` / `TSMAMT` Required)
    pub trailing_amount: Option<ClassInstance<'env, Decimal>>,
    /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
    pub trailing_percent: Option<ClassInstance<'env, Decimal>>,
    /// Limit depth level
    pub limit_depth_level: Option<i32>,
    /// Trigger count
    pub trigger_count: Option<i32>,
    /// Monitor price
    pub monitor_price: Option<ClassInstance<'env, Decimal>>,
    /// Remark (Maximum 64 characters)
    pub remark: Option<String>,
    /// Attached order parameters
    pub attached_params: Option<ReplaceAttachedParams<'env>>,
}

impl<'env> From<ReplaceOrderOptions<'env>> for longbridge::trade::ReplaceOrderOptions {
    #[inline]
    fn from(opts: ReplaceOrderOptions<'env>) -> Self {
        let mut opts2 = longbridge::trade::ReplaceOrderOptions::new(opts.order_id, opts.quantity.0);
        if let Some(price) = opts.price {
            opts2 = opts2.price(price.0);
        }
        if let Some(trigger_price) = opts.trigger_price {
            opts2 = opts2.trigger_price(trigger_price.0);
        }
        if let Some(limit_offset) = opts.limit_offset {
            opts2 = opts2.limit_offset(limit_offset.0);
        }
        if let Some(trailing_amount) = opts.trailing_amount {
            opts2 = opts2.trailing_amount(trailing_amount.0);
        }
        if let Some(trailing_percent) = opts.trailing_percent {
            opts2 = opts2.trailing_percent(trailing_percent.0);
        }
        if let Some(limit_depth_level) = opts.limit_depth_level {
            opts2 = opts2.limit_depth_level(limit_depth_level);
        }
        if let Some(trigger_count) = opts.trigger_count {
            opts2 = opts2.trigger_count(trigger_count);
        }
        if let Some(monitor_price) = opts.monitor_price {
            opts2 = opts2.monitor_price(monitor_price.0);
        }
        if let Some(remark) = opts.remark {
            opts2 = opts2.remark(remark);
        }
        if let Some(p) = opts.attached_params {
            opts2 = opts2.attached_params(p.into());
        }
        opts2
    }
}
