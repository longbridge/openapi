use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, alert::types::*, utils::counter::symbol_to_counter_id};

struct InnerAlertContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerAlertContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("alert context dropped");
        });
    }
}

/// Price alert management context.
#[derive(Clone)]
pub struct AlertContext(Arc<InnerAlertContext>);

impl AlertContext {
    /// Create an [`AlertContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("alert");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating alert context");
        });
        let ctx = Self(Arc::new(InnerAlertContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("alert context created");
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

    async fn http_delete<R, B>(&self, path: &'static str, body: B) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        B: std::fmt::Debug + Serialize + Send + Sync + 'static,
    {
        Ok(self
            .0
            .http_cli
            .request(Method::DELETE, path)
            .body(Json(body))
            .response::<Json<R>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// List all price alerts.
    ///
    /// Path: `GET /v1/notify/reminders`
    pub async fn list(&self) -> Result<AlertList> {
        #[derive(Serialize)]
        struct Empty {}
        self.get("/v1/notify/reminders", Empty {}).await
    }

    /// Add a price alert.
    ///
    /// Path: `POST /v1/notify/reminders`
    pub async fn add(
        &self,
        symbol: impl Into<String>,
        condition: AlertCondition,
        trigger_value: impl Into<String>,
        frequency: AlertFrequency,
    ) -> Result<serde_json::Value> {
        let cid = symbol_to_counter_id(&symbol.into());
        let (key, val) = match condition {
            AlertCondition::PriceRise | AlertCondition::PriceFall => {
                ("price", trigger_value.into())
            }
            AlertCondition::PercentRise | AlertCondition::PercentFall => {
                ("chg", trigger_value.into())
            }
        };
        let indicator_id = condition as i32;
        let freq = frequency as i32;
        self.post(
            "/v1/notify/reminders",
            serde_json::json!({
                "counter_id": cid,
                "indicator_id": indicator_id.to_string(),
                "value_map": { key: val },
                "frequency": freq,
                "enabled": true,
                "scope": 0,
                "state": [1]
            }),
        )
        .await
    }

    /// Update a price alert.
    ///
    /// Requires the [`AlertItem`] from [`list`](Self::list). Set
    /// `item.enabled` to `true` to re-enable or `false` to disable before
    /// calling this method. All required fields are read from `item` directly
    /// — no extra round-trip needed.
    ///
    /// Path: `POST /v1/notify/reminders`
    pub async fn update(&self, item: &AlertItem) -> Result<serde_json::Value> {
        self.post(
            "/v1/notify/reminders",
            serde_json::json!({
                "id": item.id,
                "indicator_id": item.indicator_id,
                "frequency": item.frequency,
                "scope": item.scope,
                "state": item.state,
                "value_map": item.value_map,
                "enabled": item.enabled,
            }),
        )
        .await
    }

    /// Delete price alerts.
    ///
    /// Path: `DELETE /v1/notify/reminders`
    pub async fn delete(&self, alert_ids: Vec<String>) -> Result<serde_json::Value> {
        self.http_delete(
            "/v1/notify/reminders",
            serde_json::json!({ "ids": alert_ids }),
        )
        .await
    }
}
