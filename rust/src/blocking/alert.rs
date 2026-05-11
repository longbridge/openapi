use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    alert::{AlertContext, types::*},
    blocking::runtime::BlockingRuntime,
};

/// Blocking price alert context
pub struct AlertContextSync {
    rt: BlockingRuntime<AlertContext>,
}

impl AlertContextSync {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = AlertContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }
    pub fn list(&self) -> Result<AlertList> {
        self.rt.call(|ctx| async move { ctx.list().await })
    }
    pub fn add(
        &self,
        symbol: impl Into<String> + Send + 'static,
        condition: AlertCondition,
        trigger_value: impl Into<String> + Send + 'static,
        frequency: AlertFrequency,
    ) -> Result<serde_json::Value> {
        self.rt.call(move |ctx| async move {
            ctx.add(symbol, condition, trigger_value, frequency).await
        })
    }
    pub fn enable(&self, item: AlertItem) -> Result<serde_json::Value> {
        self.rt
            .call(move |ctx| async move { ctx.enable(&item).await })
    }
    pub fn disable(&self, item: AlertItem) -> Result<serde_json::Value> {
        self.rt
            .call(move |ctx| async move { ctx.disable(&item).await })
    }
    pub fn delete(&self, alert_ids: Vec<String>) -> Result<serde_json::Value> {
        self.rt
            .call(move |ctx| async move { ctx.delete(alert_ids).await })
    }
}
