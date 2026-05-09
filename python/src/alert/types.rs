use longbridge::alert::types as lb;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

#[derive(Debug, Clone)]
pub(crate) struct JsonValue(pub(crate) serde_json::Value);

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
impl<'py> IntoPyObject<'py> for &JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        pythonize::pythonize(py, &self.0).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub(crate) struct AlertItem {
    pub id: String,
    pub indicator_id: String,
    pub enabled: bool,
    pub frequency: i32,
    pub scope: i32,
    pub text: String,
    pub state: Vec<i32>,
    pub value_map: JsonValue,
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
            value_map: JsonValue(v.value_map),
        }
    }
}

#[pyclass(get_all)]
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

#[pyclass(get_all)]
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
