//! Sharelist context – manage share-lists (watchlists shared publicly).

use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Serialize;
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    sharelist::types::{CreateSharelistOptions, SharelistItemsOptions},
};

struct InnerSharelistContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerSharelistContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("sharelist context dropped");
        });
    }
}

/// Sharelist context for managing public share-lists.
#[derive(Clone)]
pub struct SharelistContext(Arc<InnerSharelistContext>);

impl SharelistContext {
    /// Create a `SharelistContext`.
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("sharelist");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("creating sharelist context");
        });
        let ctx = Self(Arc::new(InnerSharelistContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("sharelist context created");
        });
        ctx
    }

    /// Returns the log subscriber.
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    async fn get<Q: Serialize + Send + Sync>(
        &self,
        path: impl Into<String>,
        query: Q,
    ) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path)
            .query_params(query)
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    async fn post<B: Serialize + Send + Sync>(
        &self,
        path: impl Into<String>,
        body: B,
    ) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, path)
            .body(Json(body))
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    async fn delete<Q: Serialize + Send + Sync>(
        &self,
        path: impl Into<String>,
        query: Q,
    ) -> Result<()> {
        self.0
            .http_cli
            .request(Method::DELETE, path)
            .query_params(query)
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(())
    }

    /// List share-lists.
    ///
    /// Path: GET /v1/sharelists
    pub async fn list_sharelists(&self, count: Option<u32>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            #[serde(skip_serializing_if = "Option::is_none")]
            count: Option<u32>,
        }
        self.get("/v1/sharelists", Request { count }).await
    }

    /// Get detail for a specific share-list.
    ///
    /// Path: GET /v1/sharelists/{id}
    pub async fn sharelist_detail(&self, id: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Empty {}
        self.get(format!("/v1/sharelists/{}", id.into()), Empty {})
            .await
    }

    /// Create a new share-list.
    ///
    /// Path: POST /v1/sharelists
    pub async fn create_sharelist(
        &self,
        opts: CreateSharelistOptions,
    ) -> Result<serde_json::Value> {
        self.post("/v1/sharelists", opts).await
    }

    /// Delete a share-list.
    ///
    /// Path: DELETE /v1/sharelists/{id}
    pub async fn delete_sharelist(&self, id: impl Into<String>) -> Result<()> {
        #[derive(Serialize)]
        struct Empty {}
        self.delete(format!("/v1/sharelists/{}", id.into()), Empty {})
            .await
    }

    /// Add items to a share-list.
    ///
    /// Path: POST /v1/sharelists/{id}/items
    pub async fn add_sharelist_items(
        &self,
        id: impl Into<String>,
        opts: SharelistItemsOptions,
    ) -> Result<serde_json::Value> {
        self.post(format!("/v1/sharelists/{}/items", id.into()), opts)
            .await
    }

    /// Remove items from a share-list.
    ///
    /// Path: DELETE /v1/sharelists/{id}/items
    pub async fn remove_sharelist_items(
        &self,
        id: impl Into<String>,
        opts: SharelistItemsOptions,
    ) -> Result<()> {
        self.delete(format!("/v1/sharelists/{}/items", id.into()), opts)
            .await
    }

    /// Sort items in a share-list.
    ///
    /// Path: POST /v1/sharelists/{id}/items/sort
    pub async fn sort_sharelist_items(
        &self,
        id: impl Into<String>,
        opts: SharelistItemsOptions,
    ) -> Result<serde_json::Value> {
        self.post(
            format!("/v1/sharelists/{}/items/sort", id.into()),
            opts,
        )
        .await
    }

    /// Get popular share-lists.
    ///
    /// Path: GET /v1/sharelists/popular
    pub async fn popular_sharelists(&self, count: Option<u32>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            #[serde(skip_serializing_if = "Option::is_none")]
            count: Option<u32>,
        }
        self.get("/v1/sharelists/popular", Request { count }).await
    }
}
