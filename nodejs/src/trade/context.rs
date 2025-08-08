use std::sync::Arc;

use longport::trade::{GetFundPositionsOptions, GetStockPositionsOptions, PushEvent};
use napi::{Result, bindgen_prelude::*, threadsafe_function::ThreadsafeFunctionCallMode};
use parking_lot::Mutex;

use crate::{
    config::Config,
    error::ErrorNewType,
    trade::{
        requests::{
            EstimateMaxPurchaseQuantityOptions, GetCashFlowOptions, GetHistoryExecutionsOptions,
            GetHistoryOrdersOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions,
            ReplaceOrderOptions, SubmitOrderOptions,
        },
        types::{
            AccountBalance, CashFlow, EstimateMaxPurchaseQuantityResponse, Execution,
            FundPositionsResponse, MarginRatio, Order, OrderDetail, PushOrderChanged,
            StockPositionsResponse, SubmitOrderResponse, TopicType,
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
    ctx: longport::trade::TradeContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[napi_derive::napi]
impl TradeContext {
    #[napi]
    pub async fn new(config: &Config) -> Result<TradeContext> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let (ctx, mut receiver) =
            longport::trade::TradeContext::try_new(Arc::new(config.0.clone()))
                .await
                .map_err(ErrorNewType)?;

        tokio::spawn({
            let callbacks = callbacks.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    let callbacks = callbacks.lock();
                    match msg {
                        PushEvent::OrderChanged(order_changed) => {
                            if let Some(callback) = &callbacks.order_changed
                                && let Ok(order_changed) = order_changed.try_into()
                            {
                                callback
                                    .call(Ok(order_changed), ThreadsafeFunctionCallMode::Blocking);
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
    pub fn set_on_order_changed(&self, callback: Function<PushOrderChanged, ()>) -> Result<()> {
        self.callbacks.lock().order_changed = Some(
            callback
                .build_threadsafe_function()
                .callee_handled::<true>()
                .build()?,
        );
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
    /// } = require("longport");
    ///
    /// let config = Config.fromEnv();
    /// TradeContext.new(config)
    ///   .then((ctx) => {
    ///     ctx.setOnOrderChanged((_, event) => console.log(event.toString()));
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
    /// const { Config, TradeContext } = require("longport");
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
    /// const { Config, TradeContext } = require("longport");
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
    /// } = require("longport");
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
    /// } = require("longport");
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
    /// const { Config, TradeContext, Decimal } = require("longport");
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
    #[napi]
    pub fn replace_order<'env>(
        &self,
        env: &'env Env,
        opts: ReplaceOrderOptions<'env>,
    ) -> Result<PromiseRaw<'env, ()>> {
        let ctx = self.ctx.clone();
        let opts = longport::trade::ReplaceOrderOptions::from(opts);
        env.spawn_future(async move {
            ctx.replace_order(opts).await.map_err(ErrorNewType)?;
            Ok(())
        })
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
    /// } = require("longport");
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
    #[napi]
    pub fn submit_order<'env>(
        &self,
        env: &'env Env,
        opts: SubmitOrderOptions<'env>,
    ) -> Result<PromiseRaw<'env, SubmitOrderResponse>> {
        let ctx = self.ctx.clone();
        let opts = longport::trade::SubmitOrderOptions::from(opts);
        env.spawn_future(async move {
            let res = ctx.submit_order(opts).await.map_err(ErrorNewType)?;
            SubmitOrderResponse::try_from(res)
        })
    }

    /// Cancel order
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longport");
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
    /// const { Config, TradeContext } = require("longport");
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
    pub async fn account_balance(&self, currency: Option<String>) -> Result<Vec<AccountBalance>> {
        self.ctx
            .account_balance(currency.as_deref())
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
    /// const { Config, TradeContext, GetCashFlowOptions } = require("longport");
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
    /// const { Config, TradeContext } = require("longport")
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
    /// const { Config, TradeContext } = require("longport")
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
    /// const { Config, TradeContext } = require("longport")
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

    /// Get order detail
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.orderDetail("701276261045858304"))
    ///   .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn order_detail(&self, order_id: String) -> Result<OrderDetail> {
        self.ctx
            .order_detail(order_id)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Estimating the maximum purchase quantity for Hong Kong and US stocks,
    /// warrants, and options
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext, OrderType, OrderSide } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// TradeContext.new(config)
    ///   .then((ctx) => ctx.estimateMaxPurchaseQuantity({
    ///     symbol: "700.HK",
    ///     orderType: OrderType.LO,
    ///     side: OrderSide.Buy,
    ///   }))
    ///   .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub fn estimate_max_purchase_quantity<'env>(
        &self,
        env: &'env Env,
        opts: EstimateMaxPurchaseQuantityOptions<'env>,
    ) -> Result<PromiseRaw<'env, EstimateMaxPurchaseQuantityResponse>> {
        let ctx = self.ctx.clone();
        let opts = longport::trade::EstimateMaxPurchaseQuantityOptions::from(opts);
        env.spawn_future(async move {
            let res = ctx
                .estimate_max_purchase_quantity(opts)
                .await
                .map_err(ErrorNewType)?;
            EstimateMaxPurchaseQuantityResponse::try_from(res)
        })
    }
}
