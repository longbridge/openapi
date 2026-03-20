use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Deserialize;

use super::types::{NewsItem, TopicItem};
use crate::{Config, Result};

struct InnerContentContext {
    http_cli: HttpClient,
}

/// Content context
#[derive(Clone)]
pub struct ContentContext(Arc<InnerContentContext>);

impl ContentContext {
    /// Create a `ContentContext`
    pub fn try_new(config: Arc<Config>) -> Result<Self> {
        Ok(Self(Arc::new(InnerContentContext {
            http_cli: config.create_http_client(),
        })))
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
