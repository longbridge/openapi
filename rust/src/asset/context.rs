use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    asset::{
        GetStatementOptions, GetStatementResponse,
        GetStatementListOptions, GetStatementListResponse, core,
    },
};

struct InnerStatementContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerStatementContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("statement context dropped");
        });
    }
}

/// Statement context
#[derive(Clone)]
pub struct StatementContext(Arc<InnerStatementContext>);

impl StatementContext {
    /// Create a `StatementContext`
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("statement");

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating statement context");
        });

        let ctx = Self(Arc::new(InnerStatementContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));

        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("statement context created");
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

    /// Get statement data list
    ///
    /// Path: GET /v1/statement/list
    pub async fn statement_data_list(
        &self,
        options: GetStatementListOptions,
    ) -> Result<GetStatementListResponse> {
        self.get(core::GET_STATEMENT_DATA_LIST_PATH, options).await
    }

    /// Get statement data download url
    ///
    /// Path: GET /v1/statement/download
    pub async fn statement_data_download_url(
        &self,
        options: GetStatementOptions,
    ) -> Result<GetStatementResponse> {
        self.get(core::GET_STATEMENT_DATA_DOWNLOAD_URL_PATH, options)
            .await
    }
}
