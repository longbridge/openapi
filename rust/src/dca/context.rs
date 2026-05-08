use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, dca::types::*, utils::counter::symbol_to_counter_id};

struct InnerDCAContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerDCAContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("dca context dropped");
        });
    }
}

/// Dollar-cost averaging (DCA) plan management context.
#[derive(Clone)]
pub struct DCAContext(Arc<InnerDCAContext>);

impl DCAContext {
    /// Create a [`DCAContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("dca");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating dca context");
        });
        let ctx = Self(Arc::new(InnerDCAContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("dca context created");
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

    /// List DCA plans.
    ///
    /// Path: `GET /v1/dailycoins/query`
    pub async fn list(&self, status: Option<DCAStatus>, symbol: Option<String>) -> Result<DcaList> {
        #[derive(Serialize)]
        struct Query {
            page: i32,
            limit: i32,
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<DCAStatus>,
            #[serde(skip_serializing_if = "Option::is_none")]
            counter_id: Option<String>,
        }
        self.get(
            "/v1/dailycoins/query",
            Query {
                page: 1,
                limit: 100,
                status,
                counter_id: symbol.map(|s| symbol_to_counter_id(&s)),
            },
        )
        .await
    }

    /// Create a new DCA plan.
    ///
    /// Path: `POST /v1/dailycoins/create`
    pub async fn create(
        &self,
        symbol: impl Into<String>,
        amount: impl Into<String>,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: bool,
    ) -> Result<DcaCreateResult> {
        let cid = symbol_to_counter_id(&symbol.into());
        let mut body = serde_json::json!({
            "counter_id": cid,
            "per_invest_amount": amount.into(),
            "invest_frequency": frequency,
            "allow_margin_finance": if allow_margin { 1 } else { 0 }
        });
        if let Some(dow) = day_of_week {
            body["invest_day_of_week"] = serde_json::Value::String(dow);
        }
        if let Some(dom) = day_of_month {
            body["invest_day_of_month"] = serde_json::Value::String(dom.to_string());
        }
        self.post("/v1/dailycoins/create", body).await
    }

    /// Update a DCA plan.
    ///
    /// Path: `POST /v1/dailycoins/update`
    pub async fn update(
        &self,
        plan_id: impl Into<String>,
        amount: Option<String>,
        frequency: Option<DCAFrequency>,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
        allow_margin: Option<bool>,
    ) -> Result<DcaCreateResult> {
        let mut body = serde_json::json!({ "plan_id": plan_id.into() });
        if let Some(a) = amount {
            body["per_invest_amount"] = serde_json::Value::String(a);
        }
        if let Some(f) = frequency {
            body["invest_frequency"] = serde_json::to_value(f).unwrap_or_default();
        }
        if let Some(dow) = day_of_week {
            body["invest_day_of_week"] = serde_json::Value::String(dow);
        }
        if let Some(dom) = day_of_month {
            body["invest_day_of_month"] = serde_json::Value::String(dom.to_string());
        }
        if let Some(m) = allow_margin {
            body["allow_margin_finance"] =
                serde_json::Value::Number((if m { 1 } else { 0 }).into());
        }
        self.post::<DcaCreateResult, _>("/v1/dailycoins/update", body)
            .await
    }

    /// Pause a DCA plan.
    pub async fn pause(&self, plan_id: impl Into<String>) -> Result<()> {
        self.post::<serde_json::Value, _>(
            "/v1/dailycoins/toggle",
            serde_json::json!({ "plan_id": plan_id.into(), "status": "Suspended" }),
        )
        .await?;
        Ok(())
    }

    /// Resume a suspended DCA plan.
    pub async fn resume(&self, plan_id: impl Into<String>) -> Result<()> {
        self.post::<serde_json::Value, _>(
            "/v1/dailycoins/toggle",
            serde_json::json!({ "plan_id": plan_id.into(), "status": "Active" }),
        )
        .await?;
        Ok(())
    }

    /// Stop (permanently finish) a DCA plan.
    pub async fn stop(&self, plan_id: impl Into<String>) -> Result<()> {
        self.post::<serde_json::Value, _>(
            "/v1/dailycoins/toggle",
            serde_json::json!({ "plan_id": plan_id.into(), "status": "Finished" }),
        )
        .await?;
        Ok(())
    }

    /// Get execution history for a DCA plan.
    ///
    /// Path: `GET /v1/dailycoins/query-records`
    pub async fn history(
        &self,
        plan_id: impl Into<String>,
        page: i32,
        limit: i32,
    ) -> Result<DcaHistoryResponse> {
        #[derive(Serialize)]
        struct Query {
            plan_id: String,
            page: i32,
            limit: i32,
        }
        self.get(
            "/v1/dailycoins/query-records",
            Query {
                plan_id: plan_id.into(),
                page,
                limit,
            },
        )
        .await
    }

    /// Get DCA statistics.
    ///
    /// Path: `GET /v1/dailycoins/statistic`
    pub async fn stats(&self, symbol: Option<String>) -> Result<DcaStats> {
        #[derive(Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            counter_id: Option<String>,
        }
        self.get(
            "/v1/dailycoins/statistic",
            Query {
                counter_id: symbol.map(|s| symbol_to_counter_id(&s)),
            },
        )
        .await
    }

    /// Check DCA support for a list of securities.
    ///
    /// Path: `POST /v1/dailycoins/batch-check-support`
    pub async fn check_support(&self, symbols: Vec<String>) -> Result<DcaSupportList> {
        let counter_ids: Vec<String> = symbols.iter().map(|s| symbol_to_counter_id(s)).collect();
        self.post(
            "/v1/dailycoins/batch-check-support",
            serde_json::json!({ "counter_ids": counter_ids }),
        )
        .await
    }

    /// Calculate the next projected trade date for a DCA plan with the given
    /// schedule parameters.
    ///
    /// Path: `POST /v1/dailycoins/calc-trd-date`
    pub async fn calc_date(
        &self,
        symbol: impl Into<String>,
        frequency: DCAFrequency,
        day_of_week: Option<String>,
        day_of_month: Option<u32>,
    ) -> Result<DcaCalcDateResult> {
        let mut body = serde_json::json!({
            "counter_id": symbol_to_counter_id(&symbol.into()),
            "invest_frequency": frequency,
        });
        if let Some(dow) = day_of_week {
            body["invest_day_of_week"] = serde_json::Value::String(dow);
        }
        if let Some(dom) = day_of_month {
            body["invest_day_of_month"] = serde_json::Value::String(dom.to_string());
        }
        self.post("/v1/dailycoins/calc-trd-date", body).await
    }

    /// Update the advance reminder hours for DCA execution notifications.
    ///
    /// `hours` must be one of `"1"`, `"6"`, or `"12"`.
    ///
    /// Path: `POST /v1/dailycoins/update-alter-hours`
    pub async fn set_reminder(&self, hours: impl Into<String>) -> Result<()> {
        #[derive(serde::Deserialize)]
        struct Empty {}
        self.post::<Empty, _>(
            "/v1/dailycoins/update-alter-hours",
            serde_json::json!({ "alter_hours": hours.into() }),
        )
        .await?;
        Ok(())
    }
}
