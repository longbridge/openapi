use std::sync::Arc;

use longbridge::blocking::DCAContextSync;
use pyo3::prelude::*;

use crate::{config::Config, dca::types::*, error::ErrorNewType};

#[pyclass]
pub(crate) struct DCAContext {
    ctx: DCAContextSync,
}

#[pymethods]
impl DCAContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: DCAContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }
    #[pyo3(signature = (status = None, symbol = None))]
    fn list(&self, status: Option<DCAStatus>, symbol: Option<String>) -> PyResult<DcaList> {
        Ok(self
            .ctx
            .list(status.map(Into::into), symbol)
            .map_err(ErrorNewType)?
            .into())
    }
    #[pyo3(signature = (symbol, amount, frequency, day_of_week = None, day_of_month = None, allow_margin = false))]
    fn create(
        &self,
        symbol: String,
        amount: String,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: bool,
    ) -> PyResult<DcaCreateResult> {
        Ok(self
            .ctx
            .create(
                symbol,
                amount,
                frequency.into(),
                day_of_week,
                day_of_month,
                allow_margin,
            )
            .map_err(ErrorNewType)?
            .into())
    }
    #[pyo3(signature = (plan_id, amount = None, frequency = None, day_of_week = None, day_of_month = None, allow_margin = None))]
    fn update(
        &self,
        plan_id: String,
        amount: Option<String>,
        frequency: Option<DCAFrequency>,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: Option<bool>,
    ) -> PyResult<DcaCreateResult> {
        Ok(self
            .ctx
            .update(
                plan_id,
                amount,
                frequency.map(Into::into),
                day_of_week,
                day_of_month,
                allow_margin,
            )
            .map_err(ErrorNewType)?
            .into())
    }
    fn pause(&self, plan_id: String) -> PyResult<()> {
        Ok(self.ctx.pause(plan_id).map_err(ErrorNewType)?)
    }
    fn resume(&self, plan_id: String) -> PyResult<()> {
        Ok(self.ctx.resume(plan_id).map_err(ErrorNewType)?)
    }
    fn stop(&self, plan_id: String) -> PyResult<()> {
        Ok(self.ctx.stop(plan_id).map_err(ErrorNewType)?)
    }
    #[pyo3(signature = (plan_id, page = 1, limit = 20))]
    fn history(&self, plan_id: String, page: i32, limit: i32) -> PyResult<DcaHistoryResponse> {
        Ok(self
            .ctx
            .history(plan_id, page, limit)
            .map_err(ErrorNewType)?
            .into())
    }
    #[pyo3(signature = (symbol = None))]
    fn stats(&self, symbol: Option<String>) -> PyResult<DcaStats> {
        Ok(self.ctx.stats(symbol).map_err(ErrorNewType)?.into())
    }
    fn check_support(&self, symbols: Vec<String>) -> PyResult<DcaSupportList> {
        Ok(self
            .ctx
            .check_support(symbols)
            .map_err(ErrorNewType)?
            .into())
    }
    #[pyo3(signature = (symbol, frequency, day_of_week = None, day_of_month = None))]
    fn calc_date(
        &self,
        symbol: String,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
    ) -> PyResult<DcaCalcDateResult> {
        Ok(self
            .ctx
            .calc_date(symbol, frequency.into(), day_of_week, day_of_month)
            .map_err(ErrorNewType)?
            .into())
    }
    fn set_reminder(&self, hours: String) -> PyResult<()> {
        Ok(self.ctx.set_reminder(hours).map_err(ErrorNewType)?)
    }
}
