use std::sync::Arc;

use longbridge::trade::{GetFundPositionsOptions, GetStockPositionsOptions, PushEvent};
use napi::{threadsafe_function::ThreadsafeFunctionCallMode, JsFunction, Result};

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
            AccountBalance, CashFlow, Execution, FundPositionsResponse, Order, PushOrderChanged,
            StockPositionsResponse, SubmitOrderResponse, TopicType,
        },
    },
    utils::JsCallback,
};

type TradeCallback = JsCallback<PushOrderChanged>;

/// Trade context
#[napi_derive::napi]
#[derive(Clone)]
pub struct TradeContext {
    config: longbridge::Config,
    ctx: Option<longbridge::trade::TradeContext>,
    on_push: Option<TradeCallback>,
}

#[napi_derive::napi]
impl TradeContext {
    #[napi(
        constructor,
        ts_args_type = "callback: (err: null | Error, event: PushOrderChanged) => void"
    )]
    pub fn new(config: &Config, on_push: Option<JsFunction>) -> Result<TradeContext> {
        Ok(TradeContext {
            config: config.0.clone(),
            ctx: None,
            on_push: on_push
                .map(|on_push| on_push.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value])))
                .transpose()?,
        })
    }

    /// Open trade context
    #[napi]
    pub async fn open(&mut self) -> Result<()> {
        check_ctx_exists!(self.ctx);

        let (ctx, mut receiver) =
            longbridge::trade::TradeContext::try_new(Arc::new(self.config.clone()))
                .await
                .map_err(ErrorNewType)?;
        self.ctx = Some(ctx);

        let handler = self.on_push.take();
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                match msg {
                    PushEvent::OrderChanged(order_changed) => {
                        if let Some(handler) = &handler {
                            if let Ok(order_changed) = order_changed.try_into() {
                                handler
                                    .call(Ok(order_changed), ThreadsafeFunctionCallMode::Blocking);
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Subscribe
    #[napi]
    pub async fn subscribe(&self, topics: Vec<TopicType>) -> Result<()> {
        get_ctx!(self.ctx)
            .subscribe(topics.into_iter().map(Into::into))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    #[napi]
    pub async fn unsubscribe(&self, topics: Vec<TopicType>) -> Result<()> {
        get_ctx!(self.ctx)
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
    /// const { Config, TradeContext, GetHistoryExecutionsOptions } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// let opts = new GetHistoryExecutionsOptions()
    ///     .symbol("700.HK")
    ///     .startAt(new Date(2022, 5, 9))
    ///     .endAt(new Date(2022, 5, 12))
    /// ctx.open()
    ///     .then(() => ctx.historyExecutions(opts))
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }    
    ///     })
    /// )
    /// ```
    #[napi]
    pub async fn history_executions(
        &self,
        opts: Option<&GetHistoryExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        get_ctx!(self.ctx)
            .history_executions(opts.cloned().map(Into::into))
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
    /// const { Config, TradeContext, GetTodayExecutionsOptions } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// ctx.open()
    ///     .then(() => ctx.todayExecutions(new GetTodayExecutionsOptions().symbol("700.HK")))
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }    
    ///     })
    /// ```
    #[napi]
    pub async fn today_executions(
        &self,
        opts: Option<&GetTodayExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        get_ctx!(self.ctx)
            .today_executions(opts.cloned().map(Into::into))
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
    /// const { Config, TradeContext, GetHistoryOrdersOptions, OrderStatus, OrderSide, Market } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// let opts = new GetHistoryOrdersOptions()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus.Filled, OrderStatus.New])
    ///     .side(OrderSide.Buy)
    ///     .market(Market.HK)
    ///     .startAt(2022, 5, 9)
    ///     .endAt(2022, 5, 12)
    /// ctx.open()
    ///     .then(() => ctx.historyOrders(opts))
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }
    ///     })
    /// ```
    #[napi]
    pub async fn history_orders(
        &self,
        opts: Option<&GetHistoryOrdersOptions>,
    ) -> Result<Vec<Order>> {
        get_ctx!(self.ctx)
            .history_orders(opts.cloned().map(Into::into))
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
    /// const { Config, TradeContext, GetTodayOrdersOptions, OrderStatus, OrderSide, Market } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// let opts = new GetTodayOrdersOptions()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus.Filled, OrderStatus.New])
    ///     .side(OrderSide.Buy)
    ///     .market(Market.HK)
    /// ctx.open()
    ///     .then(() => ctx.todayOrders(opts))
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }
    ///     })
    /// )
    /// ```
    #[napi]
    pub async fn today_orders(&self, opts: Option<&GetTodayOrdersOptions>) -> Result<Vec<Order>> {
        get_ctx!(self.ctx)
            .today_orders(opts.cloned().map(Into::into))
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
    /// const { Config, TradeContext, ReplaceOrderOptions, Decimal } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// ctx.open()
    ///     .then(() => ctx.replaceOrder(new ReplaceOrderOptions("700.HK", new Decimal("100")).price(new Decimal("300"))))
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }
    ///     })
    /// ```
    #[napi]
    pub async fn replace_order(&self, opts: &ReplaceOrderOptions) -> Result<()> {
        get_ctx!(self.ctx)
            .replace_order(opts.clone().into())
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Submit order
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext, SubmitOrderOptions, OrderType, OrderSide, Decimal, TimeInForceType } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// let opts = new SubmitOrderOptions("700.HK", OrderType.LO, OrderSide.Buy, new Decimal("200"), TimeInForceType.Day)
    ///     .submittedPrice(new Decimal("300"));
    /// ctx.open()
    ///     .then(() => ctx.submitOrder(opts))
    ///     .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn submit_order(&self, opts: &SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        get_ctx!(self.ctx)
            .submit_order(opts.clone().into())
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Withdraw order
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// ctx.open()
    ///     .then(() => ctx.withdrawOrder("709043056541253632"))
    /// ```
    #[napi]
    pub async fn withdraw_order(&self, order_id: String) -> Result<()> {
        get_ctx!(self.ctx)
            .withdraw_order(order_id)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get account balance
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, TradeContext } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// ctx.open()
    ///     .then(() => ctx.accountBalance())
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }    
    ///     })
    /// ```
    #[napi]
    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        get_ctx!(self.ctx)
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
    /// const { Config, TradeContext, GetCashFlowOptions } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// ctx.open()
    ///     .then(() => ctx.cashFlow(new GetCashFlowOptions(new Date(2022, 5, 9), new Date(2022, 5, 12))))
    ///     .then((resp) => {
    ///         for (let obj of resp) {
    ///             console.log(obj.toString())
    ///         }
    ///     })
    /// ```
    #[napi]
    pub async fn cash_flow(&self, opts: &GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        get_ctx!(self.ctx)
            .cash_flow(opts.clone().into())
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
    /// const { Config, TradeContext } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    ///
    /// ctx.open()
    ///     .then(() => ctx.fundPositions())
    ///     .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn fund_positions(
        &self,
        symbols: Option<Vec<String>>,
    ) -> Result<FundPositionsResponse> {
        get_ctx!(self.ctx)
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
    /// const { Config, TradeContext } = require('longbridge')
    ///
    /// let config = Config.fromEnv()
    /// let ctx = new TradeContext(config)
    /// ctx.open()
    ///     .then(() => ctx.stockPositions())
    ///     .then((resp) => console.log(resp))
    /// ```
    #[napi]
    pub async fn stock_positions(
        &self,
        symbols: Option<Vec<String>>,
    ) -> Result<StockPositionsResponse> {
        get_ctx!(self.ctx)
            .stock_positions(GetStockPositionsOptions::new().symbols(symbols.unwrap_or_default()))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }
}
