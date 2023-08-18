use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use longbridge_wscli::WsClientError;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::{
    serde_utils,
    trade::{
        core::{Command, Core},
        AccountBalance, CashFlow, EstimateMaxPurchaseQuantityOptions, Execution,
        FundPositionsResponse, GetCashFlowOptions, GetFundPositionsOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetStockPositionsOptions,
        GetTodayExecutionsOptions, GetTodayOrdersOptions, MarginRatio, Order, OrderDetail,
        PushEvent, ReplaceOrderOptions, StockPositionsResponse, SubmitOrderOptions, TopicType,
    },
    Config, Result,
};

#[derive(Debug, Deserialize)]
struct EmptyResponse {}

/// Response for submit order request
#[derive(Debug, Deserialize)]
pub struct SubmitOrderResponse {
    /// Order id
    pub order_id: String,
}

/// Response for estimate maximum purchase quantity
#[derive(Debug, Deserialize)]
pub struct EstimateMaxPurchaseQuantityResponse {
    /// Cash available quantity
    #[serde(with = "serde_utils::int64_str")]
    pub cash_max_qty: i64,
    /// Margin available quantity
    #[serde(with = "serde_utils::int64_str")]
    pub margin_max_qty: i64,
}

/// Trade context
#[derive(Clone)]
pub struct TradeContext {
    command_tx: mpsc::UnboundedSender<Command>,
    http_cli: HttpClient,
}

impl TradeContext {
    /// Create a `TradeContext`
    pub async fn try_new(
        config: Arc<Config>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<PushEvent>)> {
        let http_cli = config.create_http_client();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        tokio::spawn(Core::try_new(config, command_rx, push_tx).await?.run());
        Ok((
            TradeContext {
                http_cli,
                command_tx,
            },
            push_rx,
        ))
    }

    /// Subscribe
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/trade-push#subscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     decimal,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    ///     Config,
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, mut receiver) = TradeContext::try_new(config).await?;
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     200,
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
        self.command_tx
            .send(Command::Subscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/trade-push#cancel-subscribe>
    pub async fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.command_tx
            .send(Command::Unsubscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Get history executions
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/execution/history_executions>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{GetHistoryExecutionsOptions, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            trades: Vec<Execution>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/execution/history")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .trades)
    }

    /// Get today executions
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/execution/today_executions>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{GetTodayExecutionsOptions, TradeContext},
    ///     Config,
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/trade/execution/today")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .trades)
    }

    /// Get history orders
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/history_orders>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus, TradeContext},
    ///     Config, Market,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/order/history")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .orders)
    }

    /// Get today orders
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/today_orders>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{GetTodayOrdersOptions, OrderSide, OrderStatus, TradeContext},
    ///     Config, Market,
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/trade/order/today")
            .query_params(options.into().unwrap_or_default())
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .orders)
    }

    /// Replace order
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/replace>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     decimal,
    ///     trade::{ReplaceOrderOptions, TradeContext},
    ///     Config,
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = ReplaceOrderOptions::new("709043056541253632", 100).price(decimal!(300i32));
    /// let resp = ctx.replace_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        Ok(self
            .http_cli
            .request(Method::PUT, "/v1/trade/order")
            .body(Json(options))
            .response::<Json<EmptyResponse>>()
            .send()
            .await
            .map(|_| ())?)
    }

    /// Submit order
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/submit>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     decimal,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    ///     Config,
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     200,
    ///     TimeInForceType::Day,
    /// )
    /// .submitted_price(decimal!(50i32));
    /// let resp = ctx.submit_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn submit_order(&self, options: SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        Ok(self
            .http_cli
            .request(Method::POST, "/v1/trade/order")
            .body(Json(options))
            .response::<Json<_>>()
            .send()
            .await?
            .0)
    }

    /// Cancel order
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/withdraw>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{trade::TradeContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// ctx.cancel_order("709043056541253632").await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cancel_order(&self, order_id: impl Into<String>) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Request {
            order_id: String,
        }

        Ok(self
            .http_cli
            .request(Method::DELETE, "/v1/trade/order")
            .response::<Json<EmptyResponse>>()
            .query_params(Request {
                order_id: order_id.into(),
            })
            .send()
            .await
            .map(|_| ())?)
    }

    /// Get account balance
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/asset/account>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{trade::TradeContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/asset/account")
            .query_params(Request { currency })
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .list)
    }

    /// Get cash flow
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/asset/cashflow>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{GetCashFlowOptions, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/asset/cashflow")
            .query_params(options)
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .list)
    }

    /// Get fund positions
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/asset/fund>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{trade::TradeContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/asset/fund")
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<FundPositionsResponse>>()
            .send()
            .await?
            .0)
    }

    /// Get stock positions
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/asset/stock>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{trade::TradeContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/asset/stock")
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<StockPositionsResponse>>()
            .send()
            .await?
            .0)
    }

    /// Get margin ratio
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/asset/margin_ratio>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{trade::TradeContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/risk/margin-ratio")
            .query_params(Request {
                symbol: symbol.into(),
            })
            .response::<Json<MarginRatio>>()
            .send()
            .await?
            .0)
    }

    /// Get order detail
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/order_detail>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus, TradeContext},
    ///     Config, Market,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
    ///
    /// let resp = ctx.order_detail("701276261045858304").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn order_detail(&self, order_id: impl Into<String>) -> Result<OrderDetail> {
        #[derive(Debug, Serialize)]
        struct Request {
            order_id: String,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/order")
            .response::<Json<OrderDetail>>()
            .query_params(Request {
                order_id: order_id.into(),
            })
            .send()
            .await?
            .0)
    }

    /// Estimating the maximum purchase quantity for Hong Kong and US stocks,
    /// warrants, and options
    ///
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/order/estimate_available_buy_limit>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     trade::{EstimateMaxPurchaseQuantityOptions, OrderSide, OrderType, TradeContext},
    ///     Config,
    /// };
    /// use time::macros::datetime;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = TradeContext::try_new(config).await?;
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
            .http_cli
            .request(Method::GET, "/v1/trade/estimate/buy_limit")
            .query_params(opts)
            .response::<Json<EstimateMaxPurchaseQuantityResponse>>()
            .send()
            .await?
            .0)
    }
}
