use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    content::{
        ContentContext, CreateReplyOptions, CreateTopicOptions, ListTopicRepliesOptions,
        MyTopicsOptions, NewsItem, OwnedTopic, TopicItem, TopicReply,
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
    pub fn my_topics(&self, opts: MyTopicsOptions) -> Result<Vec<OwnedTopic>> {
        self.rt
            .call(move |ctx| async move { ctx.my_topics(opts).await })
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

    /// Get full details of a topic by its ID
    pub fn topic_detail(&self, id: impl Into<String>) -> Result<OwnedTopic> {
        let id = id.into();
        self.rt
            .call(move |ctx| async move { ctx.topic_detail(id).await })
    }

    /// List replies on a topic
    pub fn list_topic_replies(
        &self,
        topic_id: impl Into<String>,
        opts: ListTopicRepliesOptions,
    ) -> Result<Vec<TopicReply>> {
        let topic_id = topic_id.into();
        self.rt
            .call(move |ctx| async move { ctx.list_topic_replies(topic_id, opts).await })
    }

    /// Post a reply to a topic
    pub fn create_topic_reply(
        &self,
        topic_id: impl Into<String>,
        opts: CreateReplyOptions,
    ) -> Result<TopicReply> {
        let topic_id = topic_id.into();
        self.rt
            .call(move |ctx| async move { ctx.create_topic_reply(topic_id, opts).await })
    }
}
