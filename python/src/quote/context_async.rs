//! Async quote context backed by longport's native async API.

use std::sync::Arc;

use longport::{
    QuoteContext,
    quote::{RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup},
};
use parking_lot::Mutex;
use pyo3::{prelude::*, types::PyType};
use time::PrimitiveDateTime;

use crate::{
    config::Config,
    error::ErrorNewType,
    quote::{
        context::Callbacks,
        push::handle_push_event,
        types::{
            AdjustType, CalcIndex, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
            FilterWarrantExpiryDate, FilterWarrantInOutBoundsType,
            HistoryMarketTemperatureResponse, IntradayLine, IssuerInfo, MarketTemperature,
            MarketTradingDays, MarketTradingSession, OptionQuote, ParticipantInfo, Period,
            QuotePackageDetail, RealtimeQuote, SecuritiesUpdateMode, Security, SecurityBrokers,
            SecurityCalcIndex, SecurityDepth, SecurityListCategory, SecurityQuote,
            SecurityStaticInfo, SortOrderType, StrikePriceInfo, SubType, SubTypes, Subscription,
            Trade, TradeSessions, WarrantInfo, WarrantQuote, WarrantSortBy, WarrantStatus,
            WarrantType, WatchlistGroup,
        },
    },
    time::{PyDateWrapper, PyOffsetDateTimeWrapper},
    types::Market,
};

/// Async quote context. Create via `AsyncQuoteContext.create(config)` and await
/// in asyncio.
#[pyclass]
pub(crate) struct AsyncQuoteContext {
    ctx: Arc<QuoteContext>,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[pymethods]
impl AsyncQuoteContext {
    /// Create an async quote context. Returns an awaitable; must be awaited
    /// inside asyncio.
    #[classmethod]
    fn create(cls: &Bound<PyType>, config: &Config) -> PyResult<Py<PyAny>> {
        let py = cls.py();
        let config = Arc::new(config.0.clone());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (ctx, mut event_rx) = QuoteContext::try_new(config).await.map_err(ErrorNewType)?;
            let callbacks = Arc::new(Mutex::new(Callbacks::default()));
            let callbacks_clone = callbacks.clone();
            pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                while let Some(event) = event_rx.recv().await {
                    handle_push_event(&callbacks_clone.lock(), event);
                }
            });
            Ok(AsyncQuoteContext {
                ctx: Arc::new(ctx),
                callbacks,
            })
        })
        .map(|b| b.unbind())
    }

    /// Returns the member ID.
    fn member_id(&self) -> PyResult<i64> {
        Ok(self.ctx.member_id())
    }

    /// Returns the quote level.
    fn quote_level(&self) -> PyResult<String> {
        Ok(self.ctx.quote_level().to_string())
    }

    /// Returns the quote package details.
    fn quote_package_details(&self) -> PyResult<Vec<QuotePackageDetail>> {
        self.ctx
            .quote_package_details()
            .to_vec()
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Set quote callback.
    fn set_on_quote(&self, py: Python<'_>, callback: Py<PyAny>) {
        if callback.is_none(py) {
            self.callbacks.lock().quote = None;
        } else {
            self.callbacks.lock().quote = Some(callback);
        }
    }

    /// Set depth callback.
    fn set_on_depth(&self, py: Python<'_>, callback: Py<PyAny>) {
        if callback.is_none(py) {
            self.callbacks.lock().depth = None;
        } else {
            self.callbacks.lock().depth = Some(callback);
        }
    }

    /// Set brokers callback.
    fn set_on_brokers(&self, py: Python<'_>, callback: Py<PyAny>) {
        if callback.is_none(py) {
            self.callbacks.lock().brokers = None;
        } else {
            self.callbacks.lock().brokers = Some(callback);
        }
    }

    /// Set trades callback.
    fn set_on_trades(&self, py: Python<'_>, callback: Py<PyAny>) {
        if callback.is_none(py) {
            self.callbacks.lock().trades = None;
        } else {
            self.callbacks.lock().trades = Some(callback);
        }
    }

    /// Set candlestick callback.
    fn set_on_candlestick(&self, py: Python<'_>, callback: Py<PyAny>) {
        if callback.is_none(py) {
            self.callbacks.lock().candlestick = None;
        } else {
            self.callbacks.lock().candlestick = Some(callback);
        }
    }

    /// Subscribe. Returns awaitable.
    #[pyo3(signature = (symbols, sub_types))]
    fn subscribe(
        &self,
        py: Python<'_>,
        symbols: Vec<String>,
        sub_types: Vec<SubType>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let sub_flags = longport::quote::SubFlags::from(SubTypes(sub_types));
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.subscribe(symbols, sub_flags)
                .await
                .map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Unsubscribe. Returns awaitable.
    fn unsubscribe(
        &self,
        py: Python<'_>,
        symbols: Vec<String>,
        sub_types: Vec<SubType>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let sub_flags = longport::quote::SubFlags::from(SubTypes(sub_types));
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.unsubscribe(symbols, sub_flags)
                .await
                .map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Subscribe security candlesticks. Returns awaitable.
    #[pyo3(signature = (symbol, period, trade_sessions = TradeSessions::Intraday))]
    fn subscribe_candlesticks(
        &self,
        py: Python<'_>,
        symbol: String,
        period: Period,
        trade_sessions: TradeSessions,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .subscribe_candlesticks(symbol, period.into(), trade_sessions.into())
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Candlestick> { x.try_into() })
                .collect::<PyResult<Vec<Candlestick>>>()
        })
        .map(|b| b.unbind())
    }

    /// Unsubscribe security candlesticks. Returns awaitable.
    fn unsubscribe_candlesticks(
        &self,
        py: Python<'_>,
        symbol: String,
        period: Period,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.unsubscribe_candlesticks(symbol, period.into())
                .await
                .map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Get subscription information. Returns awaitable.
    fn subscriptions(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.subscriptions().await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Subscription> { x.try_into() })
                .collect::<PyResult<Vec<Subscription>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get basic information of securities. Returns awaitable.
    fn static_info(&self, py: Python<'_>, symbols: Vec<String>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.static_info(symbols).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<SecurityStaticInfo> { x.try_into() })
                .collect::<PyResult<Vec<SecurityStaticInfo>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get quote of securities. Returns awaitable.
    fn quote(&self, py: Python<'_>, symbols: Vec<String>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.quote(symbols).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<SecurityQuote> { x.try_into() })
                .collect::<PyResult<Vec<SecurityQuote>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get quote of option securities. Returns awaitable.
    fn option_quote(&self, py: Python<'_>, symbols: Vec<String>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.option_quote(symbols).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<OptionQuote> { x.try_into() })
                .collect::<PyResult<Vec<OptionQuote>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get quote of warrant securities. Returns awaitable.
    fn warrant_quote(&self, py: Python<'_>, symbols: Vec<String>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.warrant_quote(symbols).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<WarrantQuote> { x.try_into() })
                .collect::<PyResult<Vec<WarrantQuote>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get security depth. Returns awaitable.
    fn depth(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: SecurityDepth = ctx.depth(symbol).await.map_err(ErrorNewType)?.try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get security brokers. Returns awaitable.
    fn brokers(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: SecurityBrokers = ctx
                .brokers(symbol)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get participants. Returns awaitable.
    fn participants(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.participants().await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<ParticipantInfo> { x.try_into() })
                .collect::<PyResult<Vec<ParticipantInfo>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get security trades. Returns awaitable.
    fn trades(&self, py: Python<'_>, symbol: String, count: usize) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.trades(symbol, count).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Trade> { x.try_into() })
                .collect::<PyResult<Vec<Trade>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get security intraday. Returns awaitable.
    #[pyo3(signature = (symbol, trade_sessions = TradeSessions::Intraday))]
    fn intraday(
        &self,
        py: Python<'_>,
        symbol: String,
        trade_sessions: TradeSessions,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .intraday(symbol, trade_sessions.into())
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<IntradayLine> { x.try_into() })
                .collect::<PyResult<Vec<IntradayLine>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get security candlesticks. Returns awaitable.
    #[pyo3(signature = (symbol, period, count, adjust_type, trade_sessions = TradeSessions::Intraday))]
    fn candlesticks(
        &self,
        py: Python<'_>,
        symbol: String,
        period: Period,
        count: usize,
        adjust_type: AdjustType,
        trade_sessions: TradeSessions,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .candlesticks(
                    symbol,
                    period.into(),
                    count,
                    adjust_type.into(),
                    trade_sessions.into(),
                )
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Candlestick> { x.try_into() })
                .collect::<PyResult<Vec<Candlestick>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get security history candlesticks by offset. Returns awaitable.
    #[pyo3(signature = (symbol, period, adjust_type, forward, count, time = None, trade_sessions = TradeSessions::Intraday))]
    #[allow(clippy::too_many_arguments)]
    fn history_candlesticks_by_offset(
        &self,
        py: Python<'_>,
        symbol: String,
        period: Period,
        adjust_type: AdjustType,
        forward: bool,
        count: usize,
        time: Option<PyOffsetDateTimeWrapper>,
        trade_sessions: TradeSessions,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let time_prim = time.map(|t| PrimitiveDateTime::new(t.0.date(), t.0.time()));
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .history_candlesticks_by_offset(
                    symbol,
                    period.into(),
                    adjust_type.into(),
                    forward,
                    time_prim,
                    count,
                    trade_sessions.into(),
                )
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Candlestick> { x.try_into() })
                .collect::<PyResult<Vec<Candlestick>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get security history candlesticks by date. Returns awaitable.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (symbol, period, adjust_type, start = None, end = None, trade_sessions = TradeSessions::Intraday))]
    fn history_candlesticks_by_date(
        &self,
        py: Python<'_>,
        symbol: String,
        period: Period,
        adjust_type: AdjustType,
        start: Option<PyDateWrapper>,
        end: Option<PyDateWrapper>,
        trade_sessions: TradeSessions,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let start_d = start.map(|d| d.0);
        let end_d = end.map(|d| d.0);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .history_candlesticks_by_date(
                    symbol,
                    period.into(),
                    adjust_type.into(),
                    start_d,
                    end_d,
                    trade_sessions.into(),
                )
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Candlestick> { x.try_into() })
                .collect::<PyResult<Vec<Candlestick>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get option chain expiry date list. Returns awaitable.
    fn option_chain_expiry_date_list(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .option_chain_expiry_date_list(symbol)
                .await
                .map_err(ErrorNewType)?;
            Ok(v.into_iter()
                .map(Into::into)
                .collect::<Vec<PyDateWrapper>>())
        })
        .map(|b| b.unbind())
    }

    /// Get option chain info by date. Returns awaitable.
    fn option_chain_info_by_date(
        &self,
        py: Python<'_>,
        symbol: String,
        expiry_date: PyDateWrapper,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .option_chain_info_by_date(symbol, expiry_date.0)
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<StrikePriceInfo> { x.try_into() })
                .collect::<PyResult<Vec<StrikePriceInfo>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get warrant issuers. Returns awaitable.
    fn warrant_issuers(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.warrant_issuers().await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<IssuerInfo> { x.try_into() })
                .collect::<PyResult<Vec<IssuerInfo>>>()
        })
        .map(|b| b.unbind())
    }

    /// Query warrant list. Returns awaitable.
    #[pyo3(signature = (symbol, sort_by, sort_order, warrant_type = None, issuer = None, expiry_date = None, price_type = None, status = None))]
    #[allow(clippy::too_many_arguments)]
    fn warrant_list(
        &self,
        py: Python<'_>,
        symbol: String,
        sort_by: WarrantSortBy,
        sort_order: SortOrderType,
        warrant_type: Option<Vec<WarrantType>>,
        issuer: Option<Vec<i32>>,
        expiry_date: Option<Vec<FilterWarrantExpiryDate>>,
        price_type: Option<Vec<FilterWarrantInOutBoundsType>>,
        status: Option<Vec<WarrantStatus>>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let warrant_type: Option<Vec<longport::quote::WarrantType>> =
            warrant_type.map(|v| v.into_iter().map(Into::into).collect());
        let expiry_date: Option<Vec<longport::quote::FilterWarrantExpiryDate>> =
            expiry_date.map(|v| v.into_iter().map(Into::into).collect());
        let price_type: Option<Vec<longport::quote::FilterWarrantInOutBoundsType>> =
            price_type.map(|v| v.into_iter().map(Into::into).collect());
        let status: Option<Vec<longport::quote::WarrantStatus>> =
            status.map(|v| v.into_iter().map(Into::into).collect());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .warrant_list(
                    symbol,
                    sort_by.into(),
                    sort_order.into(),
                    warrant_type.as_deref(),
                    issuer.as_deref(),
                    expiry_date.as_deref(),
                    price_type.as_deref(),
                    status.as_deref(),
                )
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<WarrantInfo> { x.try_into() })
                .collect::<PyResult<Vec<WarrantInfo>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get trading session. Returns awaitable.
    fn trading_session(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.trading_session().await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<MarketTradingSession> { x.try_into() })
                .collect::<PyResult<Vec<MarketTradingSession>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get trading days. Returns awaitable.
    fn trading_days(
        &self,
        py: Python<'_>,
        market: Market,
        begin: PyDateWrapper,
        end: PyDateWrapper,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: MarketTradingDays = ctx
                .trading_days(market.into(), begin.0, end.0)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get capital flow intraday. Returns awaitable.
    fn capital_flow(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.capital_flow(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<CapitalFlowLine> { x.try_into() })
                .collect::<PyResult<Vec<CapitalFlowLine>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get capital distribution. Returns awaitable.
    fn capital_distribution(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: CapitalDistributionResponse = ctx
                .capital_distribution(symbol)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get calc indexes. Returns awaitable.
    fn calc_indexes(
        &self,
        py: Python<'_>,
        symbols: Vec<String>,
        indexes: Vec<CalcIndex>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .calc_indexes(symbols, indexes.into_iter().map(Into::into))
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<SecurityCalcIndex> { x.try_into() })
                .collect::<PyResult<Vec<SecurityCalcIndex>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get watch list. Returns awaitable.
    fn watchlist(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.watchlist().await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<WatchlistGroup> { x.try_into() })
                .collect::<PyResult<Vec<WatchlistGroup>>>()
        })
        .map(|b| b.unbind())
    }

    /// Create watchlist group. Returns awaitable.
    #[pyo3(signature = (name, securities = None))]
    fn create_watchlist_group(
        &self,
        py: Python<'_>,
        name: String,
        securities: Option<Vec<String>>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut req = RequestCreateWatchlistGroup::new(name);
        if let Some(securities) = securities {
            req = req.securities(securities);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let id = ctx
                .create_watchlist_group(req)
                .await
                .map_err(ErrorNewType)?;
            Ok(id)
        })
        .map(|b| b.unbind())
    }

    /// Delete watchlist group. Returns awaitable.
    #[pyo3(signature = (id, purge = false))]
    fn delete_watchlist_group(&self, py: Python<'_>, id: i64, purge: bool) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.delete_watchlist_group(id, purge)
                .await
                .map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Update watchlist group. Returns awaitable.
    #[pyo3(signature = (id, name = None, securities = None, mode = None))]
    fn update_watchlist_group(
        &self,
        py: Python<'_>,
        id: i64,
        name: Option<String>,
        securities: Option<Vec<String>>,
        mode: Option<SecuritiesUpdateMode>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        let mut req = RequestUpdateWatchlistGroup::new(id);
        if let Some(name) = name {
            req = req.name(name);
        }
        if let Some(securities) = securities {
            req = req.securities(securities);
        }
        if let Some(mode) = mode {
            req = req.mode(mode.into());
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.update_watchlist_group(req)
                .await
                .map_err(ErrorNewType)?;
            Ok(())
        })
        .map(|b| b.unbind())
    }

    /// Get security list. Returns awaitable.
    #[pyo3(signature = (market, category = None))]
    fn security_list(
        &self,
        py: Python<'_>,
        market: Market,
        category: Option<SecurityListCategory>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .security_list(market.into(), category.map(Into::into))
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Security> { x.try_into() })
                .collect::<PyResult<Vec<Security>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get current market temperature. Returns awaitable.
    fn market_temperature(&self, py: Python<'_>, market: Market) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: MarketTemperature = ctx
                .market_temperature(market.into())
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get historical market temperature. Returns awaitable.
    fn history_market_temperature(
        &self,
        py: Python<'_>,
        market: Market,
        start_date: PyDateWrapper,
        end: PyDateWrapper,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: HistoryMarketTemperatureResponse = ctx
                .history_market_temperature(market.into(), start_date.0, end.0)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get real-time quote. Returns awaitable.
    fn realtime_quote(&self, py: Python<'_>, symbols: Vec<String>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.realtime_quote(symbols).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<RealtimeQuote> { x.try_into() })
                .collect::<PyResult<Vec<RealtimeQuote>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get real-time depth. Returns awaitable.
    fn realtime_depth(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: SecurityDepth = ctx
                .realtime_depth(symbol)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get real-time brokers. Returns awaitable.
    fn realtime_brokers(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let r: SecurityBrokers = ctx
                .realtime_brokers(symbol)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(r)
        })
        .map(|b| b.unbind())
    }

    /// Get real-time trades. Returns awaitable.
    #[pyo3(signature = (symbol, count = 500))]
    fn realtime_trades(&self, py: Python<'_>, symbol: String, count: usize) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .realtime_trades(symbol, count)
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Trade> { x.try_into() })
                .collect::<PyResult<Vec<Trade>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get real-time candlesticks. Returns awaitable.
    #[pyo3(signature = (symbol, period, count = 500))]
    fn realtime_candlesticks(
        &self,
        py: Python<'_>,
        symbol: String,
        period: Period,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .realtime_candlesticks(symbol, period.into(), count)
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<Candlestick> { x.try_into() })
                .collect::<PyResult<Vec<Candlestick>>>()
        })
        .map(|b| b.unbind())
    }
}
