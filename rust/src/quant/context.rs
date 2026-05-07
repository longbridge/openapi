//! Quant context – run quantitative scripts.

use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, quant::types::RunQuantScriptOptions};

struct InnerQuantContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerQuantContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("quant context dropped");
        });
    }
}

/// Quant context for executing quantitative scripts.
#[derive(Clone)]
pub struct QuantContext(Arc<InnerQuantContext>);

impl QuantContext {
    /// Create a `QuantContext`.
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("quant");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("creating quant context");
        });
        let ctx = Self(Arc::new(InnerQuantContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("quant context created");
        });
        ctx
    }

    /// Returns the log subscriber.
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Run a quantitative script.
    ///
    /// Path: POST /v1/quant/run_script
    pub async fn run_quant_script(
        &self,
        opts: RunQuantScriptOptions,
    ) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/quant/run_script")
            .body(Json(opts))
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
