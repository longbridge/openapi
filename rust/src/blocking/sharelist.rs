use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    sharelist::{SharelistContext, types::*},
};

/// Blocking community sharelist management context.
pub struct SharelistContextSync {
    rt: BlockingRuntime<SharelistContext>,
}

impl SharelistContextSync {
    /// Create a [`SharelistContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = SharelistContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }
    /// List user's own and subscribed sharelists.
    pub fn list(&self, count: u32) -> Result<SharelistList> {
        self.rt
            .call(move |ctx| async move { ctx.list(count).await })
    }
    /// Get sharelist detail.
    pub fn detail(&self, id: i64) -> Result<SharelistDetail> {
        self.rt.call(move |ctx| async move { ctx.detail(id).await })
    }
    /// Get popular sharelists.
    pub fn popular(&self, count: u32) -> Result<SharelistList> {
        self.rt
            .call(move |ctx| async move { ctx.popular(count).await })
    }
    /// Create a new sharelist.
    pub fn create(
        &self,
        name: impl Into<String> + Send + 'static,
        description: Option<String>,
    ) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.create(name, description).await })
    }
    /// Delete a sharelist.
    pub fn delete(&self, id: i64) -> Result<serde_json::Value> {
        self.rt.call(move |ctx| async move { ctx.delete(id).await })
    }
    /// Add securities to a sharelist.
    pub fn add_securities(&self, id: i64, symbols: Vec<String>) -> Result<serde_json::Value> {
        self.rt
            .call(move |ctx| async move { ctx.add_securities(id, symbols).await })
    }
    /// Remove securities from a sharelist.
    pub fn remove_securities(&self, id: i64, symbols: Vec<String>) -> Result<serde_json::Value> {
        self.rt
            .call(move |ctx| async move { ctx.remove_securities(id, symbols).await })
    }
    /// Reorder securities in a sharelist.
    pub fn sort_securities(&self, id: i64, symbols: Vec<String>) -> Result<serde_json::Value> {
        self.rt
            .call(move |ctx| async move { ctx.sort_securities(id, symbols).await })
    }
}
