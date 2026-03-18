use std::sync::Arc;

use napi::Result;

use crate::{
    config::Config,
    content::types::{NewsItem, TopicItem},
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
    pub async fn new(config: &Config) -> napi::Result<ContentContext> {
        Ok(Self {
            ctx: longbridge::content::ContentContext::try_new(Arc::new(config.0.clone()))
                .map_err(ErrorNewType)?,
        })
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
