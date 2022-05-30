use std::{collections::HashMap, sync::Arc, time::Duration};

use longbridge_proto::quote::{SubscribeRequest, SubscriptionResponse, UnsubscribeRequest};
use longbridge_wscli::{CodecType, Platform, ProtocolVersion, WsClient, WsEvent, WsSession};
use tokio::sync::{mpsc, oneshot};

use crate::{
    quote::{
        cmd_code, store::Store, sub_flags::SubFlags, PushEvent, RealtimeQuote, SecurityBrokers,
        SecurityDepth, Trade,
    },
    Config, Result,
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
}

pub(crate) struct Core {
    config: Arc<Config>,
    command_rx: mpsc::UnboundedReceiver<Command>,
    push_tx: mpsc::UnboundedSender<PushEvent>,
    event_tx: mpsc::UnboundedSender<WsEvent>,
    event_rx: mpsc::UnboundedReceiver<WsEvent>,
    ws_cli: WsClient,
    session: WsSession,
    close: bool,
    subscriptions: HashMap<String, SubFlags>,
    store: Store,
}

impl Core {
    pub(crate) async fn try_new(
        config: Arc<Config>,
        command_rx: mpsc::UnboundedReceiver<Command>,
        push_tx: mpsc::UnboundedSender<PushEvent>,
    ) -> Result<Self> {
        let otp = config.create_http_client().get_otp().await?;

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        tracing::debug!(
            url = config.quote_ws_url.as_str(),
            "connecting to quote server",
        );
        let ws_cli = WsClient::open(
            &config.quote_ws_url,
            ProtocolVersion::Version1,
            CodecType::Protobuf,
            Platform::OpenAPI,
            event_tx.clone(),
        )
        .await?;

        tracing::debug!(url = config.quote_ws_url.as_str(), "quote server connected");

        let session = ws_cli.request_auth(otp).await?;

        Ok(Self {
            config,
            command_rx,
            push_tx,
            event_tx,
            event_rx,
            ws_cli,
            session,
            close: false,
            subscriptions: HashMap::new(),
            store: Store::default(),
        })
    }

    pub(crate) async fn run(mut self) {
        while !self.close {
            match self.main_loop().await {
                Ok(()) => tracing::error!("quote disconnected"),
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
                    &self.config.quote_ws_url,
                    ProtocolVersion::Version1,
                    CodecType::Protobuf,
                    Platform::OpenAPI,
                    self.event_tx.clone(),
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
                match self
                    .ws_cli
                    .request_reconnect(&self.session.session_id)
                    .await
                {
                    Ok(new_session) => self.session = new_session,
                    Err(err) => {
                        tracing::error!(error = %err, "failed to request session id");
                        continue;
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
                let res = self.handle_unsubscribe(symbols, sub_types).await;
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
        }
    }

    async fn handle_request(
        &mut self,
        command_code: u8,
        body: Vec<u8>,
        reply_tx: oneshot::Sender<Result<Vec<u8>>>,
    ) -> Result<()> {
        let ws_cli = self.ws_cli.clone();
        tokio::spawn(async move {
            let res = ws_cli.request_raw(command_code, None, body).await;
            let _ = reply_tx.send(res.map_err(Into::into));
        });
        Ok(())
    }

    async fn handle_subscribe(
        &mut self,
        symbols: Vec<String>,
        sub_types: SubFlags,
        is_first_push: bool,
    ) -> Result<()> {
        tracing::debug!(symbols = ?symbols, sub_types = ?sub_types, "subscribe");

        let req = SubscribeRequest {
            symbol: symbols,
            sub_type: sub_types.into(),
            is_first_push,
        };
        let resp: SubscriptionResponse =
            self.ws_cli.request(cmd_code::SUBSCRIBE, None, req).await?;

        for sub in resp.sub_list {
            let sub_flags: SubFlags = sub.sub_type.into();

            self.subscriptions
                .entry(sub.symbol)
                .and_modify(|flags| *flags |= sub_flags)
                .or_insert(sub_flags);
        }

        Ok(())
    }

    async fn handle_unsubscribe(
        &mut self,
        symbols: Vec<String>,
        sub_types: SubFlags,
    ) -> Result<()> {
        tracing::debug!(symbols = ?symbols, sub_types = ?sub_types, "unsubscribe");

        let req = if !sub_types.is_all() {
            UnsubscribeRequest {
                symbol: symbols,
                sub_type: sub_types.into(),
                unsub_all: false,
            }
        } else {
            UnsubscribeRequest {
                symbol: symbols,
                sub_type: vec![],
                unsub_all: true,
            }
        };

        let resp: SubscriptionResponse = self
            .ws_cli
            .request(cmd_code::UNSUBSCRIBE, None, req)
            .await?;

        let mut remove_symbols = Vec::new();

        for sub in resp.sub_list {
            let sub_flags: SubFlags = sub.sub_type.into();
            if let Some(cur_flags) = self.subscriptions.get_mut(&sub.symbol) {
                *cur_flags &= !sub_flags;
                if cur_flags.is_empty() {
                    remove_symbols.push(sub.symbol.clone());
                }
            }
        }

        for symbol in remove_symbols {
            self.subscriptions.remove(&symbol);
        }

        Ok(())
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
            subscriptions
                .entry(*flags)
                .or_default()
                .push(symbol.clone());
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
}
