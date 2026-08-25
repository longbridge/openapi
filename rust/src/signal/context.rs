use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Deserialize;
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, signal::types::*};

struct InnerSignalContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerSignalContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("signal context dropped");
        });
    }
}

/// Signal context — strategy signals and the catalyst facts behind them.
#[derive(Clone)]
pub struct SignalContext(Arc<InnerSignalContext>);

impl SignalContext {
    /// Create a [`SignalContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("signal");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating signal context");
        });
        let ctx = Self(Arc::new(InnerSignalContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("signal context created");
        });
        ctx
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Query signals, filtered by symbol, strategy, catalyst and time range.
    ///
    /// Path: `GET /v1/signals`
    pub async fn signals(&self, opts: SignalsOptions) -> Result<SignalsResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/signals")
            .query_params(opts)
            .response::<Json<SignalsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get one signal by ID, including the full analysis in
    /// [`Signal::json_data`].
    ///
    /// Path: `GET /v1/signals/{signal_id}`
    pub async fn signal(&self, signal_id: impl Into<String>) -> Result<Signal> {
        #[derive(Debug, Deserialize)]
        struct Response {
            signal: Signal,
        }

        let signal_id = signal_id.into();
        Ok(self
            .0
            .http_cli
            .request(Method::GET, format!("/v1/signals/{signal_id}"))
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .signal)
    }

    /// List the fact (catalyst) events for one security — anomaly detections,
    /// factor readings, data sources and natural-language summaries.
    ///
    /// Facts are what strategies react to: a signal names the fact that
    /// triggered it in [`Signal::key_fact_id`].
    ///
    /// Each fact is returned verbatim as a JSON object; the payload is
    /// fact-type specific (news, fundamental, technical) and carries different
    /// fields per type.
    ///
    /// Path: `GET /v1/facts/security_facts`
    pub async fn security_facts(
        &self,
        opts: SecurityFactsOptions,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(default)]
            facts: Vec<serde_json::Value>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/facts/security_facts")
            .query_params(opts)
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .facts)
    }
}
