use longbridge::sharelist::types as lb;
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

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct SharelistStock {
    pub symbol: String,
    pub name: String,
    pub market: String,
    pub code: String,
    pub intro: String,
    pub unread_change_log_category: String,
    pub change: Option<String>,
    pub last_done: Option<String>,
    pub trade_status: Option<i32>,
    pub latency: Option<bool>,
}
impl From<lb::SharelistStock> for SharelistStock {
    fn from(v: lb::SharelistStock) -> Self {
        Self {
            symbol: v.symbol,
            name: v.name,
            market: v.market,
            code: v.code,
            intro: v.intro,
            unread_change_log_category: v.unread_change_log_category,
            change: v.change.map(|d| d.to_string()),
            last_done: v.last_done.map(|d| d.to_string()),
            trade_status: v.trade_status,
            latency: v.latency,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct SharelistScopes {
    pub subscription: bool,
    pub is_self: bool,
}
impl From<lb::SharelistScopes> for SharelistScopes {
    fn from(v: lb::SharelistScopes) -> Self {
        Self {
            subscription: v.subscription,
            is_self: v.is_self,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct SharelistInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub cover: String,
    pub subscribers_count: i64,
    pub created_at: crate::time::PyOffsetDateTimeWrapper,
    pub edited_at: crate::time::PyOffsetDateTimeWrapper,
    pub this_year_chg: Option<String>,
    pub creator: JsonValue,
    pub stocks: Vec<SharelistStock>,
    pub subscribed: bool,
    pub chg: Option<String>,
    pub sharelist_type: i32,
    pub industry_code: String,
}
impl From<lb::SharelistInfo> for SharelistInfo {
    fn from(v: lb::SharelistInfo) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            cover: v.cover,
            subscribers_count: v.subscribers_count,
            created_at: crate::time::PyOffsetDateTimeWrapper(v.created_at),
            edited_at: crate::time::PyOffsetDateTimeWrapper(v.edited_at),
            this_year_chg: v.this_year_chg.map(|d| d.to_string()),
            creator: JsonValue(v.creator),
            stocks: v.stocks.into_iter().map(Into::into).collect(),
            subscribed: v.subscribed,
            chg: v.chg.map(|d| d.to_string()),
            sharelist_type: v.sharelist_type,
            industry_code: v.industry_code,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct SharelistList {
    pub sharelists: Vec<SharelistInfo>,
    pub subscribed_sharelists: Vec<SharelistInfo>,
    pub tail_mark: String,
}
impl From<lb::SharelistList> for SharelistList {
    fn from(v: lb::SharelistList) -> Self {
        Self {
            sharelists: v.sharelists.into_iter().map(Into::into).collect(),
            subscribed_sharelists: v
                .subscribed_sharelists
                .into_iter()
                .map(Into::into)
                .collect(),
            tail_mark: v.tail_mark,
        }
    }
}

#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct SharelistDetail {
    pub sharelist: SharelistInfo,
    pub scopes: SharelistScopes,
}
impl From<lb::SharelistDetail> for SharelistDetail {
    fn from(v: lb::SharelistDetail) -> Self {
        Self {
            sharelist: v.sharelist.into(),
            scopes: v.scopes.into(),
        }
    }
}
