use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    signal::{SignalContext, types::*},
};

/// Blocking signal context
pub struct SignalContextSync {
    rt: BlockingRuntime<SignalContext>,
}

impl SignalContextSync {
    /// Create a [`SignalContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = SignalContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Query signals
    pub fn signals(&self, opts: SignalsOptions) -> Result<SignalsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.signals(opts).await })
    }

    /// Get one signal by ID
    pub fn signal(&self, signal_id: impl Into<String> + Send + 'static) -> Result<Signal> {
        self.rt
            .call(move |ctx| async move { ctx.signal(signal_id).await })
    }

    /// List the fact (catalyst) events for one security
    pub fn security_facts(&self, opts: SecurityFactsOptions) -> Result<Vec<SecurityFact>> {
        self.rt
            .call(move |ctx| async move { ctx.security_facts(opts).await })
    }
}
