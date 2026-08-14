use std::sync::Arc;

use napi::{Result, bindgen_prelude::*};

use crate::{
    config::Config,
    error::ErrorNewType,
    grid::{
        requests::{
            GetGridOrderDetailOptions, GetGridOrdersByIdsOptions, GetGridOrdersOptions,
            GetGridTriggerHistoryOptions, ReplaceGridOrderOptions, SubmitGridOrderOptions,
        },
        types::{
            GridOrder, GridOrderDetail, GridOrdersResponse, GridSymbolInfo,
            GridTriggerHistoryResponse, SubmitGridOrderResponse, TriggerOrder,
        },
    },
};

/// Grid trading context.
#[napi_derive::napi]
#[derive(Clone)]
pub struct GridContext {
    ctx: longbridge::grid::GridContext,
}

#[napi_derive::napi]
impl GridContext {
    /// Create a new `GridContext`.
    #[napi]
    pub fn new(config: &Config) -> GridContext {
        Self {
            ctx: longbridge::grid::GridContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Submit a grid trading order
    #[napi]
    pub fn submit<'env>(
        &self,
        env: &'env Env,
        opts: SubmitGridOrderOptions<'env>,
    ) -> Result<PromiseRaw<'env, SubmitGridOrderResponse>> {
        let ctx = self.ctx.clone();
        let opts = longbridge::grid::SubmitGridOrderOptions::from(opts);
        env.spawn_future(async move {
            let res = ctx.submit(opts).await.map_err(ErrorNewType)?;
            SubmitGridOrderResponse::try_from(res)
        })
    }

    /// Replace (modify) a grid trading order
    #[napi]
    pub fn replace<'env>(
        &self,
        env: &'env Env,
        opts: ReplaceGridOrderOptions<'env>,
    ) -> Result<PromiseRaw<'env, ()>> {
        let ctx = self.ctx.clone();
        let opts = longbridge::grid::ReplaceGridOrderOptions::from(opts);
        env.spawn_future(async move {
            ctx.replace(opts).await.map_err(ErrorNewType)?;
            Ok(())
        })
    }

    /// Get grid trading orders (paged list)
    #[napi]
    pub async fn list(&self, opts: Option<GetGridOrdersOptions>) -> Result<GridOrdersResponse> {
        let resp = self
            .ctx
            .list(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?;
        let grid_order = resp
            .grid_order
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<GridOrder>>>()?;
        Ok(GridOrdersResponse::new(grid_order, resp.has_more))
    }

    /// Query grid trading orders by IDs
    #[napi]
    pub async fn list_by_ids(&self, opts: GetGridOrdersByIdsOptions) -> Result<Vec<GridOrder>> {
        self.ctx
            .list_by_ids(opts.into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get grid trading order detail (and paged history)
    #[napi]
    pub async fn detail(&self, opts: GetGridOrderDetailOptions) -> Result<GridOrderDetail> {
        self.ctx
            .detail(opts.into())
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get grid trading trigger history
    #[napi]
    pub async fn trigger_history(
        &self,
        opts: GetGridTriggerHistoryOptions,
    ) -> Result<GridTriggerHistoryResponse> {
        let resp = self
            .ctx
            .trigger_history(opts.into())
            .await
            .map_err(ErrorNewType)?;
        let trigger_orders = resp
            .trigger_orders
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<TriggerOrder>>>()?;
        Ok(GridTriggerHistoryResponse::new(
            trigger_orders,
            resp.has_more,
        ))
    }

    /// Cancel a grid trading order
    #[napi]
    pub async fn cancel(&self, order_id: String) -> Result<()> {
        self.ctx.cancel(order_id).await.map_err(ErrorNewType)?;
        Ok(())
    }

    /// Suspend a grid trading order
    #[napi]
    pub async fn suspend(&self, order_id: String) -> Result<()> {
        self.ctx.suspend(order_id).await.map_err(ErrorNewType)?;
        Ok(())
    }

    /// Restart a grid trading order
    #[napi]
    pub async fn restart(&self, order_id: String) -> Result<()> {
        self.ctx.restart(order_id).await.map_err(ErrorNewType)?;
        Ok(())
    }

    /// Submit the strategy risk-disclosure questionnaire record (grid trading
    /// compliance authorization).
    #[napi]
    pub async fn submit_strategy_questionnaire(&self) -> Result<()> {
        self.ctx
            .submit_strategy_questionnaire(
                longbridge::grid::SubmitStrategyQuestionnaireOptions::new(),
            )
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get the security (symbol) info used to build a grid order (lot size,
    /// authorization flag, settlement currency, etc.).
    #[napi]
    pub async fn symbol_info(&self, symbol: String) -> Result<GridSymbolInfo> {
        self.ctx
            .symbol_info(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }
}
