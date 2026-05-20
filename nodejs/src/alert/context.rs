use std::sync::Arc;

use napi::Result;

use crate::{alert::types::*, config::Config, error::ErrorNewType};

/// Price alert management context.
#[napi_derive::napi]
#[derive(Clone)]
pub struct AlertContext {
    ctx: longbridge::AlertContext,
}

#[napi_derive::napi]
impl AlertContext {
    /// Create a new AlertContext.
    #[napi]
    pub fn new(config: &Config) -> AlertContext {
        Self {
            ctx: longbridge::AlertContext::new(Arc::new(config.0.clone())),
        }
    }

    /// List all price alerts.
    #[napi]
    pub async fn list(&self) -> Result<AlertList> {
        Ok(self.ctx.list().await.map_err(ErrorNewType)?.into())
    }

    /// Add a price alert for a security.
    ///
    /// `triggerValue` is a price or percentage string depending on `condition`.
    #[napi]
    pub async fn add(
        &self,
        symbol: String,
        condition: AlertCondition,
        trigger_value: String,
        frequency: AlertFrequency,
    ) -> Result<()> {
        self.ctx
            .add(symbol, condition.into(), trigger_value, frequency.into())
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Update a price alert.
    ///
    /// Pass the [`AlertItem`] obtained from [`list`](Self::list). Set
    /// `item.enabled` to `true` to re-enable or `false` to disable before
    /// calling this method.
    #[napi]
    pub async fn update(&self, item: AlertItem) -> Result<()> {
        self.ctx.update(&item.into()).await.map_err(ErrorNewType)?;
        Ok(())
    }

    /// Delete one or more price alerts by ID.
    #[napi]
    pub async fn delete(&self, alert_ids: Vec<String>) -> Result<()> {
        self.ctx.delete(alert_ids).await.map_err(ErrorNewType)?;
        Ok(())
    }
}
