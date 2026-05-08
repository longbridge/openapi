use std::sync::Arc;

use napi::Result;

use crate::{config::Config, error::ErrorNewType, sharelist::types::*};

/// Community sharelist management context.
#[napi_derive::napi]
#[derive(Clone)]
pub struct SharelistContext {
    ctx: longbridge::SharelistContext,
}

#[napi_derive::napi]
impl SharelistContext {
    /// Create a new SharelistContext.
    #[napi]
    pub fn new(config: &Config) -> SharelistContext {
        Self {
            ctx: longbridge::SharelistContext::new(Arc::new(config.0.clone())),
        }
    }

    /// List user's own and subscribed sharelists.
    ///
    /// `count` controls how many sharelists are returned per category.
    #[napi]
    pub async fn list(&self, count: u32) -> Result<SharelistList> {
        Ok(self.ctx.list(count).await.map_err(ErrorNewType)?.into())
    }

    /// Get sharelist detail including constituent stocks and subscription info.
    #[napi]
    pub async fn detail(&self, id: i64) -> Result<SharelistDetail> {
        Ok(self.ctx.detail(id).await.map_err(ErrorNewType)?.into())
    }

    /// Get popular (trending) sharelists.
    #[napi]
    pub async fn popular(&self, count: u32) -> Result<SharelistList> {
        Ok(self.ctx.popular(count).await.map_err(ErrorNewType)?.into())
    }

    /// Create a new sharelist.
    #[napi]
    pub async fn create(&self, name: String, description: Option<String>) -> Result<()> {
        Ok(self
            .ctx
            .create(name, description)
            .await
            .map_err(ErrorNewType)?)
    }

    /// Delete a sharelist.
    #[napi]
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.ctx.delete(id).await.map_err(ErrorNewType)?;
        Ok(())
    }

    /// Add securities to a sharelist.
    #[napi]
    pub async fn add_securities(&self, id: i64, symbols: Vec<String>) -> Result<()> {
        self.ctx
            .add_securities(id, symbols)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Remove securities from a sharelist.
    #[napi]
    pub async fn remove_securities(&self, id: i64, symbols: Vec<String>) -> Result<()> {
        self.ctx
            .remove_securities(id, symbols)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Reorder securities in a sharelist.
    #[napi]
    pub async fn sort_securities(&self, id: i64, symbols: Vec<String>) -> Result<()> {
        self.ctx
            .sort_securities(id, symbols)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }
}
