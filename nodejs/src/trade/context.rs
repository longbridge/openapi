use std::sync::Arc;

use longbridge::trade::{GetFundPositionsOptions, GetStockPositionsOptions, PushEvent};
use napi::{threadsafe_function::ThreadsafeFunctionCallMode, Env, JsFunction, JsObject, Result};
use parking_lot::Mutex;

use crate::{
    config::Config,
    error::ErrorNewType,
    trade::{
        requests::{
            GetCashFlowOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
            GetTodayExecutionsOptions, GetTodayOrdersOptions, ReplaceOrderOptions,
            SubmitOrderOptions,
        },
        types::{
            AccountBalance, CashFlow, Execution, FundPositionsResponse, MarginRatio, Order,
            PushOrderChanged, StockPositionsResponse, SubmitOrderResponse, TopicType,
        },
    },
    utils::JsCallback,
};

#[derive(Default)]
struct Callbacks {
    order_changed: Option<JsCallback<PushOrderChanged>>,
}

/// Trade context
#[napi_derive::napi]
#[derive(Clone)]
pub struct TradeContext {
    ctx: longbridge::trade::TradeContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[napi_derive::napi]
impl TradeContext {
    #[napi]
    pub async fn new(config: &Config) -> Result<TradeContext> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let (ctx, mut receiver) =
            longbridge::trade::TradeContext::try_new(Arc::new(config.0.clone()))
                .await
                .map_err(ErrorNewType)?;

        tokio::spawn({
            let callbacks = callbacks.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    let callbacks = callbacks.lock();
                    match msg {
                        PushEvent::OrderChanged(order_changed) => {
                            if let Some(callback) = &callbacks.order_changed {
                                if let Ok(order_changed) = order_changed.try_into() {
                                    callback.call(
                                        Ok(order_changed),
                                        ThreadsafeFunctionCallMode::Blocking,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(TradeContext { ctx, callbacks })
    }

    /// Set order changed callback, after receiving the order changed event, it
    /// will call back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushOrderChanged) => void")]
    pub fn set_on_order_changed(&self, callback: JsFunction) -> Result<()> {
        self.callbacks.lock().order_changed =
            Some(callback.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    /// Subscribe
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const {
    ///  Config,
    ///  TradeContext,
    ///  Decimal,
    ///  OrderSide,
    ///  TimeInForceType,
    ///  OrderType,
    ///  TopicType,
    /// } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) => {
    ///     ctx.setOnQuote((_, event) => console.log(event.toString()));
    ///     ctx.subscribe([TopicType.Private]);
    ///     return ctx.submitOrder({
    ///       symbol: "700.HK",
    ///       orderType: OrderType.LO,
    ///       side: OrderSide.Buy,
    ///       timeInForce: TimeInForceType.Day,
    ///       submittedPrice: new Decimal("50"),
    ///       submittedQuantity: 200,
    ///     });
    ///   })
    ///   .then((resp) => console.log(resp.toString()));
    /// ```
    #[napi]
    pub async fn subscribe(&self, topics: Vec<TopicType>) -> Result<()> {
        self.ctx
            .subscribe(topics.into_iter().map(Into::into))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    #[napi]
    pub async fn unsubscribe(&self, topics: Vec<TopicType>) -> Result<()> {
        self.ctx
            .unsubscribe(topics.into_iter().map(Into::into))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get history executions
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) =>
    ///     ctx.historyExecutions({
    ///       symbol: "700.HK",
    ///       startAt: new Date(2022, 5, 9),
    ///       endAt: new Date(2022, 5, 12),
    ///     })
    ///   )
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi]
    pub async fn history_executions(
        &self,
        opts: Option<GetHistoryExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        self.ctx
            .history_executions(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get today executions
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.todayExecutions({ symbol: "700.HK" }))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi]
    pub async fn today_executions(
        &self,
        opts: Option<GetTodayExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        self.ctx
            .today_executions(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get history orders
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const {
    ///   Config,
    ///   TradeContext,
    ///   OrderStatus,
    ///   OrderSide,
    ///   Market,
    /// } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) =>
    ///     ctx.historyOrders({
    ///       symbol: "700.HK",
    ///       status: [OrderStatus.Filled, OrderStatus.New],
    ///       side: OrderSide.Buy,
    ///       market: Market.HK,
    ///       startAt: new Date(2022, 5, 9),
    ///       endAt: new Date(2022, 5, 12),
    ///     })
    ///   )
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi]
    pub async fn history_orders(
        &self,
        opts: Option<GetHistoryOrdersOptions>,
    ) -> Result<Vec<Order>> {
        self.ctx
            .history_orders(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get today orders
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const {
    ///   Config,
    ///   TradeContext,
    ///   OrderStatus,
    ///   OrderSide,
    ///   Market,
    /// } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) =>
    ///     ctx.todayOrders({
    ///       symbol: "700.HK",
    ///       status: [OrderStatus.Filled, OrderStatus.New],
    ///       side: OrderSide.Buy,
    ///       market: Market.HK,
    ///     })
    ///   )
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi]
    pub async fn today_orders(&self, opts: Option<GetTodayOrdersOptions>) -> Result<Vec<Order>> {
        self.ctx
            .today_orders(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Replace order
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext, Decimal } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) =>
    ///     ctx.replaceOrder({
    ///       orderId: "709043056541253632",
    ///       quantity: 100,
    ///       price: new Decimal("300"),
    ///     })
    ///   )
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi(ts_return_type = "Promise<void>")]
    pub fn replace_order(&self, env: Env, opts: ReplaceOrderOptions) -> Result<JsObject> {
        let opts: longbridge::trade::ReplaceOrderOptions = opts.into();
        let ctx = self.ctx.clone();
        env.execute_tokio_future(
            async move {
                ctx.replace_order(opts)
                    .await
                    .map_err(ErrorNewType)
                    .map_err(napi::Error::from)
            },
            |_, _| Ok(()),
        )
    }

    /// Submit order
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const {
    ///   Config,
    ///   TradeContext,
    ///   OrderType,
    ///   OrderSide,
    ///   Decimal,
    ///   TimeInForceType,
    /// } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) =>
    ///     ctx.submitOrder({
    ///       symbol: "700.HK",
    ///       orderType: OrderType.LO,
    ///       side: OrderSide.Buy,
    ///       timeInForce: TimeInForceType.Day,
    ///       submittedQuantity: 200,
    ///       submittedPrice: new Decimal("300"),
    ///     })
    ///   )
    ///   .then((resp) => console.log(resp.toString()));
    /// ```
    #[napi(ts_return_type = "Promise<SubmitOrderResponse>")]
    pub fn submit_order(&self, env: Env, opts: SubmitOrderOptions) -> Result<JsObject> {
        let opts: longbridge::trade::SubmitOrderOptions = opts.into();
        let ctx = self.ctx.clone();
        env.execute_tokio_future(
            async move {
                let res = ctx
                    .submit_order(opts)
                    .await
                    .map_err(ErrorNewType)
                    .map_err(napi::Error::from)?;
                res.try_into()
            },
            |_, res: SubmitOrderResponse| Ok(res),
        )
    }

    /// Cancel order
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config).then((ctx) => ctx.cancelOrder("709043056541253632"));
    /// ```
    #[napi]
    pub async fn cancel_order(&self, order_id: String) -> Result<()> {
        self.ctx
            .cancel_order(order_id)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get account balance
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.accountBalance())
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi]
    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        self.ctx
            .account_balance()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()
    }

    /// Get cash flow
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext, GetCashFlowOptions } = require("longbridge");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) =>
    ///     ctx.cashFlow({
    ///       startAt: new Date(2022, 5, 9),
    ///       endAt: new Date(2022, 5, 12),
    ///     })
    ///   )
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   });
    /// ```
    #[napi]
    pub async fn cash_flow(&self, opts: GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        self.ctx
            .cash_flow(opts.into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()
    }

    /// Get fund positions
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge")
    ///
    /// let config = Config.fromEnv()
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.fundPositions())
    ///   .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn fund_positions(
        &self,
        symbols: Option<Vec<String>>,
    ) -> Result<FundPositionsResponse> {
        self.ctx
            .fund_positions(GetFundPositionsOptions::new().symbols(symbols.unwrap_or_default()))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get stock positions
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge")
    ///
    /// let config = Config.fromEnv()
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.stockPositions())
    ///   .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn stock_positions(
        &self,
        symbols: Option<Vec<String>>,
    ) -> Result<StockPositionsResponse> {
        self.ctx
            .stock_positions(GetStockPositionsOptions::new().symbols(symbols.unwrap_or_default()))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get margin ratio
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longbridge")
    ///
    /// let config = Config.fromEnv()
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.marginRatio("700.HK"))
    ///   .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn margin_ratio(&self, symbol: String) -> Result<MarginRatio> {
        self.ctx
            .margin_ratio(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }
}
