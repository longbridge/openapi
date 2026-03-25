use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Deserialize;

use super::types::{CreateTopicOptions, ListMyTopicsOptions, NewsItem, OwnedTopic, TopicItem};
use crate::{Config, Result};

struct InnerContentContext {
    http_cli: HttpClient,
}

/// Content context
#[derive(Clone)]
pub struct ContentContext(Arc<InnerContentContext>);

impl ContentContext {
    /// Create a `ContentContext`
    pub fn new(config: Arc<Config>) -> Self {
        Self(Arc::new(InnerContentContext {
            http_cli: config.create_http_client(),
        }))
    }

    /// Get topics created by the current authenticated user
    ///
    /// Path: GET /v1/content/topics/mine
    pub async fn topics_mine(&self, opts: ListMyTopicsOptions) -> Result<Vec<OwnedTopic>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<OwnedTopic>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/content/topics/mine")
            .query_params(opts)
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .items)
    }

    /// Create a new topic
    ///
    /// Path: POST /v1/content/topics
    pub async fn create_topic(&self, opts: CreateTopicOptions) -> Result<String> {
        #[derive(Debug, Deserialize)]
        struct TopicId {
            id: String,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            item: TopicId,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/content/topics")
            .body(Json(opts))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .item
            .id)
    }

    /// Get discussion topics list
    pub async fn topics(&self, symbol: impl Into<String>) -> Result<Vec<TopicItem>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<TopicItem>,
        }

        let symbol = symbol.into();
        Ok(self
            .0
            .http_cli
            .request(Method::GET, format!("/v1/content/{symbol}/topics"))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .items)
    }

    /// Get news list
    pub async fn news(&self, symbol: impl Into<String>) -> Result<Vec<NewsItem>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<NewsItem>,
        }

        let symbol = symbol.into();
        Ok(self
            .0
            .http_cli
            .request(Method::GET, format!("/v1/content/{symbol}/news"))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .items)
    }
}
