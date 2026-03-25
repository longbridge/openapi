use std::sync::Arc;

use napi::Result;

use crate::{
    config::Config,
    content::{
        requests::{CreateTopicRequest, ListMyTopicsRequest},
        types::{NewsItem, OwnedTopic, TopicItem},
    },
    error::ErrorNewType,
};

/// Content context
#[napi_derive::napi]
#[derive(Clone)]
pub struct ContentContext {
    ctx: longbridge::content::ContentContext,
}

#[napi_derive::napi]
impl ContentContext {
    /// Create a new `ContentContext`
    #[napi]
    pub fn new(config: &Config) -> ContentContext {
        Self {
            ctx: longbridge::content::ContentContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get topics created by the current authenticated user
    #[napi]
    pub async fn topics_mine(&self, req: Option<ListMyTopicsRequest>) -> Result<Vec<OwnedTopic>> {
        self.ctx
            .topics_mine(req.unwrap_or_default().into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Create a new topic
    #[napi]
    pub async fn create_topic(&self, req: CreateTopicRequest) -> Result<String> {
        Ok(self
            .ctx
            .create_topic(req.into())
            .await
            .map_err(ErrorNewType)?)
    }

    /// Get discussion topics list
    #[napi]
    pub async fn topics(&self, symbol: String) -> Result<Vec<TopicItem>> {
        self.ctx
            .topics(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get news list
    #[napi]
    pub async fn news(&self, symbol: String) -> Result<Vec<NewsItem>> {
        self.ctx
            .news(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
