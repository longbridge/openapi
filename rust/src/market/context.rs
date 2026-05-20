use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    market::types::*,
    utils::counter::{index_symbol_to_counter_id, symbol_to_counter_id},
};

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
}
