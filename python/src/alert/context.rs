use std::sync::Arc;

use longbridge::blocking::AlertContextSync;
use pyo3::prelude::*;

use crate::{alert::types::*, config::Config, error::ErrorNewType};

#[pyclass]
pub(crate) struct AlertContext {
    ctx: AlertContextSync,
}

#[pymethods]
impl AlertContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: AlertContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }
    fn list(&self) -> PyResult<AlertList> {
        Ok(self.ctx.list().map_err(ErrorNewType)?.into())
    }
    fn add(
        &self,
        symbol: String,
        condition: AlertCondition,
        trigger_value: String,
        frequency: AlertFrequency,
    ) -> PyResult<()> {
        self.ctx
            .add(symbol, condition.into(), trigger_value, frequency.into())
            .map_err(ErrorNewType)?;
        Ok(())
    }
    fn enable(&self, item: AlertItem) -> PyResult<()> {
        self.ctx.enable(item.into()).map_err(ErrorNewType)?;
        Ok(())
    }
    fn disable(&self, item: AlertItem) -> PyResult<()> {
        self.ctx.disable(item.into()).map_err(ErrorNewType)?;
        Ok(())
    }
    fn delete(&self, alert_ids: Vec<String>) -> PyResult<()> {
        self.ctx.delete(alert_ids).map_err(ErrorNewType)?;
        Ok(())
    }
}
