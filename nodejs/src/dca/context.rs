use std::sync::Arc;

use napi::Result;

use crate::{config::Config, dca::types::*, error::ErrorNewType};

/// Dollar-cost averaging (DCA) plan management context.
#[napi_derive::napi]
#[derive(Clone)]
pub struct DCAContext {
    ctx: longbridge::DCAContext,
}

#[napi_derive::napi]
impl DCAContext {
    /// Create a new DCAContext.
    #[napi]
    pub fn new(config: &Config) -> DCAContext {
        Self {
            ctx: longbridge::DCAContext::new(Arc::new(config.0.clone())),
        }
    }

    /// List DCA plans.
    ///
    /// Pass `null` for `status` to return all plans regardless of status.
    #[napi]
    pub async fn list(&self, status: Option<DCAStatus>, symbol: Option<String>) -> Result<DcaList> {
        Ok(self
            .ctx
            .list(status.map(Into::into), symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Create a new DCA plan.
    ///
    /// `dayOfWeek` is required when `frequency` is `Weekly` or `Fortnightly`
    /// (e.g. `"Mon"`). `dayOfMonth` is required when `frequency` is
    /// `Monthly` (e.g. `"15"`).
    #[napi]
    pub async fn create(
        &self,
        symbol: String,
        amount: String,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: bool,
    ) -> Result<DcaCreateResult> {
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
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Update an existing DCA plan.
    #[napi]
    pub async fn update(
        &self,
        plan_id: String,
        amount: Option<String>,
        frequency: Option<DCAFrequency>,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: Option<bool>,
    ) -> Result<DcaCreateResult> {
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
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Pause (suspend) a DCA plan.
    #[napi]
    pub async fn pause(&self, plan_id: String) -> Result<()> {
        Ok(self.ctx.pause(plan_id).await.map_err(ErrorNewType)?)
    }

    /// Resume a suspended DCA plan.
    #[napi]
    pub async fn resume(&self, plan_id: String) -> Result<()> {
        Ok(self.ctx.resume(plan_id).await.map_err(ErrorNewType)?)
    }

    /// Permanently stop a DCA plan.
    #[napi]
    pub async fn stop(&self, plan_id: String) -> Result<()> {
        Ok(self.ctx.stop(plan_id).await.map_err(ErrorNewType)?)
    }

    /// Get execution history for a DCA plan.
    #[napi]
    pub async fn history(
        &self,
        plan_id: String,
        page: i32,
        limit: i32,
    ) -> Result<DcaHistoryResponse> {
        Ok(self
            .ctx
            .history(plan_id, page, limit)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get DCA statistics.
    ///
    /// Pass `null` for `symbol` to get aggregate statistics across all plans.
    #[napi]
    pub async fn stats(&self, symbol: Option<String>) -> Result<DcaStats> {
        Ok(self.ctx.stats(symbol).await.map_err(ErrorNewType)?.into())
    }

    /// Check DCA support for a list of securities.
    #[napi]
    pub async fn check_support(&self, symbols: Vec<String>) -> Result<DcaSupportList> {
        Ok(self
            .ctx
            .check_support(symbols)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Calculate the next projected trade date for a DCA plan.
    ///
    /// `dayOfWeek` is used for `Weekly`/`Fortnightly` frequency (e.g. `"Mon"`).
    /// `dayOfMonth` is used for `Monthly` frequency (1–28).
    #[napi]
    pub async fn calc_date(
        &self,
        symbol: String,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
    ) -> Result<DcaCalcDateResult> {
        Ok(self
            .ctx
            .calc_date(symbol, frequency.into(), day_of_week, day_of_month)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Update the advance reminder hours for DCA execution notifications.
    ///
    /// `hours` must be one of `"1"`, `"6"`, or `"12"`.
    #[napi]
    pub async fn set_reminder(&self, hours: String) -> Result<()> {
        Ok(self.ctx.set_reminder(hours).await.map_err(ErrorNewType)?)
    }
}
