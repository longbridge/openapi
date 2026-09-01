use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    grid::{
        GetGridOrderDetailOptions, GetGridOrdersByIdsOptions, GetGridOrdersOptions,
        GetGridTriggerHistoryOptions, GridContext, GridOrder, GridOrderDetail, GridOrdersResponse,
        GridSymbolInfo, GridTriggerHistoryResponse, ReplaceGridOrderOptions,
        SubmitGridOrderOptions, SubmitGridOrderResponse,
    },
};

/// Blocking grid trading management context.
pub struct GridContextSync {
    rt: BlockingRuntime<GridContext>,
}

impl GridContextSync {
    /// Create a [`GridContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = GridContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Submit a grid trading order (blocking)
    pub fn submit(&self, options: SubmitGridOrderOptions) -> Result<SubmitGridOrderResponse> {
        self.rt
            .call(move |ctx| async move { ctx.submit(options).await })
    }

    /// Replace (modify) a grid trading order (blocking)
    pub fn replace(&self, options: ReplaceGridOrderOptions) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.replace(options).await })
    }

    /// Get grid trading orders (paged list) (blocking)
    pub fn list(
        &self,
        options: impl Into<Option<GetGridOrdersOptions>> + Send + 'static,
    ) -> Result<GridOrdersResponse> {
        self.rt
            .call(move |ctx| async move { ctx.list(options).await })
    }

    /// Query grid trading orders by IDs (blocking)
    pub fn list_by_ids(&self, options: GetGridOrdersByIdsOptions) -> Result<Vec<GridOrder>> {
        self.rt
            .call(move |ctx| async move { ctx.list_by_ids(options).await })
    }

    /// Get grid trading order detail (blocking)
    pub fn detail(&self, options: GetGridOrderDetailOptions) -> Result<GridOrderDetail> {
        self.rt
            .call(move |ctx| async move { ctx.detail(options).await })
    }

    /// Get grid trading trigger history (blocking)
    pub fn trigger_history(
        &self,
        options: GetGridTriggerHistoryOptions,
    ) -> Result<GridTriggerHistoryResponse> {
        self.rt
            .call(move |ctx| async move { ctx.trigger_history(options).await })
    }

    /// Cancel a grid trading order (blocking)
    pub fn cancel(&self, order_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.cancel(order_id).await })
    }

    /// Suspend a grid trading order (blocking)
    pub fn suspend(&self, order_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.suspend(order_id).await })
    }

    /// Restart a grid trading order (blocking)
    pub fn restart(&self, order_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.restart(order_id).await })
    }

    /// Get the security (symbol) info used to build a grid order (blocking)
    pub fn symbol_info(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<GridSymbolInfo> {
        self.rt
            .call(move |ctx| async move { ctx.symbol_info(symbol).await })
    }
}
