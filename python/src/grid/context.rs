use std::sync::Arc;

use longbridge::{
    blocking::GridContextSync,
    grid::{
        GetGridOrderDetailOptions, GetGridOrdersByIdsOptions, GetGridOrdersOptions,
        GetGridTriggerHistoryOptions, ReplaceGridOrderOptions, SubmitGridOrderOptions,
    },
};
use pyo3::{PyResult, pyclass, pymethods};

use crate::{
    config::Config,
    error::ErrorNewType,
    grid::types::{
        GridOrder, GridOrderDetail, GridOrdersResponse, GridSymbolInfo, GridTradeRule,
        GridTriggerHistoryResponse, SubmitGridOrderResponse, TriggerOrder,
    },
    types::Market,
};

/// Grid trading management context (REST-only).
#[pyclass]
pub(crate) struct GridContext {
    ctx: GridContextSync,
}

#[pymethods]
impl GridContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: GridContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Submit a grid trading order
    fn submit(
        &self,
        symbol: String,
        settlement_currency: String,
        grid_trading_rule: GridTradeRule,
    ) -> PyResult<SubmitGridOrderResponse> {
        let opts = SubmitGridOrderOptions::new(symbol, settlement_currency, grid_trading_rule.0);
        self.ctx.submit(opts).map_err(ErrorNewType)?.try_into()
    }

    /// Replace (modify) a grid trading order
    fn replace(&self, order_id: String, grid_trading_rule: GridTradeRule) -> PyResult<()> {
        let opts = ReplaceGridOrderOptions::new(order_id, grid_trading_rule.0);
        self.ctx.replace(opts).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get grid trading orders (paged list)
    #[pyo3(signature = (page = None, limit = None, market = None, status = None, symbol = None, sort_by = None, sort_order = None))]
    #[allow(clippy::too_many_arguments)]
    fn list(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        market: Option<Market>,
        status: Option<String>,
        symbol: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> PyResult<GridOrdersResponse> {
        let mut opts = GetGridOrdersOptions::new();
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }
        if let Some(market) = market {
            opts = opts.market(market.into());
        }
        if let Some(status) = status {
            opts = opts.status(status);
        }
        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }
        if let Some(sort_by) = sort_by {
            opts = opts.sort_by(sort_by);
        }
        if let Some(sort_order) = sort_order {
            opts = opts.sort_order(sort_order);
        }
        let resp = self.ctx.list(opts).map_err(ErrorNewType)?;
        let grid_order = resp
            .grid_order
            .into_iter()
            .map(TryInto::try_into)
            .collect::<PyResult<Vec<GridOrder>>>()?;
        Ok(GridOrdersResponse {
            grid_order,
            has_more: resp.has_more,
        })
    }

    /// Query grid trading orders by IDs
    fn list_by_ids(&self, order_ids: Vec<String>) -> PyResult<Vec<GridOrder>> {
        let opts = GetGridOrdersByIdsOptions::new(order_ids);
        self.ctx
            .list_by_ids(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get grid trading order detail (and paged history)
    #[pyo3(signature = (order_id, history_id = None, limit = None))]
    fn detail(
        &self,
        order_id: String,
        history_id: Option<String>,
        limit: Option<i32>,
    ) -> PyResult<GridOrderDetail> {
        let mut opts = GetGridOrderDetailOptions::new(order_id);
        if let Some(history_id) = history_id {
            opts = opts.history_id(history_id);
        }
        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }
        self.ctx.detail(opts).map_err(ErrorNewType)?.try_into()
    }

    /// Get grid trading trigger history
    #[pyo3(signature = (grid_order_id, page = None, limit = None))]
    fn trigger_history(
        &self,
        grid_order_id: String,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<GridTriggerHistoryResponse> {
        let mut opts = GetGridTriggerHistoryOptions::new(grid_order_id);
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }
        let resp = self.ctx.trigger_history(opts).map_err(ErrorNewType)?;
        let trigger_orders = resp
            .trigger_orders
            .into_iter()
            .map(TryInto::try_into)
            .collect::<PyResult<Vec<TriggerOrder>>>()?;
        Ok(GridTriggerHistoryResponse {
            trigger_orders,
            has_more: resp.has_more,
        })
    }

    /// Cancel a grid trading order
    fn cancel(&self, order_id: String) -> PyResult<()> {
        self.ctx.cancel(order_id).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Suspend a grid trading order
    fn suspend(&self, order_id: String) -> PyResult<()> {
        self.ctx.suspend(order_id).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Restart a grid trading order
    fn restart(&self, order_id: String) -> PyResult<()> {
        self.ctx.restart(order_id).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get the security (symbol) info used to build a grid order (lot size,
    /// authorization flag, settlement currency, etc.).
    fn symbol_info(&self, symbol: String) -> PyResult<GridSymbolInfo> {
        self.ctx
            .symbol_info(symbol)
            .map_err(ErrorNewType)?
            .try_into()
    }
}
