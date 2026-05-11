use longbridge::alert::types as lb;

/// One price alert
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
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
    pub state: Vec<i32>,
    /// Trigger value: `{"price":"500"}` or `{"chg":"5"}`
    pub value_map: serde_json::Value,
}
impl From<lb::AlertItem> for AlertItem {
    fn from(v: lb::AlertItem) -> Self {
        Self {
            id: v.id,
            indicator_id: v.indicator_id,
            enabled: v.enabled,
            frequency: v.frequency,
            scope: v.scope,
            text: v.text,
            state: v.state,
            value_map: v.value_map,
        }
    }
}

impl From<AlertItem> for lb::AlertItem {
    fn from(v: AlertItem) -> Self {
        Self {
            id: v.id,
            indicator_id: v.indicator_id,
            enabled: v.enabled,
            frequency: v.frequency,
            scope: v.scope,
            text: v.text,
            state: v.state,
            value_map: v.value_map,
        }
    }
}

/// Alert items for one security
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
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
    pub price: String,
    /// Day change amount
    pub chg: String,
    /// Day change percentage
    pub p_chg: String,
    /// Product type (may be empty)
    pub product: String,
    /// Alert items
    pub indicators: Vec<AlertItem>,
}
impl From<lb::AlertSymbolGroup> for AlertSymbolGroup {
    fn from(v: lb::AlertSymbolGroup) -> Self {
        Self {
            symbol: v.symbol,
            code: v.code,
            market: v.market,
            name: v.name,
            price: v.price.map(|d| d.to_string()).unwrap_or_default(),
            chg: v.chg.map(|d| d.to_string()).unwrap_or_default(),
            p_chg: v.p_chg.map(|d| d.to_string()).unwrap_or_default(),
            product: v.product,
            indicators: v.indicators.into_iter().map(Into::into).collect(),
        }
    }
}

/// Alert list response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AlertList {
    /// Alert groups per security
    pub lists: Vec<AlertSymbolGroup>,
}
impl From<lb::AlertList> for AlertList {
    fn from(v: lb::AlertList) -> Self {
        Self {
            lists: v.lists.into_iter().map(Into::into).collect(),
        }
    }
}

/// Alert condition
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum AlertCondition {
    /// Price rises above threshold
    PriceRise,
    /// Price falls below threshold
    PriceFall,
    /// Percentage rise above threshold
    PercentRise,
    /// Percentage fall below threshold
    PercentFall,
}
impl From<AlertCondition> for lb::AlertCondition {
    fn from(v: AlertCondition) -> Self {
        match v {
            AlertCondition::PriceRise => lb::AlertCondition::PriceRise,
            AlertCondition::PriceFall => lb::AlertCondition::PriceFall,
            AlertCondition::PercentRise => lb::AlertCondition::PercentRise,
            AlertCondition::PercentFall => lb::AlertCondition::PercentFall,
        }
    }
}

/// Alert trigger frequency
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum AlertFrequency {
    /// Trigger once per day
    Daily,
    /// Trigger every time condition is met
    EveryTime,
    /// Trigger only once
    Once,
}
impl From<AlertFrequency> for lb::AlertFrequency {
    fn from(v: AlertFrequency) -> Self {
        match v {
            AlertFrequency::Daily => lb::AlertFrequency::Daily,
            AlertFrequency::EveryTime => lb::AlertFrequency::EveryTime,
            AlertFrequency::Once => lb::AlertFrequency::Once,
        }
    }
}
