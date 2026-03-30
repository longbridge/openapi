use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    asset::{
        AssetContext, GetStatementListOptions, GetStatementListResponse, GetStatementOptions,
        GetStatementResponse,
    },
    blocking::runtime::BlockingRuntime,
};

/// Blocking asset context
pub struct AssetContextSync {
    rt: BlockingRuntime<AssetContext>,
}

impl AssetContextSync {
    /// Create a `AssetContextSync`
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = AssetContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get statement data list
    pub fn statements(&self, options: GetStatementListOptions) -> Result<GetStatementListResponse> {
        self.rt
            .call(move |ctx| async move { ctx.statements(options).await })
    }

    /// Get statement data download url
    pub fn statement_download_url(
        &self,
        options: GetStatementOptions,
    ) -> Result<GetStatementResponse> {
        self.rt
            .call(move |ctx| async move { ctx.statement_download_url(options).await })
    }
}
