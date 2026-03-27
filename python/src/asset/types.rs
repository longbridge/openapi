use pyo3::prelude::*;

/// Statement type
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StatementType {
    /// Daily statement
    Daily = 1,
    /// Monthly statement
    Monthly = 2,
}

impl From<StatementType> for longbridge::asset::StatementType {
    fn from(value: StatementType) -> Self {
        match value {
            StatementType::Daily => longbridge::asset::StatementType::Daily,
            StatementType::Monthly => longbridge::asset::StatementType::Monthly,
        }
    }
}

/// Statement item
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct StatementItem {
    /// Statement date (integer, e.g. 20250301)
    #[pyo3(get)]
    pub dt: i32,
    /// File key used to request the download URL
    #[pyo3(get)]
    pub file_key: String,
}

impl From<longbridge::asset::StatementItem> for StatementItem {
    fn from(item: longbridge::asset::StatementItem) -> Self {
        Self {
            dt: item.dt,
            file_key: item.file_key,
        }
    }
}

/// Response for get statement list
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct GetStatementListResponse {
    /// List of statement items
    #[pyo3(get)]
    pub list: Vec<StatementItem>,
}

impl From<longbridge::asset::GetStatementListResponse> for GetStatementListResponse {
    fn from(resp: longbridge::asset::GetStatementListResponse) -> Self {
        Self {
            list: resp.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response for get statement download URL
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct GetStatementResponse {
    /// Presigned download URL
    #[pyo3(get)]
    pub url: String,
}

impl From<longbridge::asset::GetStatementResponse> for GetStatementResponse {
    fn from(resp: longbridge::asset::GetStatementResponse) -> Self {
        Self { url: resp.url }
    }
}
