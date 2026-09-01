use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Deserialize, Serialize};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    grid::{
        GetGridOrderDetailOptions, GetGridOrdersByIdsOptions, GetGridOrdersOptions,
        GetGridTriggerHistoryOptions, GridOrder, GridOrderDetail, GridSymbolInfo,
        ReplaceGridOrderOptions, SubmitGridOrderOptions, TriggerOrder,
    },
};

#[derive(Debug, Deserialize)]
struct EmptyResponse {}

/// Response for submit grid trading order request
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitGridOrderResponse {
    /// Grid master order id
    pub order_id: String,
}

/// Response for get grid trading orders (list) request
#[derive(Debug, Deserialize)]
pub struct GridOrdersResponse {
    /// Grid orders
    #[serde(default)]
    pub grid_order: Vec<GridOrder>,
    /// Whether there are more pages
    #[serde(default)]
    pub has_more: bool,
}

/// Response for get grid trading trigger history request
#[derive(Debug, Deserialize)]
pub struct GridTriggerHistoryResponse {
    /// Trigger history entries
    #[serde(default)]
    pub trigger_orders: Vec<TriggerOrder>,
    /// Whether there are more pages
    #[serde(default)]
    pub has_more: bool,
}

struct InnerGridContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerGridContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("grid context dropped");
        });
    }
}

/// Grid trading management context.
#[derive(Clone)]
pub struct GridContext(Arc<InnerGridContext>);

impl GridContext {
    /// Create a [`GridContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("grid");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating grid context");
        });
        let ctx = Self(Arc::new(InnerGridContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("grid context created");
        });
        ctx
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Submit a grid trading order
    pub async fn submit(&self, options: SubmitGridOrderOptions) -> Result<SubmitGridOrderResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/gridtrading/submit")
            .body(Json(options))
            .response::<Json<SubmitGridOrderResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Replace (modify) a grid trading order
    pub async fn replace(&self, options: ReplaceGridOrderOptions) -> Result<()> {
        self.0
            .http_cli
            .request(Method::POST, "/v1/gridtrading/replace")
            .body(Json(options))
            .response::<Json<EmptyResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(())
    }

    /// Get grid trading orders (paged list)
    pub async fn list(
        &self,
        options: impl Into<Option<GetGridOrdersOptions>>,
    ) -> Result<GridOrdersResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/gridtrading/list")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<GridOrdersResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Query grid trading orders by IDs
    pub async fn list_by_ids(&self, options: GetGridOrdersByIdsOptions) -> Result<Vec<GridOrder>> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(default)]
            grid_order: Vec<GridOrder>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/gridtrading/list")
            .body(Json(options))
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .grid_order)
    }

    /// Get grid trading order detail (and paged history)
    pub async fn detail(&self, options: GetGridOrderDetailOptions) -> Result<GridOrderDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/gridtrading/detail")
            .query_params(options)
            .response::<Json<GridOrderDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get grid trading trigger history
    pub async fn trigger_history(
        &self,
        options: GetGridTriggerHistoryOptions,
    ) -> Result<GridTriggerHistoryResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/gridtrading/trigger_history_list")
            .query_params(options)
            .response::<Json<GridTriggerHistoryResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Cancel a grid trading order
    pub async fn cancel(&self, order_id: impl Into<String>) -> Result<()> {
        self.grid_action("/v1/gridtrading/cancel", order_id).await
    }

    /// Suspend a grid trading order
    pub async fn suspend(&self, order_id: impl Into<String>) -> Result<()> {
        self.grid_action("/v1/gridtrading/suspend", order_id).await
    }

    /// Restart a grid trading order
    pub async fn restart(&self, order_id: impl Into<String>) -> Result<()> {
        self.grid_action("/v1/gridtrading/restart", order_id).await
    }

    /// Shared body for the cancel / suspend / restart grid actions.
    async fn grid_action(&self, path: &'static str, order_id: impl Into<String>) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Body {
            order_id: String,
        }

        self.0
            .http_cli
            .request(Method::POST, path)
            .body(Json(Body {
                order_id: order_id.into(),
            }))
            .response::<Json<EmptyResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(())
    }

    /// Get the security (symbol) info used to build a grid order (lot size,
    /// authorization flag, settlement currency, etc.).
    pub async fn symbol_info(&self, symbol: impl Into<String>) -> Result<GridSymbolInfo> {
        #[derive(Debug, Serialize)]
        struct Query {
            symbol: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/orders/info")
            .query_params(Query {
                symbol: symbol.into(),
            })
            .response::<Json<GridSymbolInfo>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
