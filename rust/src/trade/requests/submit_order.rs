use rust_decimal::Decimal;
use serde::Serialize;
use time::Date;

use crate::{
    serde_utils,
    trade::{AttachedOrderType, OrderSide, OrderType, OutsideRTH, TimeInForceType},
};

/// Options for submit order request
#[derive(Debug, Serialize, Clone)]
pub struct SubmitOrderOptions {
    symbol: String,
    order_type: OrderType,
    side: OrderSide,
    submitted_quantity: Decimal,
    time_in_force: TimeInForceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    submitted_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_offset: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_percent: Option<Decimal>,
    #[serde(with = "serde_utils::date_opt")]
    expire_date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outside_rth: Option<OutsideRTH>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_depth_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attached_params: Option<SubmitAttachedParams>,
}

impl SubmitOrderOptions {
    /// Create a new `SubmitOrderOptions`
    #[inline]
    pub fn new(
        symbol: impl Into<String>,
        order_type: OrderType,
        side: OrderSide,
        submitted_quantity: Decimal,
        time_in_force: TimeInForceType,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_type,
            side,
            submitted_quantity,
            time_in_force,
            submitted_price: None,
            trigger_price: None,
            limit_offset: None,
            trailing_amount: None,
            trailing_percent: None,
            expire_date: None,
            outside_rth: None,
            limit_depth_level: None,
            trigger_count: None,
            monitor_price: None,
            remark: None,
            attached_params: None,
        }
    }

    /// Set the submitted price
    #[inline]
    #[must_use]
    pub fn submitted_price(self, submitted_price: Decimal) -> Self {
        Self {
            submitted_price: Some(submitted_price),
            ..self
        }
    }

    /// Set the trigger price
    #[inline]
    #[must_use]
    pub fn trigger_price(self, trigger_price: Decimal) -> Self {
        Self {
            trigger_price: Some(trigger_price),
            ..self
        }
    }

    /// Set the limit offset
    #[inline]
    #[must_use]
    pub fn limit_offset(self, limit_offset: Decimal) -> Self {
        Self {
            limit_offset: Some(limit_offset),
            ..self
        }
    }

    /// Set the trailing amount
    #[inline]
    #[must_use]
    pub fn trailing_amount(self, trailing_amount: Decimal) -> Self {
        Self {
            trailing_amount: Some(trailing_amount),
            ..self
        }
    }

    /// Set the trailing percent
    #[inline]
    #[must_use]
    pub fn trailing_percent(self, trailing_percent: Decimal) -> Self {
        Self {
            trailing_percent: Some(trailing_percent),
            ..self
        }
    }

    /// Set the expire date
    #[inline]
    #[must_use]
    pub fn expire_date(self, expire_date: Date) -> Self {
        Self {
            expire_date: Some(expire_date),
            ..self
        }
    }

    /// Enable or disable outside regular trading hours
    #[inline]
    #[must_use]
    pub fn outside_rth(self, outside_rth: OutsideRTH) -> Self {
        Self {
            outside_rth: Some(outside_rth),
            ..self
        }
    }

    /// Set the limit depth level
    pub fn limit_depth_level(self, level: i32) -> Self {
        Self {
            limit_depth_level: Some(level),
            ..self
        }
    }

    /// Set the trigger count
    pub fn trigger_count(self, count: i32) -> Self {
        Self {
            trigger_count: Some(count),
            ..self
        }
    }

    /// Set the monitor price
    pub fn monitor_price(self, price: Decimal) -> Self {
        Self {
            monitor_price: Some(price),
            ..self
        }
    }

    /// Set the remark
    #[inline]
    #[must_use]
    pub fn remark(self, remark: impl Into<String>) -> Self {
        Self {
            remark: Some(remark.into()),
            ..self
        }
    }

    /// Set attached order parameters
    pub fn attached_params(self, params: SubmitAttachedParams) -> Self {
        Self {
            attached_params: Some(params),
            ..self
        }
    }
}

/// Attached order parameters for submit order
#[derive(Debug, Serialize, Clone)]
pub struct SubmitAttachedParams {
    attached_order_type: AttachedOrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    profit_taker_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_loss_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activate_order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profit_taker_submit_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_loss_submit_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activate_rth: Option<OutsideRTH>,
}

impl SubmitAttachedParams {
    /// Create new SubmitAttachedParams
    pub fn new(attached_order_type: AttachedOrderType) -> Self {
        Self {
            attached_order_type,
            profit_taker_price: None,
            stop_loss_price: None,
            time_in_force: None,
            expire_time: None,
            activate_order_type: None,
            profit_taker_submit_price: None,
            stop_loss_submit_price: None,
            activate_rth: None,
        }
    }
    /// Set the take-profit trigger price
    pub fn profit_taker_price(self, v: Decimal) -> Self {
        Self {
            profit_taker_price: Some(v),
            ..self
        }
    }
    /// Set the stop-loss trigger price
    pub fn stop_loss_price(self, v: Decimal) -> Self {
        Self {
            stop_loss_price: Some(v),
            ..self
        }
    }
    /// Set the time in force type
    pub fn time_in_force(self, v: TimeInForceType) -> Self {
        Self {
            time_in_force: Some(v),
            ..self
        }
    }
    /// Set the expiry time (unix timestamp seconds)
    pub fn expire_time(self, v: i64) -> Self {
        Self {
            expire_time: Some(v),
            ..self
        }
    }
    /// Set the order type to submit after trigger
    pub fn activate_order_type(self, v: OrderType) -> Self {
        Self {
            activate_order_type: Some(v),
            ..self
        }
    }
    /// Set the take-profit limit price
    pub fn profit_taker_submit_price(self, v: Decimal) -> Self {
        Self {
            profit_taker_submit_price: Some(v),
            ..self
        }
    }
    /// Set the stop-loss limit price
    pub fn stop_loss_submit_price(self, v: Decimal) -> Self {
        Self {
            stop_loss_submit_price: Some(v),
            ..self
        }
    }
    /// Set the RTH setting for the activated order
    pub fn activate_rth(self, v: OutsideRTH) -> Self {
        Self {
            activate_rth: Some(v),
            ..self
        }
    }
}
