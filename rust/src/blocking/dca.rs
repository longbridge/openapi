use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    dca::{DCAContext, types::*},
};

pub struct DCAContextSync {
    rt: BlockingRuntime<DCAContext>,
}

impl DCAContextSync {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = DCAContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }
    pub fn list(&self, status: Option<DCAStatus>, symbol: Option<String>) -> Result<DcaList> {
        self.rt
            .call(move |ctx| async move { ctx.list(status, symbol).await })
    }
    pub fn create(
        &self,
        symbol: impl Into<String> + Send + 'static,
        amount: impl Into<String> + Send + 'static,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: bool,
    ) -> Result<DcaCreateResult> {
        self.rt.call(move |ctx| async move {
            ctx.create(
                symbol,
                amount,
                frequency,
                day_of_week,
                day_of_month,
                allow_margin,
            )
            .await
        })
    }
    pub fn update(
        &self,
        plan_id: impl Into<String> + Send + 'static,
        amount: Option<String>,
        frequency: Option<DCAFrequency>,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: Option<bool>,
    ) -> Result<DcaCreateResult> {
        self.rt.call(move |ctx| async move {
            ctx.update(
                plan_id,
                amount,
                frequency,
                day_of_week,
                day_of_month,
                allow_margin,
            )
            .await
        })
    }
    pub fn pause(&self, plan_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.pause(plan_id).await })
    }
    pub fn resume(&self, plan_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.resume(plan_id).await })
    }
    pub fn stop(&self, plan_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.stop(plan_id).await })
    }
    pub fn history(
        &self,
        plan_id: impl Into<String> + Send + 'static,
        page: i32,
        limit: i32,
    ) -> Result<DcaHistoryResponse> {
        self.rt
            .call(move |ctx| async move { ctx.history(plan_id, page, limit).await })
    }
    pub fn stats(&self, symbol: Option<String>) -> Result<DcaStats> {
        self.rt
            .call(move |ctx| async move { ctx.stats(symbol).await })
    }
    pub fn check_support(&self, symbols: Vec<String>) -> Result<DcaSupportList> {
        self.rt
            .call(move |ctx| async move { ctx.check_support(symbols).await })
    }
    pub fn calc_date(
        &self,
        symbol: impl Into<String> + Send + 'static,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
    ) -> Result<DcaCalcDateResult> {
        self.rt.call(move |ctx| async move {
            ctx.calc_date(symbol, frequency, day_of_week, day_of_month)
                .await
        })
    }
    pub fn set_reminder(&self, hours: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.set_reminder(hours).await })
    }
}
