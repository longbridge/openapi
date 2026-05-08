use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, portfolio::types::*, utils::counter::symbol_to_counter_id};

struct InnerPortfolioContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerPortfolioContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("portfolio context dropped");
        });
    }
}

/// Portfolio analytics context — exchange rates, P&L analysis.
#[derive(Clone)]
pub struct PortfolioContext(Arc<InnerPortfolioContext>);

impl PortfolioContext {
    /// Create a [`PortfolioContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("portfolio");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating portfolio context");
        });
        let ctx = Self(Arc::new(InnerPortfolioContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("portfolio context created");
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

    // ── exchange_rate ─────────────────────────────────────────────

    /// Get exchange rates for supported currencies.
    ///
    /// Path: `GET /v1/asset/exchange_rates`
    pub async fn exchange_rate(&self) -> Result<ExchangeRates> {
        #[derive(Serialize)]
        struct Empty {}
        self.get("/v1/asset/exchange_rates", Empty {}).await
    }

    // ── profit_analysis ───────────────────────────────────────────

    /// Get portfolio P&L analysis (summary + per-security breakdown).
    ///
    /// Combines `GET /v1/portfolio/profit-analysis-summary` and
    /// `GET /v1/portfolio/profit-analysis-sublist` concurrently.
    pub async fn profit_analysis(
        &self,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysis> {
        let start_ts = date_to_unix_opt(start.as_deref());
        let end_ts = date_to_unix_end_opt(end.as_deref());

        #[derive(Serialize)]
        struct SummaryQuery {
            #[serde(skip_serializing_if = "Option::is_none")]
            start: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end: Option<i64>,
        }
        #[derive(Serialize)]
        struct SublistQuery {
            profit_or_loss: &'static str,
            #[serde(skip_serializing_if = "Option::is_none")]
            start: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end: Option<i64>,
        }

        let (summary, sublist) = tokio::join!(
            self.get::<ProfitAnalysisSummary, _>(
                "/v1/portfolio/profit-analysis-summary",
                SummaryQuery {
                    start: start_ts,
                    end: end_ts
                }
            ),
            self.get::<ProfitAnalysisSublist, _>(
                "/v1/portfolio/profit-analysis-sublist",
                SublistQuery {
                    profit_or_loss: "all",
                    start: start_ts,
                    end: end_ts
                }
            ),
        );

        Ok(ProfitAnalysis {
            summary: summary?,
            sublist: sublist?,
        })
    }

    /// Get paginated P&L analysis filtered by market.
    ///
    /// Path: `GET /v1/portfolio/profit-analysis/by-market`
    pub async fn profit_analysis_by_market(
        &self,
        market: Option<String>,
        start: Option<String>,
        end: Option<String>,
        currency: Option<String>,
        page: u32,
        size: u32,
    ) -> Result<ProfitAnalysisByMarket> {
        #[derive(Serialize)]
        struct Query {
            page: u32,
            size: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            market: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            currency: Option<String>,
        }
        self.get(
            "/v1/portfolio/profit-analysis/by-market",
            Query {
                page,
                size,
                market,
                start: date_to_unix_opt(start.as_deref()),
                end: date_to_unix_end_opt(end.as_deref()),
                currency,
            },
        )
        .await
    }

    /// Get P&L detail for a specific security.
    ///
    /// Path: `GET /v1/portfolio/profit-analysis/detail`
    pub async fn profit_analysis_detail(
        &self,
        symbol: impl Into<String>,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysisDetail> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            start: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end: Option<i64>,
        }
        self.get(
            "/v1/portfolio/profit-analysis/detail",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                start: date_to_unix_opt(start.as_deref()),
                end: date_to_unix_end_opt(end.as_deref()),
            },
        )
        .await
    }

    // ── profit_analysis_flows ─────────────────────────────────────

    /// Get paginated P&L flow records for a security.
    ///
    /// Path: `GET /v1/portfolio/profit-analysis/flows`
    #[allow(clippy::too_many_arguments)]
    pub async fn profit_analysis_flows(
        &self,
        symbol: impl Into<String>,
        page: u32,
        size: u32,
        derivative: bool,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysisFlows> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            page: u32,
            size: u32,
            derivative: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            start: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end: Option<String>,
        }
        self.get(
            "/v1/portfolio/profit-analysis/flows",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                page,
                size,
                derivative,
                start,
                end,
            },
        )
        .await
    }
}

/// Convert an optional `YYYY-MM-DD` date string to a unix timestamp (midnight
/// UTC).
fn date_to_unix_opt(date: Option<&str>) -> Option<i64> {
    date.and_then(|d| {
        let parts: Vec<&str> = d.split('-').collect();
        if parts.len() == 3 {
            let y: i32 = parts[0].parse().ok()?;
            let m: u8 = parts[1].parse().ok()?;
            let d: u8 = parts[2].parse().ok()?;
            let date = time::Date::from_calendar_date(y, time::Month::try_from(m).ok()?, d).ok()?;
            let dt = date.midnight().assume_utc();
            Some(dt.unix_timestamp())
        } else {
            None
        }
    })
}

/// Convert to end-of-day unix timestamp (23:59:59 UTC).
fn date_to_unix_end_opt(date: Option<&str>) -> Option<i64> {
    date_to_unix_opt(date).map(|ts| ts + 86399)
}
