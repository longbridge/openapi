//! DCA context – Dollar-Cost Averaging plan management.

use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Serialize;
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    dca::types::{
        CheckDcaSupportOptions, CreateDcaPlanOptions, DcaHistoryOptions, UpdateDcaPlanOptions,
    },
};

struct InnerDcaContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerDcaContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("dca context dropped");
        });
    }
}

/// DCA context for managing Dollar-Cost Averaging plans.
#[derive(Clone)]
pub struct DcaContext(Arc<InnerDcaContext>);

impl DcaContext {
    /// Create a `DcaContext`.
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("dca");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("creating dca context");
        });
        let ctx = Self(Arc::new(InnerDcaContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("dca context created");
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
        path: &'static str,
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
        path: &'static str,
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

    /// List DCA plans.
    ///
    /// Path: GET /v1/dailycoins/query
    pub async fn list_dca_plans(&self, status: Option<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
        }
        self.get("/v1/dailycoins/query", Request { status }).await
    }

    /// Create a DCA plan.
    ///
    /// Path: POST /v1/dailycoins/create
    pub async fn create_dca_plan(&self, opts: CreateDcaPlanOptions) -> Result<serde_json::Value> {
        self.post("/v1/dailycoins/create", opts).await
    }

    /// Update a DCA plan.
    ///
    /// Path: POST /v1/dailycoins/update
    pub async fn update_dca_plan(&self, opts: UpdateDcaPlanOptions) -> Result<serde_json::Value> {
        self.post("/v1/dailycoins/update", opts).await
    }

    /// Pause a DCA plan.
    ///
    /// Path: POST /v1/dailycoins/toggle
    pub async fn pause_dca_plan(&self, plan_id: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            plan_id: String,
            action: &'static str,
        }
        self.post(
            "/v1/dailycoins/toggle",
            Request {
                plan_id: plan_id.into(),
                action: "pause",
            },
        )
        .await
    }

    /// Resume a DCA plan.
    ///
    /// Path: POST /v1/dailycoins/toggle
    pub async fn resume_dca_plan(&self, plan_id: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            plan_id: String,
            action: &'static str,
        }
        self.post(
            "/v1/dailycoins/toggle",
            Request {
                plan_id: plan_id.into(),
                action: "resume",
            },
        )
        .await
    }

    /// Stop a DCA plan.
    ///
    /// Path: POST /v1/dailycoins/toggle
    pub async fn stop_dca_plan(&self, plan_id: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            plan_id: String,
            action: &'static str,
        }
        self.post(
            "/v1/dailycoins/toggle",
            Request {
                plan_id: plan_id.into(),
                action: "stop",
            },
        )
        .await
    }

    /// Get DCA plan execution history.
    ///
    /// Path: GET /v1/dailycoins/query-records
    pub async fn dca_history(&self, opts: DcaHistoryOptions) -> Result<serde_json::Value> {
        self.get("/v1/dailycoins/query-records", opts).await
    }

    /// Get DCA statistics, optionally filtered by symbol.
    ///
    /// Path: GET /v1/dailycoins/statistic
    pub async fn dca_statistics(&self, symbol: Option<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            #[serde(skip_serializing_if = "Option::is_none")]
            symbol: Option<String>,
        }
        self.get("/v1/dailycoins/statistic", Request { symbol }).await
    }

    /// Check DCA support for a list of symbols.
    ///
    /// Path: POST /v1/dailycoins/batch-check-support
    pub async fn check_dca_support(
        &self,
        opts: CheckDcaSupportOptions,
    ) -> Result<serde_json::Value> {
        self.post("/v1/dailycoins/batch-check-support", opts).await
    }
}
