use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    content::{ContentContext, NewsItem, TopicItem},
};

/// Blocking content context
pub struct ContentContextSync {
    rt: BlockingRuntime<ContentContext>,
}

impl ContentContextSync {
    /// Create a `ContentContextSync`
    pub fn try_new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || async move {
                let ctx = ContentContext::try_new(config)?;
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx); // keep sender alive so event_rx never closes
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get discussion topics list
    pub fn topics(&self, symbol: impl Into<String>) -> Result<Vec<TopicItem>> {
        let symbol = symbol.into();
        self.rt
            .call(move |ctx| async move { ctx.topics(symbol).await })
    }

    /// Get news list
    pub fn news(&self, symbol: impl Into<String>) -> Result<Vec<NewsItem>> {
        let symbol = symbol.into();
        self.rt
            .call(move |ctx| async move { ctx.news(symbol).await })
    }
}
