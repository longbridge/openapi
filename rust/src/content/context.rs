use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Deserialize;

use super::types::{
    CreateReplyOptions, CreateTopicOptions, ListTopicRepliesOptions, MyTopicsOptions, NewsItem,
    OwnedTopic, TopicItem, TopicReply,
};
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
    pub async fn my_topics(&self, opts: MyTopicsOptions) -> Result<Vec<OwnedTopic>> {
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

    /// Create a new community topic.
    ///
    /// Path: POST /v1/content/topics
    ///
    /// Two content types are supported:
    /// - `post` (default): plain text only; Markdown is NOT rendered.
    /// - `article`: Markdown body (server converts to HTML); `title` is required.
    ///
    /// Permission: user must hold a funded Longbridge account (returns 403 otherwise).
    ///
    /// Symbols in body (e.g. "700.HK", "TSLA.US") are automatically linked. Use `tickers`
    /// to associate additional symbols not mentioned in the body.
    /// WARNING: do not abuse symbol linking for unrelated stocks — moderation may restrict
    /// publishing or suspend the account.
    ///
    /// Rate limit: max 3 topics/min and 10/24h per user (429 on excess).
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

    /// Get full details of a topic by its ID
    ///
    /// Path: GET /v1/content/topics/{id}
    pub async fn topic_detail(&self, id: impl Into<String>) -> Result<OwnedTopic> {
        #[derive(Debug, Deserialize)]
        struct Response {
            item: OwnedTopic,
        }

        let id = id.into();
        Ok(self
            .0
            .http_cli
            .request(Method::GET, format!("/v1/content/topics/{id}"))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .item)
    }

    /// List replies on a topic
    ///
    /// Path: GET /v1/content/topics/{topic_id}/comments
    pub async fn list_topic_replies(
        &self,
        topic_id: impl Into<String>,
        opts: ListTopicRepliesOptions,
    ) -> Result<Vec<TopicReply>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<TopicReply>,
        }

        let topic_id = topic_id.into();
        Ok(self
            .0
            .http_cli
            .request(Method::GET, format!("/v1/content/topics/{topic_id}/comments"))
            .query_params(opts)
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .items)
    }

    /// Post a reply to a community topic.
    ///
    /// Path: POST /v1/content/topics/{topic_id}/comments
    ///
    /// Body is plain text only — Markdown is not rendered.
    ///
    /// Permission: user must hold a funded Longbridge account (returns 403 otherwise).
    ///
    /// Symbols in body (e.g. "700.HK", "TSLA.US") are automatically linked.
    /// WARNING: do not abuse symbol linking for unrelated stocks — moderation may restrict
    /// publishing or suspend the account.
    ///
    /// Rate limit per user per topic: first 3 replies have no interval; subsequent replies
    /// must wait an incrementally longer interval (3 s → 5 s → 8 s → 13 s → 21 s → 34 s → 55 s cap).
    /// Returns 429 on excess.
    pub async fn create_topic_reply(
        &self,
        topic_id: impl Into<String>,
        opts: CreateReplyOptions,
    ) -> Result<TopicReply> {
        #[derive(Debug, Deserialize)]
        struct Response {
            item: TopicReply,
        }

        let topic_id = topic_id.into();
        Ok(self
            .0
            .http_cli
            .request(
                Method::POST,
                format!("/v1/content/topics/{topic_id}/comments"),
            )
            .body(Json(opts))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .item)
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
