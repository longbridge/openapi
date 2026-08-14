use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Response for [`crate::AlertContext::list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertList {
    /// Alert groups per security
    pub lists: Vec<AlertSymbolGroup>,
}

/// Alert items for one security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSymbolGroup {
    /// Security symbol
    pub symbol: String,
    /// Ticker code (without market)
    pub code: String,
    /// Market, e.g. `"HK"`
    pub market: String,
    /// Security name
    pub name: String,
    /// Latest price
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub price: Option<Decimal>,
    /// Day change amount
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub chg: Option<Decimal>,
    /// Day change percentage
    #[serde(with = "crate::serde_utils::decimal_opt_str_is_none")]
    pub p_chg: Option<Decimal>,
    /// Product type (may be empty)
    #[serde(default)]
    pub product: String,
    /// Alert items
    pub indicators: Vec<AlertItem>,
}

/// One price alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertItem {
    /// Alert ID
    pub id: String,
    /// Condition: "1"=price_rise, "2"=price_fall, "3"=pct_rise, "4"=pct_fall
    pub indicator_id: String,
    /// Whether the alert is active
    pub enabled: bool,
    /// Frequency: 1=daily, 2=every_time, 3=once
    pub frequency: i32,
    /// Scope
    pub scope: i32,
    /// Display text, e.g. "价格涨到 600"
    pub text: String,
    /// Trigger state flags
    #[serde(default)]
    pub state: Vec<i32>,
    /// Trigger value, e.g. `{"price":"500"}` or `{"chg":"5"}`
    pub value_map: AlertValueMap,
}

/// Trigger value of a price alert.
///
/// Exactly one field is populated, depending on the alert condition: `price`
/// for absolute-price alerts ([`AlertCondition::PriceRise`] /
/// [`AlertCondition::PriceFall`]), `chg` for percentage-change alerts
/// ([`AlertCondition::PercentRise`] / [`AlertCondition::PercentFall`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertValueMap {
    /// Absolute price threshold, e.g. `500`.
    #[serde(
        default,
        with = "crate::serde_utils::decimal_opt_str_is_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Percentage-change threshold, e.g. `5`.
    #[serde(
        default,
        with = "crate::serde_utils::f64_opt_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub chg: Option<f64>,
}

/// Alert condition
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AlertCondition {
    /// Price rises above threshold
    #[serde(rename = "1")]
    PriceRise = 1,
    /// Price falls below threshold
    #[serde(rename = "2")]
    PriceFall = 2,
    /// Percentage rise above threshold
    #[serde(rename = "3")]
    PercentRise = 3,
    /// Percentage fall below threshold
    #[serde(rename = "4")]
    PercentFall = 4,
}

/// Alert trigger frequency
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AlertFrequency {
    /// Trigger once per day
    #[serde(rename = "1")]
    Daily = 1,
    /// Trigger every time condition is met
    #[serde(rename = "2")]
    EveryTime = 2,
    /// Trigger only once
    #[serde(rename = "3")]
    Once = 3,
}
