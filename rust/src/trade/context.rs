use std::sync::Arc;

use longbridge_httpcli::{DcRegion, HttpClient, Json, Method};
use longbridge_wscli::WsClientError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result, serde_utils,
    trade::{
        AccountBalance, AllExecutionsResponse, CancelOrderOptions, CashFlow,
        EstimateMaxPurchaseQuantityOptions, Execution, FundPositionsResponse,
        GetAllExecutionsOptions, GetCashFlowOptions, GetFundPositionsOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetOrderDetailOptions,
        GetStockPositionsOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions,
        GetUSAvailableHoldingOptions, GetUSHistoryOrders, GetUSRealizedPLOptions,
        GetUSTransactionHistoryOptions, MarginRatio, Order, OrderDetail, OrderSide, PushEvent,
        QueryUSOrdersResponse, ReplaceOrderOptions, RiskAssessment, StockPositionsResponse,
        SubmitMultiLegOrderOptions, SubmitOrderOptions, TopicType, USAssetOverview,
        USAvailableHolding, USCashAndBuyPower, USExchangeRates, USOrderDetailResponse,
        USRealizedPL, USTransactionHistory,
        core::{Command, Core},
    },
    types::normalize_symbol_owned,
};

#[derive(Debug, Deserialize)]
struct EmptyResponse {}

/// Response for submit order request
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitOrderResponse {
    /// Order id
    pub order_id: String,
}

/// Response for estimate maximum purchase quantity
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub cash_max_qty: Decimal,
    /// Margin available quantity
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub margin_max_qty: Decimal,
}

struct InnerTradeContext {
    command_tx: mpsc::UnboundedSender<Command>,
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
    /// Kept alive only so the background `Core::run` task can observe the
    /// context being dropped (via this channel closing) and stop reconnecting.
    _shutdown_tx: mpsc::UnboundedSender<()>,
}

impl Drop for InnerTradeContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("trade context dropped");
        });
    }
}

/// Trade context
#[derive(Clone)]
pub struct TradeContext(Arc<InnerTradeContext>);

impl TradeContext {
    /// Create a `TradeContext`
    pub fn new(config: Arc<Config>) -> (Self, mpsc::UnboundedReceiver<PushEvent>) {
        let log_subscriber = config.create_log_subscriber("trade");

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating trade context");
        });

        let http_cli = config.create_http_client();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
        let core = Core::new(config, command_rx, push_tx);
        crate::runtime::RUNTIME.handle().spawn(
            core.run(shutdown_rx)
                .with_subscriber(log_subscriber.clone()),
        );

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("trade context created");
        });

        (
            TradeContext(Arc::new(InnerTradeContext {
                http_cli,
                command_tx,
                log_subscriber,
                _shutdown_tx: shutdown_tx,
            })),
            push_rx,
        )
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Subscribe
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/trade-push#subscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config, decimal,
    ///     oauth::OAuthBuilder,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, mut receiver) = TradeContext::new(config);
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     decimal!(200),
    ///     TimeInForceType::Day,
    /// )
    /// .submitted_price(decimal!(50i32));
    /// let resp = ctx.submit_order(opts).await?;
    /// println!("{:?}", resp);
    ///
    /// while let Some(event) = receiver.recv().await {
    ///     println!("{:?}", event);
    /// }
    ///
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn subscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Subscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/trade-push#cancel-subscribe>
    pub async fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Unsubscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Get history executions
    ///
    /// `side` may be `Unknown` on Longbridge US accounts.
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/execution/history_executions>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     oauth::OAuthBuilder,
    ///     trade::{GetHistoryExecutionsOptions, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = GetHistoryExecutionsOptions::new()
    ///     .symbol("700.HK")
    ///     .start_at(datetime!(2022-05-09 0:00 UTC))
    ///     .end_at(datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.history_executions(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn history_executions(
        &self,
        options: impl Into<Option<GetHistoryExecutionsOptions>>,
    ) -> Result<Vec<Execution>> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(default)]
            has_more: bool,
            trades: Vec<Execution>,
        }

        // The endpoint caps each response at 1000 records; walk the `page`
        // param (1-based) until `has_more` is false.
        let options = options.into().unwrap_or_default();
        fetch_all_pages(
            |page| {
                let options = options.clone().with_page(page);
                async move {
                    let resp = self
                        .0
                        .http_cli
                        .request(Method::GET, "/v1/trade/execution/history")
                        .query_params(options)
                        .response::<Json<Response>>()
                        .send()
                        .with_subscriber(self.0.log_subscriber.clone())
                        .await?
                        .0;
                    Ok((resp.trades, resp.has_more))
                }
            },
            |t: &Execution| (t.order_id.clone(), t.trade_id.clone(), t.symbol.clone()),
        )
        .await
    }

    /// Get today executions
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/execution/today_executions>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     trade::{GetTodayExecutionsOptions, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = GetTodayExecutionsOptions::new().symbol("700.HK");
    /// let resp = ctx.today_executions(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn today_executions(
        &self,
        options: impl Into<Option<GetTodayExecutionsOptions>>,
    ) -> Result<Vec<Execution>> {
        #[derive(Deserialize)]
        struct Response {
            trades: Vec<Execution>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/execution/today")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .trades)
    }

    /// Get all executions
    ///
    /// AP accounts only: US data-center accounts get a region error; use
    /// `history_executions` instead.
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/execution/all_executions>
    pub async fn all_executions(
        &self,
        options: impl Into<Option<GetAllExecutionsOptions>>,
    ) -> Result<AllExecutionsResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v3/trade/execution/all")
            .dc_restrict(DcRegion::Ap)
            .query_params(options.into().unwrap_or_default())
            .response::<Json<AllExecutionsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get history orders
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/history_orders>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     oauth::OAuthBuilder,
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus, TradeContext},
    ///     Config, Market,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = GetHistoryOrdersOptions::new()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus::Filled, OrderStatus::New])
    ///     .side(OrderSide::Buy)
    ///     .market(Market::HK)
    ///     .start_at(datetime!(2022-05-09 0:00 UTC))
    ///     .end_at(datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.history_orders(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn history_orders(
        &self,
        options: impl Into<Option<GetHistoryOrdersOptions>>,
    ) -> Result<Vec<Order>> {
        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
            #[serde(default)]
            has_more: bool,
        }

        // The endpoint caps each response at 1000 records and reports the
        // overflow in `has_more`; walk the `page` param (1-based) until it is
        // false.
        let options = options.into().unwrap_or_default();
        fetch_all_pages(
            |page| {
                let options = options.clone().with_page(page);
                async move {
                    let resp = self
                        .0
                        .http_cli
                        .request(Method::GET, "/v1/trade/order/history")
                        .query_params(options)
                        .response::<Json<Response>>()
                        .send()
                        .with_subscriber(self.0.log_subscriber.clone())
                        .await?
                        .0;
                    Ok((resp.orders, resp.has_more))
                }
            },
            |o: &Order| (o.order_id.clone(), o.symbol.clone()),
        )
        .await
    }

    /// Get today orders
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/today_orders>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config, Market,
    ///     oauth::OAuthBuilder,
    ///     trade::{GetTodayOrdersOptions, OrderSide, OrderStatus, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = GetTodayOrdersOptions::new()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus::Filled, OrderStatus::New])
    ///     .side(OrderSide::Buy)
    ///     .market(Market::HK);
    /// let resp = ctx.today_orders(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn today_orders(
        &self,
        options: impl Into<Option<GetTodayOrdersOptions>>,
    ) -> Result<Vec<Order>> {
        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/order/today")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .orders)
    }

    /// Replace order
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/replace>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config, decimal,
    ///     oauth::OAuthBuilder,
    ///     trade::{ReplaceOrderOptions, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts =
    ///     ReplaceOrderOptions::new("709043056541253632", decimal!(100)).price(decimal!(300i32));
    /// let resp = ctx.replace_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        Ok(self
            .0
            .http_cli
            .request(Method::PUT, "/v1/trade/order")
            .body(Json(options))
            .response::<Json<EmptyResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await
            .map(|_| ())?)
    }

    /// Submit order
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/submit>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config, decimal,
    ///     oauth::OAuthBuilder,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     decimal!(200),
    ///     TimeInForceType::Day,
    /// )
    /// .submitted_price(decimal!(50i32));
    /// let resp = ctx.submit_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn submit_order(&self, options: SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        let resp: SubmitOrderResponse = self
            .0
            .http_cli
            .request(Method::POST, "/v1/trade/order")
            .body(Json(options))
            .response::<Json<_>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0;
        _ = self.0.command_tx.send(Command::SubmittedOrder {
            order_id: resp.order_id.clone(),
        });
        Ok(resp)
    }

    /// Submit a multi-leg option combination order (such as vertical spreads,
    /// straddles, strangles, collars, etc.). All legs are submitted together
    /// as a single strategy order.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config, decimal,
    ///     oauth::OAuthBuilder,
    ///     trade::{
    ///         MultiLegStrategy, OrderSide, OrderType, SubmitMultiLegOrderLeg,
    ///         SubmitMultiLegOrderOptions, TradeContext,
    ///     },
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = SubmitMultiLegOrderOptions::new(
    ///     OrderSide::Buy,
    ///     OrderType::LO,
    ///     decimal!(1i32),
    ///     MultiLegStrategy::VerticalCallSpread,
    ///     [
    ///         SubmitMultiLegOrderLeg::new("QQQ260731C764000.US", decimal!(1i32)),
    ///         SubmitMultiLegOrderLeg::new("QQQ260731C767000.US", decimal!(1i32)),
    ///     ],
    /// )
    /// .submitted_price(decimal!(1.5));
    /// let resp = ctx.submit_multileg(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn submit_multileg(
        &self,
        options: SubmitMultiLegOrderOptions,
    ) -> Result<SubmitOrderResponse> {
        let resp: SubmitOrderResponse = self
            .0
            .http_cli
            .request(Method::POST, "/v1/trade/order/multileg")
            .body(Json(options))
            .response::<Json<_>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0;
        _ = self.0.command_tx.send(Command::SubmittedOrder {
            order_id: resp.order_id.clone(),
        });
        Ok(resp)
    }

    /// Cancel order
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/withdraw>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// ctx.cancel_order("709043056541253632").await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cancel_order(&self, options: impl Into<CancelOrderOptions>) -> Result<()> {
        Ok(self
            .0
            .http_cli
            .request(Method::DELETE, "/v1/trade/order")
            .response::<Json<EmptyResponse>>()
            .query_params(options.into())
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await
            .map(|_| ())?)
    }

    /// Get account balance
    ///
    /// AP accounts only: US data-center accounts get a region error; use
    /// `us_cash_and_buy_power` (or `us_asset_overview`) instead.
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/asset/account>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx.account_balance(None).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn account_balance(&self, currency: Option<&str>) -> Result<Vec<AccountBalance>> {
        #[derive(Debug, Serialize)]
        struct Request<'a> {
            currency: Option<&'a str>,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<AccountBalance>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/account")
            .dc_restrict(DcRegion::Ap)
            .query_params(Request { currency })
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get cash flow
    ///
    /// AP accounts only: US data-center accounts get a region error; use
    /// `us_transaction_history` instead.
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/asset/cashflow>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     oauth::OAuthBuilder,
    ///     trade::{GetCashFlowOptions, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let opts = GetCashFlowOptions::new(datetime!(2022-05-09 0:00 UTC), datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.cash_flow(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cash_flow(&self, options: GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<CashFlow>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/cashflow")
            .dc_restrict(DcRegion::Ap)
            .query_params(options)
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get fund positions
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/asset/fund>
    ///
    /// AP accounts only: US data-center accounts have no fund holdings and
    /// get a region error. No US equivalent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx.fund_positions(None).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn fund_positions(
        &self,
        opts: impl Into<Option<GetFundPositionsOptions>>,
    ) -> Result<FundPositionsResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/fund")
            .dc_restrict(DcRegion::Ap)
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<FundPositionsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get stock positions
    ///
    /// AP accounts only: US data-center accounts get a region error; use
    /// `us_asset_overview` (`stock_list`) instead.
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/asset/stock>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx.stock_positions(None).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn stock_positions(
        &self,
        opts: impl Into<Option<GetStockPositionsOptions>>,
    ) -> Result<StockPositionsResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/stock")
            .dc_restrict(DcRegion::Ap)
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<StockPositionsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get margin ratio
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/asset/margin_ratio>
    ///
    /// AP accounts only: US data-center accounts get a region error. No US
    /// equivalent; for US margin info see `us_cash_and_buy_power`
    /// (`margin_status`).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx.margin_ratio("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn margin_ratio(&self, symbol: impl Into<String>) -> Result<MarginRatio> {
        #[derive(Debug, Serialize)]
        struct Request {
            symbol: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/risk/margin-ratio")
            .dc_restrict(DcRegion::Ap)
            .query_params(Request {
                symbol: normalize_symbol_owned(symbol),
            })
            .response::<Json<MarginRatio>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the risk assessment (suitability questionnaire) result of the
    /// current member
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, trade::TradeContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx.risk_assessment().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn risk_assessment(&self) -> Result<RiskAssessment> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/members/me/risk-assessment")
            .response::<Json<RiskAssessment>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get order detail
    ///
    /// AP accounts only: US data-center accounts get a region error; use
    /// `us_order_detail` instead.
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/order_detail>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config, Market,
    ///     oauth::OAuthBuilder,
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus, TradeContext},
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx.order_detail("701276261045858304").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn order_detail(
        &self,
        options: impl Into<GetOrderDetailOptions>,
    ) -> Result<OrderDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/order")
            .dc_restrict(DcRegion::Ap)
            .response::<Json<OrderDetail>>()
            .query_params(options.into())
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Estimating the maximum purchase quantity for Hong Kong and US stocks,
    /// warrants, and options
    ///
    ///
    /// Reference: <https://open.longbridge.com/en/docs/trade/order/estimate_available_buy_limit>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     trade::{EstimateMaxPurchaseQuantityOptions, OrderSide, OrderType, TradeContext},
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = TradeContext::new(config);
    ///
    /// let resp = ctx
    ///     .estimate_max_purchase_quantity(EstimateMaxPurchaseQuantityOptions::new(
    ///         "700.HK",
    ///         OrderType::LO,
    ///         OrderSide::Buy,
    ///     ))
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn estimate_max_purchase_quantity(
        &self,
        opts: EstimateMaxPurchaseQuantityOptions,
    ) -> Result<EstimateMaxPurchaseQuantityResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/trade/estimate/buy_limit")
            .query_params(opts)
            .response::<Json<EstimateMaxPurchaseQuantityResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    // ── US-market APIs
    // ────────────────────────────────────────────────────────

    /// Query the paginated US order list.
    ///
    /// Path: `POST /v1/us/orders/query`
    ///
    /// US token required.
    pub async fn us_query_orders(&self, opts: GetUSHistoryOrders) -> Result<QueryUSOrdersResponse> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let action = match opts.side {
            OrderSide::Buy => 1,
            OrderSide::Sell => 2,
            _ => 0,
        };

        let symbols = opts
            .symbol
            .as_deref()
            .filter(|s| !s.is_empty())
            .map(|s| vec![s.to_string()])
            .unwrap_or_default();

        let start_at = if opts.start_at == 0 {
            (now - 90 * 24 * 3600) as f64
        } else {
            opts.start_at as f64
        };
        let end_at = if opts.end_at == 0 {
            now as f64
        } else {
            opts.end_at as f64
        };
        let page = if opts.page <= 0 { 1 } else { opts.page };
        let limit = if opts.limit <= 0 { 20 } else { opts.limit };

        let body = super::types::USQueryOrdersBody {
            account_channel: String::new(),
            action,
            start_at,
            end_at,
            symbols,
            security_types: vec![],
            query_type: opts.query_type,
            page,
            limit,
            query_version: now as f64,
        };

        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/us/orders/query")
            .dc_restrict(DcRegion::Us)
            .body(Json(body))
            .response::<Json<QueryUSOrdersResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get US order detail.
    ///
    /// Path: `GET /v1/us/orders/{order_id}`
    ///
    /// US token required.
    pub async fn us_order_detail(
        &self,
        order_id: impl Into<String>,
    ) -> Result<USOrderDetailResponse> {
        let order_id = order_id.into();
        let path = format!("/v1/us/orders/{order_id}");

        Ok(self
            .0
            .http_cli
            .request(Method::GET, path.as_str())
            .dc_restrict(DcRegion::Us)
            .response::<Json<USOrderDetailResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the full US account asset snapshot (stocks, options, crypto, buying
    /// power).
    ///
    /// Path: `GET /v1/us/assets/overview`
    ///
    /// US token required.
    pub async fn us_asset_overview(&self) -> Result<USAssetOverview> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/us/assets/overview")
            .dc_restrict(DcRegion::Us)
            .response::<Json<USAssetOverview>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get realized profit-and-loss for the US account.
    ///
    /// `currency`: required, e.g. `"USD"`.
    /// `category`: optional filter — `"ALL"`, `"STOCK"`, `"OPTION"`, or
    /// `"CRYPTO"`.
    ///
    /// Path: `GET /v1/us/assets/pl/realized`
    ///
    /// US token required.
    pub async fn us_realized_pl(&self, opts: GetUSRealizedPLOptions) -> Result<USRealizedPL> {
        #[derive(Serialize)]
        struct Query {
            currency: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<String>,
        }

        let currency = if opts.currency.is_empty() {
            "USD".to_string()
        } else {
            opts.currency
        };
        let category = if opts.category.is_empty() {
            None
        } else {
            Some(opts.category)
        };

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/us/assets/pl/realized")
            .dc_restrict(DcRegion::Us)
            .query_params(Query { currency, category })
            .response::<Json<USRealizedPL>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get cash, buying power and margin status for the US account.
    ///
    /// `currency`: optional, e.g. `"USD"`.
    ///
    /// Path: `GET /v1/us/assets/cash-and-buy-power`
    ///
    /// US token required.
    pub async fn us_cash_and_buy_power(&self, currency: Option<&str>) -> Result<USCashAndBuyPower> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            currency: Option<&'a str>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/us/assets/cash-and-buy-power")
            .dc_restrict(DcRegion::Us)
            .query_params(Query { currency })
            .response::<Json<USCashAndBuyPower>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the quantity of a US holding that is available to trade.
    ///
    /// Path: `GET /v1/us/assets/available-holding`
    ///
    /// US token required.
    pub async fn us_available_holding(
        &self,
        opts: GetUSAvailableHoldingOptions,
    ) -> Result<USAvailableHolding> {
        #[derive(Serialize)]
        struct Query {
            symbol: String,
            equity_type: String,
            position_side: String,
        }

        let equity_type = if opts.equity_type.is_empty() {
            "STOCK".to_string()
        } else {
            opts.equity_type
        };
        let position_side = if opts.position_side.is_empty() {
            "LONG".to_string()
        } else {
            opts.position_side
        };

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/us/assets/available-holding")
            .dc_restrict(DcRegion::Us)
            .query_params(Query {
                symbol: opts.symbol,
                equity_type,
                position_side,
            })
            .response::<Json<USAvailableHolding>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the US account transaction history (deposits, withdrawals, trades,
    /// corporate actions, transfers and so on), one page at a time.
    ///
    /// Path: `POST /v1/us/assets/transaction-history`
    ///
    /// US token required.
    pub async fn us_transaction_history(
        &self,
        opts: GetUSTransactionHistoryOptions,
    ) -> Result<USTransactionHistory> {
        #[derive(Debug, Serialize)]
        struct Body {
            start: i64,
            end: i64,
            page: i32,
            size: i32,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            types: Vec<String>,
            #[serde(skip_serializing_if = "String::is_empty")]
            currency: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/us/assets/transaction-history")
            .dc_restrict(DcRegion::Us)
            .body(Json(Body {
                start: opts.start_at.unix_timestamp(),
                end: opts.end_at.unix_timestamp(),
                page: if opts.page > 0 { opts.page } else { 1 },
                size: if opts.size > 0 { opts.size } else { 20 },
                types: opts.types,
                currency: opts.currency,
            }))
            .response::<Json<USTransactionHistory>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get the exchange rates used by the US account.
    ///
    /// The upstream two-level map is flattened into
    /// [`USExchangeRates::exchanges`]; each entry keeps the two upstream map
    /// keys as `universal_key` and `spec_key`.
    ///
    /// Path: `GET /v1/us/assets/exchange-rates`
    ///
    /// US token required.
    pub async fn us_exchange_rates(&self) -> Result<USExchangeRates> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/us/assets/exchange-rates")
            .dc_restrict(DcRegion::Us)
            .response::<Json<USExchangeRates>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}

/// Upper bound on the pages [`fetch_all_pages`] requests, as a runaway guard.
const MAX_PAGES: u32 = 1000;

/// Collect every row of a `page`-paginated endpoint (1-based pages, with the
/// server reporting `has_more`).
///
/// Rows are returned exactly as the server sends them — nothing is
/// deduplicated, so rows that happen to share an id are all kept wherever the
/// page boundary falls. `key` only identifies a page: if a page has the same
/// keys, in the same order, as the page before it, the gateway is taken to be
/// ignoring `page` (answering every request with the same page), that repeat
/// is discarded and paging stops. Paging also stops on an empty page, when
/// `has_more` is false, or after `MAX_PAGES`.
async fn fetch_all_pages<T, K, F, Fut>(mut fetch: F, key: impl Fn(&T) -> K) -> Result<Vec<T>>
where
    K: Eq,
    F: FnMut(u32) -> Fut,
    Fut: std::future::Future<Output = Result<(Vec<T>, bool)>>,
{
    let mut all = Vec::new();
    let mut prev_keys: Option<Vec<K>> = None;
    for page in 1..=MAX_PAGES {
        let (rows, has_more) = fetch(page).await?;
        if rows.is_empty() {
            break;
        }
        let keys: Vec<K> = rows.iter().map(&key).collect();
        if prev_keys.as_ref() == Some(&keys) {
            break;
        }
        prev_keys = Some(keys);
        all.extend(rows);
        if !has_more {
            break;
        }
    }
    Ok(all)
}

#[cfg(test)]
mod dc_restrict_tests {
    use longbridge_httpcli::HttpClientError;
    use time::macros::datetime;

    use super::*;
    use crate::Error;

    fn us_trade_context() -> TradeContext {
        let config = Arc::new(Config::from_apikey("us_app_key", "app_secret", "us_token"));
        TradeContext::new(config).0
    }

    #[track_caller]
    fn assert_ap_restricted<T>(res: Result<T>, path: &str)
    where
        T: std::fmt::Debug,
    {
        match res {
            Err(Error::HttpClient(HttpClientError::DcRegionRestricted {
                path: p,
                required,
                current,
            })) => {
                assert_eq!(p, path, "restricted path mismatch");
                assert_eq!(required, DcRegion::Ap, "expected AP-only restriction");
                assert_eq!(current, DcRegion::Us, "expected US session region");
            }
            other => panic!("expected DcRegionRestricted for {path}, got {other:?}"),
        }
    }

    /// AP-only asset/risk/order endpoints fail fast for a US (`us_`) token
    /// instead of sending a request the US data center rejects.
    #[tokio::test(flavor = "multi_thread")]
    async fn ap_only_endpoints_reject_us_token() {
        let ctx = us_trade_context();

        assert_ap_restricted(ctx.account_balance(None).await, "/v1/asset/account");
        assert_ap_restricted(
            ctx.cash_flow(GetCashFlowOptions::new(
                datetime!(2026-01-01 0:00 UTC),
                datetime!(2026-01-02 0:00 UTC),
            ))
            .await,
            "/v1/asset/cashflow",
        );
        assert_ap_restricted(ctx.fund_positions(None).await, "/v1/asset/fund");
        assert_ap_restricted(ctx.stock_positions(None).await, "/v1/asset/stock");
        assert_ap_restricted(ctx.margin_ratio("700.HK").await, "/v1/risk/margin-ratio");
        assert_ap_restricted(ctx.order_detail("1").await, "/v1/trade/order");
        assert_ap_restricted(ctx.all_executions(None).await, "/v3/trade/execution/all");
    }
}

#[cfg(test)]
mod symbol_normalization_tests {
    use time::macros::datetime;

    use super::*;
    use crate::trade::{
        GetCashFlowOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
        GetStockPositionsOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions, OrderSide,
        OrderType, SubmitOrderOptions, TimeInForceType,
    };

    fn query(value: &impl Serialize) -> String {
        serde_json::to_string(&serde_json::to_value(value).unwrap()).unwrap()
    }

    /// `/v1/asset/stock?symbol=00700.HK` answers `200` with **no** positions
    /// rather than an error, so a padded HK symbol has to be canonicalized
    /// before it leaves the SDK. Checked against the gateway on 2026-11-18:
    /// `700.HK` returned the position, `00700.HK` returned none.
    #[test]
    fn stock_positions_strips_hk_padding() {
        let opts = GetStockPositionsOptions::new().symbols(["00700.HK", "AAPL.US", "000001.SZ"]);
        let q = query(&opts);
        assert!(q.contains("700.HK"), "{q}");
        assert!(!q.contains("00700.HK"), "{q}");
        // Every other market keeps its symbol verbatim.
        assert!(q.contains("AAPL.US"), "{q}");
        assert!(q.contains("000001.SZ"), "{q}");
    }

    #[test]
    fn every_symbol_filter_is_normalized() {
        assert!(query(&GetHistoryOrdersOptions::new().symbol("00700.HK")).contains("\"700.HK\""));
        assert!(query(&GetTodayOrdersOptions::new().symbol("00700.HK")).contains("\"700.HK\""));
        assert!(
            query(&GetHistoryExecutionsOptions::new().symbol("00700.HK")).contains("\"700.HK\"")
        );
        assert!(query(&GetTodayExecutionsOptions::new().symbol("00700.HK")).contains("\"700.HK\""));
        assert!(
            query(
                &GetCashFlowOptions::new(
                    datetime!(2026-01-01 0:00 UTC),
                    datetime!(2026-01-31 0:00 UTC)
                )
                .symbol("00700.HK")
            )
            .contains("\"700.HK\"")
        );
    }

    #[test]
    fn submit_order_normalizes_the_symbol() {
        let opts = SubmitOrderOptions::new(
            "00700.HK",
            OrderType::LO,
            OrderSide::Buy,
            decimal!(100i32),
            TimeInForceType::Day,
        );
        let body = serde_json::to_value(&opts).unwrap();
        assert_eq!(body["symbol"], "700.HK");
    }
}

#[cfg(test)]
mod fetch_all_pages_tests {
    use std::sync::Mutex;

    use super::*;

    type Row = (&'static str, u32);

    /// Serve `pages` (1-based) as `(rows, has_more)`, recording each request.
    /// Rows are keyed by their id (`.0`) only, so rows sharing an id are
    /// distinct rows with the same key.
    async fn run(pages: Vec<(Vec<Row>, bool)>) -> (Vec<Row>, Vec<u32>) {
        let requested = Mutex::new(Vec::new());
        let rows = fetch_all_pages(
            |page| {
                requested.lock().unwrap().push(page);
                let resp = pages
                    .get(page as usize - 1)
                    .cloned()
                    .unwrap_or((Vec::new(), false));
                async move { Ok(resp) }
            },
            |row: &Row| row.0,
        )
        .await
        .unwrap();
        (rows, requested.into_inner().unwrap())
    }

    #[tokio::test]
    async fn keeps_rows_that_share_a_key_within_a_page() {
        let (rows, requested) = run(vec![(vec![("a", 1), ("a", 2), ("b", 3)], false)]).await;
        assert_eq!(rows, vec![("a", 1), ("a", 2), ("b", 3)]);
        assert_eq!(requested, vec![1]);
    }

    #[tokio::test]
    async fn keeps_rows_that_share_a_key_across_a_page_boundary() {
        // The result must not depend on where the page boundary falls.
        let (rows, _) = run(vec![
            (vec![("a", 1), ("b", 2)], true),
            (vec![("b", 3)], false),
        ])
        .await;
        assert_eq!(rows, vec![("a", 1), ("b", 2), ("b", 3)]);
    }

    #[tokio::test]
    async fn walks_pages_until_has_more_is_false() {
        let (rows, requested) = run(vec![
            (vec![("a", 1)], true),
            (vec![("b", 2)], true),
            (vec![("c", 3)], false),
        ])
        .await;
        assert_eq!(rows, vec![("a", 1), ("b", 2), ("c", 3)]);
        assert_eq!(requested, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn stops_when_the_gateway_ignores_page() {
        // Every request returns page 1 with has_more = true.
        let page = (vec![("a", 1), ("b", 2)], true);
        let (rows, requested) = run(vec![page.clone(), page.clone(), page]).await;
        assert_eq!(rows, vec![("a", 1), ("b", 2)]);
        assert_eq!(requested, vec![1, 2]);
    }

    #[tokio::test]
    async fn stops_on_an_empty_page() {
        let (rows, requested) = run(vec![(vec![("a", 1)], true), (vec![], true)]).await;
        assert_eq!(rows, vec![("a", 1)]);
        assert_eq!(requested, vec![1, 2]);
    }
}
