use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, screener::types::*};

struct InnerScreenerContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerScreenerContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("screener context dropped");
        });
    }
}

/// Screener context — stock screener strategies, search, and indicators.
#[derive(Clone)]
pub struct ScreenerContext(Arc<InnerScreenerContext>);

impl ScreenerContext {
    /// Create a [`ScreenerContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("screener");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating screener context");
        });
        let ctx = Self(Arc::new(InnerScreenerContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("screener context created");
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

    // ── screener_recommend_strategies ─────────────────────────────

    /// Get recommended built-in screener strategies.
    ///
    /// Path: `GET /v1/quote/screener/strategies/recommend`
    pub async fn screener_recommend_strategies(
        &self,
    ) -> Result<ScreenerRecommendStrategiesResponse> {
        #[derive(Serialize)]
        struct Empty {}
        let raw: serde_json::Value = self
            .get("/v1/quote/screener/strategies/recommend", Empty {})
            .await?;
        Ok(ScreenerRecommendStrategiesResponse { data: raw })
    }

    // ── screener_user_strategies ──────────────────────────────────

    /// Get the current user's saved screener strategies.
    ///
    /// Path: `GET /v1/quote/screener/strategies/mine`
    pub async fn screener_user_strategies(&self) -> Result<ScreenerUserStrategiesResponse> {
        #[derive(Serialize)]
        struct Empty {}
        let raw: serde_json::Value = self
            .get("/v1/quote/screener/strategies/mine", Empty {})
            .await?;
        Ok(ScreenerUserStrategiesResponse { data: raw })
    }

    // ── screener_strategy ─────────────────────────────────────────

    /// Get detail for one screener strategy by ID.
    ///
    /// Path: `GET /v1/quote/screener/strategy?id=<id>`
    pub async fn screener_strategy(&self, id: i64) -> Result<ScreenerStrategyResponse> {
        #[derive(Serialize)]
        struct Query {
            id: i64,
        }
        let raw: serde_json::Value = self
            .get("/v1/quote/screener/strategy", Query { id })
            .await?;
        Ok(ScreenerStrategyResponse { data: raw })
    }

    // ── screener_search ───────────────────────────────────────────

    /// Search / screen securities using a strategy.
    ///
    /// Path: `POST /v1/quote/screener/search`
    ///
    /// When `strategy_id` is `Some`, it is included in the request body.
    /// When `None`, only `market`, `page`, and `size` are sent (custom
    /// filter support is out of scope for this SDK).
    pub async fn screener_search(
        &self,
        market: impl Into<String>,
        strategy_id: Option<i64>,
        page: u32,
        size: u32,
    ) -> Result<ScreenerSearchResponse> {
        #[derive(Debug, Serialize)]
        struct Body {
            market: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            strategy_id: Option<i64>,
            page: u32,
            size: u32,
        }
        let raw: serde_json::Value = self
            .post(
                "/v1/quote/screener/search",
                Body {
                    market: market.into(),
                    strategy_id,
                    page,
                    size,
                },
            )
            .await?;
        Ok(ScreenerSearchResponse { data: raw })
    }

    // ── screener_indicators ───────────────────────────────────────

    /// Get all available screener indicator definitions.
    ///
    /// Path: `GET /v1/quote/screener/indicators`
    pub async fn screener_indicators(&self) -> Result<ScreenerIndicatorsResponse> {
        #[derive(Serialize)]
        struct Empty {}
        let raw: serde_json::Value = self.get("/v1/quote/screener/indicators", Empty {}).await?;
        Ok(ScreenerIndicatorsResponse { data: raw })
    }
}
