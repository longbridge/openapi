use std::sync::Arc;

use longbridge::quote::PushEventDetail;
use napi::{threadsafe_function::ThreadsafeFunctionCallMode, JsFunction, Result};

use crate::{
    config::Config,
    error::ErrorNewType,
    quote::{
        push::{PushBrokersEvent, PushDepthEvent, PushQuoteEvent, PushTradesEvent},
        types::{
            AdjustType, Candlestick, IntradayLine, IssuerInfo, MarketTradingDays,
            MarketTradingSession, OptionQuote, ParticipantInfo, Period, RealtimeQuote,
            SecurityBrokers, SecurityDepth, SecurityQuote, SecurityStaticInfo, StrikePriceInfo,
            SubType, SubTypes, Trade, WarrantQuote,
        },
    },
    time::NaiveDate,
    types::Market,
    utils::JsCallback,
};

/// Quote context
#[napi_derive::napi]
#[derive(Clone)]
pub struct QuoteContext {
    ctx: longbridge::quote::QuoteContext,
    on_quote: JsCallback<PushQuoteEvent>,
    on_depth: JsCallback<PushDepthEvent>,
    on_brokers: JsCallback<PushBrokersEvent>,
    on_trades: JsCallback<PushTradesEvent>,
}

#[napi_derive::napi]
impl QuoteContext {
    #[napi]
    pub async fn new(config: &Config) -> Result<QuoteContext> {
        let (ctx, mut receiver) =
            longbridge::quote::QuoteContext::try_new(Arc::new(config.0.clone()))
                .await
                .map_err(ErrorNewType)?;
        let js_ctx = Self {
            ctx,
            on_quote: Default::default(),
            on_depth: Default::default(),
            on_brokers: Default::default(),
            on_trades: Default::default(),
        };

        tokio::spawn({
            let js_ctx = js_ctx.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    match msg.detail {
                        PushEventDetail::Quote(quote) => {
                            if let Some(handler) = js_ctx.on_quote.lock().clone() {
                                if let Ok(quote) = quote.try_into() {
                                    handler.call(
                                        Ok(PushQuoteEvent {
                                            symbol: msg.symbol,
                                            data: quote,
                                        }),
                                        ThreadsafeFunctionCallMode::Blocking,
                                    );
                                }
                            }
                        }
                        PushEventDetail::Depth(depth) => {
                            if let Some(handler) = js_ctx.on_depth.lock().clone() {
                                if let Ok(depth) = depth.try_into() {
                                    handler.call(
                                        Ok(PushDepthEvent {
                                            symbol: msg.symbol,
                                            data: depth,
                                        }),
                                        ThreadsafeFunctionCallMode::Blocking,
                                    );
                                }
                            }
                        }
                        PushEventDetail::Brokers(brokers) => {
                            if let Some(handler) = js_ctx.on_brokers.lock().clone() {
                                if let Ok(brokers) = brokers.try_into() {
                                    handler.call(
                                        Ok(PushBrokersEvent {
                                            symbol: msg.symbol,
                                            data: brokers,
                                        }),
                                        ThreadsafeFunctionCallMode::Blocking,
                                    );
                                }
                            }
                        }
                        PushEventDetail::Trade(trades) => {
                            if let Some(handler) = js_ctx.on_trades.lock().clone() {
                                if let Ok(trades) = trades.try_into() {
                                    handler.call(
                                        Ok(PushTradesEvent {
                                            symbol: msg.symbol,
                                            data: trades,
                                        }),
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
        ts_args_type = "callback: (err: null | Error, event: PushQuoteEvent) => void"
    )]
    pub fn on_quote(&mut self, handler: JsFunction) -> Result<()> {
        *self.on_quote.lock() =
            Some(handler.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    #[napi(
        setter,
        ts_args_type = "callback: (err: null | Error, event: PushDepthEvent) => void"
    )]
    pub fn on_depth(&mut self, handler: JsFunction) -> Result<()> {
        *self.on_depth.lock() =
            Some(handler.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    #[napi(
        setter,
        ts_args_type = "callback: (err: null | Error, event: PushBrokersEvent) => void"
    )]
    pub fn on_brokers(&mut self, handler: JsFunction) -> Result<()> {
        *self.on_brokers.lock() =
            Some(handler.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    #[napi(
        setter,
        ts_args_type = "callback: (err: null | Error, event: PushTradesEvent) => void"
    )]
    pub fn on_trades(&mut self, handler: JsFunction) -> Result<()> {
        *self.on_trades.lock() =
            Some(handler.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    /// Subscribe
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
    #[napi]
    pub async fn unsubscribe(&self, symbols: Vec<String>, sub_types: Vec<SubType>) -> Result<()> {
        self.ctx
            .unsubscribe(symbols, SubTypes(sub_types))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get basic information of securities
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
    #[napi]
    pub async fn depth(&self, symbol: String) -> Result<SecurityDepth> {
        self.ctx
            .depth(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get security brokers
    #[napi]
    pub async fn brokers(&self, symbol: String) -> Result<SecurityBrokers> {
        self.ctx
            .brokers(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get participants
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
    #[napi]
    pub async fn intraday(&self, symbol: String) -> Result<Vec<IntradayLine>> {
        self.ctx
            .intraday(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security candlesticks
    #[napi]
    pub async fn candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: i32,
        adjust_type: AdjustType,
    ) -> Result<Vec<Candlestick>> {
        self.ctx
            .candlesticks(
                symbol,
                period.into(),
                count.max(0) as usize,
                adjust_type.into(),
            )
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get option chain expiry date list
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

    /// Get trading session of the day
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

    /// Get real-time quote
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
    #[napi]
    pub async fn realtime_depth(&self, symbol: String) -> Result<SecurityDepth> {
        self.ctx
            .realtime_depth(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time brokers
    #[napi]
    pub async fn realtime_brokers(&self, symbol: String) -> Result<SecurityBrokers> {
        self.ctx
            .realtime_brokers(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time trades
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
}
