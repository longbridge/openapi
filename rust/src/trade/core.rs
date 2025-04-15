use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
    time::Duration,
};

use longport_httpcli::HttpClient;
use longport_proto::trade::{Sub, SubResponse, Unsub, UnsubResponse};
use longport_wscli::{
    CodecType, Platform, ProtocolVersion, WsClient, WsClientError, WsEvent, WsSession,
};
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};

use crate::{
    Config, Result,
    trade::{PushEvent, PushOrderChanged, TopicType, cmd_code},
};

const RECONNECT_DELAY: Duration = Duration::from_secs(2);

pub(crate) enum Command {
    Subscribe {
        topics: Vec<TopicType>,
        reply_tx: oneshot::Sender<Result<()>>,
    },
    Unsubscribe {
        topics: Vec<TopicType>,
        reply_tx: oneshot::Sender<Result<()>>,
    },
    SubmittedOrder {
        order_id: String,
    },
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
    subscriptions: HashSet<String>,
    unknown_orders: VecDeque<(Instant, PushOrderChanged)>,
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

        tracing::info!("connecting to trade server");
        let (url, res) = config.create_trade_ws_request().await;
        let request = res.map_err(WsClientError::from)?;
        let ws_cli = WsClient::open(
            request,
            ProtocolVersion::Version1,
            CodecType::Protobuf,
            Platform::OpenAPI,
            event_tx.clone(),
            vec![],
        )
        .await?;

        tracing::info!(url = url, "trade server connected");

        let session = ws_cli.request_auth(otp, Default::default()).await?;

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
            subscriptions: HashSet::new(),
            unknown_orders: VecDeque::new(),
        })
    }

    pub(crate) async fn run(mut self) {
        while !self.close {
            match self.main_loop().await {
                Ok(()) => return,
                Err(err) => tracing::error!(error = %err, "trade disconnected"),
            }

            loop {
                // reconnect
                tokio::time::sleep(RECONNECT_DELAY).await;

                tracing::info!("connecting to trade server");
                let (url, res) = self.config.create_trade_ws_request().await;
                let request = res.expect("BUG: failed to create trade ws request");

                match WsClient::open(
                    request,
                    ProtocolVersion::Version1,
                    CodecType::Protobuf,
                    Platform::OpenAPI,
                    self.event_tx.clone(),
                    vec![],
                )
                .await
                {
                    Ok(ws_cli) => self.ws_cli = ws_cli,
                    Err(err) => {
                        tracing::error!(error = %err, "failed to connect trade server");
                        continue;
                    }
                }

                tracing::info!(url = url, "trade server connected");

                // request new session
                match &self.session {
                    Some(session) if !session.is_expired() => {
                        match self
                            .ws_cli
                            .request_reconnect(&session.session_id, Default::default())
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
                        let otp = match self.http_cli.get_otp_v2().await {
                            Ok(otp) => otp,
                            Err(err) => {
                                tracing::error!(error = %err, "failed to request otp");
                                continue;
                            }
                        };

                        match self.ws_cli.request_auth(otp, Default::default()).await {
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
        let mut tick = tokio::time::interval(Duration::from_millis(500));

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
                now = tick.tick() => self.handle_tick(now),
            }
        }
    }

    async fn handle_ws_event(&mut self, event: WsEvent) -> Result<()> {
        match event {
            WsEvent::Error(err) => Err(err.into()),
            WsEvent::Push { command_code, body } => self.handle_push(command_code, body).await,
        }
    }

    async fn handle_push(&mut self, command_code: u8, body: Vec<u8>) -> Result<()> {
        match PushEvent::parse(command_code, &body) {
            Ok(Some(event)) => {
                tracing::info!(event = ?event, "push event");
                let _ = self.push_tx.send(event);
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!(error = %err, "failed to parse push message")
            }
        }
        Ok(())
    }

    fn handle_tick(&mut self, now: Instant) {
        while let Some((t, _)) = self.unknown_orders.front() {
            if now - *t > Duration::from_secs(1) {
                let (_, order_changed) = self.unknown_orders.pop_front().unwrap();
                _ = self.push_tx.send(PushEvent::OrderChanged(order_changed));
            } else {
                break;
            }
        }
    }

    async fn handle_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Subscribe { topics, reply_tx } => {
                let res = self.handle_subscribe(topics).await;
                let _ = reply_tx.send(res);
                Ok(())
            }
            Command::Unsubscribe { topics, reply_tx } => {
                let res = self.handle_unsubscribe(topics).await;
                let _ = reply_tx.send(res);
                Ok(())
            }
            Command::SubmittedOrder { order_id } => {
                while let Some((idx, _)) = self
                    .unknown_orders
                    .iter()
                    .enumerate()
                    .find(|(_, (_, order_changed))| order_changed.order_id == order_id)
                {
                    let Some((_, order_changed)) = self.unknown_orders.remove(idx) else {
                        unreachable!();
                    };
                    let _ = self.push_tx.send(PushEvent::OrderChanged(order_changed));
                }
                Ok(())
            }
        }
    }

    async fn handle_subscribe(&mut self, topics: Vec<TopicType>) -> Result<()> {
        let req = Sub {
            topics: topics.iter().map(ToString::to_string).collect(),
        };
        tracing::info!(topics = ?req.topics, "subscribing topics");
        let resp: SubResponse = self.ws_cli.request(cmd_code::SUBSCRIBE, None, req).await?;
        self.subscriptions = resp.current.into_iter().collect();
        Ok(())
    }

    async fn handle_unsubscribe(&mut self, topics: Vec<TopicType>) -> Result<()> {
        let req = Unsub {
            topics: topics.iter().map(ToString::to_string).collect(),
        };
        tracing::info!(topics = ?req.topics, "unsubscribing topics");
        let resp: UnsubResponse = self
            .ws_cli
            .request(cmd_code::UNSUBSCRIBE, None, req)
            .await?;
        self.subscriptions = resp.current.into_iter().collect();

        Ok(())
    }

    async fn resubscribe(&mut self) -> Result<()> {
        let req = Sub {
            topics: self.subscriptions.iter().cloned().collect(),
        };
        let resp: SubResponse = self.ws_cli.request(cmd_code::SUBSCRIBE, None, req).await?;
        self.subscriptions = resp.current.into_iter().collect();
        Ok(())
    }
}
