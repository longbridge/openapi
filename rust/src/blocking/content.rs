use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    content::{
        ContentContext, CreateTopicOptions, ListMyTopicsOptions, NewsItem, OwnedTopic, TopicItem,
    },
};

/// Blocking content context
pub struct ContentContextSync {
    rt: BlockingRuntime<ContentContext>,
}

impl ContentContextSync {
    /// Create a `ContentContextSync`
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = ContentContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx); // keep sender alive so event_rx never closes
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get topics created by the current authenticated user
    pub fn topics_mine(&self, opts: ListMyTopicsOptions) -> Result<Vec<OwnedTopic>> {
        self.rt
            .call(move |ctx| async move { ctx.topics_mine(opts).await })
    }

    /// Create a new topic
    pub fn create_topic(&self, opts: CreateTopicOptions) -> Result<String> {
        self.rt
            .call(move |ctx| async move { ctx.create_topic(opts).await })
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
