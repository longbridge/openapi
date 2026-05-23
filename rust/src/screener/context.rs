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

    async fn get<R, Q>(&self, path: &str, query: Q) -> Result<R>
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

    async fn post<R, B>(&self, path: &str, body: B) -> Result<R>
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

    /// Get preset built-in screener strategies.
    ///
    /// Path: `GET /v1/quote/ai/screener/strategies/recommend`
    pub async fn screener_recommend_strategies(
        &self,
        market: impl Into<String>,
    ) -> Result<ScreenerRecommendStrategiesResponse> {
        #[derive(Serialize)]
        struct Query {
            market: String,
        }
        let raw: serde_json::Value = self
            .get(
                "/v1/quote/ai/screener/strategies/recommend",
                Query {
                    market: market.into(),
                },
            )
            .await?;
        Ok(ScreenerRecommendStrategiesResponse { data: raw })
    }

    // ── screener_user_strategies ──────────────────────────────────

    /// Get the current user's saved screener strategies.
    ///
    /// Path: `GET /v1/quote/ai/screener/strategies/mine`
    pub async fn screener_user_strategies(
        &self,
        market: impl Into<String>,
    ) -> Result<ScreenerUserStrategiesResponse> {
        #[derive(Serialize)]
        struct Query {
            market: String,
        }
        let raw: serde_json::Value = self
            .get(
                "/v1/quote/ai/screener/strategies/mine",
                Query {
                    market: market.into(),
                },
            )
            .await?;
        Ok(ScreenerUserStrategiesResponse { data: raw })
    }

    // ── screener_strategy ─────────────────────────────────────────

    /// Get detail for one screener strategy by ID.
    ///
    /// Path: `GET /v1/quote/ai/screener/strategy/{id}`
    ///
    /// The `filter_` prefix is stripped from every `filters[].key` before
    /// returning so callers see clean keys like `pettm` instead of
    /// `filter_pettm`.
    pub async fn screener_strategy(&self, id: i64) -> Result<ScreenerStrategyResponse> {
        let path = format!("/v1/quote/ai/screener/strategy/{id}");
        #[derive(Serialize)]
        struct Empty {}
        let mut raw: serde_json::Value = self.get(&path, Empty {}).await?;
        // Strip filter_ prefix from filter.filters[].key
        if let Some(filters) = raw["filter"]["filters"].as_array_mut() {
            for f in filters.iter_mut() {
                if let Some(k) = f["key"].as_str() {
                    let stripped = k.strip_prefix("filter_").unwrap_or(k).to_string();
                    f["key"] = serde_json::Value::String(stripped);
                }
            }
        }
        Ok(ScreenerStrategyResponse { data: raw })
    }

    // ── screener_search ───────────────────────────────────────────

    /// Default return columns always included in a screener search request.
    const DEFAULT_RETURNS: &'static [&'static str] = &[
        "filter_prevclose",
        "filter_prevchg",
        "filter_marketcap",
        "filter_salesgrowthyoy",
        "filter_pettm",
        "filter_pbmrq",
        "filter_industry",
    ];

    /// Search / screen securities using a strategy or custom conditions.
    ///
    /// Path: `POST /v1/quote/ai/screener/search`
    ///
    /// ## Mode A — strategy ID given
    ///
    /// When `strategy_id` is `Some`, the strategy is fetched from
    /// `GET /v1/quote/ai/screener/strategy/{id}` and its `filter.filters[]`
    /// are forwarded to the search endpoint together with
    /// [`DEFAULT_RETURNS`].  The `market` is taken from the strategy
    /// response (falls back to `"US"` if absent or `"-"`).
    ///
    /// ## Mode B — custom conditions
    ///
    /// When `strategy_id` is `None` and `conditions` is non-empty each
    /// element is either a `"KEY:MIN:MAX"` string **or** a JSON object with
    /// `key`, `min`, `max`, and optional `tech_values` fields.  The
    /// supplied `market` is used directly.  `DEFAULT_RETURNS` plus every
    /// condition key are added to the `returns` list.
    ///
    /// The `filter_` prefix is stripped from every `items[].indicators[].key`
    /// in the response before it is returned to the caller.
    ///
    /// `page` is 0-indexed.
    pub async fn screener_search(
        &self,
        market: impl Into<String>,
        strategy_id: Option<i64>,
        conditions: Vec<String>,
        show: Vec<String>,
        page: u32,
        size: u32,
    ) -> Result<ScreenerSearchResponse> {
        let market: String = market.into();

        // ── build filters and effective market ──────────────────────────────
        let (effective_market, filters) = if let Some(sid) = strategy_id {
            // Mode A: fetch strategy from AI endpoint
            let path = format!("/v1/quote/ai/screener/strategy/{sid}");
            #[derive(Serialize)]
            struct Empty {}
            let strategy: serde_json::Value = self.get(&path, Empty {}).await?;

            let mkt_val = strategy["market"].as_str().unwrap_or("US").to_uppercase();
            let mkt = if mkt_val.is_empty() || mkt_val == "-" {
                "US".to_string()
            } else {
                mkt_val
            };

            let mut filters: Vec<serde_json::Value> = Vec::new();
            if let Some(f) = strategy["filter"]["filters"].as_array() {
                for ind in f {
                    let key = ind["key"].as_str().unwrap_or("").to_string();
                    if key.is_empty() {
                        continue;
                    }
                    let min = ind["min"].as_str().unwrap_or("").to_string();
                    let max = ind["max"].as_str().unwrap_or("").to_string();
                    let tech_values = if ind["tech_values"].is_object() {
                        ind["tech_values"].clone()
                    } else {
                        serde_json::json!({})
                    };
                    filters.push(serde_json::json!({
                        "key": key,
                        "min": min,
                        "max": max,
                        "tech_values": tech_values,
                    }));
                }
            }
            (mkt, filters)
        } else {
            // Mode B: custom conditions ("KEY:MIN:MAX" strings)
            let filters: Vec<serde_json::Value> = conditions
                .iter()
                .filter_map(|cond| {
                    let parts: Vec<&str> = cond.splitn(3, ':').collect();
                    if parts.is_empty() || parts[0].is_empty() {
                        return None;
                    }
                    let key = parts[0].to_string();
                    let min = parts.get(1).copied().unwrap_or("").to_string();
                    let max = parts.get(2).copied().unwrap_or("").to_string();
                    Some(serde_json::json!({
                        "key": key,
                        "min": min,
                        "max": max,
                        "tech_values": {},
                    }))
                })
                .collect();
            (market, filters)
        };

        // ── build returns list ───────────────────────────────────────────────
        let mut returns: Vec<String> = Self::DEFAULT_RETURNS
            .iter()
            .map(|s| s.to_string())
            .collect();
        // add keys from filters (with filter_ prefix for the API)
        for f in &filters {
            if let Some(k) = f["key"].as_str() {
                let api_key = if k.starts_with("filter_") {
                    k.to_string()
                } else {
                    format!("filter_{k}")
                };
                if !returns.contains(&api_key) {
                    returns.push(api_key);
                }
            }
        }
        // add extra columns requested by the caller
        for s in &show {
            let api_key = if s.starts_with("filter_") {
                s.clone()
            } else {
                format!("filter_{s}")
            };
            if !returns.contains(&api_key) {
                returns.push(api_key);
            }
        }

        // ── POST request ────────────────────────────────────────────────────
        let body = serde_json::json!({
            "market": effective_market,
            "filters": filters,
            "returns": returns,
            "page": page,
            "size": size,
        });

        let raw: serde_json::Value = self.post("/v1/quote/ai/screener/search", body).await?;
        Ok(ScreenerSearchResponse {
            data: strip_filter_prefix_from_search_results(raw),
        })
    }

    // ── screener_indicators ───────────────────────────────────────

    /// Get all available screener indicator definitions.
    ///
    /// Path: `GET /v1/quote/ai/screener/indicators`
    ///
    /// Post-processing applied before returning:
    /// - `filter_` prefix is stripped from every `groups[].indicators[].key`
    /// - `tech_values` is built from `tech_indicators`: `{tech_key: [{value,
    ///   label}]}`
    pub async fn screener_indicators(&self) -> Result<ScreenerIndicatorsResponse> {
        #[derive(Serialize)]
        struct Empty {}
        let mut raw: serde_json::Value = self
            .get("/v1/quote/ai/screener/indicators", Empty {})
            .await?;
        if let Some(groups) = raw["groups"].as_array_mut() {
            for group in groups.iter_mut() {
                if let Some(indicators) = group["indicators"].as_array_mut() {
                    for ind in indicators.iter_mut() {
                        // Strip filter_ prefix from key
                        if let Some(k) = ind["key"].as_str() {
                            let stripped = k.strip_prefix("filter_").unwrap_or(k).to_string();
                            ind["key"] = serde_json::Value::String(stripped);
                        }
                        // Build tech_values from tech_indicators
                        if let Some(tech_inds) = ind["tech_indicators"].as_array().cloned() {
                            let tv: serde_json::Map<String, serde_json::Value> = tech_inds
                                .iter()
                                .filter_map(|ti| {
                                    let key = ti["tech_key"].as_str()?.to_string();
                                    let opts: Vec<serde_json::Value> = ti["tech_items"]
                                        .as_array()
                                        .unwrap_or(&vec![])
                                        .iter()
                                        .map(|item| {
                                            serde_json::json!({
                                                "value": item["item_value"].as_str().unwrap_or(""),
                                                "label": item["item_name"].as_str().unwrap_or(""),
                                            })
                                        })
                                        .collect();
                                    Some((key, serde_json::Value::Array(opts)))
                                })
                                .collect();
                            if !tv.is_empty() {
                                ind["tech_values"] = serde_json::Value::Object(tv);
                            }
                        }
                    }
                }
            }
        }
        Ok(ScreenerIndicatorsResponse { data: raw })
    }
}

/// Strip `filter_` prefix from every `items[].indicators[].key` in a raw
/// screener search result.
fn strip_filter_prefix_from_search_results(mut raw: serde_json::Value) -> serde_json::Value {
    if let Some(items) = raw["items"].as_array_mut() {
        for item in items.iter_mut() {
            if let Some(indicators) = item["indicators"].as_array_mut() {
                for ind in indicators.iter_mut() {
                    if let Some(k) = ind["key"].as_str() {
                        let stripped = k.strip_prefix("filter_").unwrap_or(k).to_string();
                        ind["key"] = serde_json::Value::String(stripped);
                    }
                }
            }
        }
    }
    raw
}
