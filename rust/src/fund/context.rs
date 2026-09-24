use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Deserialize, Serialize};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    fund::{
        FundAnalysis, FundAnalysisDetail, FundAnnualReturn, FundBrief, FundDetail, FundDividends,
        FundFilters, FundHoldings, FundNavRangeOptions, FundNavValue, FundOrder, FundOrderDetail,
        FundOrderSubmitResponse, FundOrderValidation, FundPageOptions, FundPerformance,
        FundPerformanceComparison, FundPositionDetail, FundPositionNav, FundPositionPerformance,
        FundPositionProfits, FundPositions, FundQuarterlyReturn, FundStockHolding, FundTransaction,
        FundTrend, GetFundAnalysisOptions, GetFundHoldingsOptions, GetFundOrdersOptions,
        GetFundPositionDividendsOptions, GetFundPositionOptions, GetFundPositionProfitsOptions,
        GetFundPositionsOptions, GetFundStockHoldingsOptions, GetFundTransactionsOptions,
        GetFundsOptions, HotFund, SubmitFundOrderOptions, ValidateFundOrderOptions,
    },
};

#[derive(Debug, Deserialize)]
struct WList<T> {
    #[serde(default = "Vec::new")]
    list: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct WFunds {
    #[serde(default)]
    funds: Vec<FundBrief>,
}

#[derive(Debug, Deserialize)]
struct WValue<T> {
    #[serde(default = "Vec::new")]
    value: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct WHistory<T> {
    #[serde(default = "Vec::new")]
    history_value: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct WLists<T> {
    #[serde(default = "Vec::new")]
    lists: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct WOrders {
    #[serde(default)]
    orders: Vec<FundOrder>,
}

/// Query wrapper that prepends the fund `counter_id` before flattening the
/// endpoint-specific options. The fund identifier (`counter_id`, e.g.
/// `UT/FD/HK0000384492`) contains `/`, so it cannot live in the URL path and is
/// passed as the `counter_id` query parameter instead.
#[derive(Serialize)]
struct CounterIdQuery<T> {
    counter_id: String,
    #[serde(flatten)]
    options: T,
}

/// Empty option set for `counter_id`-only endpoints.
#[derive(Serialize, Default)]
struct NoQuery {}

/// Query wrapper for the batch endpoints (latest NAV, daily performance, held
/// fund performance) whose backend takes a JSON-array `counter_ids` parameter.
/// The single `counter_id` is wrapped into a one-element JSON array to match
/// the backend contract — sending the scalar `counter_id` makes the backend
/// fail.
#[derive(Serialize)]
struct CounterIdsQuery {
    counter_ids: String,
}

impl CounterIdsQuery {
    fn single(counter_id: String) -> Self {
        Self {
            counter_ids: serde_json::to_string(&[counter_id]).expect("serialize counter_ids array"),
        }
    }
}

struct InnerFundContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerFundContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("fund context dropped");
        });
    }
}

/// Fund (mutual fund) channel context.
#[derive(Clone)]
pub struct FundContext(Arc<InnerFundContext>);

impl FundContext {
    /// Create a [`FundContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("fund");
        let ctx = Self(Arc::new(InnerFundContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("fund context created");
        });
        ctx
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    // ----- fund catalog / market data (scope: quote) -----

    /// Get the hot-selling fund list.
    pub async fn hot_funds(&self) -> Result<Vec<HotFund>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/hot-funds")
            .response::<Json<WList<HotFund>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get the fund list.
    pub async fn funds(
        &self,
        options: impl Into<Option<GetFundsOptions>>,
    ) -> Result<Vec<FundBrief>> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/fund/funds")
            .body(Json(options.into().unwrap_or_default()))
            .response::<Json<WFunds>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .funds)
    }

    /// Get the fund list filter options.
    pub async fn filters(&self) -> Result<FundFilters> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/filters")
            .response::<Json<FundFilters>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get fund detail.
    pub async fn detail(&self, counter_id: impl Into<String>) -> Result<FundDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/detail")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: NoQuery {},
            })
            .response::<Json<FundDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get fund analysis (level 1).
    pub async fn analysis(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundAnalysisOptions>>,
    ) -> Result<FundAnalysis> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/analysis")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundAnalysis>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get fund analysis detail (level 2).
    pub async fn analysis_detail(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundAnalysisOptions>>,
    ) -> Result<FundAnalysisDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/analysis/detail")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundAnalysisDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get fund trend chart.
    pub async fn trend(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundAnalysisOptions>>,
    ) -> Result<FundTrend> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/trend")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundTrend>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get fund annual returns.
    pub async fn annual_returns(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<FundPageOptions>>,
    ) -> Result<Vec<FundAnnualReturn>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/returns/annual")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<WList<FundAnnualReturn>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get fund quarterly returns.
    pub async fn quarterly_returns(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<FundPageOptions>>,
    ) -> Result<Vec<FundQuarterlyReturn>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/returns/quarterly")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<WList<FundQuarterlyReturn>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get fund performance figures.
    pub async fn performance(&self, counter_id: impl Into<String>) -> Result<Vec<FundPerformance>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/performance")
            .query_params(CounterIdsQuery::single(counter_id.into()))
            .response::<Json<WValue<FundPerformance>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .value)
    }

    /// Get fund performance comparison.
    pub async fn performance_comparison(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundAnalysisOptions>>,
    ) -> Result<FundPerformanceComparison> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/performance/comparison")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundPerformanceComparison>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get fund latest net value.
    pub async fn nav(&self, counter_id: impl Into<String>) -> Result<Vec<FundNavValue>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/nav")
            .query_params(CounterIdsQuery::single(counter_id.into()))
            .response::<Json<WValue<FundNavValue>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .value)
    }

    /// Get fund historical net value (paged).
    pub async fn nav_history(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<FundPageOptions>>,
    ) -> Result<Vec<FundNavValue>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/nav-history")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<WHistory<FundNavValue>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .history_value)
    }

    /// Get fund historical net value by relative time range.
    pub async fn nav_range(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<FundNavRangeOptions>>,
    ) -> Result<Vec<FundNavValue>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/nav-range")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<WHistory<FundNavValue>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .history_value)
    }

    /// Get a fund's top-10 holdings.
    pub async fn holdings(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundHoldingsOptions>>,
    ) -> Result<FundHoldings> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/holdings")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundHoldings>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the stocks held by a fund (reverse lookup).
    pub async fn stock_holdings(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundStockHoldingsOptions>>,
    ) -> Result<Vec<FundStockHolding>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/funds/stock-holdings")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<WLists<FundStockHolding>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .lists)
    }

    // ----- user fund positions (scope: portfolio-asset) -----

    /// Get the user's fund positions overview.
    pub async fn positions(
        &self,
        options: impl Into<Option<GetFundPositionsOptions>>,
    ) -> Result<FundPositions> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/funds")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<FundPositions>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the user's single fund position detail.
    pub async fn position(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundPositionOptions>>,
    ) -> Result<FundPositionDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/funds/detail")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundPositionDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the performance figures of a held fund.
    pub async fn position_performance(
        &self,
        counter_id: impl Into<String>,
    ) -> Result<Vec<FundPositionPerformance>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/funds/performance")
            .query_params(CounterIdsQuery::single(counter_id.into()))
            .response::<Json<WValue<FundPositionPerformance>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .value)
    }

    /// Get the cumulative-profit series of a held fund.
    pub async fn position_profits(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundPositionProfitsOptions>>,
    ) -> Result<FundPositionProfits> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/funds/profits")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundPositionProfits>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the net-value history of a held fund.
    pub async fn position_nav(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<FundNavRangeOptions>>,
    ) -> Result<Vec<FundPositionNav>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/funds/nav-history")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<WHistory<FundPositionNav>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .history_value)
    }

    /// Get the dividend records of a held fund.
    pub async fn position_dividends(
        &self,
        counter_id: impl Into<String>,
        options: impl Into<Option<GetFundPositionDividendsOptions>>,
    ) -> Result<FundDividends> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/funds/dividends")
            .query_params(CounterIdQuery {
                counter_id: counter_id.into(),
                options: options.into().unwrap_or_default(),
            })
            .response::<Json<FundDividends>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    // ----- fund orders & trading (scope: order) -----

    /// Get the user's fund orders (also serves as the trade/execution record).
    pub async fn orders(
        &self,
        options: impl Into<Option<GetFundOrdersOptions>>,
    ) -> Result<Vec<FundOrder>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/orders")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<WOrders>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .orders)
    }

    /// Get a fund order detail.
    pub async fn order(&self, order_id: i64) -> Result<FundOrderDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, format!("/v1/fund/orders/{order_id}"))
            .response::<Json<FundOrderDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the user's fund transactions (cash-flow records).
    pub async fn transactions(
        &self,
        options: impl Into<Option<GetFundTransactionsOptions>>,
    ) -> Result<Vec<FundTransaction>> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/fund/transactions")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<WList<FundTransaction>>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Validate a fund order before submitting.
    pub async fn validate_order(
        &self,
        options: ValidateFundOrderOptions,
    ) -> Result<FundOrderValidation> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/fund/orders/validate")
            .body(Json(options))
            .response::<Json<FundOrderValidation>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Submit a fund order (buy / sell).
    pub async fn submit_order(
        &self,
        options: SubmitFundOrderOptions,
    ) -> Result<FundOrderSubmitResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/fund/orders")
            .body(Json(options))
            .response::<Json<FundOrderSubmitResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Cancel (withdraw) a fund order.
    pub async fn cancel_order(&self, order_id: i64) -> Result<()> {
        #[derive(Debug, serde::Serialize)]
        struct Req {
            ut_id: i64,
        }
        #[derive(Deserialize)]
        struct Resp {
            #[serde(default)]
            #[allow(dead_code)]
            msg: String,
        }
        self.0
            .http_cli
            .request(Method::POST, format!("/v1/fund/orders/{order_id}/cancel"))
            .body(Json(Req { ut_id: order_id }))
            .response::<Json<Resp>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(())
    }
}
