use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryFutureExt,
};
use leaky_bucket::RateLimiter;
use longbridge_proto::control::{AuthRequest, AuthResponse, ReconnectRequest, ReconnectResponse};
use num_enum::IntoPrimitive;
use prost::Message as _;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
};
use tokio_tungstenite::{
    tungstenite::{client::IntoClientRequest, http::Uri, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{
    codec::Packet, WsClientError, WsClientResult, WsCloseReason, WsEvent, WsResponseErrorDetail,
};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(120);
const AUTH_TIMEOUT: Duration = Duration::from_secs(5);
const RECONNECT_TIMEOUT: Duration = Duration::from_secs(5);

const COMMAND_CODE_AUTH: u8 = 2;
const COMMAND_CODE_RECONNECT: u8 = 3;

/// Longbridge websocket protocol version
#[derive(Debug, IntoPrimitive, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ProtocolVersion {
    /// Version 1
    Version1 = 1,
}

/// Longbridge websocket codec type
#[derive(Debug, IntoPrimitive, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum CodecType {
    /// Protobuf
    Protobuf = 1,
}

/// Longbridge websocket platform type
#[derive(Debug, IntoPrimitive, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Platform {
    /// OpenAPI
    OpenAPI = 9,
}

enum Command {
    Request {
        command_code: u8,
        timeout_millis: u16,
        body: Vec<u8>,
        reply_tx: oneshot::Sender<WsClientResult<Vec<u8>>>,
    },
}

/// Rate limiter config
#[derive(Debug, Copy, Clone)]
pub struct RateLimit {
    /// The time duration between which we add refill number to the bucket
    pub interval: Duration,
    /// The initial number of tokens
    pub initial: usize,
    /// The max number of tokens to use
    pub max: usize,
    /// The number of tokens to add at each interval interval
    pub refill: usize,
}

struct Context<'a> {
    request_id: u32,
    inflight_requests: HashMap<u32, oneshot::Sender<WsClientResult<Vec<u8>>>>,
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    command_rx: &'a mut mpsc::UnboundedReceiver<Command>,
    event_sender: &'a mut mpsc::UnboundedSender<WsEvent>,
    rate_limiter: Option<RateLimiter>,
}

impl<'a> Context<'a> {
    fn new(
        conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
        command_rx: &'a mut mpsc::UnboundedReceiver<Command>,
        event_sender: &'a mut mpsc::UnboundedSender<WsEvent>,
        rate_limit: Option<RateLimit>,
    ) -> Self {
        let (sink, stream) = conn.split();
        Context {
            request_id: 0,
            inflight_requests: Default::default(),
            sink,
            stream,
            command_rx,
            event_sender,
            rate_limiter: rate_limit.map(|config| {
                RateLimiter::builder()
                    .interval(config.interval)
                    .refill(config.refill)
                    .max(config.max)
                    .initial(config.initial)
                    .build()
            }),
        }
    }

    #[inline]
    fn get_request_id(&mut self) -> u32 {
        self.request_id += 1;
        self.request_id
    }

    fn send_event(&mut self, event: WsEvent) {
        let _ = self.event_sender.send(event);
    }

    async fn process_loop(&mut self) -> WsClientResult<()> {
        let mut ping_time = Instant::now();
        let mut checkout_timeout = tokio::time::interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                item = self.stream.next() => {
                    match item.transpose()? {
                        Some(msg) => {
                            if msg.is_ping() {
                                ping_time = Instant::now();
                            }
                            self.handle_message(msg).await?;
                        },
                        None => return Err(WsClientError::ConnectionClosed { reason: None }),
                    }
                }
                item = self.command_rx.recv() => {
                    match item {
                        Some(command) => self.handle_command(command).await?,
                        None => return Ok(()),
                    }
                }
                _ = checkout_timeout.tick() => {
                    if (Instant::now() - ping_time) > HEARTBEAT_TIMEOUT {
                        return Err(WsClientError::ConnectionClosed { reason: None });
                    }
                }
            }
        }
    }

    async fn handle_command(&mut self, command: Command) -> WsClientResult<()> {
        match command {
            Command::Request {
                command_code,
                timeout_millis: timeout,
                body,
                reply_tx,
            } => {
                if let Some(rate_limiter) = &mut self.rate_limiter {
                    rate_limiter.acquire_one().await;
                }

                let request_id = self.get_request_id();
                let msg = Message::Binary(
                    Packet::Request {
                        command_code,
                        request_id,
                        timeout_millis: timeout,
                        body,
                        signature: None,
                    }
                    .encode(),
                );
                self.inflight_requests.insert(request_id, reply_tx);
                self.sink.send(msg).await?;
                Ok(())
            }
        }
    }

    async fn handle_message(&mut self, msg: Message) -> WsClientResult<()> {
        match msg {
            Message::Ping(data) => {
                self.sink.send(Message::Pong(data)).await?;
            }
            Message::Binary(data) => match Packet::decode(&data)? {
                Packet::Response {
                    request_id,
                    status,
                    body,
                    ..
                } => {
                    if let Some(sender) = self.inflight_requests.remove(&request_id) {
                        if status == 0 {
                            let _ = sender.send(Ok(body));
                        } else {
                            let detail = longbridge_proto::Error::decode(&*body).ok().map(
                                |longbridge_proto::Error { code, msg }| WsResponseErrorDetail {
                                    code,
                                    msg,
                                },
                            );
                            let _ =
                                sender.send(Err(WsClientError::ResponseError { status, detail }));
                        }
                    }
                }
                Packet::Push {
                    command_code, body, ..
                } => {
                    let _ = self.event_sender.send(WsEvent::Push { command_code, body });
                }
                _ => return Err(WsClientError::UnexpectedResponse),
            },
            Message::Close(Some(close_frame)) => {
                return Err(WsClientError::ConnectionClosed {
                    reason: Some(WsCloseReason {
                        code: close_frame.code,
                        message: close_frame.reason.into_owned(),
                    }),
                });
            }
            _ => return Err(WsClientError::UnexpectedResponse),
        }

        Ok(())
    }
}

/// The session for the Websocket connection
#[derive(Debug)]
pub struct WsSession {
    /// Session id
    pub session_id: String,
    /// The expiration time of the session id.
    pub deadline: SystemTime,
}

impl WsSession {
    /// Returns `true` if the session id is expired, otherwise returns `false
    #[inline]
    pub fn is_expired(&self) -> bool {
        self.deadline < SystemTime::now()
    }
}

/// Longbridge Websocket client
#[derive(Clone)]
pub struct WsClient {
    command_tx: mpsc::UnboundedSender<Command>,
}

impl WsClient {
    /// Connect to `url` and returns a `WsClient` object
    pub async fn open(
        request: impl IntoClientRequest,
        version: ProtocolVersion,
        codec: CodecType,
        platform: Platform,
        event_sender: mpsc::UnboundedSender<WsEvent>,
        rate_limit: Option<RateLimit>,
    ) -> WsClientResult<Self> {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let conn = do_connect(request, version, codec, platform).await?;
        tokio::spawn(client_loop(conn, command_rx, event_sender, rate_limit));
        Ok(Self { command_tx })
    }

    /// Send an authentication request to get a [`WsSession`]
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/socket-token-api>
    /// Reference: <https://open.longbridgeapp.com/en/docs/socket/control-command#auth>
    pub async fn request_auth(&self, otp: impl Into<String>) -> WsClientResult<WsSession> {
        let resp: AuthResponse = self
            .request(
                COMMAND_CODE_AUTH,
                Some(AUTH_TIMEOUT),
                AuthRequest { token: otp.into() },
            )
            .await?;
        let expires_mills = resp.expires.saturating_sub(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        ) as u64;
        let deadline = SystemTime::now() + Duration::from_millis(expires_mills);
        Ok(WsSession {
            session_id: resp.session_id,
            deadline,
        })
    }

    /// Send a reconnect request to get a [`WsSession`]
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/socket/control-command#reconnect>
    pub async fn request_reconnect(
        &self,
        session_id: impl Into<String>,
    ) -> WsClientResult<WsSession> {
        let resp: ReconnectResponse = self
            .request(
                COMMAND_CODE_RECONNECT,
                Some(RECONNECT_TIMEOUT),
                ReconnectRequest {
                    session_id: session_id.into(),
                },
            )
            .await?;
        Ok(WsSession {
            session_id: resp.session_id,
            deadline: SystemTime::now() + Duration::from_millis(resp.expires as u64),
        })
    }

    /// Send a raw request
    pub async fn request_raw(
        &self,
        command_code: u8,
        timeout: Option<Duration>,
        body: Vec<u8>,
    ) -> WsClientResult<Vec<u8>> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.command_tx
            .send(Command::Request {
                command_code,
                timeout_millis: timeout.unwrap_or(REQUEST_TIMEOUT).as_millis().min(60000) as u16,
                body,
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        let resp = tokio::time::timeout(
            REQUEST_TIMEOUT,
            reply_rx.map_err(|_| WsClientError::ClientClosed),
        )
        .map_err(|_| WsClientError::RequestTimeout)
        .await???;
        Ok(resp)
    }

    /// Send a request `T` to get a response `R`
    pub async fn request<T, R>(
        &self,
        command_code: u8,
        timeout: Option<Duration>,
        req: T,
    ) -> WsClientResult<R>
    where
        T: prost::Message,
        R: prost::Message + Default,
    {
        let resp = self
            .request_raw(command_code, timeout, req.encode_to_vec())
            .await?;
        Ok(R::decode(&*resp)?)
    }
}

async fn do_connect(
    request: impl IntoClientRequest,
    version: ProtocolVersion,
    codec: CodecType,
    platform: Platform,
) -> WsClientResult<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let mut request = request.into_client_request()?;
    let mut url_obj = Url::parse(&request.uri().to_string())?;
    url_obj.query_pairs_mut().extend_pairs(&[
        ("version", i32::from(version).to_string()),
        ("codec", i32::from(codec).to_string()),
        ("platform", i32::from(platform).to_string()),
    ]);
    *request.uri_mut() = Uri::from_str(url_obj.as_ref()).expect("valid url");

    let conn = match tokio::time::timeout(
        CONNECT_TIMEOUT,
        tokio_tungstenite::connect_async(request).map_err(WsClientError::from),
    )
    .map_err(|_| WsClientError::ConnectTimeout)
    .await
    .and_then(std::convert::identity)
    {
        Ok((conn, _)) => conn,
        Err(err) => return Err(err),
    };

    Ok(conn)
}

async fn client_loop(
    conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut command_tx: mpsc::UnboundedReceiver<Command>,
    mut event_sender: mpsc::UnboundedSender<WsEvent>,
    rate_limit: Option<RateLimit>,
) {
    let mut ctx = Context::new(conn, &mut command_tx, &mut event_sender, rate_limit);

    let res = ctx.process_loop().await;
    match res {
        Ok(()) => return,
        Err(err) => {
            ctx.send_event(WsEvent::Error(err));
        }
    };

    for sender in ctx.inflight_requests.into_values() {
        let _ = sender.send(Err(WsClientError::Cancelled));
    }
}
