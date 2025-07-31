use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use comfy_table::Table;
use itertools::Itertools;
use longport_candlesticks::{TradeSessionType, UpdateAction};
use longport_httpcli::HttpClient;
use longport_proto::quote::{
    self, AdjustType, MarketTradeDayRequest, MarketTradeDayResponse, MultiSecurityRequest, Period,
    PushQuoteTag, SecurityCandlestickRequest, SecurityCandlestickResponse,
    SecurityStaticInfoResponse, SubscribeRequest, UnsubscribeRequest,
};
use longport_wscli::{
    CodecType, Platform, ProtocolVersion, RateLimit, WsClient, WsClientError, WsEvent, WsSession,
};
use time::{Date, OffsetDateTime};
use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, Instant},
};

use crate::{
    Config, Error, Market, Result,
    config::PushCandlestickMode,
    quote::{
        Candlestick, PushCandlestick, PushEvent, PushEventDetail, PushQuote, PushTrades,
        RealtimeQuote, SecurityBoard, SecurityBrokers, SecurityDepth, Subscription, Trade,
        TradeSession, TradeSessions, cmd_code,
        store::{Candlesticks, Store, TailCandlestick},
        sub_flags::SubFlags,
        types::QuotePackageDetail,
        utils::{format_date, parse_date},
    },
};

const RECONNECT_DELAY: Duration = Duration::from_secs(2);

pub(crate) enum Command {
    Request {
        command_code: u8,
        body: Vec<u8>,
        reply_tx: oneshot::Sender<Result<Vec<u8>>>,
    },
    Subscribe {
        symbols: Vec<String>,
        sub_types: SubFlags,
        is_first_push: bool,
        reply_tx: oneshot::Sender<Result<()>>,
    },
    Unsubscribe {
        symbols: Vec<String>,
        sub_types: SubFlags,
        reply_tx: oneshot::Sender<Result<()>>,
    },
    SubscribeCandlesticks {
        symbol: String,
        period: Period,
        trade_sessions: TradeSessions,
        reply_tx: oneshot::Sender<Result<Vec<Candlestick>>>,
    },
    UnsubscribeCandlesticks {
        symbol: String,
        period: Period,
        reply_tx: oneshot::Sender<Result<()>>,
    },
    Subscriptions {
        reply_tx: oneshot::Sender<Vec<Subscription>>,
    },
    GetRealtimeQuote {
        symbols: Vec<String>,
        reply_tx: oneshot::Sender<Vec<RealtimeQuote>>,
    },
    GetRealtimeDepth {
        symbol: String,
        reply_tx: oneshot::Sender<SecurityDepth>,
    },
    GetRealtimeTrade {
        symbol: String,
        count: usize,
        reply_tx: oneshot::Sender<Vec<Trade>>,
    },
    GetRealtimeBrokers {
        symbol: String,
        reply_tx: oneshot::Sender<SecurityBrokers>,
    },
    GetRealtimeCandlesticks {
        symbol: String,
        period: Period,
        count: usize,
        reply_tx: oneshot::Sender<Vec<Candlestick>>,
    },
}

#[derive(Debug, Default)]
struct TradingDays {
    normal_days: HashMap<Market, HashSet<Date>>,
    half_days: HashMap<Market, HashSet<Date>>,
}

impl TradingDays {
    #[inline]
    fn half_days(&self, market: Market) -> Days {
        Days(self.half_days.get(&market))
    }
}

#[derive(Debug, Copy, Clone)]
struct Days<'a>(Option<&'a HashSet<Date>>);

impl longport_candlesticks::Days for Days<'_> {
    #[inline]
    fn contains(&self, date: Date) -> bool {
        match self.0 {
            Some(days) => days.contains(&date),
            None => false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct MarketPackageDetail {
    pub(crate) market: String,
    pub(crate) packages: Vec<QuotePackageDetail>,
    pub(crate) warning: String,
}

pub(crate) struct Core {
    config: Arc<Config>,
    rate_limit: Vec<(u8, RateLimit)>,
    command_rx: mpsc::UnboundedReceiver<Command>,
    push_tx: mpsc::UnboundedSender<PushEvent>,
    event_tx: mpsc::UnboundedSender<WsEvent>,
    event_rx: mpsc::UnboundedReceiver<WsEvent>,
    http_cli: HttpClient,
    ws_cli: WsClient,
    session: Option<WsSession>,
    close: bool,
    subscriptions: HashMap<String, SubFlags>,
    trading_days: TradingDays,
    store: Store,
    member_id: i64,
    quote_level: String,
    quote_package_details: Vec<QuotePackageDetail>,
    push_candlestick_mode: PushCandlestickMode,
}

impl Core {
    pub(crate) async fn try_new(
        config: Arc<Config>,
        command_rx: mpsc::UnboundedReceiver<Command>,
        push_tx: mpsc::UnboundedSender<PushEvent>,
    ) -> Result<Self> {
        let http_cli = config.create_http_client();
        let otp = http_cli.get_otp().await?;

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        tracing::info!("connecting to quote server");
        let (url, res) = config.create_quote_ws_request().await;
        let request = res.map_err(WsClientError::from)?;

        let mut ws_cli = WsClient::open(
            request,
            ProtocolVersion::Version1,
            CodecType::Protobuf,
            Platform::OpenAPI,
            event_tx.clone(),
            vec![],
        )
        .await?;

        tracing::info!(url = url, "quote server connected");

        let session = ws_cli.request_auth(otp, config.create_metadata()).await?;

        // fetch user profile
        let resp = ws_cli
            .request::<_, quote::UserQuoteProfileResponse>(
                cmd_code::QUERY_USER_QUOTE_PROFILE,
                None,
                quote::UserQuoteProfileRequest {
                    language: config.language.to_string(),
                },
            )
            .await?;
        let member_id = resp.member_id;
        let quote_level = resp.quote_level;
        let (quote_package_details, quote_package_details_by_market) = resp
            .quote_level_detail
            .map(|details| {
                Ok::<_, Error>((
                    details
                        .by_package_key
                        .into_values()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<_>>>()?,
                    details
                        .by_market_code
                        .into_iter()
                        .map(|(market, market_packages)| {
                            Ok(MarketPackageDetail {
                                market,
                                packages: market_packages
                                    .packages
                                    .into_iter()
                                    .map(TryInto::try_into)
                                    .collect::<Result<Vec<_>>>()?,
                                warning: market_packages.warning_msg,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                ))
            })
            .transpose()?
            .unwrap_or_default();
        let rate_limit: Vec<(u8, RateLimit)> = resp
            .rate_limit
            .iter()
            .map(|config| {
                (
                    config.command as u8,
                    RateLimit {
                        interval: Duration::from_secs(1),
                        initial: config.burst as usize,
                        max: config.burst as usize,
                        refill: config.limit as usize,
                    },
                )
            })
            .collect();
        ws_cli.set_rate_limit(rate_limit.clone());

        let current_trade_days = fetch_trading_days(&ws_cli).await?;
        let push_candlestick_mode = config.push_candlestick_mode.unwrap_or_default();

        let mut table = Table::new();
        for market_packages in quote_package_details_by_market {
            if market_packages.warning.is_empty() {
                table.add_row(vec![
                    market_packages.market,
                    market_packages
                        .packages
                        .into_iter()
                        .map(|package| package.name)
                        .join(", "),
                ]);
            } else {
                table.add_row(vec![market_packages.market, market_packages.warning]);
            }
        }

        if config.enable_print_quote_packages {
            println!("{table}");
        }

        tracing::info!(
            member_id = member_id,
            quote_level = quote_level,
            quote_package_details = ?quote_package_details,
            "quote context initialized",
        );

        Ok(Self {
            config,
            rate_limit,
            command_rx,
            push_tx,
            event_tx,
            event_rx,
            http_cli,
            ws_cli,
            session: Some(session),
            close: false,
            subscriptions: HashMap::new(),
            trading_days: current_trade_days,
            store: Store::default(),
            member_id,
            quote_level,
            quote_package_details,
            push_candlestick_mode,
        })
    }

    #[inline]
    pub(crate) fn member_id(&self) -> i64 {
        self.member_id
    }

    #[inline]
    pub(crate) fn quote_level(&self) -> &str {
        &self.quote_level
    }

    #[inline]
    pub(crate) fn quote_package_details(&self) -> &[QuotePackageDetail] {
        &self.quote_package_details
    }

    pub(crate) async fn run(mut self) {
        while !self.close {
            match self.main_loop().await {
                Ok(()) => return,
                Err(err) => tracing::error!(error = %err, "quote disconnected"),
            }

            loop {
                // reconnect
                tokio::time::sleep(RECONNECT_DELAY).await;

                tracing::info!("connecting to quote server");
                let (url, res) = self.config.create_quote_ws_request().await;
                let request = res.expect("BUG: failed to create quote ws request");

                match WsClient::open(
                    request,
                    ProtocolVersion::Version1,
                    CodecType::Protobuf,
                    Platform::OpenAPI,
                    self.event_tx.clone(),
                    self.rate_limit.clone(),
                )
                .await
                {
                    Ok(ws_cli) => self.ws_cli = ws_cli,
                    Err(err) => {
                        tracing::error!(error = %err, "failed to connect quote server");
                        continue;
                    }
                }

                tracing::info!(url = url, "quote server connected");

                // request new session
                match &self.session {
                    Some(session) if !session.is_expired() => {
                        match self
                            .ws_cli
                            .request_reconnect(&session.session_id, self.config.create_metadata())
                            .await
                        {
                            Ok(new_session) => self.session = Some(new_session),
                            Err(err) => {
                                self.session = None; // invalid session
                                tracing::error!(error = %err, "failed to request session id");
                                continue;
                            }
                        }
                    }
                    _ => {
                        let otp = match self.http_cli.get_otp().await {
                            Ok(otp) => otp,
                            Err(err) => {
                                tracing::error!(error = %err, "failed to request otp");
                                continue;
                            }
                        };

                        match self
                            .ws_cli
                            .request_auth(otp, self.config.create_metadata())
                            .await
                        {
                            Ok(new_session) => self.session = Some(new_session),
                            Err(err) => {
                                tracing::error!(error = %err, "failed to request session id");
                                continue;
                            }
                        }
                    }
                }

                // handle reconnect
                match self.resubscribe().await {
                    Ok(()) => break,
                    Err(err) => {
                        tracing::error!(error = %err, "failed to subscribe topics");
                        continue;
                    }
                }
            }
        }
    }

    async fn main_loop(&mut self) -> Result<()> {
        let mut update_trading_days_interval = tokio::time::interval_at(
            Instant::now() + Duration::from_secs(60 * 60 * 24),
            Duration::from_secs(60 * 60 * 24),
        );

        loop {
            tokio::select! {
                item = self.event_rx.recv() => {
                    match item {
                        Some(event) => self.handle_ws_event(event).await?,
                        None => unreachable!(),
                    }
                }
                item = self.command_rx.recv() => {
                    match item {
                        Some(command) => self.handle_command(command).await?,
                        None => {
                            self.close = true;
                            return Ok(());
                        }
                    }
                }
                _ = update_trading_days_interval.tick() => {
                    if let Ok(days) = fetch_trading_days(&self.ws_cli).await {
                        self.trading_days = days;
                    }
                }
            }
        }
    }

    async fn handle_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Request {
                command_code,
                body,
                reply_tx,
            } => self.handle_request(command_code, body, reply_tx).await,
            Command::Subscribe {
                symbols,
                sub_types,
                is_first_push,
                reply_tx,
            } => {
                let res = self
                    .handle_subscribe(symbols, sub_types, is_first_push)
                    .await;
                let _ = reply_tx.send(res);
                Ok(())
            }
            Command::Unsubscribe {
                symbols,
                sub_types,
                reply_tx,
            } => {
                let _ = reply_tx.send(self.handle_unsubscribe(symbols, sub_types).await);
                Ok(())
            }
            Command::SubscribeCandlesticks {
                symbol,
                period,
                trade_sessions,
                reply_tx,
            } => {
                let _ = reply_tx.send(
                    self.handle_subscribe_candlesticks(symbol, period, trade_sessions)
                        .await,
                );
                Ok(())
            }
            Command::UnsubscribeCandlesticks {
                symbol,
                period,
                reply_tx,
            } => {
                let _ = reply_tx.send(self.handle_unsubscribe_candlesticks(symbol, period).await);
                Ok(())
            }
            Command::Subscriptions { reply_tx } => {
                let res = self.handle_subscriptions().await;
                let _ = reply_tx.send(res);
                Ok(())
            }
            Command::GetRealtimeQuote { symbols, reply_tx } => {
                let _ = reply_tx.send(self.handle_get_realtime_quote(symbols));
                Ok(())
            }
            Command::GetRealtimeDepth { symbol, reply_tx } => {
                let _ = reply_tx.send(self.handle_get_realtime_depth(symbol));
                Ok(())
            }
            Command::GetRealtimeTrade {
                symbol,
                count,
                reply_tx,
            } => {
                let _ = reply_tx.send(self.handle_get_realtime_trades(symbol, count));
                Ok(())
            }
            Command::GetRealtimeBrokers { symbol, reply_tx } => {
                let _ = reply_tx.send(self.handle_get_realtime_brokers(symbol));
                Ok(())
            }
            Command::GetRealtimeCandlesticks {
                symbol,
                period,
                count,
                reply_tx,
            } => {
                let _ = reply_tx.send(self.handle_get_realtime_candlesticks(symbol, period, count));
                Ok(())
            }
        }
    }

    async fn handle_request(
        &mut self,
        command_code: u8,
        body: Vec<u8>,
        reply_tx: oneshot::Sender<Result<Vec<u8>>>,
    ) -> Result<()> {
        let res = self.ws_cli.request_raw(command_code, None, body).await;
        let _ = reply_tx.send(res.map_err(Into::into));
        Ok(())
    }

    async fn handle_subscribe(
        &mut self,
        symbols: Vec<String>,
        sub_types: SubFlags,
        is_first_push: bool,
    ) -> Result<()> {
        // send request
        let req = SubscribeRequest {
            symbol: symbols.clone(),
            sub_type: sub_types.into(),
            is_first_push,
        };
        self.ws_cli
            .request::<_, ()>(cmd_code::SUBSCRIBE, None, req)
            .await?;

        // update subscriptions
        for symbol in symbols {
            self.subscriptions
                .entry(symbol)
                .and_modify(|flags| *flags |= sub_types)
                .or_insert(sub_types);
        }

        Ok(())
    }

    async fn handle_unsubscribe(
        &mut self,
        symbols: Vec<String>,
        sub_types: SubFlags,
    ) -> Result<()> {
        tracing::info!(symbols = ?symbols, sub_types = ?sub_types, "unsubscribe");

        // send requests
        let mut st_group: HashMap<SubFlags, Vec<&str>> = HashMap::new();

        for symbol in &symbols {
            let mut st = sub_types;

            if let Some(candlesticks) = self
                .store
                .securities
                .get(symbol)
                .map(|data| &data.candlesticks)
            {
                if !candlesticks.is_empty() {
                    st.remove(SubFlags::QUOTE | SubFlags::TRADE);
                }
            }

            if !st.is_empty() {
                st_group.entry(st).or_default().push(symbol.as_ref());
            }
        }

        let requests = st_group
            .iter()
            .map(|(st, symbols)| UnsubscribeRequest {
                symbol: symbols.iter().map(ToString::to_string).collect(),
                sub_type: (*st).into(),
                unsub_all: false,
            })
            .collect::<Vec<_>>();

        for req in requests {
            self.ws_cli
                .request::<_, ()>(cmd_code::UNSUBSCRIBE, None, req)
                .await?;
        }

        // update subscriptions
        let mut remove_symbols = Vec::new();
        for symbol in &symbols {
            if let Some(cur_flags) = self.subscriptions.get_mut(symbol) {
                *cur_flags &= !sub_types;
                if cur_flags.is_empty() {
                    remove_symbols.push(symbol);
                }
            }
        }

        for symbol in remove_symbols {
            self.subscriptions.remove(symbol);
        }
        Ok(())
    }

    async fn handle_subscribe_candlesticks(
        &mut self,
        symbol: String,
        period: Period,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        tracing::info!(symbol = symbol, period = ?period, "subscribe candlesticks");

        if let Some(candlesticks) = self
            .store
            .securities
            .get_mut(&symbol)
            .and_then(|data| data.candlesticks.get_mut(&period))
            .filter(|candlesticks| candlesticks.trade_sessions == trade_sessions)
        {
            candlesticks.trade_sessions = trade_sessions;
            tracing::info!(symbol = symbol, period = ?period, trade_sessions = ?trade_sessions, "subscribed, returns candlesticks in memory");
            return Ok(candlesticks.candlesticks.clone());
        }

        tracing::info!(symbol = symbol, "fetch symbol board");

        let security_data = self.store.securities.entry(symbol.clone()).or_default();
        if security_data.board != SecurityBoard::Unknown {
            // update board
            let resp: SecurityStaticInfoResponse = self
                .ws_cli
                .request(
                    cmd_code::GET_BASIC_INFO,
                    None,
                    MultiSecurityRequest {
                        symbol: vec![symbol.clone()],
                    },
                )
                .await?;
            if resp.secu_static_info.is_empty() {
                return Err(Error::InvalidSecuritySymbol {
                    symbol: symbol.clone(),
                });
            }
            security_data.board = resp.secu_static_info[0].board.parse().unwrap_or_default();
        }

        tracing::info!(symbol = symbol, board = ?security_data.board, "got the symbol board");

        // pull candlesticks
        tracing::info!(symbol = symbol, period = ?period, "pull history candlesticks");
        let resp: SecurityCandlestickResponse = self
            .ws_cli
            .request(
                cmd_code::GET_SECURITY_CANDLESTICKS,
                None,
                SecurityCandlestickRequest {
                    symbol: symbol.clone(),
                    period: period.into(),
                    count: 1000,
                    adjust_type: AdjustType::NoAdjust.into(),
                    trade_session: trade_sessions as i32,
                },
            )
            .await?;
        tracing::info!(symbol = symbol, period = ?period, len = resp.candlesticks.len(), "got history candlesticks");

        let mut candlesticks = vec![];
        let mut tails = HashMap::new();

        for candlestick in resp.candlesticks {
            let candlestick: Candlestick = candlestick.try_into()?;
            let index = candlesticks.len();
            candlesticks.push(candlestick);
            tails.insert(
                candlestick.trade_session,
                TailCandlestick { index, candlestick },
            );
        }

        tracing::info!(symbol = symbol, period = ?period, count = candlesticks.len(), tails = ?tails, "candlesticks loaded");

        security_data
            .candlesticks
            .entry(period)
            .or_insert_with(|| Candlesticks {
                trade_sessions,
                candlesticks: candlesticks.clone(),
                tails,
            });

        // subscribe
        if self
            .subscriptions
            .get(&symbol)
            .copied()
            .unwrap_or_else(SubFlags::empty)
            .contains(SubFlags::QUOTE | SubFlags::TRADE)
        {
            return Ok(candlesticks);
        }

        tracing::info!(symbol = symbol, period = ?period, "subscribe quote for candlesticks");

        let req = SubscribeRequest {
            symbol: vec![symbol.clone()],
            sub_type: (SubFlags::QUOTE | SubFlags::TRADE).into(),
            is_first_push: true,
        };
        self.ws_cli
            .request::<_, ()>(cmd_code::SUBSCRIBE, None, req)
            .await?;

        tracing::info!(symbol = symbol, period = ?period, "subscribed quote for candlesticks");
        Ok(candlesticks)
    }

    async fn handle_unsubscribe_candlesticks(
        &mut self,
        symbol: String,
        period: Period,
    ) -> Result<()> {
        if let Some(periods) = self
            .store
            .securities
            .get_mut(&symbol)
            .map(|data| &mut data.candlesticks)
        {
            periods.remove(&period);

            let sub_flags = self
                .subscriptions
                .get(&symbol)
                .copied()
                .unwrap_or_else(SubFlags::empty);

            if periods.is_empty() && !sub_flags.intersects(SubFlags::QUOTE | SubFlags::TRADE) {
                tracing::info!(symbol = symbol, "unsubscribe quote for candlesticks");
                self.ws_cli
                    .request::<_, ()>(
                        cmd_code::UNSUBSCRIBE,
                        None,
                        UnsubscribeRequest {
                            symbol: vec![symbol],
                            sub_type: (SubFlags::QUOTE | SubFlags::TRADE).into(),
                            unsub_all: false,
                        },
                    )
                    .await?;
            }
        }

        Ok(())
    }

    async fn handle_subscriptions(&mut self) -> Vec<Subscription> {
        let mut subscriptions = HashMap::new();

        for (symbol, sub_flags) in &self.subscriptions {
            if sub_flags.is_empty() {
                continue;
            }

            subscriptions.insert(
                symbol.clone(),
                Subscription {
                    symbol: symbol.clone(),
                    sub_types: *sub_flags,
                    candlesticks: vec![],
                },
            );
        }

        for (symbol, data) in &self.store.securities {
            subscriptions
                .entry(symbol.clone())
                .or_insert_with(|| Subscription {
                    symbol: symbol.clone(),
                    sub_types: SubFlags::empty(),
                    candlesticks: vec![],
                })
                .candlesticks = data.candlesticks.keys().copied().collect();
        }

        subscriptions.into_values().collect()
    }

    async fn handle_ws_event(&mut self, event: WsEvent) -> Result<()> {
        match event {
            WsEvent::Error(err) => Err(err.into()),
            WsEvent::Push { command_code, body } => self.handle_push(command_code, body),
        }
    }

    async fn resubscribe(&mut self) -> Result<()> {
        let mut subscriptions: HashMap<SubFlags, HashSet<String>> = HashMap::new();

        for (symbol, flags) in &self.subscriptions {
            subscriptions
                .entry(*flags)
                .or_default()
                .insert(symbol.clone());
        }

        for (symbol, data) in &self.store.securities {
            if !data.candlesticks.is_empty() {
                subscriptions
                    .entry(SubFlags::QUOTE | SubFlags::TRADE)
                    .or_default()
                    .insert(symbol.clone());
            }
        }

        tracing::info!(subscriptions = ?subscriptions, "resubscribe");

        for (flags, symbols) in subscriptions {
            self.ws_cli
                .request::<_, ()>(
                    cmd_code::SUBSCRIBE,
                    None,
                    SubscribeRequest {
                        symbol: symbols.into_iter().collect(),
                        sub_type: flags.into(),
                        is_first_push: false,
                    },
                )
                .await?;
        }
        Ok(())
    }

    fn merge_candlesticks_by_quote(&mut self, symbol: &str, push_quote: &PushQuote) {
        if push_quote.trade_session != TradeSession::Intraday {
            return;
        }

        let Some(market_type) = parse_market_from_symbol(symbol) else {
            return;
        };
        let Some(security_data) = self.store.securities.get_mut(symbol) else {
            return;
        };
        let half_days = self.trading_days.half_days(market_type);

        if let Some(candlesticks) = security_data.candlesticks.get_mut(&Period::Day) {
            let action = candlesticks.merge_quote(
                market_type,
                half_days,
                security_data.board,
                Period::Day,
                push_quote,
            );
            update_and_push_candlestick(
                candlesticks,
                push_quote.trade_session,
                symbol,
                Period::Day,
                action,
                self.push_candlestick_mode,
                &mut self.push_tx,
            );
        }
    }

    fn merge_candlesticks_by_trades(&mut self, symbol: &str, push_trades: &PushTrades) {
        let Some(market_type) = parse_market_from_symbol(symbol) else {
            return;
        };
        let Some(security_data) = self.store.securities.get_mut(symbol) else {
            return;
        };
        let half_days = self.trading_days.half_days(market_type);

        for trade in &push_trades.trades {
            for (period, candlesticks) in &mut security_data.candlesticks {
                if *period >= Period::Day && !trade.trade_session.is_intraday() {
                    continue;
                }

                let action = candlesticks.merge_trade(
                    market_type,
                    half_days,
                    security_data.board,
                    *period,
                    trade,
                );
                update_and_push_candlestick(
                    candlesticks,
                    trade.trade_session,
                    symbol,
                    *period,
                    action,
                    self.push_candlestick_mode,
                    &mut self.push_tx,
                );
            }
        }
    }

    fn handle_push(&mut self, command_code: u8, body: Vec<u8>) -> Result<()> {
        match PushEvent::parse(command_code, &body) {
            Ok((mut event, tag)) => {
                tracing::info!(event = ?event, tag = ?tag, "push event");

                if tag != Some(PushQuoteTag::Eod) {
                    self.store.handle_push(&mut event);
                }

                if let PushEventDetail::Quote(push_quote) = &event.detail {
                    self.merge_candlesticks_by_quote(&event.symbol, push_quote);

                    if !self
                        .subscriptions
                        .get(&event.symbol)
                        .map(|sub_flags| sub_flags.contains(SubFlags::QUOTE))
                        .unwrap_or_default()
                    {
                        return Ok(());
                    }
                } else if let PushEventDetail::Trade(trades) = &event.detail {
                    self.merge_candlesticks_by_trades(&event.symbol, trades);

                    if !self
                        .subscriptions
                        .get(&event.symbol)
                        .map(|sub_flags| sub_flags.contains(SubFlags::TRADE))
                        .unwrap_or_default()
                    {
                        return Ok(());
                    }
                }

                if tag == Some(PushQuoteTag::Eod) {
                    return Ok(());
                }

                let _ = self.push_tx.send(event);
            }
            Err(err) => {
                tracing::error!(error = %err, "failed to parse push message");
            }
        }
        Ok(())
    }

    fn handle_get_realtime_quote(&self, symbols: Vec<String>) -> Vec<RealtimeQuote> {
        let mut result = Vec::new();

        for symbol in symbols {
            if let Some(data) = self.store.securities.get(&symbol) {
                result.push(RealtimeQuote {
                    symbol,
                    last_done: data.quote.last_done,
                    open: data.quote.open,
                    high: data.quote.high,
                    low: data.quote.low,
                    timestamp: data.quote.timestamp,
                    volume: data.quote.volume,
                    turnover: data.quote.turnover,
                    trade_status: data.quote.trade_status,
                });
            }
        }

        result
    }

    fn handle_get_realtime_depth(&self, symbol: String) -> SecurityDepth {
        let mut result = SecurityDepth::default();
        if let Some(data) = self.store.securities.get(&symbol) {
            result.asks.clone_from(&data.asks);
            result.bids.clone_from(&data.bids);
        }
        result
    }

    fn handle_get_realtime_trades(&self, symbol: String, count: usize) -> Vec<Trade> {
        let mut res = Vec::new();

        if let Some(data) = self.store.securities.get(&symbol) {
            let trades = if data.trades.len() >= count {
                &data.trades[data.trades.len() - count..]
            } else {
                &data.trades
            };
            res = trades.to_vec();
        }
        res
    }

    fn handle_get_realtime_brokers(&self, symbol: String) -> SecurityBrokers {
        let mut result = SecurityBrokers::default();
        if let Some(data) = self.store.securities.get(&symbol) {
            result.ask_brokers.clone_from(&data.ask_brokers);
            result.bid_brokers.clone_from(&data.bid_brokers);
        }
        result
    }

    fn handle_get_realtime_candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: usize,
    ) -> Vec<Candlestick> {
        self.store
            .securities
            .get(&symbol)
            .map(|data| &data.candlesticks)
            .and_then(|periods| periods.get(&period))
            .map(|candlesticks| {
                let candlesticks = if candlesticks.candlesticks.len() >= count {
                    &candlesticks.candlesticks[candlesticks.candlesticks.len() - count..]
                } else {
                    &candlesticks.candlesticks
                };
                candlesticks.to_vec()
            })
            .unwrap_or_default()
    }
}

async fn fetch_trading_days(cli: &WsClient) -> Result<TradingDays> {
    let mut days = TradingDays::default();
    let begin_day = OffsetDateTime::now_utc().date() - time::Duration::days(5);
    let end_day = begin_day + time::Duration::days(30);

    for market in [Market::HK, Market::US, Market::SG, Market::CN] {
        let resp = cli
            .request::<_, MarketTradeDayResponse>(
                cmd_code::GET_TRADING_DAYS,
                None,
                MarketTradeDayRequest {
                    market: market.to_string(),
                    beg_day: format_date(begin_day),
                    end_day: format_date(end_day),
                },
            )
            .await?;

        days.normal_days.insert(
            market,
            resp.trade_day
                .iter()
                .map(|value| {
                    parse_date(value).map_err(|err| Error::parse_field_error("half_trade_day", err))
                })
                .collect::<Result<HashSet<_>>>()?,
        );

        days.half_days.insert(
            market,
            resp.half_trade_day
                .iter()
                .map(|value| {
                    parse_date(value).map_err(|err| Error::parse_field_error("half_trade_day", err))
                })
                .collect::<Result<HashSet<_>>>()?,
        );
    }

    Ok(days)
}

#[allow(clippy::too_many_arguments)]
fn update_and_push_candlestick(
    candlesticks: &mut Candlesticks,
    ts: TradeSession,
    symbol: &str,
    period: Period,
    action: UpdateAction<Candlestick>,
    push_candlestick_mode: PushCandlestickMode,
    tx: &mut mpsc::UnboundedSender<PushEvent>,
) {
    let mut push_candlesticks = Vec::new();

    match action {
        UpdateAction::UpdateLast(candlestick) => {
            let tail = candlesticks.tails.get_mut(&ts).unwrap();
            candlesticks.candlesticks[tail.index] = candlestick;
            tail.candlestick = candlestick;

            if push_candlestick_mode == PushCandlestickMode::Realtime {
                push_candlesticks.push((candlestick, false));
            }
        }
        UpdateAction::AppendNew { confirmed, new } => {
            let index = if let Some(tail) = candlesticks.tails.get_mut(&ts) {
                candlesticks.candlesticks.insert(tail.index + 1, new);
                tail.index += 1;
                tail.candlestick = new;
                tail.index
            } else {
                let index = candlesticks.insert_candlestick_by_time(new);
                candlesticks.tails.insert(
                    ts,
                    TailCandlestick {
                        index,
                        candlestick: new,
                    },
                );
                index
            };

            for tail in candlesticks.tails.values_mut() {
                if tail.index > index {
                    tail.index += 1;
                }
            }

            candlesticks.check_and_remove();

            match push_candlestick_mode {
                PushCandlestickMode::Realtime => {
                    if let Some(confirmed) = confirmed {
                        push_candlesticks.push((confirmed, true));
                    }
                    push_candlesticks.push((new, false));
                }
                PushCandlestickMode::Confirmed => {
                    if let Some(confirmed) = confirmed {
                        push_candlesticks.push((confirmed, true));
                    }
                }
            }
        }
        UpdateAction::None => {}
    };

    for (candlestick, is_confirmed) in push_candlesticks {
        if candlesticks.trade_sessions.contains(ts) {
            tracing::info!(
                symbol = symbol,
                period = ?period,
                is_confirmed = is_confirmed,
                candlestick = ?candlestick,
                trade_session = ?ts,
                "push candlestick"
            );
            let _ = tx.send(PushEvent {
                sequence: 0,
                symbol: symbol.to_string(),
                detail: PushEventDetail::Candlestick(PushCandlestick {
                    period,
                    candlestick,
                    is_confirmed,
                }),
            });
        }
    }
}

fn parse_market_from_symbol(symbol: &str) -> Option<Market> {
    let market = symbol.rfind('.').map(|idx| &symbol[idx + 1..])?;
    Some(match market {
        "US" => Market::US,
        "HK" => Market::HK,
        "SG" => Market::SG,
        "SH" | "SZ" => Market::CN,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_market_from_symbol() {
        assert_eq!(parse_market_from_symbol("AAPL.US"), Some(Market::US));
        assert_eq!(parse_market_from_symbol("BRK.A.US"), Some(Market::US));
    }
}
