use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Method};
use longbridge_wscli::WsClientError;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::{
    trade::{
        core::{Command, Core},
        AccountBalance, CashFlow, Execution, FundPositionsResponse, GetCashFlowOptions,
        GetFundPositionsOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
        GetStockPositionsOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions, Order,
        PushEvent, ReplaceOrderOptions, StockPositionsResponse, SubmitOrderOptions, TopicType,
    },
    Config, Result,
};

/// Response for withdraw order request
#[derive(Debug, Deserialize)]
pub struct SubmitOrderResponse {
    /// Order id
    pub order_id: String,
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
            .response::<Response>()
            .send()
            .await?
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
            .response::<Response>()
            .send()
            .await?
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
            .response::<Response>()
            .send()
            .await?
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
            .response::<Response>()
            .send()
            .await?
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
    /// let opts =
    ///     ReplaceOrderOptions::new("709043056541253632", decimal!(100i32)).price(decimal!(300i32));
    /// let resp = ctx.replace_order(opts).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        Ok(self
            .http_cli
            .request(Method::PUT, "/v1/trade/order")
            .body(options)
            .send()
            .await?)
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
    ///     decimal!(200i32),
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
            .body(options)
            .response::<_>()
            .send()
            .await?)
    }

    /// Withdraw order
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
    /// ctx.withdraw_order("709043056541253632").await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn withdraw_order(&self, order_id: impl Into<String>) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Request {
            order_id: String,
        }

        Ok(self
            .http_cli
            .request(Method::DELETE, "/v1/trade/order")
            .query_params(Request {
                order_id: order_id.into(),
            })
            .send()
            .await?)
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
    /// let resp = ctx.account_balance().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<AccountBalance>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/asset/account")
            .response::<Response>()
            .send()
            .await?
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
            .response::<Response>()
            .send()
            .await?
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
            .response::<FundPositionsResponse>()
            .send()
            .await?)
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
            .response::<StockPositionsResponse>()
            .send()
            .await?)
    }
}
