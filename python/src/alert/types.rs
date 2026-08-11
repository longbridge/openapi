use longbridge::alert::types as lb;
use pyo3::prelude::*;

use crate::decimal::PyDecimal;

/// Trigger value of a price alert (exactly one field is populated).
#[pyclass(get_all, from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct AlertValueMap {
    /// Absolute price threshold, e.g. `500`.
    pub price: Option<PyDecimal>,
    /// Percentage-change threshold, e.g. `5`.
    pub chg: Option<f64>,
}

impl From<lb::AlertValueMap> for AlertValueMap {
    fn from(v: lb::AlertValueMap) -> Self {
        Self {
            price: v.price.map(Into::into),
            chg: v.chg,
        }
    }
}

impl From<AlertValueMap> for lb::AlertValueMap {
    fn from(v: AlertValueMap) -> Self {
        Self {
            price: v.price.map(Into::into),
            chg: v.chg,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct AlertItem {
    pub id: String,
    pub indicator_id: String,
    #[pyo3(set)]
    pub enabled: bool,
    pub frequency: i32,
    pub scope: i32,
    pub text: String,
    pub state: Vec<i32>,
    pub value_map: AlertValueMap,
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
            value_map: v.value_map.into(),
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
            value_map: v.value_map.into(),
        }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for AlertItem {
    type Error = PyErr;

    fn extract(ob: pyo3::Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        let value_map = ob
            .getattr("value_map")
            .ok()
            .and_then(|v| v.extract().ok())
            .unwrap_or_default();
        Ok(AlertItem {
            id: ob.getattr("id")?.extract()?,
            indicator_id: ob.getattr("indicator_id")?.extract()?,
            enabled: ob.getattr("enabled")?.extract()?,
            frequency: ob.getattr("frequency")?.extract()?,
            scope: ob.getattr("scope")?.extract()?,
            text: ob.getattr("text")?.extract()?,
            state: ob.getattr("state")?.extract()?,
            value_map,
        })
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct AlertSymbolGroup {
    pub symbol: String,
    pub code: String,
    pub market: String,
    pub name: String,
    pub price: Option<String>,
    pub chg: Option<String>,
    pub p_chg: Option<String>,
    pub product: String,
    pub indicators: Vec<AlertItem>,
}

impl From<lb::AlertSymbolGroup> for AlertSymbolGroup {
    fn from(v: lb::AlertSymbolGroup) -> Self {
        Self {
            symbol: v.symbol,
            code: v.code,
            market: v.market,
            name: v.name,
            price: v.price.map(|d| d.to_string()),
            chg: v.chg.map(|d| d.to_string()),
            p_chg: v.p_chg.map(|d| d.to_string()),
            product: v.product,
            indicators: v.indicators.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct AlertList {
    pub lists: Vec<AlertSymbolGroup>,
}

impl From<lb::AlertList> for AlertList {
    fn from(v: lb::AlertList) -> Self {
        Self {
            lists: v.lists.into_iter().map(Into::into).collect(),
        }
    }
}

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum AlertCondition {
    PriceRise = 1,
    PriceFall = 2,
    PercentRise = 3,
    PercentFall = 4,
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

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum AlertFrequency {
    Daily = 1,
    EveryTime = 2,
    Once = 3,
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
