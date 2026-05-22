use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    market::types::*,
    utils::counter::{counter_id_to_symbol, index_symbol_to_counter_id, symbol_to_counter_id},
};

/// Convert a Unix-seconds value (integer or string) to RFC 3339.
fn unix_secs_to_rfc3339(ts: i64) -> String {
    time::OffsetDateTime::from_unix_timestamp(ts)
        .map(|dt| {
            use time::format_description::well_known::Rfc3339;
            dt.format(&Rfc3339).unwrap_or_default()
        })
        .unwrap_or_else(|_| ts.to_string())
}

/// Convert a Unix-seconds string to RFC 3339.
fn unix_secs_str_to_rfc3339(s: &str) -> String {
    s.parse::<i64>()
        .map(unix_secs_to_rfc3339)
        .unwrap_or_else(|_| s.to_string())
}

struct InnerMarketContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerMarketContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("market context dropped");
        });
    }
}

/// Market data context — broker holdings, A/H premium, trade statistics,
/// market anomalies, index constituents and more.
#[derive(Clone)]
pub struct MarketContext(Arc<InnerMarketContext>);

impl MarketContext {
    /// Create a [`MarketContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("market");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating market context");
        });
        let ctx = Self(Arc::new(InnerMarketContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("market context created");
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

    // ── market_status ─────────────────────────────────────────────

    /// Get current trading status for all markets.
    ///
    /// Path: `GET /v1/quote/market-status`
    pub async fn market_status(&self) -> Result<MarketStatusResponse> {
        #[derive(Serialize)]
        struct Empty {}
        self.get("/v1/quote/market-status", Empty {}).await
    }

    // ── broker_holding ────────────────────────────────────────────

    /// Get top broker holdings (buy/sell leaders) for a security.
    ///
    /// Path: `GET /v1/quote/broker-holding`
    pub async fn broker_holding(
        &self,
        symbol: impl Into<String>,
        period: BrokerHoldingPeriod,
    ) -> Result<BrokerHoldingTop> {
        let period_str = match period {
            BrokerHoldingPeriod::Rct1 => "rct_1",
            BrokerHoldingPeriod::Rct5 => "rct_5",
            BrokerHoldingPeriod::Rct20 => "rct_20",
            BrokerHoldingPeriod::Rct60 => "rct_60",
        };
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            #[serde(rename = "type")]
            period: &'static str,
        }
        self.get(
            "/v1/quote/broker-holding",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                period: period_str,
            },
        )
        .await
    }

    /// Get full broker holding details for a security.
    ///
    /// Path: `GET /v1/quote/broker-holding/detail`
    pub async fn broker_holding_detail(
        &self,
        symbol: impl Into<String>,
    ) -> Result<BrokerHoldingDetail> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/broker-holding/detail",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get daily holding history for a specific broker.
    ///
    /// Path: `GET /v1/quote/broker-holding/daily`
    pub async fn broker_holding_daily(
        &self,
        symbol: impl Into<String>,
        broker_id: impl Into<String>,
    ) -> Result<BrokerHoldingDailyHistory> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            parti_number: String,
        }
        self.get(
            "/v1/quote/broker-holding/daily",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                parti_number: broker_id.into(),
            },
        )
        .await
    }

    // ── ah_premium ────────────────────────────────────────────────

    /// Get A/H premium K-line data for a dual-listed security.
    ///
    /// Path: `GET /v1/quote/ahpremium/klines`
    pub async fn ah_premium(
        &self,
        symbol: impl Into<String>,
        period: AhPremiumPeriod,
        count: u32,
    ) -> Result<AhPremiumKlines> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            line_type: &'static str,
            line_num: u32,
        }
        self.get(
            "/v1/quote/ahpremium/klines",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                line_type: period.to_line_type(),
                line_num: count,
            },
        )
        .await
    }

    /// Get A/H premium intraday data for a dual-listed security.
    ///
    /// Path: `GET /v1/quote/ahpremium/timeshares`
    pub async fn ah_premium_intraday(
        &self,
        symbol: impl Into<String>,
    ) -> Result<AhPremiumIntraday> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            days: &'static str,
        }
        self.get(
            "/v1/quote/ahpremium/timeshares",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                days: "1",
            },
        )
        .await
    }

    // ── trade_stats ───────────────────────────────────────────────

    /// Get buy/sell/neutral trade statistics for a security.
    ///
    /// Path: `GET /v1/quote/trades-statistics`
    pub async fn trade_stats(&self, symbol: impl Into<String>) -> Result<TradeStatsResponse> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/trades-statistics",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── anomaly ───────────────────────────────────────────────────

    /// Get market anomaly alerts (unusual price/volume events).
    ///
    /// Path: `GET /v1/quote/changes`
    pub async fn anomaly(&self, market: impl Into<String>) -> Result<AnomalyResponse> {
        #[derive(Serialize)]
        struct Query {
            market: String,
            category: &'static str,
        }
        self.get(
            "/v1/quote/changes",
            Query {
                market: market.into().to_uppercase(),
                category: "0",
            },
        )
        .await
    }

    // ── constituent ───────────────────────────────────────────────

    /// Get constituent stocks for an index.
    ///
    /// `symbol` should be an index symbol such as `"HSI.HK"`.
    ///
    /// Path: `GET /v1/quote/index-constituents`
    pub async fn constituent(&self, symbol: impl Into<String>) -> Result<IndexConstituents> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/index-constituents",
            Query {
                counter_id: index_symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── top_movers ────────────────────────────────────────────────

    /// Get top movers (stocks with unusual price movements) across one or more
    /// markets.
    ///
    /// Path: `POST /v1/quote/market/stock-events`
    ///
    /// `sort` is the sort order code (0 = ascending, 1 = descending).
    /// `date` is an optional date filter in `"YYYY-MM-DD"` format.
    pub async fn top_movers(
        &self,
        markets: Vec<String>,
        sort: u32,
        date: Option<String>,
        limit: u32,
    ) -> Result<TopMoversResponse> {
        #[derive(Debug, Serialize)]
        struct Body {
            limit: u32,
            sort: u32,
            markets: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<String>,
        }
        let raw: serde_json::Value = self
            .post(
                "/v1/quote/market/stock-events",
                Body {
                    limit,
                    sort,
                    markets,
                    date,
                },
            )
            .await?;

        let events = raw["events"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|ev| {
                let ts = if let Some(n) = ev["timestamp"].as_i64() {
                    unix_secs_to_rfc3339(n)
                } else if let Some(s) = ev["timestamp"].as_str() {
                    unix_secs_str_to_rfc3339(s)
                } else {
                    String::new()
                };
                let stock_val = &ev["stock"];
                let stock = TopMoversStock {
                    symbol: counter_id_to_symbol(stock_val["counter_id"].as_str().unwrap_or("")),
                    code: stock_val["code"].as_str().unwrap_or("").to_string(),
                    name: stock_val["name"].as_str().unwrap_or("").to_string(),
                    full_name: stock_val["full_name"].as_str().unwrap_or("").to_string(),
                    change: stock_val["change"].as_str().unwrap_or("").to_string(),
                    last_done: stock_val["last_done"].as_str().unwrap_or("").to_string(),
                    market: stock_val["market"].as_str().unwrap_or("").to_string(),
                    labels: stock_val["labels"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|l| l.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default(),
                    logo: stock_val["logo"].as_str().unwrap_or("").to_string(),
                };
                TopMoversEvent {
                    timestamp: ts,
                    alert_reason: ev["alert_reason"].as_str().unwrap_or("").to_string(),
                    alert_type: ev["alert_type"].as_i64().unwrap_or(0),
                    stock,
                    post: ev["post"].clone(),
                }
            })
            .collect();
        let next_params = raw["next_params"].clone();
        Ok(TopMoversResponse {
            events,
            next_params,
        })
    }

    // ── rank_categories ───────────────────────────────────────────

    /// Get all available rank category keys and labels.
    ///
    /// Path: `GET /v1/quote/market/rank/categories`
    pub async fn rank_categories(&self) -> Result<RankCategoriesResponse> {
        #[derive(Serialize)]
        struct Empty {}
        let raw: serde_json::Value = self
            .get("/v1/quote/market/rank/categories", Empty {})
            .await?;
        Ok(RankCategoriesResponse { data: raw })
    }

    // ── rank_list ─────────────────────────────────────────────────

    /// Get a ranked list of securities for the given category key.
    ///
    /// Path: `GET /v1/quote/market/rank/list`
    pub async fn rank_list(
        &self,
        key: impl Into<String>,
        need_article: bool,
    ) -> Result<RankListResponse> {
        #[derive(Serialize)]
        struct Query {
            key: String,
            delay_bmp: &'static str,
            need_article: &'static str,
        }
        let raw: serde_json::Value = self
            .get(
                "/v1/quote/market/rank/list",
                Query {
                    key: key.into(),
                    delay_bmp: "false",
                    need_article: if need_article { "true" } else { "false" },
                },
            )
            .await?;
        let bmp = raw["bmp"].as_bool().unwrap_or(false);
        let lists = raw["lists"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|item| RankListItem {
                symbol: counter_id_to_symbol(item["counter_id"].as_str().unwrap_or("")),
                code: item["code"].as_str().unwrap_or("").to_string(),
                name: item["name"].as_str().unwrap_or("").to_string(),
                last_done: item["last_done"].as_str().unwrap_or("").to_string(),
                chg: item["chg"].as_str().unwrap_or("").to_string(),
                change: item["change"].as_str().unwrap_or("").to_string(),
                inflow: item["inflow"].as_str().unwrap_or("").to_string(),
                market_cap: item["market_cap"].as_str().unwrap_or("").to_string(),
                industry: item["industry"].as_str().unwrap_or("").to_string(),
                pre_post_price: item["pre_post_price"].as_str().unwrap_or("").to_string(),
                pre_post_chg: item["pre_post_chg"].as_str().unwrap_or("").to_string(),
                amplitude: item["amplitude"].as_str().unwrap_or("").to_string(),
                five_day_chg: item["five_day_chg"].as_str().unwrap_or("").to_string(),
                turnover_rate: item["turnover_rate"].as_str().unwrap_or("").to_string(),
                volume_rate: item["volume_rate"].as_str().unwrap_or("").to_string(),
                pb_ttm: item["pb_ttm"].as_str().unwrap_or("").to_string(),
            })
            .collect();
        Ok(RankListResponse { bmp, lists })
    }
}
