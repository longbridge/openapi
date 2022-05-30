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

/// Trade context
#[napi_derive::napi]
#[derive(Clone)]
pub struct TradeContext {
    ctx: longbridge::trade::TradeContext,
    on_order_changed: JsCallback<PushOrderChanged>,
}

#[napi_derive::napi]
impl TradeContext {
    #[napi]
    pub async fn new(config: &Config) -> Result<TradeContext> {
        let (ctx, mut receiver) =
            longbridge::trade::TradeContext::try_new(Arc::new(config.0.clone()))
                .await
                .map_err(ErrorNewType)?;
        let js_ctx = Self {
            ctx,
            on_order_changed: Default::default(),
        };

        tokio::spawn({
            let js_ctx = js_ctx.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    match msg {
                        PushEvent::OrderChanged(order_changed) => {
                            if let Some(handler) = js_ctx.on_order_changed.lock().clone() {
                                if let Ok(order_changed) = order_changed.try_into() {
                                    handler.call(
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

        Ok(js_ctx)
    }

    #[napi(
        setter,
        ts_args_type = "callback: (err: null | Error, event: PushOrderChanged) => void"
    )]
    pub fn on_order_changed(&mut self, handler: JsFunction) -> Result<()> {
        *self.on_order_changed.lock() =
            Some(handler.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    /// Subscribe
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
    #[napi]
    pub async fn history_executions(
        &self,
        opts: Option<&GetHistoryExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        self.ctx
            .history_executions(opts.cloned().map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get today executions
    #[napi]
    pub async fn today_executions(
        &self,
        opts: Option<&GetTodayExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        self.ctx
            .today_executions(opts.cloned().map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get history orders
    #[napi]
    pub async fn history_orders(
        &self,
        opts: Option<&GetHistoryOrdersOptions>,
    ) -> Result<Vec<Order>> {
        self.ctx
            .history_orders(opts.cloned().map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get today orders
    #[napi]
    pub async fn today_orders(&self, opts: Option<&GetTodayOrdersOptions>) -> Result<Vec<Order>> {
        self.ctx
            .today_orders(opts.cloned().map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Replace order
    #[napi]
    pub async fn replace_order(&self, opts: &ReplaceOrderOptions) -> Result<()> {
        self.ctx
            .replace_order(opts.clone().into())
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Submit order
    #[napi]
    pub async fn submit_order(&self, opts: &SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        self.ctx
            .submit_order(opts.clone().into())
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Withdraw order
    #[napi]
    pub async fn withdraw_order(&self, order_id: String) -> Result<()> {
        self.ctx
            .withdraw_order(order_id)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get account balance
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
    #[napi]
    pub async fn cash_flow(&self, opts: &GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        self.ctx
            .cash_flow(opts.clone().into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()
    }

    /// Get fund positions
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
}
