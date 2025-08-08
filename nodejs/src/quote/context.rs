use std::sync::Arc;

use longport::quote::PushEventDetail;
use napi::{Result, bindgen_prelude::*, threadsafe_function::ThreadsafeFunctionCallMode};
use parking_lot::Mutex;

use crate::{
    config::Config,
    error::ErrorNewType,
    quote::{
        push::{
            PushBrokersEvent, PushCandlestickEvent, PushDepthEvent, PushQuoteEvent, PushTradesEvent,
        },
        requests::{CreateWatchlistGroup, DeleteWatchlistGroup, UpdateWatchlistGroup},
        types::{
            AdjustType, CalcIndex, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
            FilterWarrantExpiryDate, FilterWarrantInOutBoundsType,
            HistoryMarketTemperatureResponse, IntradayLine, IssuerInfo, MarketTemperature,
            MarketTradingDays, MarketTradingSession, OptionQuote, ParticipantInfo, Period,
            QuotePackageDetail, RealtimeQuote, Security, SecurityBrokers, SecurityCalcIndex,
            SecurityDepth, SecurityListCategory, SecurityQuote, SecurityStaticInfo, SortOrderType,
            StrikePriceInfo, SubType, SubTypes, Subscription, Trade, TradeSessions, WarrantInfo,
            WarrantQuote, WarrantSortBy, WarrantStatus, WarrantType, WatchlistGroup,
        },
    },
    time::{NaiveDate, NaiveDatetime},
    types::Market,
    utils::JsCallback,
};

#[derive(Default)]
struct Callbacks {
    quote: Option<JsCallback<PushQuoteEvent>>,
    depth: Option<JsCallback<PushDepthEvent>>,
    brokers: Option<JsCallback<PushBrokersEvent>>,
    trades: Option<JsCallback<PushTradesEvent>>,
    candlestick: Option<JsCallback<PushCandlestickEvent>>,
}

/// Quote context
#[napi_derive::napi]
#[derive(Clone)]
pub struct QuoteContext {
    ctx: longport::quote::QuoteContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[napi_derive::napi]
impl QuoteContext {
    #[napi]
    pub async fn new(config: &Config) -> Result<QuoteContext> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let (ctx, mut receiver) =
            longport::quote::QuoteContext::try_new(Arc::new(config.0.clone()))
                .await
                .map_err(ErrorNewType)?;

        tokio::spawn({
            let callbacks = callbacks.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    let callbacks = callbacks.lock();
                    match msg.detail {
                        PushEventDetail::Quote(quote) => {
                            if let Some(callback) = &callbacks.quote
                                && let Ok(quote) = quote.try_into()
                            {
                                callback.call(
                                    Ok(PushQuoteEvent {
                                        symbol: msg.symbol,
                                        data: quote,
                                    }),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                        PushEventDetail::Depth(depth) => {
                            if let Some(callback) = &callbacks.depth
                                && let Ok(depth) = depth.try_into()
                            {
                                callback.call(
                                    Ok(PushDepthEvent {
                                        symbol: msg.symbol,
                                        data: depth,
                                    }),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                        PushEventDetail::Brokers(brokers) => {
                            if let Some(callback) = &callbacks.brokers
                                && let Ok(brokers) = brokers.try_into()
                            {
                                callback.call(
                                    Ok(PushBrokersEvent {
                                        symbol: msg.symbol,
                                        data: brokers,
                                    }),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                        PushEventDetail::Trade(trades) => {
                            if let Some(callback) = &callbacks.trades
                                && let Ok(trades) = trades.try_into()
                            {
                                callback.call(
                                    Ok(PushTradesEvent {
                                        symbol: msg.symbol,
                                        data: trades,
                                    }),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                        PushEventDetail::Candlestick(candlestick) => {
                            if let Some(callback) = &callbacks.candlestick
                                && let Ok(candlestick) = candlestick.try_into()
                            {
                                callback.call(
                                    Ok(PushCandlestickEvent {
                                        symbol: msg.symbol,
                                        data: candlestick,
                                    }),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                    }
                }
            }
        });

        Ok(QuoteContext { ctx, callbacks })
    }

    /// Returns the member ID
    #[napi]
    pub fn member_id(&self) -> i64 {
        self.ctx.member_id()
    }

    /// Returns the quote level
    #[napi]
    pub fn quote_level(&self) -> &str {
        self.ctx.quote_level()
    }

    /// Returns the quote package details
    #[napi]
    pub fn quote_package_details(&self) -> Result<Vec<QuotePackageDetail>> {
        self.ctx
            .quote_package_details()
            .iter()
            .cloned()
            .map(TryInto::try_into)
            .collect()
    }

    /// Set quote callback, after receiving the quote data push, it will call
    /// back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushQuoteEvent) => void")]
    pub fn set_on_quote(&self, callback: Function<PushQuoteEvent, ()>) -> Result<()> {
        self.callbacks.lock().quote = Some(
            callback
                .build_threadsafe_function()
                .callee_handled::<true>()
                .build()?,
        );
        Ok(())
    }

    /// Set depth callback, after receiving the depth data push, it will call
    /// back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushDepthEvent) => void")]
    pub fn set_on_depth(&self, callback: Function<PushDepthEvent, ()>) -> Result<()> {
        self.callbacks.lock().depth = Some(
            callback
                .build_threadsafe_function()
                .callee_handled::<true>()
                .build()?,
        );
        Ok(())
    }

    /// Set brokers callback, after receiving the brokers data push, it will
    /// call back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushBrokersEvent) => void")]
    pub fn set_on_brokers(&self, callback: Function<PushBrokersEvent, ()>) -> Result<()> {
        self.callbacks.lock().brokers = Some(
            callback
                .build_threadsafe_function()
                .callee_handled::<true>()
                .build()?,
        );
        Ok(())
    }

    /// Set trades callback, after receiving the trades data push, it will call
    /// back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushTradesEvent) => void")]
    pub fn set_on_trades(&self, callback: Function<PushTradesEvent, ()>) -> Result<()> {
        self.callbacks.lock().trades = Some(
            callback
                .build_threadsafe_function()
                .callee_handled::<true>()
                .build()?,
        );
        Ok(())
    }

    /// Set candlestick callback, after receiving the trades data push, it will
    /// call back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushCandlestickEvent) => void")]
    pub fn set_on_candlestick(&self, callback: Function<PushCandlestickEvent, ()>) -> Result<()> {
        self.callbacks.lock().candlestick = Some(
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
    /// const { Config, QuoteContext, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => {
    ///     ctx.setOnQuote((_, event) => console.log(event.toString()));
    ///     ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote], true);
    ///   });
    /// ```
    #[napi]
    pub async fn subscribe(
        &self,
        symbols: Vec<String>,
        sub_types: Vec<SubType>,
        is_first_push: bool,
    ) -> Result<()> {
        self.ctx
            .subscribe(symbols, SubTypes(sub_types), is_first_push)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => {
    ///     ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote], true)
    ///       .then(() => ctx.unsubscribe(["AAPL.US"], [SubType.Quote], true)))
    ///   })
    /// ```
    #[napi]
    pub async fn unsubscribe(&self, symbols: Vec<String>, sub_types: Vec<SubType>) -> Result<()> {
        self.ctx
            .unsubscribe(symbols, SubTypes(sub_types))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Subscribe security candlesticks
    #[napi]
    pub async fn subscribe_candlesticks(
        &self,
        symbol: String,
        period: Period,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.ctx
            .subscribe_candlesticks(symbol, period.into(), trade_sessions.into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Unsubscribe security candlesticks
    #[napi]
    pub async fn unsubscribe_candlesticks(&self, symbol: String, period: Period) -> Result<()> {
        self.ctx
            .unsubscribe_candlesticks(symbol, period.into())
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get subscription information
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then((ctx) => {
    ///     return ctx
    ///       .subscribe(["700.HK", "AAPL.US"], [SubType.Quote], true)
    ///       .then(() => ctx.subscriptions());
    ///   })
    ///   .then((resp) => console.log(resp.toString()));
    /// ```
    #[napi]
    pub async fn subscriptions(&self) -> Result<Vec<Subscription>> {
        self.ctx
            .subscriptions()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get basic information of securities
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.staticInfo(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"]))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }    
    ///   })
    /// ```
    #[napi]
    pub async fn static_info(&self, symbols: Vec<String>) -> Result<Vec<SecurityStaticInfo>> {
        self.ctx
            .static_info(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of securities
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"]))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn quote(&self, symbols: Vec<String>) -> Result<Vec<SecurityQuote>> {
        self.ctx
            .quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of option securities
    ///
    /// #### Example
    ///
    /// ```javascript
    /// import { Config, QuoteContext } from 'longport'
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.optionQuote(["AAPL230317P160000.US"]))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn option_quote(&self, symbols: Vec<String>) -> Result<Vec<OptionQuote>> {
        self.ctx
            .option_quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of warrant securities
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.warrantQuote(["21125.HK"]))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn warrant_quote(&self, symbols: Vec<String>) -> Result<Vec<WarrantQuote>> {
        self.ctx
            .warrant_quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security depth
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.depth("700.HK"))
    ///   .then((resp) => console.log(resp.toString()))
    /// ```
    #[napi]
    pub async fn depth(&self, symbol: String) -> Result<SecurityDepth> {
        self.ctx
            .depth(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get security brokers
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.brokers("700.HK"))
    ///   .then((resp) => console.log(resp.toString()))
    /// ```
    #[napi]
    pub async fn brokers(&self, symbol: String) -> Result<SecurityBrokers> {
        self.ctx
            .brokers(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get participants
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.participants())
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn participants(&self) -> Result<Vec<ParticipantInfo>> {
        self.ctx
            .participants()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security trades
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.trades("700.HK", 10))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn trades(&self, symbol: String, count: i32) -> Result<Vec<Trade>> {
        self.ctx
            .trades(symbol, count.max(0) as usize)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security intraday
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, TradeSessions } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.intraday("700.HK", TradeSessions.Intraday))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn intraday(
        &self,
        symbol: String,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<IntradayLine>> {
        self.ctx
            .intraday(symbol, trade_sessions.into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security candlesticks
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, Period, AdjustType, TradeSessions } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.candlesticks("700.HK", Period.Day, 10, AdjustType.NoAdjust, TradeSessions.Intraday))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString());
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: i32,
        adjust_type: AdjustType,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.ctx
            .candlesticks(
                symbol,
                period.into(),
                count.max(0) as usize,
                adjust_type.into(),
                trade_sessions.into(),
            )
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security history candlesticks by offset
    #[napi]
    #[allow(clippy::too_many_arguments)]
    pub async fn history_candlesticks_by_offset(
        &self,
        symbol: String,
        period: Period,
        adjust_type: AdjustType,
        forward: bool,
        datetime: Option<&NaiveDatetime>,
        count: i32,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.ctx
            .history_candlesticks_by_offset(
                symbol,
                period.into(),
                adjust_type.into(),
                forward,
                datetime.map(|datetime| datetime.0),
                count as usize,
                trade_sessions.into(),
            )
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security history candlesticks by date
    #[napi]
    pub async fn history_candlesticks_by_date(
        &self,
        symbol: String,
        period: Period,
        adjust_type: AdjustType,
        start: Option<&NaiveDate>,
        end: Option<&NaiveDate>,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.ctx
            .history_candlesticks_by_date(
                symbol,
                period.into(),
                adjust_type.into(),
                start.map(|date| date.0),
                end.map(|date| date.0),
                trade_sessions.into(),
            )
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get option chain expiry date list
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.optionChainExpiryDateList("AAPL.US"))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn option_chain_expiry_date_list(&self, symbol: String) -> Result<Vec<NaiveDate>> {
        Ok(self
            .ctx
            .option_chain_expiry_date_list(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Get option chain info by date
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, NaiveDate } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.optionChainInfoByDate("AAPL.US", new NaiveDate(2023, 1, 20)))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn option_chain_info_by_date(
        &self,
        symbol: String,
        expiry_date: &NaiveDate,
    ) -> Result<Vec<StrikePriceInfo>> {
        self.ctx
            .option_chain_info_by_date(symbol, expiry_date.0)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get warrant issuers
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, NaiveDate } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.warrantIssuers())
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn warrant_issuers(&self) -> Result<Vec<IssuerInfo>> {
        self.ctx
            .warrant_issuers()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Query warrant list
    ///
    /// #### Example
    /// ```javascript
    /// const { Config, QuoteContext, WarrantSortBy, SortOrderType } = require("longport")
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///  .then((ctx) => ctx.warrantList("700.HK", WarrantSortBy.LastDone, SortOrderType.Asc))
    /// .then((resp) => {
    ///  for (let obj of resp) {
    ///   console.log(obj.toString())
    /// }
    /// })
    /// ```
    #[napi]
    #[allow(clippy::too_many_arguments)]
    pub async fn warrant_list(
        &self,
        symbol: String,
        sort_by: WarrantSortBy,
        sort_order: SortOrderType,
        warrant_type: Option<Vec<WarrantType>>,
        issuer: Option<Vec<i32>>,
        expiry_date: Option<Vec<FilterWarrantExpiryDate>>,
        price_type: Option<Vec<FilterWarrantInOutBoundsType>>,
        status: Option<Vec<WarrantStatus>>,
    ) -> Result<Vec<WarrantInfo>> {
        let warrant_type: Option<Vec<longport::quote::WarrantType>> =
            warrant_type.map(|v| v.into_iter().map(Into::into).collect());
        let expiry_date: Option<Vec<longport::quote::FilterWarrantExpiryDate>> =
            expiry_date.map(|v| v.into_iter().map(Into::into).collect());
        let price_type: Option<Vec<longport::quote::FilterWarrantInOutBoundsType>> =
            price_type.map(|v| v.into_iter().map(Into::into).collect());
        let status: Option<Vec<longport::quote::WarrantStatus>> =
            status.map(|v| v.into_iter().map(Into::into).collect());
        self.ctx
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
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, NaiveDate } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.tradingSession())
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn trading_session(&self) -> Result<Vec<MarketTradingSession>> {
        self.ctx
            .trading_session()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, Market, NaiveDate } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.tradingDays(Market.HK, new NaiveDate(2022, 1, 20), new NaiveDate(2022, 2, 20)))
    ///   .then((resp) => console.log(resp.toString()))
    /// ```
    #[napi]
    pub async fn trading_days(
        &self,
        market: Market,
        begin: &NaiveDate,
        end: &NaiveDate,
    ) -> Result<MarketTradingDays> {
        self.ctx
            .trading_days(market.into(), begin.0, end.0)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get capital flow intraday
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.capitalFlow("700.HK"))
    ///   .then((resp) => {
    ///     for (let obj of resp) {
    ///       console.log(obj.toString())
    ///     }
    ///   })
    /// ```
    #[napi]
    pub async fn capital_flow(&self, symbol: String) -> Result<Vec<CapitalFlowLine>> {
        self.ctx
            .capital_flow(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get capital distribution
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.capitalDistribution("700.HK"))
    ///   .then((resp) => console.log(resp.toString()))
    /// ```
    #[napi]
    pub async fn capital_distribution(
        &self,
        symbol: String,
    ) -> Result<CapitalDistributionResponse> {
        self.ctx
            .capital_distribution(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get calc indexes
    #[napi]
    pub async fn calc_indexes(
        &self,
        symbols: Vec<String>,
        indexes: Vec<CalcIndex>,
    ) -> Result<Vec<SecurityCalcIndex>> {
        self.ctx
            .calc_indexes(symbols, indexes.into_iter().map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get watchlist
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv()
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.watchList())
    ///   .then((resp) => console.log(resp.toString()))
    /// ```
    #[napi]
    pub async fn watchlist(&self) -> Result<Vec<WatchlistGroup>> {
        self.ctx
            .watchlist()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Create watchlist group
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then((ctx) => {
    ///     ctx.createWatchlistGroup({
    ///       name: "Watchlist1",
    ///       securities: ["700.HK", "BABA.US"],
    ///     })
    ///   .then((group_id) => console.log(group_id));
    /// });
    #[napi]
    pub async fn create_watchlist_group(&self, req: CreateWatchlistGroup) -> Result<i64> {
        Ok(self
            .ctx
            .create_watchlist_group(req.into())
            .await
            .map_err(ErrorNewType)?)
    }

    /// Delete watchlist group
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then(ctx => ctx.deleteWatchlistGroup({ id: 10086 });
    /// ```
    #[napi]
    pub async fn delete_watchlist_group(&self, req: DeleteWatchlistGroup) -> Result<()> {
        Ok(self
            .ctx
            .delete_watchlist_group(req.id, req.purge)
            .await
            .map_err(ErrorNewType)?)
    }

    /// Update watchlist group
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext } = require("longport")
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then(ctx => ctx.updateWatchlistGroup({
    ///     id: 10086,
    ///     name: "Watchlist2",
    ///     securities: ["700.HK", "BABA.US"],
    ///   });
    /// ```
    #[napi]
    pub async fn update_watchlist_group(&self, req: UpdateWatchlistGroup) -> Result<()> {
        Ok(self
            .ctx
            .update_watchlist_group(req.into())
            .await
            .map_err(ErrorNewType)?)
    }

    /// Get security list
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, Market, SecurityListCategory } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.securityList(Market.US, SecurityListCategory.Overnight))
    ///   .then((resp) => console.log(resp.toString()));
    /// ```
    #[napi]
    pub async fn security_list(
        &self,
        market: Market,
        category: Option<SecurityListCategory>,
    ) -> Result<Vec<Security>> {
        self.ctx
            .security_list(market.into(), category.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get current market temperature
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, Market } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.marketTemperature(Market.HK))
    ///   .then((resp) => console.log(resp.toString()));
    /// ```
    #[napi]
    pub async fn market_temperature(&self, market: Market) -> Result<MarketTemperature> {
        self.ctx
            .market_temperature(market.into())
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get historical market temperature
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, Market, NaiveDate } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config)
    ///   .then((ctx) => ctx.historyMarketTemperature(Market.HK, new NaiveDate(2023, 1, 20), new NaiveDate(2023, 2, 20)))
    ///   .then((resp) => console.log(resp.toString()));
    /// ```
    #[napi]
    pub async fn history_market_temperature(
        &self,
        market: Market,
        start_date: &NaiveDate,
        end: &NaiveDate,
    ) -> Result<HistoryMarketTemperatureResponse> {
        self.ctx
            .history_market_temperature(market.into(), start_date.0, end.0)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time quote
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config).then((ctx) => {
    ///   ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote], true).then(() => {
    ///     setTimeout(() => {
    ///       ctx.realtimeQuote(["700.HK", "AAPL.US"]).then((resp) => {
    ///         for (let obj of resp) {
    ///           console.log(obj.toString());
    ///         }
    ///       });
    ///     }, 5000);
    ///   });
    /// });    
    /// ```
    #[napi]
    pub async fn realtime_quote(&self, symbols: Vec<String>) -> Result<Vec<RealtimeQuote>> {
        self.ctx
            .realtime_quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get real-time depth
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config).then((ctx) => {
    ///   ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Depth], true);
    ///   setTimeout(
    ///     () =>
    ///       ctx.realtimeDepth("700.HK").then((resp) => console.log(resp.toString())),
    ///     5000
    ///   );
    /// });    
    /// ```
    #[napi]
    pub async fn realtime_depth(&self, symbol: String) -> Result<SecurityDepth> {
        self.ctx
            .realtime_depth(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time brokers
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, NaiveDate, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config).then((ctx) => {
    ///   ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Brokers], true).then(() => {
    ///     setTimeout(
    ///       () =>
    ///         ctx
    ///           .realtimeBrokers("700.HK")
    ///           .then((resp) => console.log(resp.toString())),
    ///       5000
    ///     );
    ///   });
    /// });    
    /// ```
    #[napi]
    pub async fn realtime_brokers(&self, symbol: String) -> Result<SecurityBrokers> {
        self.ctx
            .realtime_brokers(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time trades
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, SubType } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config).then((ctx) => {
    ///   ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Trade], false).then(() => {
    ///     setTimeout(() => {
    ///       ctx.realtimeTrades("700.HK", 10).then((resp) => {
    ///         for (let obj of resp) {
    ///           console.log(obj.toString());
    ///         }
    ///       });
    ///     }, 5000);
    ///   });
    /// });    
    /// ```
    #[napi]
    pub async fn realtime_trades(&self, symbol: String, count: i32) -> Result<Vec<Trade>> {
        self.ctx
            .realtime_trades(symbol, count.max(0) as usize)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get real-time candlesticks
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, QuoteContext, Period } = require("longport")
    ///
    /// let config = Config.fromEnv();
    /// QuoteContext.new(config).then((ctx) => {
    ///   ctx.subscribeCandlesticks("700.HK", Period.Min_1).then(() => {
    ///     setTimeout(() => {
    ///       ctx.realtimeCandlesticks("700.HK", Period.Min_1, 10).then((resp) => {
    ///         for (let obj of resp) {
    ///           console.log(obj.toString());
    ///         }
    ///       });
    ///     }, 5000);
    ///   });
    /// });    
    /// ```
    #[napi]
    pub async fn realtime_candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: i32,
    ) -> Result<Vec<Candlestick>> {
        self.ctx
            .realtime_candlesticks(symbol, period.into(), count.max(0) as usize)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
