use std::sync::Arc;

use longbridge::{
    blocking::TradeContextSync,
    trade::{
        EstimateMaxPurchaseQuantityOptions, GetCashFlowOptions, GetFundPositionsOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetStockPositionsOptions,
        GetTodayExecutionsOptions, GetTodayOrdersOptions, ReplaceOrderOptions, SubmitOrderOptions,
    },
};
use parking_lot::Mutex;
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};

use crate::{
    config::Config,
    decimal::PyDecimal,
    error::ErrorNewType,
    time::{PyDateWrapper, PyOffsetDateTimeWrapper},
    trade::{
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

#[derive(Debug, Default)]
pub(crate) struct Callbacks {
    pub(crate) order_changed: Option<PyObject>,
}

#[pyclass]
pub(crate) struct TradeContext {
    ctx: TradeContextSync,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[pymethods]
impl TradeContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let ctx = TradeContextSync::try_new(Arc::new(config.0.clone()), {
            let callbacks = callbacks.clone();
            move |event| {
                handle_push_event(&callbacks.lock(), event);
            }
        })
        .map_err(ErrorNewType)?;
        Ok(Self { ctx, callbacks })
    }

    /// Set order changed callback, after receiving the order changed event, it
    /// will call back to this function.
    fn set_on_order_changed(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().order_changed = None;
        } else {
            self.callbacks.lock().order_changed = Some(callback);
        }
    }

    /// Subscribe
    fn subscribe(&self, topics: Vec<TopicType>) -> PyResult<()> {
        self.ctx
            .subscribe(topics.into_iter().map(Into::into))
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    fn unsubscribe(&self, topics: Vec<TopicType>) -> PyResult<()> {
        self.ctx
            .unsubscribe(topics.into_iter().map(Into::into))
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get history executions
    fn history_executions(
        &self,
        symbol: Option<String>,
        start_at: Option<PyOffsetDateTimeWrapper>,
        end_at: Option<PyOffsetDateTimeWrapper>,
    ) -> PyResult<Vec<Execution>> {
        let mut opts = GetHistoryExecutionsOptions::new();

        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }
        if let Some(start_at) = start_at {
            opts = opts.start_at(start_at.0);
        }
        if let Some(end_at) = end_at {
            opts = opts.end_at(end_at.0);
        }

        self.ctx
            .history_executions(Some(opts))
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get today executions
    fn today_executions(
        &self,
        symbol: Option<String>,
        order_id: Option<String>,
    ) -> PyResult<Vec<Execution>> {
        let mut opts = GetTodayExecutionsOptions::new();

        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }
        if let Some(order_id) = order_id {
            opts = opts.order_id(order_id);
        }

        self.ctx
            .today_executions(Some(opts))
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get history orders
    fn history_orders(
        &self,
        symbol: Option<String>,
        status: Option<Vec<OrderStatus>>,
        side: Option<OrderSide>,
        market: Option<Market>,
        start_at: Option<PyOffsetDateTimeWrapper>,
        end_at: Option<PyOffsetDateTimeWrapper>,
    ) -> PyResult<Vec<Order>> {
        let mut opts = GetHistoryOrdersOptions::new();

        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }
        opts = opts.status(status.unwrap_or_default().into_iter().map(Into::into));
        if let Some(side) = side {
            opts = opts.side(side.into());
        }
        if let Some(market) = market {
            opts = opts.market(market.into());
        }
        if let Some(start_at) = start_at {
            opts = opts.start_at(start_at.0);
        }
        if let Some(end_at) = end_at {
            opts = opts.end_at(end_at.0);
        }

        self.ctx
            .history_orders(Some(opts))
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get today orders
    fn today_orders(
        &self,
        symbol: Option<String>,
        status: Option<Vec<OrderStatus>>,
        side: Option<OrderSide>,
        market: Option<Market>,
        order_id: Option<String>,
    ) -> PyResult<Vec<Order>> {
        let mut opts = GetTodayOrdersOptions::new();

        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }
        opts = opts.status(status.unwrap_or_default().into_iter().map(Into::into));
        if let Some(side) = side {
            opts = opts.side(side.into());
        }
        if let Some(market) = market {
            opts = opts.market(market.into());
        }
        if let Some(order_id) = order_id {
            opts = opts.order_id(order_id);
        }

        self.ctx
            .today_orders(Some(opts))
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Replace order
    #[allow(clippy::too_many_arguments)]
    fn replace_order(
        &self,
        order_id: String,
        quantity: i64,
        price: Option<PyDecimal>,
        trigger_price: Option<PyDecimal>,
        limit_offset: Option<PyDecimal>,
        trailing_amount: Option<PyDecimal>,
        trailing_percent: Option<PyDecimal>,
        remark: Option<String>,
    ) -> PyResult<()> {
        let mut opts = ReplaceOrderOptions::new(order_id, quantity);

        if let Some(price) = price {
            opts = opts.price(price.into());
        }
        if let Some(trigger_price) = trigger_price {
            opts = opts.trigger_price(trigger_price.into());
        }
        if let Some(limit_offset) = limit_offset {
            opts = opts.limit_offset(limit_offset.into());
        }
        if let Some(trailing_amount) = trailing_amount {
            opts = opts.trailing_amount(trailing_amount.into());
        }
        if let Some(trailing_percent) = trailing_percent {
            opts = opts.trailing_percent(trailing_percent.into());
        }
        if let Some(remark) = remark {
            opts = opts.remark(remark);
        }

        self.ctx.replace_order(opts).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Submit order
    #[allow(clippy::too_many_arguments)]
    fn submit_order(
        &self,
        symbol: String,
        order_type: OrderType,
        side: OrderSide,
        submitted_quantity: i64,
        time_in_force: TimeInForceType,
        submitted_price: Option<PyDecimal>,
        trigger_price: Option<PyDecimal>,
        limit_offset: Option<PyDecimal>,
        trailing_amount: Option<PyDecimal>,
        trailing_percent: Option<PyDecimal>,
        expire_date: Option<PyDateWrapper>,
        outside_rth: Option<OutsideRTH>,
        remark: Option<String>,
    ) -> PyResult<SubmitOrderResponse> {
        let mut opts = SubmitOrderOptions::new(
            symbol,
            order_type.into(),
            side.into(),
            submitted_quantity,
            time_in_force.into(),
        );

        if let Some(submitted_price) = submitted_price {
            opts = opts.submitted_price(submitted_price.into());
        }
        if let Some(trigger_price) = trigger_price {
            opts = opts.trigger_price(trigger_price.into());
        }
        if let Some(limit_offset) = limit_offset {
            opts = opts.limit_offset(limit_offset.into());
        }
        if let Some(trailing_amount) = trailing_amount {
            opts = opts.trailing_amount(trailing_amount.into());
        }
        if let Some(trailing_percent) = trailing_percent {
            opts = opts.trailing_percent(trailing_percent.into());
        }
        if let Some(expire_date) = expire_date {
            opts = opts.expire_date(expire_date.0);
        }
        if let Some(outside_rth) = outside_rth {
            opts = opts.outside_rth(outside_rth.into());
        }
        if let Some(remark) = remark {
            opts = opts.remark(remark);
        }

        self.ctx
            .submit_order(opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Cancel order
    fn cancel_order(&self, order_id: String) -> PyResult<()> {
        self.ctx.cancel_order(order_id).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get account balance
    fn account_balance(&self, currency: Option<String>) -> PyResult<Vec<AccountBalance>> {
        self.ctx
            .account_balance(currency.as_deref())
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<PyResult<Vec<_>>>()
    }

    /// Get cash flow
    fn cash_flow(
        &self,
        start_at: PyOffsetDateTimeWrapper,
        end_at: PyOffsetDateTimeWrapper,
        business_type: Option<BalanceType>,
        symbol: Option<String>,
        page: Option<usize>,
        size: Option<usize>,
    ) -> PyResult<Vec<CashFlow>> {
        let mut opts = GetCashFlowOptions::new(start_at.0, end_at.0);

        if let Some(business_type) = business_type {
            opts = opts.business_type(business_type.into());
        }
        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(size) = size {
            opts = opts.size(size);
        }

        self.ctx
            .cash_flow(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<PyResult<Vec<_>>>()
    }

    /// Get fund positions
    fn fund_positions(&self, symbols: Option<Vec<String>>) -> PyResult<FundPositionsResponse> {
        self.ctx
            .fund_positions(GetFundPositionsOptions::new().symbols(symbols.unwrap_or_default()))
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get stock positions
    fn stock_positions(&self, symbols: Option<Vec<String>>) -> PyResult<StockPositionsResponse> {
        self.ctx
            .stock_positions(GetStockPositionsOptions::new().symbols(symbols.unwrap_or_default()))
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get margin ratio
    fn margin_ratio(&self, symbol: String) -> PyResult<MarginRatio> {
        self.ctx
            .margin_ratio(symbol)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get order detail
    fn order_detail(&self, order_id: String) -> PyResult<OrderDetail> {
        self.ctx
            .order_detail(order_id)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Estimating the maximum purchase quantity for Hong Kong and US stocks,
    /// warrants, and options
    pub fn estimate_max_purchase_quantity(
        &self,
        symbol: String,
        order_type: OrderType,
        side: OrderSide,
        price: Option<PyDecimal>,
        currency: Option<String>,
        order_id: Option<String>,
    ) -> PyResult<EstimateMaxPurchaseQuantityResponse> {
        let mut opts =
            EstimateMaxPurchaseQuantityOptions::new(symbol, order_type.into(), side.into());

        if let Some(price) = price {
            opts = opts.price(price.into());
        }
        if let Some(currency) = currency {
            opts = opts.currency(currency);
        }
        if let Some(order_id) = order_id {
            opts = opts.order_id(order_id);
        }

        self.ctx
            .estimate_max_purchase_quantity(opts)
            .map_err(ErrorNewType)?
            .try_into()
    }
}
