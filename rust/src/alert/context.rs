//! Alert context – CRUD for price reminders.

use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Serialize;
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, alert::types::AddAlertOptions};

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

/// Alert context for managing price reminders.
#[derive(Clone)]
pub struct AlertContext(Arc<InnerAlertContext>);

impl AlertContext {
    /// Create an `AlertContext`.
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("alert");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("creating alert context");
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

    /// Returns the log subscriber.
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// List all price alerts.
    ///
    /// Path: GET /v1/notify/reminders
    pub async fn list_alerts(&self) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Empty {}
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/notify/reminders")
            .query_params(Empty {})
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Add a new price alert.
    ///
    /// Path: POST /v1/notify/reminders
    pub async fn add_alert(&self, opts: AddAlertOptions) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/notify/reminders")
            .body(Json(opts))
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Delete price alerts by IDs.
    ///
    /// Path: DELETE /v1/notify/reminders
    pub async fn delete_alerts(&self, ids: Vec<String>) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            ids: String,
        }
        self.0
            .http_cli
            .request(Method::DELETE, "/v1/notify/reminders")
            .query_params(Request {
                ids: ids.join(","),
            })
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(())
    }

    /// Enable a price alert.
    ///
    /// Path: PUT /v1/notify/reminders
    pub async fn enable_alert(&self, id: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            id: String,
            enabled: bool,
        }
        Ok(self
            .0
            .http_cli
            .request(Method::PUT, "/v1/notify/reminders")
            .body(Json(Request {
                id: id.into(),
                enabled: true,
            }))
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Disable a price alert.
    ///
    /// Path: PUT /v1/notify/reminders
    pub async fn disable_alert(&self, id: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            id: String,
            enabled: bool,
        }
        Ok(self
            .0
            .http_cli
            .request(Method::PUT, "/v1/notify/reminders")
            .body(Json(Request {
                id: id.into(),
                enabled: false,
            }))
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
