use std::sync::Arc;

use longbridge::blocking::SharelistContextSync;
use pyo3::prelude::*;

use crate::{config::Config, error::ErrorNewType, sharelist::types::*};

#[pyclass]
pub(crate) struct SharelistContext {
    ctx: SharelistContextSync,
}

#[pymethods]
impl SharelistContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: SharelistContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }
    #[pyo3(signature = (count = 20))]
    fn list(&self, count: u32) -> PyResult<SharelistList> {
        Ok(self.ctx.list(count).map_err(ErrorNewType)?.into())
    }
    fn detail(&self, id: i64) -> PyResult<SharelistDetail> {
        Ok(self.ctx.detail(id).map_err(ErrorNewType)?.into())
    }
    #[pyo3(signature = (count = 20))]
    fn popular(&self, count: u32) -> PyResult<SharelistList> {
        Ok(self.ctx.popular(count).map_err(ErrorNewType)?.into())
    }
    #[pyo3(signature = (name, description = None))]
    fn create(&self, name: String, description: Option<String>) -> PyResult<()> {
        Ok(self.ctx.create(name, description).map_err(ErrorNewType)?)
    }
    fn delete(&self, id: i64) -> PyResult<()> {
        self.ctx.delete(id).map_err(ErrorNewType)?;
        Ok(())
    }
    fn add_securities(&self, id: i64, symbols: Vec<String>) -> PyResult<()> {
        self.ctx.add_securities(id, symbols).map_err(ErrorNewType)?;
        Ok(())
    }
    fn remove_securities(&self, id: i64, symbols: Vec<String>) -> PyResult<()> {
        self.ctx
            .remove_securities(id, symbols)
            .map_err(ErrorNewType)?;
        Ok(())
    }
    fn sort_securities(&self, id: i64, symbols: Vec<String>) -> PyResult<()> {
        self.ctx
            .sort_securities(id, symbols)
            .map_err(ErrorNewType)?;
        Ok(())
    }
}
