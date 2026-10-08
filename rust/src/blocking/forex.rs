use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    forex::{
        ForexContext, ForexOrderDetail, ForexQuote, GetForexQuoteOptions, SubmitForexOrderOptions,
    },
};

/// Blocking forex (currency exchange) channel context.
pub struct ForexContextSync {
    rt: BlockingRuntime<ForexContext>,
}

impl ForexContextSync {
    /// Create a [`ForexContextSync`].
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = ForexContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get a forex quote (blocking).
    pub fn quote(
        &self,
        from: impl Into<String> + Send + 'static,
        to: impl Into<String> + Send + 'static,
        options: impl Into<Option<GetForexQuoteOptions>> + Send + 'static,
    ) -> Result<ForexQuote> {
        self.rt
            .call(move |ctx| async move { ctx.quote(from, to, options).await })
    }

    /// Submit a forex order (blocking).
    pub fn submit_order(&self, options: SubmitForexOrderOptions) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.submit_order(options).await })
    }

    /// Query a forex order by `client_order_id` (blocking).
    pub fn order(
        &self,
        client_order_id: impl Into<String> + Send + 'static,
    ) -> Result<ForexOrderDetail> {
        self.rt
            .call(move |ctx| async move { ctx.order(client_order_id).await })
    }
}
