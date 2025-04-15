use std::{sync::Arc, time::Duration};

use longport_httpcli::{HttpClient, Json, Method};
use longport_proto::quote;
use longport_wscli::WsClientError;
use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime};
use tokio::sync::{mpsc, oneshot};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Error, Language, Market, Result,
    quote::{
        AdjustType, CalcIndex, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
        IntradayLine, IssuerInfo, MarketTradingDays, MarketTradingSession, OptionQuote,
        ParticipantInfo, Period, PushEvent, QuotePackageDetail, RealtimeQuote,
        RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup, Security, SecurityBrokers,
        SecurityCalcIndex, SecurityDepth, SecurityListCategory, SecurityQuote, SecurityStaticInfo,
        StrikePriceInfo, Subscription, Trade, TradeSessions, WarrantInfo, WarrantQuote,
        WarrantType, WatchlistGroup,
        cache::{Cache, CacheWithKey},
        cmd_code,
        core::{Command, Core},
        sub_flags::SubFlags,
        types::{
            FilterWarrantExpiryDate, FilterWarrantInOutBoundsType, SecuritiesUpdateMode,
            SortOrderType, WarrantSortBy, WarrantStatus,
        },
        utils::{format_date, parse_date},
    },
    serde_utils,
};

const RETRY_COUNT: usize = 3;
const PARTICIPANT_INFO_CACHE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const ISSUER_INFO_CACHE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const OPTION_CHAIN_EXPIRY_DATE_LIST_CACHE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const OPTION_CHAIN_STRIKE_INFO_CACHE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const TRADING_SESSION_CACHE_TIMEOUT: Duration = Duration::from_secs(60 * 60 * 2);

struct InnerQuoteContext {
    language: Language,
    http_cli: HttpClient,
    command_tx: mpsc::UnboundedSender<Command>,
    cache_participants: Cache<Vec<ParticipantInfo>>,
    cache_issuers: Cache<Vec<IssuerInfo>>,
    cache_option_chain_expiry_date_list: CacheWithKey<String, Vec<Date>>,
    cache_option_chain_strike_info: CacheWithKey<(String, Date), Vec<StrikePriceInfo>>,
    cache_trading_session: Cache<Vec<MarketTradingSession>>,
    member_id: i64,
    quote_level: String,
    quote_package_details: Vec<QuotePackageDetail>,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerQuoteContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("quote context dropped");
        });
    }
}

/// Quote context
#[derive(Clone)]
pub struct QuoteContext(Arc<InnerQuoteContext>);

impl QuoteContext {
    /// Create a `QuoteContext`
    pub async fn try_new(
        config: Arc<Config>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<PushEvent>)> {
        let log_subscriber = config.create_log_subscriber("quote");

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(
                language = ?config.language,
                enable_overnight = ?config.enable_overnight,
                push_candlestick_mode = ?config.push_candlestick_mode,
                enable_print_quote_packages = ?config.enable_print_quote_packages,
                "creating quote context"
            );
        });

        let language = config.language.unwrap_or_default();
        let http_cli = config.create_http_client();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        let core = Core::try_new(config, command_rx, push_tx)
            .with_subscriber(log_subscriber.clone())
            .await?;
        let member_id = core.member_id();
        let quote_level = core.quote_level().to_string();
        let quote_package_details = core.quote_package_details().to_vec();
        tokio::spawn(core.run().with_subscriber(log_subscriber.clone()));

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("quote context created");
        });

        Ok((
            QuoteContext(Arc::new(InnerQuoteContext {
                language,
                http_cli,
                command_tx,
                cache_participants: Cache::new(PARTICIPANT_INFO_CACHE_TIMEOUT),
                cache_issuers: Cache::new(ISSUER_INFO_CACHE_TIMEOUT),
                cache_option_chain_expiry_date_list: CacheWithKey::new(
                    OPTION_CHAIN_EXPIRY_DATE_LIST_CACHE_TIMEOUT,
                ),
                cache_option_chain_strike_info: CacheWithKey::new(
                    OPTION_CHAIN_STRIKE_INFO_CACHE_TIMEOUT,
                ),
                cache_trading_session: Cache::new(TRADING_SESSION_CACHE_TIMEOUT),
                member_id,
                quote_level,
                quote_package_details,
                log_subscriber,
            })),
            push_rx,
        ))
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Returns the member ID
    #[inline]
    pub fn member_id(&self) -> i64 {
        self.0.member_id
    }

    /// Returns the quote level
    #[inline]
    pub fn quote_level(&self) -> &str {
        &self.0.quote_level
    }

    /// Returns the quote package details
    #[inline]
    pub fn quote_package_details(&self) -> &[QuotePackageDetail] {
        &self.0.quote_package_details
    }

    /// Send a raw request
    async fn request_raw(&self, command_code: u8, body: Vec<u8>) -> Result<Vec<u8>> {
        for _ in 0..RETRY_COUNT {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.0
                .command_tx
                .send(Command::Request {
                    command_code,
                    body: body.clone(),
                    reply_tx,
                })
                .map_err(|_| WsClientError::ClientClosed)?;
            let res = reply_rx.await.map_err(|_| WsClientError::ClientClosed)?;

            match res {
                Ok(resp) => return Ok(resp),
                Err(Error::WsClient(WsClientError::Cancelled)) => {}
                Err(err) => return Err(err),
            }
        }

        Err(Error::WsClient(WsClientError::RequestTimeout))
    }

    /// Send a request `T` to get a response `R`
    async fn request<T, R>(&self, command_code: u8, req: T) -> Result<R>
    where
        T: prost::Message,
        R: prost::Message + Default,
    {
        let resp = self.request_raw(command_code, req.encode_to_vec()).await?;
        Ok(R::decode(&*resp)?)
    }

    /// Send a request to get a response `R`
    async fn request_without_body<R>(&self, command_code: u8) -> Result<R>
    where
        R: prost::Message + Default,
    {
        let resp = self.request_raw(command_code, vec![]).await?;
        Ok(R::decode(&*resp)?)
    }

    /// Subscribe
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/subscribe/subscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE, false)
    ///     .await?;
    /// while let Some(msg) = receiver.recv().await {
    ///     println!("{:?}", msg);
    /// }
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn subscribe<I, T>(
        &self,
        symbols: I,
        sub_types: impl Into<SubFlags>,
        is_first_push: bool,
    ) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Subscribe {
                symbols: symbols
                    .into_iter()
                    .map(|symbol| normalize_symbol(symbol.as_ref()).to_string())
                    .collect(),
                sub_types: sub_types.into(),
                is_first_push,
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/subscribe/unsubscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE, false)
    ///     .await?;
    /// ctx.unsubscribe(["AAPL.US"], SubFlags::QUOTE).await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn unsubscribe<I, T>(&self, symbols: I, sub_types: impl Into<SubFlags>) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Unsubscribe {
                symbols: symbols
                    .into_iter()
                    .map(|symbol| normalize_symbol(symbol.as_ref()).to_string())
                    .collect(),
                sub_types: sub_types.into(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Subscribe security candlesticks
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{Period, QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute, TradeSessions::Normal)
    ///     .await?;
    /// while let Some(msg) = receiver.recv().await {
    ///     println!("{:?}", msg);
    /// }
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn subscribe_candlesticks<T>(
        &self,
        symbol: T,
        period: Period,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>>
    where
        T: AsRef<str>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::SubscribeCandlesticks {
                symbol: normalize_symbol(symbol.as_ref()).into(),
                period,
                trade_sessions,
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe security candlesticks
    pub async fn unsubscribe_candlesticks<T>(&self, symbol: T, period: Period) -> Result<()>
    where
        T: AsRef<str>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::UnsubscribeCandlesticks {
                symbol: normalize_symbol(symbol.as_ref()).into(),
                period,
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Get subscription information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE, false)
    ///     .await?;
    /// let resp = ctx.subscriptions().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn subscriptions(&self) -> Result<Vec<Subscription>> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::Subscriptions { reply_tx })
            .map_err(|_| WsClientError::ClientClosed)?;
        Ok(reply_rx.await.map_err(|_| WsClientError::ClientClosed)?)
    }

    /// Get basic information of securities
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/static>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx
    ///     .static_info(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn static_info<I, T>(&self, symbols: I) -> Result<Vec<SecurityStaticInfo>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let resp: quote::SecurityStaticInfoResponse = self
            .request(
                cmd_code::GET_BASIC_INFO,
                quote::MultiSecurityRequest {
                    symbol: symbols.into_iter().map(Into::into).collect(),
                },
            )
            .await?;
        resp.secu_static_info
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of securities
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/quote>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx
    ///     .quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn quote<I, T>(&self, symbols: I) -> Result<Vec<SecurityQuote>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let resp: quote::SecurityQuoteResponse = self
            .request(
                cmd_code::GET_REALTIME_QUOTE,
                quote::MultiSecurityRequest {
                    symbol: symbols.into_iter().map(Into::into).collect(),
                },
            )
            .await?;
        resp.secu_quote.into_iter().map(TryInto::try_into).collect()
    }

    /// Get quote of option securities
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/option-quote>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.option_quote(["AAPL230317P160000.US"]).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn option_quote<I, T>(&self, symbols: I) -> Result<Vec<OptionQuote>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let resp: quote::OptionQuoteResponse = self
            .request(
                cmd_code::GET_REALTIME_OPTION_QUOTE,
                quote::MultiSecurityRequest {
                    symbol: symbols.into_iter().map(Into::into).collect(),
                },
            )
            .await?;
        resp.secu_quote.into_iter().map(TryInto::try_into).collect()
    }

    /// Get quote of warrant securities
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/warrant-quote>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.warrant_quote(["21125.HK"]).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn warrant_quote<I, T>(&self, symbols: I) -> Result<Vec<WarrantQuote>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let resp: quote::WarrantQuoteResponse = self
            .request(
                cmd_code::GET_REALTIME_WARRANT_QUOTE,
                quote::MultiSecurityRequest {
                    symbol: symbols.into_iter().map(Into::into).collect(),
                },
            )
            .await?;
        resp.secu_quote.into_iter().map(TryInto::try_into).collect()
    }

    /// Get security depth
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/depth>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.depth("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn depth(&self, symbol: impl Into<String>) -> Result<SecurityDepth> {
        let resp: quote::SecurityDepthResponse = self
            .request(
                cmd_code::GET_SECURITY_DEPTH,
                quote::SecurityRequest {
                    symbol: symbol.into(),
                },
            )
            .await?;
        Ok(SecurityDepth {
            asks: resp
                .ask
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            bids: resp
                .bid
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
        })
    }

    /// Get security brokers
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/brokers>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.brokers("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn brokers(&self, symbol: impl Into<String>) -> Result<SecurityBrokers> {
        let resp: quote::SecurityBrokersResponse = self
            .request(
                cmd_code::GET_SECURITY_BROKERS,
                quote::SecurityRequest {
                    symbol: symbol.into(),
                },
            )
            .await?;
        Ok(SecurityBrokers {
            ask_brokers: resp.ask_brokers.into_iter().map(Into::into).collect(),
            bid_brokers: resp.bid_brokers.into_iter().map(Into::into).collect(),
        })
    }

    /// Get participants
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/broker-ids>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.participants().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn participants(&self) -> Result<Vec<ParticipantInfo>> {
        self.0
            .cache_participants
            .get_or_update(|| async {
                let resp = self
                    .request_without_body::<quote::ParticipantBrokerIdsResponse>(
                        cmd_code::GET_BROKER_IDS,
                    )
                    .await?;

                Ok(resp
                    .participant_broker_numbers
                    .into_iter()
                    .map(Into::into)
                    .collect())
            })
            .await
    }

    /// Get security trades
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/trade>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.trades("700.HK", 10).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn trades(&self, symbol: impl Into<String>, count: usize) -> Result<Vec<Trade>> {
        let resp: quote::SecurityTradeResponse = self
            .request(
                cmd_code::GET_SECURITY_TRADES,
                quote::SecurityTradeRequest {
                    symbol: symbol.into(),
                    count: count as i32,
                },
            )
            .await?;
        let trades = resp
            .trades
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(trades)
    }

    /// Get security intraday lines
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/intraday>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.intraday("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn intraday(&self, symbol: impl Into<String>) -> Result<Vec<IntradayLine>> {
        let resp: quote::SecurityIntradayResponse = self
            .request(
                cmd_code::GET_SECURITY_INTRADAY,
                quote::SecurityIntradayRequest {
                    symbol: symbol.into(),
                },
            )
            .await?;
        let lines = resp
            .lines
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(lines)
    }

    /// Get security candlesticks
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/candlestick>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{AdjustType, Period, QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx
    ///     .candlesticks(
    ///         "700.HK",
    ///         Period::Day,
    ///         10,
    ///         AdjustType::NoAdjust,
    ///         TradeSessions::Normal,
    ///     )
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn candlesticks(
        &self,
        symbol: impl Into<String>,
        period: Period,
        count: usize,
        adjust_type: AdjustType,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        let resp: quote::SecurityCandlestickResponse = self
            .request(
                cmd_code::GET_SECURITY_CANDLESTICKS,
                quote::SecurityCandlestickRequest {
                    symbol: symbol.into(),
                    period: period.into(),
                    count: count as i32,
                    adjust_type: adjust_type.into(),
                    trade_session: trade_sessions as i32,
                },
            )
            .await?;
        let candlesticks = resp
            .candlesticks
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(candlesticks)
    }

    /// Get security history candlesticks by offset
    #[allow(clippy::too_many_arguments)]
    pub async fn history_candlesticks_by_offset(
        &self,
        symbol: impl Into<String>,
        period: Period,
        adjust_type: AdjustType,
        forward: bool,
        time: Option<PrimitiveDateTime>,
        count: usize,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        let resp: quote::SecurityCandlestickResponse = self
            .request(
                cmd_code::GET_SECURITY_HISTORY_CANDLESTICKS,
                quote::SecurityHistoryCandlestickRequest {
                    symbol: symbol.into(),
                    period: period.into(),
                    adjust_type: adjust_type.into(),
                    query_type: quote::HistoryCandlestickQueryType::QueryByOffset.into(),
                    offset_request: Some(
                        quote::security_history_candlestick_request::OffsetQuery {
                            direction: if forward {
                                quote::Direction::Forward
                            } else {
                                quote::Direction::Backward
                            }
                            .into(),
                            date: time
                                .map(|time| {
                                    format!(
                                        "{:04}{:02}{:02}",
                                        time.year(),
                                        time.month() as u8,
                                        time.day()
                                    )
                                })
                                .unwrap_or_default(),
                            minute: time
                                .map(|time| format!("{:02}{:02}", time.hour(), time.minute()))
                                .unwrap_or_default(),
                            count: count as i32,
                        },
                    ),
                    date_request: None,
                    trade_session: trade_sessions as i32,
                },
            )
            .await?;
        let candlesticks = resp
            .candlesticks
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(candlesticks)
    }

    /// Get security history candlesticks by date
    pub async fn history_candlesticks_by_date(
        &self,
        symbol: impl Into<String>,
        period: Period,
        adjust_type: AdjustType,
        start: Option<Date>,
        end: Option<Date>,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        let resp: quote::SecurityCandlestickResponse = self
            .request(
                cmd_code::GET_SECURITY_HISTORY_CANDLESTICKS,
                quote::SecurityHistoryCandlestickRequest {
                    symbol: symbol.into(),
                    period: period.into(),
                    adjust_type: adjust_type.into(),
                    query_type: quote::HistoryCandlestickQueryType::QueryByDate.into(),
                    offset_request: None,
                    date_request: Some(quote::security_history_candlestick_request::DateQuery {
                        start_date: start
                            .map(|date| {
                                format!(
                                    "{:04}{:02}{:02}",
                                    date.year(),
                                    date.month() as u8,
                                    date.day()
                                )
                            })
                            .unwrap_or_default(),
                        end_date: end
                            .map(|date| {
                                format!(
                                    "{:04}{:02}{:02}",
                                    date.year(),
                                    date.month() as u8,
                                    date.day()
                                )
                            })
                            .unwrap_or_default(),
                    }),
                    trade_session: trade_sessions as i32,
                },
            )
            .await?;
        let candlesticks = resp
            .candlesticks
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(candlesticks)
    }

    /// Get option chain expiry date list
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/optionchain-date>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.option_chain_expiry_date_list("AAPL.US").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn option_chain_expiry_date_list(
        &self,
        symbol: impl Into<String>,
    ) -> Result<Vec<Date>> {
        self.0
            .cache_option_chain_expiry_date_list
            .get_or_update(symbol.into(), |symbol| async {
                let resp: quote::OptionChainDateListResponse = self
                    .request(
                        cmd_code::GET_OPTION_CHAIN_EXPIRY_DATE_LIST,
                        quote::SecurityRequest { symbol },
                    )
                    .await?;
                resp.expiry_date
                    .iter()
                    .map(|value| {
                        parse_date(value).map_err(|err| Error::parse_field_error("date", err))
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .await
    }

    /// Get option chain info by date
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/optionchain-date-strike>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    /// use time::macros::date;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx
    ///     .option_chain_info_by_date("AAPL.US", date!(2023 - 01 - 20))
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn option_chain_info_by_date(
        &self,
        symbol: impl Into<String>,
        expiry_date: Date,
    ) -> Result<Vec<StrikePriceInfo>> {
        self.0
            .cache_option_chain_strike_info
            .get_or_update(
                (symbol.into(), expiry_date),
                |(symbol, expiry_date)| async move {
                    let resp: quote::OptionChainDateStrikeInfoResponse = self
                        .request(
                            cmd_code::GET_OPTION_CHAIN_INFO_BY_DATE,
                            quote::OptionChainDateStrikeInfoRequest {
                                symbol,
                                expiry_date: format_date(expiry_date),
                            },
                        )
                        .await?;
                    resp.strike_price_info
                        .into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<_>>>()
                },
            )
            .await
    }

    /// Get warrant issuers
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/issuer>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.warrant_issuers().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn warrant_issuers(&self) -> Result<Vec<IssuerInfo>> {
        self.0
            .cache_issuers
            .get_or_update(|| async {
                let resp = self
                    .request_without_body::<quote::IssuerInfoResponse>(
                        cmd_code::GET_WARRANT_ISSUER_IDS,
                    )
                    .await?;
                Ok(resp.issuer_info.into_iter().map(Into::into).collect())
            })
            .await
    }

    /// Query warrant list
    #[allow(clippy::too_many_arguments)]
    pub async fn warrant_list(
        &self,
        symbol: impl Into<String>,
        sort_by: WarrantSortBy,
        sort_order: SortOrderType,
        warrant_type: Option<&[WarrantType]>,
        issuer: Option<&[i32]>,
        expiry_date: Option<&[FilterWarrantExpiryDate]>,
        price_type: Option<&[FilterWarrantInOutBoundsType]>,
        status: Option<&[WarrantStatus]>,
    ) -> Result<Vec<WarrantInfo>> {
        let resp = self
            .request::<_, quote::WarrantFilterListResponse>(
                cmd_code::GET_FILTERED_WARRANT,
                quote::WarrantFilterListRequest {
                    symbol: symbol.into(),
                    filter_config: Some(quote::FilterConfig {
                        sort_by: sort_by.into(),
                        sort_order: sort_order.into(),
                        sort_offset: 0,
                        sort_count: 0,
                        r#type: warrant_type
                            .map(|types| types.iter().map(|ty| (*ty).into()).collect())
                            .unwrap_or_default(),
                        issuer: issuer.map(|types| types.to_vec()).unwrap_or_default(),
                        expiry_date: expiry_date
                            .map(|e| e.iter().map(|e| (*e).into()).collect())
                            .unwrap_or_default(),
                        price_type: price_type
                            .map(|types| types.iter().map(|ty| (*ty).into()).collect())
                            .unwrap_or_default(),
                        status: status
                            .map(|status| status.iter().map(|status| (*status).into()).collect())
                            .unwrap_or_default(),
                    }),
                    language: self.0.language.into(),
                },
            )
            .await?;
        resp.warrant_list
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()
    }

    /// Get trading session of the day
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/trade-session>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.trading_session().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn trading_session(&self) -> Result<Vec<MarketTradingSession>> {
        self.0
            .cache_trading_session
            .get_or_update(|| async {
                let resp = self
                    .request_without_body::<quote::MarketTradePeriodResponse>(
                        cmd_code::GET_TRADING_SESSION,
                    )
                    .await?;
                resp.market_trade_session
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>>>()
            })
            .await
    }

    /// Get market trading days
    ///
    /// The interval must be less than one month, and only the most recent year
    /// is supported.
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/trade-day>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, Market, quote::QuoteContext};
    /// use time::macros::date;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx
    ///     .trading_days(Market::HK, date!(2022 - 01 - 20), date!(2022 - 02 - 20))
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn trading_days(
        &self,
        market: Market,
        begin: Date,
        end: Date,
    ) -> Result<MarketTradingDays> {
        let resp = self
            .request::<_, quote::MarketTradeDayResponse>(
                cmd_code::GET_TRADING_DAYS,
                quote::MarketTradeDayRequest {
                    market: market.to_string(),
                    beg_day: format_date(begin),
                    end_day: format_date(end),
                },
            )
            .await?;
        let trading_days = resp
            .trade_day
            .iter()
            .map(|value| {
                parse_date(value).map_err(|err| Error::parse_field_error("trade_day", err))
            })
            .collect::<Result<Vec<_>>>()?;
        let half_trading_days = resp
            .half_trade_day
            .iter()
            .map(|value| {
                parse_date(value).map_err(|err| Error::parse_field_error("half_trade_day", err))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(MarketTradingDays {
            trading_days,
            half_trading_days,
        })
    }

    /// Get capital flow intraday
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/capital-flow-intraday>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{quote::QuoteContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.capital_flow("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    pub async fn capital_flow(&self, symbol: impl Into<String>) -> Result<Vec<CapitalFlowLine>> {
        self.request::<_, quote::CapitalFlowIntradayResponse>(
            cmd_code::GET_CAPITAL_FLOW_INTRADAY,
            quote::CapitalFlowIntradayRequest {
                symbol: symbol.into(),
            },
        )
        .await?
        .capital_flow_lines
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }

    /// Get capital distribution
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/pull/capital-distribution>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{quote::QuoteContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.capital_distribution("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    pub async fn capital_distribution(
        &self,
        symbol: impl Into<String>,
    ) -> Result<CapitalDistributionResponse> {
        self.request::<_, quote::CapitalDistributionResponse>(
            cmd_code::GET_SECURITY_CAPITAL_DISTRIBUTION,
            quote::SecurityRequest {
                symbol: symbol.into(),
            },
        )
        .await?
        .try_into()
    }

    /// Get calc indexes
    pub async fn calc_indexes<I, T, J>(
        &self,
        symbols: I,
        indexes: J,
    ) -> Result<Vec<SecurityCalcIndex>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
        J: IntoIterator<Item = CalcIndex>,
    {
        let indexes = indexes.into_iter().collect::<Vec<CalcIndex>>();
        let resp: quote::SecurityCalcQuoteResponse = self
            .request(
                cmd_code::GET_CALC_INDEXES,
                quote::SecurityCalcQuoteRequest {
                    symbols: symbols.into_iter().map(Into::into).collect(),
                    calc_index: indexes
                        .iter()
                        .map(|i| quote::CalcIndex::from(*i).into())
                        .collect(),
                },
            )
            .await?;

        Ok(resp
            .security_calc_index
            .into_iter()
            .map(|resp| SecurityCalcIndex::from_proto(resp, &indexes))
            .collect())
    }

    /// Get watchlist
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/individual/watchlist_groups>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let resp = ctx.watchlist().await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn watchlist(&self) -> Result<Vec<WatchlistGroup>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            groups: Vec<WatchlistGroup>,
        }

        let resp = self
            .0
            .http_cli
            .request(Method::GET, "/v1/watchlist/groups")
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(resp.0.groups)
    }

    /// Create watchlist group
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/individual/watchlist_create_group>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, RequestCreateWatchlistGroup},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// let req = RequestCreateWatchlistGroup::new("Watchlist1").securities(["700.HK", "BABA.US"]);
    /// let group_id = ctx.create_watchlist_group(req).await?;
    /// println!("{}", group_id);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_watchlist_group(&self, req: RequestCreateWatchlistGroup) -> Result<i64> {
        #[derive(Debug, Serialize)]
        struct RequestCreate {
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            securities: Option<Vec<String>>,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(with = "serde_utils::int64_str")]
            id: i64,
        }

        let Json(Response { id }) = self
            .0
            .http_cli
            .request(Method::POST, "/v1/watchlist/groups")
            .body(Json(RequestCreate {
                name: req.name,
                securities: req.securities,
            }))
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;

        Ok(id)
    }

    /// Delete watchlist group
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/individual/watchlist_delete_group>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.delete_watchlist_group(10086, true).await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn delete_watchlist_group(&self, id: i64, purge: bool) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Request {
            id: i64,
            purge: bool,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::DELETE, "/v1/watchlist/groups")
            .query_params(Request { id, purge })
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?)
    }

    /// Update watchlist group
    ///
    /// Reference: <https://open.longportapp.com/en/docs/quote/individual/watchlist_update_group>
    /// Reference: <https://open.longportapp.com/en/docs/quote/individual/watchlist_update_group_securities>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, RequestUpdateWatchlistGroup},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    /// let req = RequestUpdateWatchlistGroup::new(10086)
    ///     .name("Watchlist2")
    ///     .securities(["700.HK", "BABA.US"]);
    /// ctx.update_watchlist_group(req).await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn update_watchlist_group(&self, req: RequestUpdateWatchlistGroup) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct RequestUpdate {
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            securities: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mode: Option<SecuritiesUpdateMode>,
        }

        self.0
            .http_cli
            .request(Method::PUT, "/v1/watchlist/groups")
            .body(Json(RequestUpdate {
                id: req.id,
                name: req.name,
                mode: req.securities.is_some().then_some(req.mode),
                securities: req.securities,
            }))
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;

        Ok(())
    }

    /// Get security list
    pub async fn security_list(
        &self,
        market: Market,
        category: SecurityListCategory,
    ) -> Result<Vec<Security>> {
        #[derive(Debug, Serialize)]
        struct Request {
            market: Market,
            category: SecurityListCategory,
        }

        #[derive(Debug, Deserialize)]
        struct Resposne {
            list: Vec<Security>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/get_security_list")
            .query_params(Request { market, category })
            .response::<Json<Resposne>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get real-time quotes
    ///
    /// Get real-time quotes of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE, true)
    ///     .await?;
    /// tokio::time::sleep(Duration::from_secs(5)).await;
    ///
    /// let resp = ctx.realtime_quote(["700.HK", "AAPL.US"]).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn realtime_quote<I, T>(&self, symbols: I) -> Result<Vec<RealtimeQuote>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::GetRealtimeQuote {
                symbols: symbols.into_iter().map(Into::into).collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        Ok(reply_rx.await.map_err(|_| WsClientError::ClientClosed)?)
    }

    /// Get real-time depth
    ///
    /// Get real-time depth of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::DEPTH, true)
    ///     .await?;
    /// tokio::time::sleep(Duration::from_secs(5)).await;
    ///
    /// let resp = ctx.realtime_depth("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn realtime_depth(&self, symbol: impl Into<String>) -> Result<SecurityDepth> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::GetRealtimeDepth {
                symbol: symbol.into(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        Ok(reply_rx.await.map_err(|_| WsClientError::ClientClosed)?)
    }

    /// Get real-time trades
    ///
    /// Get real-time trades of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::TRADE, false)
    ///     .await?;
    /// tokio::time::sleep(Duration::from_secs(5)).await;
    ///
    /// let resp = ctx.realtime_trades("700.HK", 10).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn realtime_trades(
        &self,
        symbol: impl Into<String>,
        count: usize,
    ) -> Result<Vec<Trade>> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::GetRealtimeTrade {
                symbol: symbol.into(),
                count,
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        Ok(reply_rx.await.map_err(|_| WsClientError::ClientClosed)?)
    }

    /// Get real-time broker queue
    ///
    ///
    /// Get real-time broker queue of the subscribed symbols, it always returns
    /// the data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::BROKER, true)
    ///     .await?;
    /// tokio::time::sleep(Duration::from_secs(5)).await;
    ///
    /// let resp = ctx.realtime_brokers("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn realtime_brokers(&self, symbol: impl Into<String>) -> Result<SecurityBrokers> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::GetRealtimeBrokers {
                symbol: symbol.into(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        Ok(reply_rx.await.map_err(|_| WsClientError::ClientClosed)?)
    }

    /// Get real-time candlesticks
    ///
    /// Get real-time candlesticks of the subscribed symbols, it always returns
    /// the data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     quote::{Period, QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = Arc::new(Config::from_env()?);
    /// let (ctx, _) = QuoteContext::try_new(config).await?;
    ///
    /// ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute, TradeSessions::Normal)
    ///     .await?;
    /// tokio::time::sleep(Duration::from_secs(5)).await;
    ///
    /// let resp = ctx
    ///     .realtime_candlesticks("AAPL.US", Period::OneMinute, 10)
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn realtime_candlesticks(
        &self,
        symbol: impl Into<String>,
        period: Period,
        count: usize,
    ) -> Result<Vec<Candlestick>> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::GetRealtimeCandlesticks {
                symbol: symbol.into(),
                period,
                count,
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        Ok(reply_rx.await.map_err(|_| WsClientError::ClientClosed)?)
    }
}

fn normalize_symbol(symbol: &str) -> &str {
    match symbol.split_once('.') {
        Some((_, market)) if market.eq_ignore_ascii_case("HK") => symbol.trim_start_matches('0'),
        _ => symbol,
    }
}
