//! Async trade context backed by longbridge's native async API.

use std::sync::Arc;

use longbridge::trade::{
    EstimateMaxPurchaseQuantityOptions, GetCashFlowOptions, GetFundPositionsOptions,
    GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetStockPositionsOptions,
    GetTodayExecutionsOptions, GetTodayOrdersOptions, ReplaceOrderOptions, SubmitOrderOptions,
    TradeContext,
};
use parking_lot::Mutex;
use pyo3::{prelude::*, types::PyType};

use crate::{
    config::Config,
    decimal::PyDecimal,
    error::ErrorNewType,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper},
    trade::{
        context::Callbacks,
        push::handle_push_event,
        types::{
            AccountBalance, BalanceType, CashFlow, EstimateMaxPurchaseQuantityResponse, Execution,
            FundPositionsResponse, MarginRatio, Order, OrderDetail, OrderSide, OrderStatus,
            OrderType, OutsideRTH, StockPositionsResponse, SubmitOrderResponse, TimeInForceType,
            TopicType,
        },
    },
    types::Market,
};

/// Async trade context. Create via `AsyncTradeContext.create(config)` and await
/// in asyncio.
#[pyclass]
pub(crate) struct AsyncTradeContext {
    ctx: Arc<TradeContext>,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[pymethods]
impl AsyncTradeContext {
    /// Create an async trade context. Returns an awaitable; must be awaited
    /// inside asyncio.
    #[classmethod]
    fn create(cls: &Bound<PyType>, config: &Config) -> PyResult<Py<PyAny>> {
        let py = cls.py();
        let config = Arc::new(config.0.clone());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (ctx, mut event_rx) = TradeContext::try_new(config).await.map_err(ErrorNewType)?;
            let callbacks = Arc::new(Mutex::new(Callbacks::default()));
            let callbacks_clone = callbacks.clone();
            pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                while let Some(event) = event_rx.recv().await {
                    handle_push_event(&callbacks_clone.lock(), event);
                }
            });
            Ok(AsyncTradeContext {
                ctx: Arc::new(ctx),
                callbacks,
            })
        })
        .map(|b| b.unbind())
    }

    /// Set order changed callback.
    fn set_on_order_changed(&self, py: Python<'_>, callback: Py<PyAny>) {
        if callback.is_none(py) {
            self.callbacks.lock().order_changed = None;
        } else {
            self.callbacks.lock().order_changed = Some(callback);
        }
    }

    /// Subscribe. Returns awaitable.
    fn subscribe(&self, py: Python<'_>, topics: Vec<TopicType>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let topics: Vec<longbridge::trade::TopicType> =
            topics.into_iter().map(Into::into).collect();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.subscribe(topics).await.map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Unsubscribe. Returns awaitable.
    fn unsubscribe(&self, py: Python<'_>, topics: Vec<TopicType>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let topics: Vec<longbridge::trade::TopicType> =
            topics.into_iter().map(Into::into).collect();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.unsubscribe(topics).await.map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Get history executions. Returns awaitable.
    #[pyo3(signature = (symbol = None, start_at = None, end_at = None))]
    fn history_executions(
        &self,
        py: Python<'_>,
        symbol: Option<String>,
        start_at: Option<PyOffsetDateTimeWrapper>,
        end_at: Option<PyOffsetDateTimeWrapper>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = GetHistoryExecutionsOptions::new();
        if let Some(s) = symbol {
            opts = opts.symbol(s);
        }
        if let Some(s) = start_at {
            opts = opts.start_at(s.0);
        }
        if let Some(e) = end_at {
            opts = opts.end_at(e.0);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .history_executions(Some(opts))
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Execution> { x.try_into() })
                .collect::<PyResult<Vec<Execution>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get today executions. Returns awaitable.
    #[pyo3(signature = (symbol = None, order_id = None))]
    fn today_executions(
        &self,
        py: Python<'_>,
        symbol: Option<String>,
        order_id: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = GetTodayExecutionsOptions::new();
        if let Some(s) = symbol {
            opts = opts.symbol(s);
        }
        if let Some(o) = order_id {
            opts = opts.order_id(o);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .today_executions(Some(opts))
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Execution> { x.try_into() })
                .collect::<PyResult<Vec<Execution>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get history orders. Returns awaitable.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (symbol = None, status = None, side = None, market = None, start_at = None, end_at = None))]
    fn history_orders(
        &self,
        py: Python<'_>,
        symbol: Option<String>,
        status: Option<Vec<OrderStatus>>,
        side: Option<OrderSide>,
        market: Option<Market>,
        start_at: Option<PyOffsetDateTimeWrapper>,
        end_at: Option<PyOffsetDateTimeWrapper>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = GetHistoryOrdersOptions::new();
        if let Some(s) = symbol {
            opts = opts.symbol(s);
        }
        opts = opts.status(status.unwrap_or_default().into_iter().map(Into::into));
        if let Some(s) = side {
            opts = opts.side(s.into());
        }
        if let Some(m) = market {
            opts = opts.market(m.into());
        }
        if let Some(s) = start_at {
            opts = opts.start_at(s.0);
        }
        if let Some(e) = end_at {
            opts = opts.end_at(e.0);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.history_orders(Some(opts)).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Order> { x.try_into() })
                .collect::<PyResult<Vec<Order>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get today orders. Returns awaitable.
    #[pyo3(signature = (symbol = None, status = None, side = None, market = None, order_id = None))]
    fn today_orders(
        &self,
        py: Python<'_>,
        symbol: Option<String>,
        status: Option<Vec<OrderStatus>>,
        side: Option<OrderSide>,
        market: Option<Market>,
        order_id: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = GetTodayOrdersOptions::new();
        if let Some(s) = symbol {
            opts = opts.symbol(s);
        }
        opts = opts.status(status.unwrap_or_default().into_iter().map(Into::into));
        if let Some(s) = side {
            opts = opts.side(s.into());
        }
        if let Some(m) = market {
            opts = opts.market(m.into());
        }
        if let Some(o) = order_id {
            opts = opts.order_id(o);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.today_orders(Some(opts)).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Order> { x.try_into() })
                .collect::<PyResult<Vec<Order>>>()
        })
        .map(|b| b.unbind())
    }

    /// Replace order. Returns awaitable.
    #[pyo3(signature = (order_id, quantity, price = None, trigger_price = None, limit_offset = None, trailing_amount = None, trailing_percent = None, limit_depth_level = None, trigger_count = None, monitor_price = None, remark = None))]
    #[allow(clippy::too_many_arguments)]
    fn replace_order(
        &self,
        py: Python<'_>,
        order_id: String,
        quantity: PyDecimal,
        price: Option<PyDecimal>,
        trigger_price: Option<PyDecimal>,
        limit_offset: Option<PyDecimal>,
        trailing_amount: Option<PyDecimal>,
        trailing_percent: Option<PyDecimal>,
        limit_depth_level: Option<i32>,
        trigger_count: Option<i32>,
        monitor_price: Option<PyDecimal>,
        remark: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = ReplaceOrderOptions::new(order_id, quantity.into());
        if let Some(p) = price {
            opts = opts.price(p.into());
        }
        if let Some(p) = trigger_price {
            opts = opts.trigger_price(p.into());
        }
        if let Some(p) = limit_offset {
            opts = opts.limit_offset(p.into());
        }
        if let Some(p) = trailing_amount {
            opts = opts.trailing_amount(p.into());
        }
        if let Some(p) = trailing_percent {
            opts = opts.trailing_percent(p.into());
        }
        if let Some(l) = limit_depth_level {
            opts = opts.limit_depth_level(l);
        }
        if let Some(c) = trigger_count {
            opts = opts.trigger_count(c);
        }
        if let Some(p) = monitor_price {
            opts = opts.monitor_price(p.into());
        }
        if let Some(r) = remark {
            opts = opts.remark(r);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.replace_order(opts).await.map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Submit order. Returns awaitable.
    #[pyo3(signature = (symbol, order_type, side, submitted_quantity, time_in_force, submitted_price = None, trigger_price = None, limit_offset = None, trailing_amount = None, trailing_percent = None, expire_date = None, outside_rth = None, limit_depth_level = None, trigger_count = None, monitor_price = None, remark = None))]
    #[allow(clippy::too_many_arguments)]
    fn submit_order(
        &self,
        py: Python<'_>,
        symbol: String,
        order_type: OrderType,
        side: OrderSide,
        submitted_quantity: PyDecimal,
        time_in_force: TimeInForceType,
        submitted_price: Option<PyDecimal>,
        trigger_price: Option<PyDecimal>,
        limit_offset: Option<PyDecimal>,
        trailing_amount: Option<PyDecimal>,
        trailing_percent: Option<PyDecimal>,
        expire_date: Option<PyDateWrapper>,
        outside_rth: Option<OutsideRTH>,
        limit_depth_level: Option<i32>,
        trigger_count: Option<i32>,
        monitor_price: Option<PyDecimal>,
        remark: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = SubmitOrderOptions::new(
            symbol,
            order_type.into(),
            side.into(),
            submitted_quantity.into(),
            time_in_force.into(),
        );
        if let Some(p) = submitted_price {
            opts = opts.submitted_price(p.into());
        }
        if let Some(p) = trigger_price {
            opts = opts.trigger_price(p.into());
        }
        if let Some(p) = limit_offset {
            opts = opts.limit_offset(p.into());
        }
        if let Some(p) = trailing_amount {
            opts = opts.trailing_amount(p.into());
        }
        if let Some(p) = trailing_percent {
            opts = opts.trailing_percent(p.into());
        }
        if let Some(d) = expire_date {
            opts = opts.expire_date(d.0);
        }
        if let Some(o) = outside_rth {
            opts = opts.outside_rth(o.into());
        }
        if let Some(l) = limit_depth_level {
            opts = opts.limit_depth_level(l);
        }
        if let Some(c) = trigger_count {
            opts = opts.trigger_count(c);
        }
        if let Some(p) = monitor_price {
            opts = opts.monitor_price(p.into());
        }
        if let Some(r) = remark {
            opts = opts.remark(r);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: SubmitOrderResponse = ctx
                .submit_order(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Cancel order. Returns awaitable.
    fn cancel_order(&self, py: Python<'_>, order_id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.cancel_order(order_id).await.map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Get account balance. Returns awaitable.
    #[pyo3(signature = (currency = None))]
    fn account_balance(&self, py: Python<'_>, currency: Option<String>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .account_balance(currency.as_deref())
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<AccountBalance> { x.try_into() })
                .collect::<PyResult<Vec<AccountBalance>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get cash flow. Returns awaitable.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (start_at, end_at, business_type = None, symbol = None, page = None, size = None))]
    fn cash_flow(
        &self,
        py: Python<'_>,
        start_at: PyOffsetDateTimeWrapper,
        end_at: PyOffsetDateTimeWrapper,
        business_type: Option<BalanceType>,
        symbol: Option<String>,
        page: Option<usize>,
        size: Option<usize>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts = GetCashFlowOptions::new(start_at.0, end_at.0);
        if let Some(b) = business_type {
            opts = opts.business_type(b.into());
        }
        if let Some(s) = symbol {
            opts = opts.symbol(s);
        }
        if let Some(p) = page {
            opts = opts.page(p);
        }
        if let Some(s) = size {
            opts = opts.size(s);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.cash_flow(opts).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<CashFlow> { x.try_into() })
                .collect::<PyResult<Vec<CashFlow>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get fund positions. Returns awaitable.
    #[pyo3(signature = (symbols = None))]
    fn fund_positions(&self, py: Python<'_>, symbols: Option<Vec<String>>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let opts = GetFundPositionsOptions::new().symbols(symbols.unwrap_or_default());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: FundPositionsResponse = ctx
                .fund_positions(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get stock positions. Returns awaitable.
    #[pyo3(signature = (symbols = None))]
    fn stock_positions(&self, py: Python<'_>, symbols: Option<Vec<String>>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let opts = GetStockPositionsOptions::new().symbols(symbols.unwrap_or_default());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: StockPositionsResponse = ctx
                .stock_positions(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get margin ratio. Returns awaitable.
    fn margin_ratio(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: MarginRatio = ctx
                .margin_ratio(symbol)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get order detail. Returns awaitable.
    fn order_detail(&self, py: Python<'_>, order_id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: OrderDetail = ctx
                .order_detail(order_id)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Estimate max purchase quantity. Returns awaitable.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (symbol, order_type, side, price = None, currency = None, order_id = None, fractional_shares = false))]
    fn estimate_max_purchase_quantity(
        &self,
        py: Python<'_>,
        symbol: String,
        order_type: OrderType,
        side: OrderSide,
        price: Option<PyDecimal>,
        currency: Option<String>,
        order_id: Option<String>,
        fractional_shares: bool,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut opts =
            EstimateMaxPurchaseQuantityOptions::new(symbol, order_type.into(), side.into());
        if let Some(p) = price {
            opts = opts.price(p.into());
        }
        if let Some(c) = currency {
            opts = opts.currency(c);
        }
        if let Some(o) = order_id {
            opts = opts.order_id(o);
        }
        if fractional_shares {
            opts = opts.fractional_shares();
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: EstimateMaxPurchaseQuantityResponse = ctx
                .estimate_max_purchase_quantity(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }
}
