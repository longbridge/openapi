use napi::bindgen_prelude::ClassInstance;

use crate::{
    decimal::Decimal,
    time::NaiveDate,
    trade::types::{AttachedOrderType, OrderSide, OrderType, OutsideRTH, TimeInForceType},
};

/// Parameters for submitting an attached order
#[napi_derive::napi(object)]
pub struct SubmitAttachedParams<'env> {
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
}

impl<'env> From<SubmitAttachedParams<'env>> for longbridge::trade::SubmitAttachedParams {
    fn from(p: SubmitAttachedParams<'env>) -> Self {
        let mut opts = longbridge::trade::SubmitAttachedParams::new(p.attached_order_type.into());
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
        opts
    }
}

/// Options for submit order request
#[napi_derive::napi(object)]
pub struct SubmitOrderOptions<'env> {
    /// Security code
    pub symbol: String,
    /// Order type
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Submitted quantity
    pub submitted_quantity: ClassInstance<'env, Decimal>,
    /// Time in force type
    pub time_in_force: TimeInForceType,
    /// Submitted price
    pub submitted_price: Option<ClassInstance<'env, Decimal>>,
    /// Trigger price (`LIT` / `MIT` Required)
    pub trigger_price: Option<ClassInstance<'env, Decimal>>,
    /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
    pub limit_offset: Option<ClassInstance<'env, Decimal>>,
    /// Trailing amount (`TSLPAMT` / `TSMAMT` Required)
    pub trailing_amount: Option<ClassInstance<'env, Decimal>>,
    /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
    pub trailing_percent: Option<ClassInstance<'env, Decimal>>,
    /// Long term order expire date (Required when `time_in_force` is
    /// `GoodTilDate`)
    pub expire_date: Option<ClassInstance<'env, NaiveDate>>,
    /// Enable or disable outside regular trading hours
    pub outside_rth: Option<OutsideRTH>,
    /// Limit depth level
    pub limit_depth_level: Option<i32>,
    /// Trigger count
    pub trigger_count: Option<i32>,
    /// Monitor price
    pub monitor_price: Option<ClassInstance<'env, Decimal>>,
    /// Remark (Maximum 64 characters)
    pub remark: Option<String>,
    /// Client request ID for idempotency control.
    /// If not specified, idempotency control is skipped.
    /// The server caches this ID for 10 minutes.
    pub client_request_id: Option<String>,
    /// Attached order parameters
    pub attached_params: Option<SubmitAttachedParams<'env>>,
}

impl<'env> From<SubmitOrderOptions<'env>> for longbridge::trade::SubmitOrderOptions {
    #[inline]
    fn from(opts: SubmitOrderOptions<'env>) -> Self {
        let mut opts2 = longbridge::trade::SubmitOrderOptions::new(
            opts.symbol,
            opts.order_type.into(),
            opts.side.into(),
            opts.submitted_quantity.0,
            opts.time_in_force.into(),
        );
        if let Some(submitted_price) = opts.submitted_price {
            opts2 = opts2.submitted_price(submitted_price.0);
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
        if let Some(expire_date) = opts.expire_date {
            opts2 = opts2.expire_date(expire_date.0);
        }
        if let Some(outside_rth) = opts.outside_rth {
            opts2 = opts2.outside_rth(outside_rth.into());
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
        if let Some(id) = opts.client_request_id {
            opts2 = opts2.client_request_id(id);
        }
        if let Some(p) = opts.attached_params {
            opts2 = opts2.attached_params(p.into());
        }
        opts2
    }
}
