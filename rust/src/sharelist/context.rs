use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, sharelist::types::*, utils::counter::symbol_to_counter_id};

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

/// Community sharelist management context.
#[derive(Clone)]
pub struct SharelistContext(Arc<InnerSharelistContext>);

impl SharelistContext {
    /// Create a [`SharelistContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("sharelist");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating sharelist context");
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

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    async fn get<R, Q>(&self, path: &'static str, query: Q) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        Q: Serialize + Send + Sync,
    {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path)
            .query_params(query)
            .response::<Json<R>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    async fn post<R, B>(&self, path: &'static str, body: B) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        B: std::fmt::Debug + Serialize + Send + Sync + 'static,
    {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, path)
            .body(Json(body))
            .response::<Json<R>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    async fn http_delete<R, B>(&self, path: String, body: B) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        B: std::fmt::Debug + Serialize + Send + Sync + 'static,
    {
        Ok(self
            .0
            .http_cli
            .request(Method::DELETE, path.leak())
            .body(Json(body))
            .response::<Json<R>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// List user's own and subscribed sharelists.
    ///
    /// Path: `GET /v1/sharelists`
    pub async fn list(&self, count: u32) -> Result<SharelistList> {
        #[derive(Serialize)]
        struct Query {
            size: u32,
            #[serde(rename = "self")]
            is_self: &'static str,
            subscription: &'static str,
        }
        self.get(
            "/v1/sharelists",
            Query {
                size: count,
                is_self: "true",
                subscription: "true",
            },
        )
        .await
    }

    /// Get sharelist detail.
    ///
    /// Path: `GET /v1/sharelists/{id}`
    pub async fn detail(&self, id: i64) -> Result<SharelistDetail> {
        #[derive(Serialize)]
        struct Query {
            constituent: &'static str,
            quote: &'static str,
            subscription: &'static str,
        }
        let path = format!("/v1/sharelists/{id}");
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path.leak())
            .query_params(Query {
                constituent: "true",
                quote: "true",
                subscription: "true",
            })
            .response::<Json<SharelistDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get popular sharelists.
    ///
    /// Path: `GET /v1/sharelists/popular`
    pub async fn popular(&self, count: u32) -> Result<SharelistList> {
        #[derive(Serialize)]
        struct Query {
            size: u32,
        }
        self.get("/v1/sharelists/popular", Query { size: count })
            .await
    }

    /// Create a new sharelist.
    ///
    /// Path: `POST /v1/sharelists`
    pub async fn create(&self, name: impl Into<String>, description: Option<String>) -> Result<()> {
        let name_str = name.into();
        let desc = description.unwrap_or_else(|| name_str.clone());
        self.post::<serde_json::Value, _>("/v1/sharelists", serde_json::json!({ "name": name_str, "description": desc, "cover": "https://pub.pbkrs.com/files/202107/kaJSk6BsvPt6NJ3Q/sharelist_v1.png" })).await?;
        Ok(())
    }

    /// Delete a sharelist.
    ///
    /// Path: `DELETE /v1/sharelists/{id}`
    pub async fn delete(&self, id: i64) -> Result<serde_json::Value> {
        self.http_delete(format!("/v1/sharelists/{id}"), serde_json::json!({}))
            .await
    }

    /// Add securities to a sharelist.
    ///
    /// Path: `POST /v1/sharelists/{id}/items`
    pub async fn add_securities(&self, id: i64, symbols: Vec<String>) -> Result<serde_json::Value> {
        let counter_ids = symbols
            .iter()
            .map(|s| symbol_to_counter_id(s))
            .collect::<Vec<_>>()
            .join(",");
        let path = format!("/v1/sharelists/{id}/items");
        self.post(
            path.leak(),
            serde_json::json!({ "counter_ids": counter_ids }),
        )
        .await
    }

    /// Remove securities from a sharelist.
    ///
    /// Path: `DELETE /v1/sharelists/{id}/items`
    pub async fn remove_securities(
        &self,
        id: i64,
        symbols: Vec<String>,
    ) -> Result<serde_json::Value> {
        let counter_ids = symbols
            .iter()
            .map(|s| symbol_to_counter_id(s))
            .collect::<Vec<_>>()
            .join(",");
        self.http_delete(
            format!("/v1/sharelists/{id}/items"),
            serde_json::json!({ "counter_ids": counter_ids }),
        )
        .await
    }

    /// Reorder securities in a sharelist.
    ///
    /// Path: `POST /v1/sharelists/{id}/items/sort`
    pub async fn sort_securities(
        &self,
        id: i64,
        symbols: Vec<String>,
    ) -> Result<serde_json::Value> {
        let counter_ids = symbols
            .iter()
            .map(|s| symbol_to_counter_id(s))
            .collect::<Vec<_>>()
            .join(",");
        let path = format!("/v1/sharelists/{id}/items/sort");
        self.post(
            path.leak(),
            serde_json::json!({ "counter_ids": counter_ids }),
        )
        .await
    }
}
