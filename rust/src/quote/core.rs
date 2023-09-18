use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use longbridge_candlesticks::{IsHalfTradeDay, Type, UpdateAction};
use longbridge_httpcli::HttpClient;
use longbridge_proto::quote::{
    AdjustType, MarketTradeDayRequest, MarketTradeDayResponse, MultiSecurityRequest, Period,
    SecurityCandlestickRequest, SecurityCandlestickResponse, SecurityStaticInfoResponse,
    SubscribeRequest, TradeSession, UnsubscribeRequest,
};
use longbridge_wscli::{
    CodecType, Platform, ProtocolVersion, RateLimit, WsClient, WsClientError, WsEvent, WsSession,
};
use time::{Date, OffsetDateTime};
use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, Instant},
};

use crate::{
    quote::{
        cmd_code,
        store::Store,
        sub_flags::SubFlags,
        utils::{format_date, get_market_from_symbol, parse_date},
        Candlestick, PushCandlestick, PushEvent, PushEventDetail, RealtimeQuote, SecurityBoard,
        SecurityBrokers, SecurityDepth, Subscription, Trade,
    },
    Config, Error, Market, Result,
};

const RECONNECT_DELAY: Duration = Duration::from_secs(2);
const MAX_CANDLESTICKS: usize = 500;

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
        reply_tx: oneshot::Sender<Result<()>>,
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
struct CurrentTradeDays {
    half_days: HashMap<Market, HashSet<Date>>,
}

impl CurrentTradeDays {
    #[inline]
    fn half_days(&self, market: longbridge_candlesticks::Market) -> HalfDays {
        use longbridge_candlesticks::Market::*;
        match market {
            HK => HalfDays(self.half_days.get(&Market::HK)),
            US => HalfDays(self.half_days.get(&Market::US)),
            SH | SZ => HalfDays(None),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct HalfDays<'a>(Option<&'a HashSet<Date>>);

impl<'a> IsHalfTradeDay for HalfDays<'a> {
    #[inline]
    fn is_half(&self, date: Date) -> bool {
        match self.0 {
            Some(days) => days.contains(&date),
            None => false,
        }
    }
}

pub(crate) struct Core {
    config: Arc<Config>,
    command_rx: mpsc::UnboundedReceiver<Command>,
    push_tx: mpsc::UnboundedSender<PushEvent>,
    event_tx: mpsc::UnboundedSender<WsEvent>,
    event_rx: mpsc::UnboundedReceiver<WsEvent>,
    http_cli: HttpClient,
    ws_cli: WsClient,
    session: Option<WsSession>,
    close: bool,
    subscriptions: HashMap<String, SubFlags>,
    current_trade_days: CurrentTradeDays,
    store: Store,
}

const fn rate_limit() -> RateLimit {
    RateLimit {
        interval: Duration::from_millis(100),
        initial: 5,
        max: 5,
        refill: 1,
    }
}

impl Core {
    pub(crate) async fn try_new(
        config: Arc<Config>,
        command_rx: mpsc::UnboundedReceiver<Command>,
        push_tx: mpsc::UnboundedSender<PushEvent>,
    ) -> Result<Self> {
        let http_cli = config.create_http_client();
        let otp = http_cli.get_otp_v2().await?;

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        tracing::debug!(
            url = config.quote_ws_url.as_str(),
            "connecting to quote server",
        );
        let ws_cli = WsClient::open(
            config
                .create_quote_ws_request()
                .map_err(WsClientError::from)?,
            ProtocolVersion::Version1,
            CodecType::Protobuf,
            Platform::OpenAPI,
            event_tx.clone(),
            Some(rate_limit()),
        )
        .await?;

        tracing::debug!(url = config.quote_ws_url.as_str(), "quote server connected");

        let session = ws_cli.request_auth(otp).await?;
        let current_trade_days = fetch_current_trade_days(&ws_cli).await?;

        Ok(Self {
            config,
            command_rx,
            push_tx,
            event_tx,
            event_rx,
            http_cli,
            ws_cli,
            session: Some(session),
            close: false,
            subscriptions: HashMap::new(),
            current_trade_days,
            store: Store::default(),
        })
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

                tracing::debug!(
                    url = self.config.quote_ws_url.as_str(),
                    "connecting to quote server",
                );

                match WsClient::open(
                    self.config.create_quote_ws_request().unwrap(),
                    ProtocolVersion::Version1,
                    CodecType::Protobuf,
                    Platform::OpenAPI,
                    self.event_tx.clone(),
                    Some(rate_limit()),
                )
                .await
                {
                    Ok(ws_cli) => self.ws_cli = ws_cli,
                    Err(err) => {
                        tracing::error!(error = %err, "failed to connect quote server");
                        continue;
                    }
                }

                tracing::debug!(
                    url = self.config.quote_ws_url.as_str(),
                    "quote server connected"
                );

                // request new session
                match &self.session {
                    Some(session) if !session.is_expired() => {
                        match self.ws_cli.request_reconnect(&session.session_id).await {
                            Ok(new_session) => self.session = Some(new_session),
                            Err(err) => {
                                self.session = None; // invalid session
                                tracing::error!(error = %err, "failed to request session id");
                                continue;
                            }
                        }
                    }
                    _ => {
                        let otp = match self.http_cli.get_otp_v2().await {
                            Ok(otp) => otp,
                            Err(err) => {
                                tracing::error!(error = %err, "failed to request otp");
                                continue;
                            }
                        };

                        match self.ws_cli.request_auth(otp).await {
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

    #[tracing::instrument(level = "debug", skip(self))]
    async fn main_loop(&mut self) -> Result<()> {
        let mut update_trade_days_interval = tokio::time::interval_at(
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
                _ = update_trade_days_interval.tick() => {
                    if let Ok(days) = fetch_current_trade_days(&self.ws_cli).await {
                        self.current_trade_days = days;
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
                reply_tx,
            } => {
                let _ = reply_tx.send(self.handle_subscribe_candlesticks(symbol, period).await);
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
        let ws_cli = self.ws_cli.clone();
        let res = ws_cli.request_raw(command_code, None, body).await;
        let _ = reply_tx.send(res.map_err(Into::into));
        Ok(())
    }

    async fn handle_subscribe(
        &mut self,
        symbols: Vec<String>,
        sub_types: SubFlags,
        is_first_push: bool,
    ) -> Result<()> {
        tracing::debug!(symbols = ?symbols, sub_types = ?sub_types, "subscribe");

        // send request
        let req = SubscribeRequest {
            symbol: symbols.clone(),
            sub_type: sub_types.into(),
            is_first_push,
        };
        self.ws_cli.request(cmd_code::SUBSCRIBE, None, req).await?;

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
        tracing::debug!(symbols = ?symbols, sub_types = ?sub_types, "unsubscribe");

        // send requests
        let mut st_group: HashMap<SubFlags, Vec<&str>> = HashMap::new();

        for symbol in &symbols {
            let mut st = sub_types;
            if self
                .store
                .securities
                .get(symbol)
                .map(|data| !data.candlesticks.is_empty())
                .unwrap_or_default()
            {
                st.remove(SubFlags::TRADE);
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
                .request(cmd_code::UNSUBSCRIBE, None, req)
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
    ) -> Result<()> {
        if self
            .store
            .securities
            .get(&symbol)
            .map(|data| data.candlesticks.contains_key(&period))
            .unwrap_or_default()
        {
            return Ok(());
        }

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

        // pull candlesticks
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
                },
            )
            .await?;

        *security_data.candlesticks.entry(period).or_default() = resp
            .candlesticks
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;

        // subscribe trades
        if self
            .subscriptions
            .get(&symbol)
            .copied()
            .unwrap_or_else(SubFlags::empty)
            .contains(SubFlags::TRADE)
        {
            return Ok(());
        }

        let req = SubscribeRequest {
            symbol: vec![symbol],
            sub_type: SubFlags::TRADE.into(),
            is_first_push: false,
        };
        self.ws_cli.request(cmd_code::SUBSCRIBE, None, req).await?;

        Ok(())
    }

    async fn handle_unsubscribe_candlesticks(
        &mut self,
        symbol: String,
        period: Period,
    ) -> Result<()> {
        let mut unsubscribe_trades = false;

        if let Some(periods) = self
            .store
            .securities
            .get_mut(&symbol)
            .map(|data| &mut data.candlesticks)
        {
            if periods.remove(&period).is_some()
                && periods.is_empty()
                && !self
                    .subscriptions
                    .get(&symbol)
                    .copied()
                    .unwrap_or_else(SubFlags::empty)
                    .contains(SubFlags::TRADE)
            {
                unsubscribe_trades = true;
            }
        }

        if unsubscribe_trades {
            self.ws_cli
                .request(
                    cmd_code::UNSUBSCRIBE,
                    None,
                    UnsubscribeRequest {
                        symbol: vec![symbol],
                        sub_type: SubFlags::TRADE.into(),
                        unsub_all: false,
                    },
                )
                .await?;
        }

        Ok(())
    }

    async fn handle_subscriptions(&mut self) -> Vec<Subscription> {
        self.subscriptions
            .iter()
            .map(|(symbol, sub_flags)| Subscription {
                symbol: symbol.clone(),
                sub_types: *sub_flags,
                candlesticks: self
                    .store
                    .securities
                    .get(symbol)
                    .map(|data| &data.candlesticks)
                    .map(|periods| periods.keys().copied().collect())
                    .unwrap_or_default(),
            })
            .collect()
    }

    async fn handle_ws_event(&mut self, event: WsEvent) -> Result<()> {
        match event {
            WsEvent::Error(err) => Err(err.into()),
            WsEvent::Push { command_code, body } => self.handle_push(command_code, body).await,
        }
    }

    async fn resubscribe(&mut self) -> Result<()> {
        let mut subscriptions: HashMap<SubFlags, Vec<String>> = HashMap::new();

        for (symbol, flags) in &self.subscriptions {
            let mut flags = *flags;

            if self
                .store
                .securities
                .get(symbol)
                .map(|data| !data.candlesticks.is_empty())
                .unwrap_or_default()
            {
                flags |= SubFlags::TRADE;
            }

            subscriptions.entry(flags).or_default().push(symbol.clone());
        }

        for (flags, symbols) in subscriptions {
            self.ws_cli
                .request(
                    cmd_code::SUBSCRIBE,
                    None,
                    SubscribeRequest {
                        symbol: symbols,
                        sub_type: flags.into(),
                        is_first_push: false,
                    },
                )
                .await?;
        }

        Ok(())
    }

    async fn handle_push(&mut self, command_code: u8, body: Vec<u8>) -> Result<()> {
        match PushEvent::parse(command_code, &body) {
            Ok(mut event) => {
                if let PushEventDetail::Trade(trades) = &event.detail {
                    // merge candlesticks
                    let market =
                        get_market_from_symbol(&event.symbol).and_then(|market| match market {
                            "HK" => Some(longbridge_candlesticks::Market::HK),
                            "US" => Some(longbridge_candlesticks::Market::US),
                            "SH" => Some(longbridge_candlesticks::Market::SH),
                            "SZ" => Some(longbridge_candlesticks::Market::SZ),
                            _ => None,
                        });
                    if let Some(market) = market {
                        if let Some((merge_ty, periods)) = self
                            .store
                            .securities
                            .get_mut(&event.symbol)
                            .map(|data| (get_merger_ty(data.board), &mut data.candlesticks))
                        {
                            for (period, candlesticks) in periods {
                                let period2 = match period {
                                    Period::UnknownPeriod => unreachable!(),
                                    Period::OneMinute => longbridge_candlesticks::Period::Min_1,
                                    Period::FiveMinute => longbridge_candlesticks::Period::Min_5,
                                    Period::FifteenMinute => {
                                        longbridge_candlesticks::Period::Min_15
                                    }
                                    Period::ThirtyMinute => longbridge_candlesticks::Period::Min_30,
                                    Period::SixtyMinute => longbridge_candlesticks::Period::Min_60,
                                    Period::Day => longbridge_candlesticks::Period::Day,
                                    Period::Week => longbridge_candlesticks::Period::Week,
                                    Period::Month => longbridge_candlesticks::Period::Month,
                                    Period::Year => longbridge_candlesticks::Period::Year,
                                };

                                let merger = longbridge_candlesticks::Merger::new(
                                    market,
                                    period2,
                                    self.current_trade_days.half_days(market),
                                );

                                for trade in &trades.trades {
                                    if trade.trade_session != TradeSession::NormalTrade {
                                        continue;
                                    }

                                    let prev = candlesticks
                                        .last()
                                        .map(|candlestick| (*candlestick).into());

                                    let res = merger.merge(
                                        merge_ty,
                                        prev.as_ref(),
                                        longbridge_candlesticks::Trade {
                                            time: trade.timestamp,
                                            price: trade.price,
                                            volume: trade.volume,
                                            trade_type: &trade.trade_type,
                                        },
                                    );
                                    let candlestick = match res {
                                        UpdateAction::UpdateLast(candlestick) => {
                                            let candlestick = candlestick.into();
                                            *candlesticks.last_mut().unwrap() = candlestick;
                                            Some(candlestick)
                                        }
                                        UpdateAction::AppendNew(candlestick) => {
                                            let candlestick = candlestick.into();
                                            candlesticks.push(candlestick);
                                            if candlesticks.len() > MAX_CANDLESTICKS * 2 {
                                                candlesticks.drain(..MAX_CANDLESTICKS);
                                            }
                                            Some(candlestick)
                                        }
                                        UpdateAction::None => None,
                                    };
                                    if let Some(candlestick) = candlestick {
                                        let _ = self.push_tx.send(PushEvent {
                                            sequence: 0,
                                            symbol: event.symbol.clone(),
                                            detail: PushEventDetail::Candlestick(PushCandlestick {
                                                period: *period,
                                                candlestick,
                                            }),
                                        });
                                    }
                                }
                            }
                        }
                    }

                    if !self
                        .subscriptions
                        .get(&event.symbol)
                        .map(|sub_flags| sub_flags.contains(SubFlags::TRADE))
                        .unwrap_or_default()
                    {
                        return Ok(());
                    }
                }

                self.store.handle_push(&mut event);
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
            result.asks = data.asks.clone();
            result.bids = data.bids.clone();
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
            result.ask_brokers = data.ask_brokers.clone();
            result.bid_brokers = data.bid_brokers.clone();
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
                let candlesticks = if candlesticks.len() >= count {
                    &candlesticks[candlesticks.len() - count..]
                } else {
                    candlesticks
                };
                candlesticks.to_vec()
            })
            .unwrap_or_default()
    }
}

#[inline]
fn get_merger_ty(board: SecurityBoard) -> Type {
    match board {
        SecurityBoard::USOptionS => Type::USOQ,
        _ => Type::Normal,
    }
}

async fn fetch_current_trade_days(cli: &WsClient) -> Result<CurrentTradeDays> {
    let mut days = CurrentTradeDays::default();
    let begin_day = OffsetDateTime::now_utc().date() - time::Duration::days(1);
    let end_day = begin_day + time::Duration::days(30);

    for market in [Market::HK, Market::US] {
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
