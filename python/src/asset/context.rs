use std::sync::Arc;

use longbridge::blocking::AssetContextSync;
use pyo3::prelude::*;

use crate::{
    asset::types::{GetStatementListResponse, GetStatementResponse, StatementType},
    config::Config,
    error::ErrorNewType,
};

#[pyclass]
pub(crate) struct AssetContext {
    ctx: AssetContextSync,
}

#[pymethods]
impl AssetContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: AssetContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get statement data list
    #[pyo3(signature = (statement_type, start_date = 1, limit = 20))]
    pub fn statements(
        &self,
        statement_type: StatementType,
        start_date: i32,
        limit: i32,
    ) -> PyResult<GetStatementListResponse> {
        let opts = longbridge::asset::GetStatementListOptions::new(statement_type.into())
            .page(start_date)
            .page_size(limit);
        Ok(self.ctx.statements(opts).map_err(ErrorNewType)?.into())
    }

    /// Get statement data download URL
    pub fn statement_download_url(&self, file_key: String) -> PyResult<GetStatementResponse> {
        let opts = longbridge::asset::GetStatementOptions::new(file_key);
        Ok(self
            .ctx
            .statement_download_url(opts)
            .map_err(ErrorNewType)?
            .into())
    }
}
