use std::sync::Arc;

use longport::{
    blocking::QuoteContextSync,
    quote::{RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup},
};
use parking_lot::Mutex;
use pyo3::prelude::*;
use time::PrimitiveDateTime;

use crate::{
    config::Config,
    error::ErrorNewType,
    quote::{
        push::handle_push_event,
        types::{
            AdjustType, CalcIndex, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
            IntradayLine, IssuerInfo, MarketTradingDays, MarketTradingSession, OptionQuote,
            ParticipantInfo, Period, RealtimeQuote, SecuritiesUpdateMode, SecurityBrokers,
            SecurityCalcIndex, SecurityDepth, SecurityQuote, SecurityStaticInfo, StrikePriceInfo,
            SubType, SubTypes, Subscription, Trade, WarrantQuote, WatchlistGroup,
        },
    },
    time::{PyDateWrapper, PyOffsetDateTimeWrapper},
    types::Market,
};

#[derive(Debug, Default)]
pub(crate) struct Callbacks {
    pub(crate) quote: Option<PyObject>,
    pub(crate) depth: Option<PyObject>,
    pub(crate) brokers: Option<PyObject>,
    pub(crate) trades: Option<PyObject>,
    pub(crate) candlestick: Option<PyObject>,
}

#[pyclass]
pub(crate) struct QuoteContext {
    ctx: QuoteContextSync,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[pymethods]
impl QuoteContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let ctx = QuoteContextSync::try_new(Arc::new(config.0.clone()), {
            let callbacks = callbacks.clone();
            move |event| {
                handle_push_event(&callbacks.lock(), event);
            }
        })
        .map_err(ErrorNewType)?;
        Ok(Self { ctx, callbacks })
    }

    /// Returns the member ID
    fn member_id(&self) -> PyResult<i64> {
        Ok(self.ctx.member_id().map_err(ErrorNewType)?)
    }

    /// Returns the quote level
    fn quote_level(&self) -> PyResult<String> {
        Ok(self.ctx.quote_level().map_err(ErrorNewType)?)
    }

    /// Set quote callback, after receiving the quote data push, it
    /// will call back to this function.
    fn set_on_quote(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().quote = None;
        } else {
            self.callbacks.lock().quote = Some(callback);
        }
    }

    /// Set depth callback, after receiving the depth data push, it
    /// will call back to this function.
    fn set_on_depth(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().depth = None;
        } else {
            self.callbacks.lock().depth = Some(callback);
        }
    }

    /// Set brokers callback, after receiving the brokers data push, it
    /// will call back to this function.
    fn set_on_brokers(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().brokers = None;
        } else {
            self.callbacks.lock().brokers = Some(callback);
        }
    }

    /// Set trades callback, after receiving the trades data push, it
    /// will call back to this function.
    fn set_on_trades(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().trades = None;
        } else {
            self.callbacks.lock().trades = Some(callback);
        }
    }

    /// Set candlestick callback, after receiving the candlestick updated event,
    /// it will call back to this function.
    fn set_on_candlestick(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().candlestick = None;
        } else {
            self.callbacks.lock().candlestick = Some(callback);
        }
    }

    /// Subscribe
    #[pyo3(signature = (symbols, sub_types, is_first_push = false))]
    fn subscribe(
        &self,
        symbols: Vec<String>,
        sub_types: Vec<SubType>,
        is_first_push: bool,
    ) -> PyResult<()> {
        self.ctx
            .subscribe(symbols, SubTypes(sub_types), is_first_push)
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    fn unsubscribe(&self, symbols: Vec<String>, sub_types: Vec<SubType>) -> PyResult<()> {
        self.ctx
            .unsubscribe(symbols, SubTypes(sub_types))
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Subscribe security candlesticks
    fn subscribe_candlesticks(&self, symbol: String, period: Period) -> PyResult<()> {
        self.ctx
            .subscribe_candlesticks(symbol, period.into())
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Subscribe security candlesticks
    fn unsubscribe_candlesticks(&self, symbol: String, period: Period) -> PyResult<()> {
        self.ctx
            .unsubscribe_candlesticks(symbol, period.into())
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get subscription information
    fn subscriptions(&self) -> PyResult<Vec<Subscription>> {
        self.ctx
            .subscriptions()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get basic information of securities
    fn static_info(&self, symbols: Vec<String>) -> PyResult<Vec<SecurityStaticInfo>> {
        self.ctx
            .static_info(symbols)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of securities
    fn quote(&self, symbols: Vec<String>) -> PyResult<Vec<SecurityQuote>> {
        self.ctx
            .quote(symbols)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of option securities
    fn option_quote(&self, symbols: Vec<String>) -> PyResult<Vec<OptionQuote>> {
        self.ctx
            .option_quote(symbols)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of warrant securities
    fn warrant_quote(&self, symbols: Vec<String>) -> PyResult<Vec<WarrantQuote>> {
        self.ctx
            .warrant_quote(symbols)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security depth
    fn depth(&self, symbol: String) -> PyResult<SecurityDepth> {
        self.ctx.depth(symbol).map_err(ErrorNewType)?.try_into()
    }

    /// Get security brokers
    fn brokers(&self, symbol: String) -> PyResult<SecurityBrokers> {
        self.ctx.brokers(symbol).map_err(ErrorNewType)?.try_into()
    }

    /// Get participants
    fn participants(&self) -> PyResult<Vec<ParticipantInfo>> {
        self.ctx
            .participants()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security trades
    fn trades(&self, symbol: String, count: usize) -> PyResult<Vec<Trade>> {
        self.ctx
            .trades(symbol, count)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security intraday
    fn intraday(&self, symbol: String) -> PyResult<Vec<IntradayLine>> {
        self.ctx
            .intraday(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security candlesticks
    fn candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: usize,
        adjust_type: AdjustType,
    ) -> PyResult<Vec<Candlestick>> {
        self.ctx
            .candlesticks(symbol, period.into(), count, adjust_type.into())
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security history candlesticks by offset
    fn history_candlesticks_by_offset(
        &self,
        symbol: String,
        period: Period,
        adjust_type: AdjustType,
        forward: bool,
        time: PyOffsetDateTimeWrapper,
        count: usize,
    ) -> PyResult<Vec<Candlestick>> {
        self.ctx
            .history_candlesticks_by_offset(
                symbol,
                period.into(),
                adjust_type.into(),
                forward,
                PrimitiveDateTime::new(time.0.date(), time.0.time()),
                count,
            )
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security history candlesticks by offset
    fn history_candlesticks_by_date(
        &self,
        symbol: String,
        period: Period,
        adjust_type: AdjustType,
        start: Option<PyDateWrapper>,
        end: Option<PyDateWrapper>,
    ) -> PyResult<Vec<Candlestick>> {
        self.ctx
            .history_candlesticks_by_date(
                symbol,
                period.into(),
                adjust_type.into(),
                start.map(|d| d.0),
                end.map(|d| d.0),
            )
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get option chain expiry date list
    fn option_chain_expiry_date_list(&self, symbol: String) -> PyResult<Vec<PyDateWrapper>> {
        Ok(self
            .ctx
            .option_chain_expiry_date_list(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Get option chain info by date
    fn option_chain_info_by_date(
        &self,
        symbol: String,
        expiry_date: PyDateWrapper,
    ) -> PyResult<Vec<StrikePriceInfo>> {
        self.ctx
            .option_chain_info_by_date(symbol, expiry_date.0)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get warrant issuers
    fn warrant_issuers(&self) -> PyResult<Vec<IssuerInfo>> {
        self.ctx
            .warrant_issuers()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    fn trading_session(&self) -> PyResult<Vec<MarketTradingSession>> {
        self.ctx
            .trading_session()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    fn trading_days(
        &self,
        market: Market,
        begin: PyDateWrapper,
        end: PyDateWrapper,
    ) -> PyResult<MarketTradingDays> {
        self.ctx
            .trading_days(market.into(), begin.0, end.0)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get capital flow intraday
    fn capital_flow(&self, symbol: String) -> PyResult<Vec<CapitalFlowLine>> {
        self.ctx
            .capital_flow(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get capital distribution
    fn capital_distribution(&self, symbol: String) -> PyResult<CapitalDistributionResponse> {
        self.ctx
            .capital_distribution(symbol)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get calc indexes
    fn calc_indexes(
        &self,
        symbols: Vec<String>,
        indexes: Vec<CalcIndex>,
    ) -> PyResult<Vec<SecurityCalcIndex>> {
        self.ctx
            .calc_indexes(symbols, indexes.into_iter().map(Into::into))
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get watch list
    fn watch_list(&self) -> PyResult<Vec<WatchlistGroup>> {
        self.watchlist()
    }

    /// Get watch list
    fn watchlist(&self) -> PyResult<Vec<WatchlistGroup>> {
        self.ctx
            .watchlist()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Create watchlist group
    fn create_watchlist_group(
        &self,
        name: String,
        securities: Option<Vec<String>>,
    ) -> PyResult<i64> {
        let mut req = RequestCreateWatchlistGroup::new(name);
        if let Some(securities) = securities {
            req = req.securities(securities);
        }
        let id = self.ctx.create_watchlist_group(req).map_err(ErrorNewType)?;
        Ok(id)
    }

    /// Delete watchlist group
    #[pyo3(signature=(id, purge = false))]
    fn delete_watchlist_group(&self, id: i64, purge: bool) -> PyResult<()> {
        self.ctx
            .delete_watchlist_group(id, purge)
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Update watchlist group
    fn update_watchlist_group(
        &self,
        id: i64,
        name: Option<String>,
        securities: Option<Vec<String>>,
        mode: Option<SecuritiesUpdateMode>,
    ) -> PyResult<()> {
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
        self.ctx.update_watchlist_group(req).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get real-time quote
    fn realtime_quote(&self, symbols: Vec<String>) -> PyResult<Vec<RealtimeQuote>> {
        self.ctx
            .realtime_quote(symbols)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get real-time depth
    fn realtime_depth(&self, symbol: String) -> PyResult<SecurityDepth> {
        self.ctx
            .realtime_depth(symbol)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time brokers
    fn realtime_brokers(&self, symbol: String) -> PyResult<SecurityBrokers> {
        self.ctx
            .realtime_brokers(symbol)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time trades
    #[pyo3(signature = (symbol, count = 500))]
    fn realtime_trades(&self, symbol: String, count: usize) -> PyResult<Vec<Trade>> {
        self.ctx
            .realtime_trades(symbol, count)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get real-time candlesticks
    #[pyo3(signature = (symbol, period, count = 500))]
    fn realtime_candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: usize,
    ) -> PyResult<Vec<Candlestick>> {
        self.ctx
            .realtime_candlesticks(symbol, period.into(), count)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
