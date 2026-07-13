use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use longbridge_httpcli::{DcRegion, HttpClient, Json, Method};
use longbridge_proto::quote;
use longbridge_wscli::WsClientError;
use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime};
use tokio::sync::{mpsc, oneshot};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Error, Language, Market, Result,
    quote::{
        AdjustType, CalcIndex, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
        FilingItem, HistoryMarketTemperatureResponse, IntradayLine, IssuerInfo, MarketTemperature,
        MarketTradingDays, MarketTradingSession, OptionQuote, OptionVolumeDaily, OptionVolumeStats,
        ParticipantInfo, Period, PushEvent, QuotePackageDetail, RealtimeQuote,
        RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup, Security, SecurityBrokers,
        SecurityCalcIndex, SecurityDepth, SecurityListCategory, SecurityQuote, SecurityStaticInfo,
        ShortPositionsItem, ShortPositionsResponse, ShortTradesItem, ShortTradesResponse,
        StrikePriceInfo, Subscription, Trade, TradeSessions, WarrantInfo, WarrantQuote,
        WarrantType, WatchlistGroup,
        cache::{Cache, CacheWithKey},
        cmd_code,
        core::{Command, Core, UserProfile},
        sub_flags::SubFlags,
        types::{
            FilterWarrantExpiryDate, FilterWarrantInOutBoundsType, PinnedMode,
            SecuritiesUpdateMode, SortOrderType, WarrantSortBy, WarrantStatus,
        },
        utils::{format_date, parse_date},
    },
    serde_utils,
};

const RETRY_COUNT: usize = 3;
const PARTICIPANT_INFO_CACHE_TIMEOUT: Duration = Duration::from_secs(30 * 60);

/// Convert a Unix-seconds string (or integer string) to an RFC 3339 timestamp.
/// If parsing fails, the original string is returned unchanged.
fn unix_secs_to_rfc3339(s: &str) -> String {
    s.parse::<i64>()
        .ok()
        .and_then(|ts| time::OffsetDateTime::from_unix_timestamp(ts).ok())
        .map(|dt| {
            use time::format_description::well_known::Rfc3339;
            dt.format(&Rfc3339).unwrap_or_default()
        })
        .unwrap_or_else(|| s.to_string())
}
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
    user_profile: Arc<RwLock<Option<UserProfile>>>,
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
    pub fn new(config: Arc<Config>) -> (Self, mpsc::UnboundedReceiver<PushEvent>) {
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

        let language = config.language;
        let http_cli = config.create_http_client();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        let user_profile = Arc::new(RwLock::new(None::<UserProfile>));
        let core = Core::new(config, command_rx, push_tx, user_profile.clone());
        crate::runtime::RUNTIME
            .handle()
            .spawn(core.run().with_subscriber(log_subscriber.clone()));

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!("quote context created");
        });

        (
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
                user_profile,
                log_subscriber,
            })),
            push_rx,
        )
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    async fn ensure_user_profile(&self) -> Result<()> {
        if self.0.user_profile.read().unwrap().is_some() {
            return Ok(());
        }
        let (reply_tx, reply_rx) = oneshot::channel();
        self.0
            .command_tx
            .send(Command::EnsureConnected { reply_tx })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Returns the member ID
    pub async fn member_id(&self) -> Result<i64> {
        self.ensure_user_profile().await?;
        Ok(self
            .0
            .user_profile
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .member_id)
    }

    /// Returns the quote level
    pub async fn quote_level(&self) -> Result<String> {
        self.ensure_user_profile().await?;
        Ok(self
            .0
            .user_profile
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .quote_level
            .clone())
    }

    /// Returns the quote package details
    pub async fn quote_package_details(&self) -> Result<Vec<QuotePackageDetail>> {
        self.ensure_user_profile().await?;
        Ok(self
            .0
            .user_profile
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .quote_package_details
            .clone())
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/subscribe/subscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, mut receiver) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)
    ///     .await?;
    /// while let Some(msg) = receiver.recv().await {
    ///     println!("{:?}", msg);
    /// }
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn subscribe<I, T>(&self, symbols: I, sub_types: impl Into<SubFlags>) -> Result<()>
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
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe
    ///
    /// Reference: <https://open.longbridge.com/en/docs/quote/subscribe/unsubscribe>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{Period, QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, mut receiver) = QuoteContext::new(config);
    ///
    /// ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute, TradeSessions::Intraday)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/static>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/quote>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/option-quote>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/warrant-quote>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/depth>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/brokers>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// let resp = ctx.brokers("700.HK").await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn brokers(&self, symbol: impl Into<String>) -> Result<SecurityBrokers> {
        // Broker queue is served only by the AP data center; short-circuit a
        // non-AP session with the same unified error the HTTP path returns.
        let current = self.0.http_cli.dc_region().await;
        if !current.allows(longbridge_httpcli::DcRegion::Ap) {
            return Err(longbridge_httpcli::HttpClientError::DcRegionRestricted {
                path: "quote/brokers (WebSocket)".to_string(),
                required: longbridge_httpcli::DcRegion::Ap,
                current,
            }
            .into());
        }
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/broker-ids>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/trade>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/intraday>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// let resp = ctx.intraday("700.HK", TradeSessions::Intraday).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn intraday(
        &self,
        symbol: impl Into<String>,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<IntradayLine>> {
        let resp: quote::SecurityIntradayResponse = self
            .request(
                cmd_code::GET_SECURITY_INTRADAY,
                quote::SecurityIntradayRequest {
                    symbol: symbol.into(),
                    trade_session: trade_sessions as i32,
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/candlestick>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{AdjustType, Period, QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// let resp = ctx
    ///     .candlesticks(
    ///         "700.HK",
    ///         Period::Day,
    ///         10,
    ///         AdjustType::NoAdjust,
    ///         TradeSessions::Intraday,
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/optionchain-date>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/optionchain-date-strike>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    /// use time::macros::date;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/issuer>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/trade-session>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/trade-day>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, Market, oauth::OAuthBuilder, quote::QuoteContext};
    /// use time::macros::date;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/capital-flow-intraday>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{oauth::OAuthBuilder, quote::QuoteContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/capital-distribution>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{oauth::OAuthBuilder, quote::QuoteContext, Config};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/individual/watchlist_groups>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/individual/watchlist_create_group>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, RequestCreateWatchlistGroup},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/individual/watchlist_delete_group>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
    /// Reference: <https://open.longbridge.com/en/docs/quote/individual/watchlist_update_group>
    /// Reference: <https://open.longbridge.com/en/docs/quote/individual/watchlist_update_group_securities>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, RequestUpdateWatchlistGroup},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
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
        category: impl Into<Option<SecurityListCategory>>,
    ) -> Result<Vec<Security>> {
        #[derive(Debug, Serialize)]
        struct Request {
            market: Market,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<SecurityListCategory>,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<Security>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/get_security_list")
            .query_params(Request {
                market,
                category: category.into(),
            })
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .list)
    }

    /// Get filings list
    pub async fn filings(&self, symbol: impl Into<String>) -> Result<Vec<FilingItem>> {
        #[derive(Debug, Serialize)]
        struct Request {
            symbol: String,
        }

        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<FilingItem>,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/filings")
            .query_params(Request {
                symbol: symbol.into(),
            })
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0
            .items)
    }

    /// Get current market temperature
    ///
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/market_temperature>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, Market, oauth::OAuthBuilder, quote::QuoteContext};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// let resp = ctx.market_temperature(Market::HK).await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn market_temperature(&self, market: Market) -> Result<MarketTemperature> {
        #[derive(Debug, Serialize)]
        struct Request {
            market: Market,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/market_temperature")
            .query_params(Request { market })
            .response::<Json<MarketTemperature>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get historical market temperature
    ///
    /// Reference: <https://open.longbridge.com/en/docs/quote/pull/history_market_temperature>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, Market, oauth::OAuthBuilder, quote::QuoteContext};
    /// use time::macros::date;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// let resp = ctx
    ///     .history_market_temperature(Market::HK, date!(2023 - 01 - 01), date!(2023 - 01 - 31))
    ///     .await?;
    /// println!("{:?}", resp);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn history_market_temperature(
        &self,
        market: Market,
        start_date: Date,
        end_date: Date,
    ) -> Result<HistoryMarketTemperatureResponse> {
        #[derive(Debug, Serialize)]
        struct Request {
            market: Market,
            start_date: String,
            end_date: String,
        }

        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/history_market_temperature")
            .query_params(Request {
                market,
                start_date: format_date(start_date),
                end_date: format_date(end_date),
            })
            .response::<Json<HistoryMarketTemperatureResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::DEPTH)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::TRADE)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{QuoteContext, SubFlags},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::BROKER)
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
    /// use longbridge::{
    ///     Config,
    ///     oauth::OAuthBuilder,
    ///     quote::{Period, QuoteContext, TradeSessions},
    /// };
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let (ctx, _) = QuoteContext::new(config);
    ///
    /// ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute, TradeSessions::Intraday)
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

    // ── short_positions ───────────────────────────────────────────

    /// Get short interest data for a US or HK security.
    ///
    /// Market is inferred from the symbol suffix:
    /// - `.HK` → `GET /v1/quote/short-positions/hk`
    /// - otherwise → `GET /v1/quote/short-positions/us`
    ///
    /// `count` controls the number of records returned (1–100, default 20).
    pub async fn short_positions(
        &self,
        symbol: impl Into<String>,
        count: u32,
    ) -> Result<ShortPositionsResponse> {
        use std::time::{SystemTime, UNIX_EPOCH};

        use crate::utils::counter::symbol_to_counter_id;

        let sym = symbol.into();
        let is_hk = sym.to_uppercase().ends_with(".HK");
        let path = if is_hk {
            "/v1/quote/short-positions/hk"
        } else {
            "/v1/quote/short-positions/us"
        };
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        #[derive(serde::Serialize)]
        struct Query {
            counter_id: String,
            last_timestamp: String,
            count: u32,
        }
        // Response: {"counter_id":"ST/US/AAPL","data":[{...}]}
        let outer: serde_json::Value = self
            .0
            .http_cli
            .request(Method::GET, path)
            .query_params(Query {
                counter_id: symbol_to_counter_id(&sym),
                last_timestamp: ts.to_string(),
                count,
            })
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0;
        let empty = vec![];
        let raw = outer["data"].as_array().unwrap_or(&empty);
        let data = raw
            .iter()
            .map(|v| {
                let ts_str = v["timestamp"].as_str().unwrap_or("").to_string();
                ShortPositionsItem {
                    timestamp: unix_secs_to_rfc3339(&ts_str),
                    rate: v["rate"].as_str().unwrap_or("").to_string(),
                    close: v["close"].as_str().unwrap_or("").to_string(),
                    current_shares_short: v["current_shares_short"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    avg_daily_share_volume: v["avg_daily_share_volume"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    days_to_cover: v["days_to_cover"].as_str().unwrap_or("").to_string(),
                    amount: v["amount"].as_str().unwrap_or("").to_string(),
                    balance: v["balance"].as_str().unwrap_or("").to_string(),
                    cost: v["cost"].as_str().unwrap_or("").to_string(),
                }
            })
            .collect();
        Ok(ShortPositionsResponse { data })
    }

    // ── option_volume ─────────────────────────────────────────────

    /// Get real-time option call/put volume for a security.
    ///
    /// Path: `GET /v1/quote/option-volume-stats`
    pub async fn option_volume(&self, symbol: impl Into<String>) -> Result<OptionVolumeStats> {
        use crate::utils::counter::symbol_to_counter_id;
        #[derive(serde::Serialize)]
        struct Query {
            underlying_counter_id: String,
        }
        let resp = self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/option-volume-stats")
            .query_params(Query {
                underlying_counter_id: symbol_to_counter_id(&symbol.into()),
            })
            .response::<Json<OptionVolumeStats>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(resp.0)
    }

    /// Get daily historical option volume for a security.
    ///
    /// Path: `GET /v1/quote/option-volume-stats/daily`
    pub async fn option_volume_daily(
        &self,
        symbol: impl Into<String>,
        timestamp: i64,
        count: u32,
    ) -> Result<OptionVolumeDaily> {
        use crate::utils::counter::symbol_to_counter_id;
        #[derive(serde::Serialize)]
        struct Query {
            counter_id: String,
            timestamp: i64,
            line_num: u32,
            direction: i32,
        }
        let resp = self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/option-volume-stats/daily")
            .query_params(Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                timestamp,
                line_num: count,
                direction: 1,
            })
            .response::<Json<OptionVolumeDaily>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(resp.0)
    }
    // ── short_trades ──────────────────────────────────────────────

    /// Get short trade records for a HK or US security.
    ///
    /// The API endpoint is auto-detected from the symbol suffix:
    /// `.HK` → `GET /v1/quote/short-trades/hk`,
    /// otherwise → `GET /v1/quote/short-trades/us`.
    pub async fn short_trades(
        &self,
        symbol: impl Into<String>,
        count: u32,
    ) -> Result<ShortTradesResponse> {
        use std::time::{SystemTime, UNIX_EPOCH};

        use crate::utils::counter::symbol_to_counter_id;
        #[derive(serde::Serialize)]
        struct Query {
            counter_id: String,
            last_timestamp: String,
            page_size: String,
        }
        let sym = symbol.into();
        let path = if sym.to_uppercase().ends_with(".HK") {
            "/v1/quote/short-trades/hk"
        } else {
            "/v1/quote/short-trades/us"
        };
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        // Response: {"counter_id":"ST/HK/700","data":[{...}]}
        let outer: serde_json::Value = self
            .0
            .http_cli
            .request(Method::GET, path)
            .query_params(Query {
                counter_id: symbol_to_counter_id(&sym),
                last_timestamp: ts.to_string(),
                page_size: count.to_string(),
            })
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0;
        let empty = vec![];
        let raw = outer["data"].as_array().unwrap_or(&empty);
        let data = raw
            .iter()
            .map(|v| {
                let ts_str = v["timestamp"].as_str().unwrap_or("").to_string();
                ShortTradesItem {
                    timestamp: unix_secs_to_rfc3339(&ts_str),
                    rate: v["rate"].as_str().unwrap_or("").to_string(),
                    close: v["close"].as_str().unwrap_or("").to_string(),
                    nus_amount: v["nus_amount"].as_str().unwrap_or("").to_string(),
                    ny_amount: v["ny_amount"].as_str().unwrap_or("").to_string(),
                    total_amount: v["total_amount"].as_str().unwrap_or("").to_string(),
                    amount: v["amount"].as_str().unwrap_or("").to_string(),
                    balance: v["balance"].as_str().unwrap_or("").to_string(),
                }
            })
            .collect();
        Ok(ShortTradesResponse { data })
    }

    // ── update_pinned ─────────────────────────────────────────────

    /// Pin or unpin watchlist securities.
    ///
    /// Path: `POST /v1/watchlist/pinned`
    pub async fn update_pinned(&self, mode: PinnedMode, symbols: Vec<String>) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Request {
            mode: PinnedMode,
            securities: Vec<String>,
        }

        self.0
            .http_cli
            .request(Method::POST, "/v1/watchlist/pinned")
            .body(Json(Request {
                mode,
                securities: symbols,
            }))
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;

        Ok(())
    }

    // ── symbol_to_counter_ids ─────────────────────────────────────

    /// Batch convert symbols to counter IDs via the remote API.
    ///
    /// Returns a map of `symbol → counter_id` (e.g. `DRAM.US` →
    /// `ETF/US/DRAM`). Symbols the backend does not recognize are omitted
    /// from the result.
    ///
    /// Path: `POST /v1/quote/symbol-to-counter-ids`
    pub async fn symbol_to_counter_ids(
        &self,
        symbols: Vec<String>,
    ) -> Result<HashMap<String, String>> {
        #[derive(Debug, Serialize)]
        struct Request {
            ticker_regions: Vec<String>,
        }
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(default)]
            list: HashMap<String, String>,
        }

        let resp = self
            .0
            .http_cli
            .request(Method::POST, "/v1/quote/symbol-to-counter-ids")
            .body(Json(Request {
                ticker_regions: symbols,
            }))
            .response::<Json<Response>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(resp.0.list)
    }

    /// Resolve counter IDs for symbols, local-first with remote fallback.
    ///
    /// Symbols found in the embedded ETF / index / warrant directory (or in
    /// the local cache of previous remote resolutions) are resolved without
    /// network access. The remaining symbols are resolved in one batch via
    /// [`symbol_to_counter_ids`](Self::symbol_to_counter_ids) and the results
    /// are persisted to the local cache for subsequent lookups. Symbols the
    /// backend does not recognize fall back to the default `ST/` conversion.
    pub async fn resolve_counter_ids(
        &self,
        symbols: Vec<String>,
    ) -> Result<HashMap<String, String>> {
        use crate::utils::counter;

        let mut result = HashMap::with_capacity(symbols.len());
        let mut unknown = Vec::new();
        for symbol in symbols {
            match counter::lookup_counter_id(&symbol) {
                Some(counter_id) => {
                    result.insert(symbol, counter_id);
                }
                None => unknown.push(symbol),
            }
        }
        if !unknown.is_empty() {
            let resolved = self.symbol_to_counter_ids(unknown.clone()).await?;
            counter::cache_counter_ids(resolved.values().map(String::as_str));
            for symbol in unknown {
                let counter_id = resolved
                    .get(&symbol)
                    .cloned()
                    .unwrap_or_else(|| counter::symbol_to_counter_id(&symbol));
                result.insert(symbol, counter_id);
            }
        }
        Ok(result)
    }

    // ── US-market APIs ────────────────────────────────────────────────────────

    /// Get cryptocurrency market overview.
    ///
    /// `symbol` must be in `PAIR.EXCHANGE` format.
    /// US DC uses the **BKKT** exchange: e.g. `"BTCUSD.BKKT"` →
    /// `"VA/BKKT/BTCUSD"`. Pass the exchange suffix explicitly.
    ///
    /// Path: `GET /v1/us/gemini/crypto-overview`
    ///
    /// US token required.
    pub async fn us_crypto_overview(
        &self,
        symbol: impl Into<String>,
    ) -> Result<crate::quote::USCryptoOverview> {
        use crate::utils::counter::symbol_to_counter_id;
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/us/gemini/crypto-overview")
            .dc_restrict(DcRegion::Us)
            .query_params(Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            })
            .response::<Json<crate::quote::USCryptoOverview>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}

fn normalize_symbol(symbol: &str) -> &str {
    match symbol.split_once('.') {
        Some((_, market)) if market.eq_ignore_ascii_case("HK") => symbol.trim_start_matches('0'),
        _ => symbol,
    }
}
