//! SDK-owned Engine.IO/Socket.IO supervisor.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::time::{sleep_until, timeout, Instant};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::LOCATION;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use super::{
    is_invalid_token_text, redirect_target, validate_endpoint, websocket_url, SocketEvent,
    SocketPayload,
};
use crate::Error;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_REDIRECT_HOPS: usize = 5;
const MAX_RECONNECT_ATTEMPTS: u32 = 10;
const MAX_BACKOFF: Duration = Duration::from_secs(30);
const EMIT_QUEUE_CAPACITY: usize = 128;

type SocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub(super) type TokenProvider = Arc<dyn Fn() -> Option<String> + Send + Sync>;
type AckSender = oneshot::Sender<Result<SocketPayload, Error>>;
type PendingAck = (u64, Instant, AckSender);

pub(super) struct Settings {
    pub base_url: String,
    pub initial_token: String,
    pub token_provider: TokenProvider,
    pub loop_active: Arc<AtomicBool>,
    pub stopped_on_failure: Arc<AtomicBool>,
    pub retry: RetryConfig,
}

#[derive(Clone, Copy)]
pub(super) struct RetryConfig {
    pub max_attempts: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: MAX_RECONNECT_ATTEMPTS,
            initial_backoff: Duration::from_secs(1),
            max_backoff: MAX_BACKOFF,
        }
    }
}

#[derive(Clone, Copy)]
struct RetryDecision {
    delay: Duration,
    escalate: bool,
    exhausted: bool,
}

struct RetryState {
    config: RetryConfig,
    attempts: u32,
    next_delay: Duration,
}

impl Default for RetryState {
    fn default() -> Self {
        Self::new(RetryConfig::default())
    }
}

impl RetryState {
    fn new(config: RetryConfig) -> Self {
        Self {
            next_delay: config.initial_backoff,
            config,
            attempts: 0,
        }
    }

    fn failed_attempt(&mut self) -> RetryDecision {
        self.attempts = self.attempts.saturating_add(1);
        let delay = self.next_delay;
        self.next_delay = (self.next_delay * 2).min(self.config.max_backoff);
        RetryDecision {
            delay,
            escalate: self.attempts == 5,
            exhausted: self.attempts >= self.config.max_attempts,
        }
    }

    fn connection_lost(&mut self) -> Duration {
        self.attempts = 0;
        self.next_delay = self.config.initial_backoff;
        let delay = self.next_delay;
        self.next_delay = (self.next_delay * 2).min(self.config.max_backoff);
        delay
    }
}

struct ActiveGuard(Arc<AtomicBool>);

impl Drop for ActiveGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

pub(super) enum Command {
    Emit {
        event: String,
        payload: OutgoingPayload,
        ack: Option<oneshot::Sender<Result<SocketPayload, Error>>>,
        ack_timeout: Option<Duration>,
        sent: Option<oneshot::Sender<Result<(), Error>>>,
    },
}

pub(super) enum OutgoingPayload {
    Json(Value),
    Binary(Vec<u8>),
}

pub(super) async fn run(
    settings: Settings,
    mut commands: mpsc::Receiver<Command>,
    events: mpsc::Sender<SocketEvent>,
    mut shutdown: watch::Receiver<bool>,
) {
    let Settings {
        base_url,
        initial_token,
        token_provider,
        loop_active,
        stopped_on_failure,
        retry: retry_config,
    } = settings;
    loop_active.store(true, Ordering::Release);
    let _active_guard = ActiveGuard(Arc::clone(&loop_active));
    stopped_on_failure.store(false, Ordering::Release);

    let mut pending = VecDeque::new();
    let mut token = Some(initial_token);
    let mut retry = RetryState::new(retry_config);

    'supervisor: loop {
        if *shutdown.borrow() || collect_commands(&mut commands, &mut pending) {
            break;
        }
        let Some(token) = token
            .take()
            .or_else(|| token_provider().filter(|value| !value.trim().is_empty()))
        else {
            stopped_on_failure.store(true, Ordering::Release);
            send_error(&events, "socket token provider returned no token").await;
            break;
        };

        let mut endpoint = websocket_url(&base_url);
        let mut socket = match connect_with_redirects(&mut endpoint, &mut shutdown).await {
            Ok(socket) => socket,
            Err(ConnectFailure::Shutdown) => break,
            Err(ConnectFailure::InvalidToken(message)) => {
                stopped_on_failure.store(true, Ordering::Release);
                send_error(&events, &message).await;
                break;
            }
            Err(ConnectFailure::Policy(message)) => {
                stopped_on_failure.store(true, Ordering::Release);
                send_error(&events, &message).await;
                break;
            }
            Err(ConnectFailure::Retryable(message)) => {
                let decision = retry.failed_attempt();
                send_error(&events, &message).await;
                if decision.escalate {
                    send_escalation(&events, retry.attempts).await;
                }
                if decision.exhausted {
                    stopped_on_failure.store(true, Ordering::Release);
                    break;
                }
                if sleep_or_disconnect(decision.delay, &mut commands, &mut pending, &mut shutdown)
                    .await
                {
                    break 'supervisor;
                }
                continue;
            }
        };

        match socket_connect_ack(&mut socket, &token, &mut shutdown, &events).await {
            Ok(heartbeat) => {
                let lost = drive_ready_socket(
                    &mut socket,
                    &mut commands,
                    &mut pending,
                    &events,
                    &mut shutdown,
                    heartbeat,
                )
                .await;
                drain_pending(&mut pending);
                if lost {
                    drain_command_queue(&mut commands);
                }
                if !lost {
                    break;
                }
                let _ = events
                    .send(SocketEvent::supervisor(
                        "close",
                        SocketPayload::Json(vec![]),
                    ))
                    .await;
                let delay = retry.connection_lost();
                if sleep_or_disconnect(delay, &mut commands, &mut pending, &mut shutdown).await {
                    break;
                }
                continue;
            }
            Err(ConnectFailure::InvalidToken(message)) => {
                stopped_on_failure.store(true, Ordering::Release);
                send_error(&events, &message).await;
                break;
            }
            Err(ConnectFailure::Policy(message)) => {
                stopped_on_failure.store(true, Ordering::Release);
                send_error(&events, &message).await;
                break;
            }
            Err(ConnectFailure::Shutdown) => break,
            Err(ConnectFailure::Retryable(message)) => {
                let decision = retry.failed_attempt();
                send_error(&events, &message).await;
                if decision.escalate {
                    send_escalation(&events, retry.attempts).await;
                }
                if decision.exhausted {
                    stopped_on_failure.store(true, Ordering::Release);
                    break;
                }
                if sleep_or_disconnect(decision.delay, &mut commands, &mut pending, &mut shutdown)
                    .await
                {
                    break 'supervisor;
                }
                continue;
            }
        }
    }

    drain_pending(&mut pending);
    while let Ok(command) = commands.try_recv() {
        fail_command(command);
    }
}

async fn connect_with_redirects(
    url: &mut String,
    shutdown: &mut watch::Receiver<bool>,
) -> Result<SocketStream, ConnectFailure> {
    connect_with_redirects_within(url, shutdown, CONNECT_TIMEOUT).await
}

async fn connect_with_redirects_within(
    url: &mut String,
    shutdown: &mut watch::Receiver<bool>,
    hop_timeout: Duration,
) -> Result<SocketStream, ConnectFailure> {
    for hop in 0..=MAX_REDIRECT_HOPS {
        validate_endpoint(url).map_err(|error| ConnectFailure::Policy(error.to_string()))?;
        let mut request = url
            .as_str()
            .into_client_request()
            .map_err(|error| ConnectFailure::Retryable(error.to_string()))?;
        request.headers_mut().insert(
            "x-sdk-client",
            tokio_tungstenite::tungstenite::http::HeaderValue::from_static("tinyhumans-rust"),
        );
        let result = tokio::select! {
            _ = shutdown.changed() => return Err(ConnectFailure::Shutdown),
            result = timeout(hop_timeout, connect_async(request)) => result,
        };
        match result {
            Err(_) => {
                return Err(ConnectFailure::Retryable(format!(
                    "websocket upgrade timed out after {hop_timeout:?}"
                )))
            }
            Ok(Ok((socket, _))) => return Ok(socket),
            Ok(Err(tokio_tungstenite::tungstenite::Error::Http(response)))
                if is_redirect(response.status()) =>
            {
                if hop == MAX_REDIRECT_HOPS {
                    return Err(ConnectFailure::Retryable(
                        "too many socket redirects".into(),
                    ));
                }
                let location = response
                    .headers()
                    .get(LOCATION)
                    .and_then(|value| value.to_str().ok())
                    .ok_or_else(|| ConnectFailure::Retryable("redirect missing Location".into()))?;
                *url = redirect_target(url, location).map_err(|error| match error {
                    Error::CrossOriginSocketRedirect | Error::UnsafeSocketEndpoint => {
                        ConnectFailure::Policy(error.to_string())
                    }
                    _ => ConnectFailure::Retryable(error.to_string()),
                })?;
            }
            Ok(Err(error)) => return Err(ConnectFailure::Retryable(error.to_string())),
        }
    }
    Err(ConnectFailure::Retryable(
        "too many socket redirects".into(),
    ))
}

#[cfg(test)]
#[path = "supervisor_tests.rs"]
mod tests;

fn is_redirect(status: StatusCode) -> bool {
    matches!(status.as_u16(), 301 | 302 | 307 | 308)
}

async fn socket_connect_ack(
    socket: &mut SocketStream,
    token: &str,
    shutdown: &mut watch::Receiver<bool>,
    events: &mpsc::Sender<SocketEvent>,
) -> Result<Duration, ConnectFailure> {
    let open = tokio::select! {
        _ = shutdown.changed() => return Err(ConnectFailure::Shutdown),
        open = timeout(CONNECT_TIMEOUT, socket.next()) => open,
    }
    .map_err(|_| ConnectFailure::Retryable("Engine.IO open timed out".into()))?
    .ok_or_else(|| ConnectFailure::Retryable("closed before Engine.IO open".into()))?
    .map_err(|error| ConnectFailure::Retryable(error.to_string()))?;
    let Message::Text(open) = open else {
        return Err(ConnectFailure::Retryable(
            "invalid Engine.IO open packet".into(),
        ));
    };
    if !open.starts_with('0') {
        return Err(ConnectFailure::Retryable(
            "missing Engine.IO open packet".into(),
        ));
    }
    let open_data = serde_json::from_str::<Value>(&open[1..]).unwrap_or(Value::Null);
    let ping_interval = open_data
        .get("pingInterval")
        .and_then(Value::as_u64)
        .unwrap_or(25_000);
    let ping_timeout = open_data
        .get("pingTimeout")
        .and_then(Value::as_u64)
        .unwrap_or(20_000);
    let _ = events
        .send(SocketEvent::supervisor(
            "open",
            SocketPayload::Json(vec![open_data]),
        ))
        .await;
    let auth = serde_json::to_string(&json!({ "token": token }))
        .map_err(|error| ConnectFailure::Retryable(error.to_string()))?;
    socket
        .send(Message::Text(format!("40{auth}")))
        .await
        .map_err(|error| ConnectFailure::Retryable(error.to_string()))?;

    loop {
        let message = tokio::select! {
            _ = shutdown.changed() => return Err(ConnectFailure::Shutdown),
            message = timeout(CONNECT_TIMEOUT, socket.next()) => message,
        }
        .map_err(|_| ConnectFailure::Retryable("Socket.IO CONNECT ACK timed out".into()))?
        .ok_or_else(|| ConnectFailure::Retryable("closed before Socket.IO CONNECT ACK".into()))?
        .map_err(|error| ConnectFailure::Retryable(error.to_string()))?;
        match message {
            Message::Text(frame) if frame.starts_with("40") => {
                let socket_namespace = frame
                    .strip_prefix("40")
                    .and_then(|payload| serde_json::from_str::<Value>(payload).ok())
                    .unwrap_or(Value::Null);
                let _ = events
                    .send(SocketEvent::supervisor(
                        "connect",
                        SocketPayload::Json(vec![socket_namespace]),
                    ))
                    .await;
                return Ok(Duration::from_millis(ping_interval + ping_timeout + 5_000));
            }
            Message::Text(frame) if frame.starts_with("44") => {
                if is_invalid_token_text(&frame) {
                    return Err(ConnectFailure::InvalidToken(frame));
                }
                return Err(ConnectFailure::Retryable(frame));
            }
            Message::Text(frame) if frame == "2" => {
                socket
                    .send(Message::Text("3".into()))
                    .await
                    .map_err(|error| ConnectFailure::Retryable(error.to_string()))?;
            }
            Message::Ping(data) => socket
                .send(Message::Pong(data))
                .await
                .map_err(|error| ConnectFailure::Retryable(error.to_string()))?,
            Message::Close(_) => {
                return Err(ConnectFailure::Retryable(
                    "closed before Socket.IO CONNECT ACK".into(),
                ));
            }
            _ => {}
        }
    }
}

enum ConnectFailure {
    Retryable(String),
    InvalidToken(String),
    Policy(String),
    Shutdown,
}

fn collect_commands(
    commands: &mut mpsc::Receiver<Command>,
    pending: &mut VecDeque<Command>,
) -> bool {
    loop {
        match commands.try_recv() {
            Ok(command) if pending.len() < EMIT_QUEUE_CAPACITY => pending.push_back(command),
            Ok(command) => fail_command(command),
            Err(_) => return false,
        }
    }
}

async fn sleep_or_disconnect(
    duration: Duration,
    commands: &mut mpsc::Receiver<Command>,
    pending: &mut VecDeque<Command>,
    shutdown: &mut watch::Receiver<bool>,
) -> bool {
    let deadline = Instant::now() + duration;
    loop {
        tokio::select! {
            _ = shutdown.changed() => return true,
            _ = sleep_until(deadline) => return false,
            command = commands.recv() => match command {
                Some(command) if pending.len() < EMIT_QUEUE_CAPACITY => pending.push_back(command),
                Some(command) => fail_command(command),
                None => return true,
            }
        }
    }
}

async fn drive_ready_socket(
    socket: &mut SocketStream,
    commands: &mut mpsc::Receiver<Command>,
    pending: &mut VecDeque<Command>,
    events: &mpsc::Sender<SocketEvent>,
    shutdown: &mut watch::Receiver<bool>,
    heartbeat_timeout: Duration,
) -> bool {
    let mut ack_id = 0u64;
    let mut acknowledgements = VecDeque::<PendingAck>::new();
    let mut binary_event: Option<(String, usize)> = None;
    let mut heartbeat_deadline = Instant::now() + heartbeat_timeout;
    loop {
        if let Some(command) = pending.pop_front() {
            if command_is_cancelled(&command) {
                fail_command(command);
                continue;
            }
            if !send_command(socket, command, &mut ack_id, &mut acknowledgements).await {
                fail_acks(acknowledgements);
                return true;
            }
            continue;
        }
        tokio::select! {
            _ = shutdown.changed() => { fail_acks(acknowledgements); return false; }
            _ = sleep_until(heartbeat_deadline) => { fail_acks(acknowledgements); return true; }
            _ = next_ack_deadline(&acknowledgements), if !acknowledgements.is_empty() => {
                expire_acks(&mut acknowledgements);
            }
            command = commands.recv() => match command {
                Some(command) => pending.push_back(command),
                None => { fail_acks(acknowledgements); return false; }
            },
            message = socket.next() => match message {
                Some(Ok(Message::Text(frame))) => {
                    if let Some(payload) = frame.strip_prefix('2') {
                        heartbeat_deadline = Instant::now() + heartbeat_timeout;
                        if socket.send(Message::Text(format!("3{payload}"))).await.is_err() { fail_acks(acknowledgements); return true; }
                    } else if frame.starts_with("43") {
                        handle_ack(&frame, &mut acknowledgements);
                    } else if let Some(frame) = frame.strip_prefix("42") {
                        if let Some((name, payload)) = parse_event(frame) {
                            let _ = events.send(SocketEvent::new(name, SocketPayload::Json(payload))).await;
                        }
                    } else if frame.starts_with("451-") {
                        binary_event = parse_binary_event(&frame[3..]);
                    } else if frame.starts_with("41") {
                        fail_acks(acknowledgements);
                        return true;
                    } else if frame.starts_with("44") {
                        let _ = events.send(SocketEvent::supervisor("error", SocketPayload::Json(vec![Value::String(frame)]))).await;
                        fail_acks(acknowledgements);
                        return true;
                    }
                }
                Some(Ok(Message::Binary(bytes))) => {
                    if let Some((name, _)) = binary_event.take() {
                        let _ = events.send(SocketEvent::new(name, SocketPayload::Binary(bytes))).await;
                    }
                }
                Some(Ok(Message::Ping(bytes))) => { if socket.send(Message::Pong(bytes)).await.is_err() { fail_acks(acknowledgements); return true; } }
                Some(Ok(Message::Close(_))) | None | Some(Err(_)) => { fail_acks(acknowledgements); return true; }
                _ => {}
            }
        }
    }
}

async fn send_command(
    socket: &mut SocketStream,
    command: Command,
    ack_id: &mut u64,
    acknowledgements: &mut VecDeque<PendingAck>,
) -> bool {
    match command {
        Command::Emit {
            event,
            payload,
            ack,
            ack_timeout,
            sent,
        } => {
            let id = ack.as_ref().map(|_| {
                *ack_id += 1;
                *ack_id
            });
            let packet = match (&payload, id) {
                (OutgoingPayload::Json(value), Some(id)) => {
                    format!("42{id}{}", json!([event, value]))
                }
                (OutgoingPayload::Json(value), None) => format!("42{}", json!([event, value])),
                (OutgoingPayload::Binary(_), Some(_)) => {
                    if let Some(sent) = sent {
                        let _ = sent.send(Err(Error::SocketAckClosed));
                    }
                    if let Some(ack) = ack {
                        let _ = ack.send(Err(Error::SocketAckClosed));
                    }
                    return true;
                }
                (OutgoingPayload::Binary(bytes), None) => {
                    let header =
                        format!("451-[{},{{\"_placeholder\":true,\"num\":0}}]", json!(event));
                    if socket.send(Message::Text(header)).await.is_err()
                        || socket.send(Message::Binary(bytes.clone())).await.is_err()
                    {
                        if let Some(sent) = sent {
                            let _ = sent.send(Err(Error::SocketAckClosed));
                        }
                        return false;
                    }
                    if let Some(sent) = sent {
                        let _ = sent.send(Ok(()));
                    }
                    return true;
                }
            };
            if socket.send(Message::Text(packet)).await.is_err() {
                if let Some(sent) = sent {
                    let _ = sent.send(Err(Error::SocketAckClosed));
                }
                if let Some(ack) = ack {
                    let _ = ack.send(Err(Error::SocketAckClosed));
                }
                return false;
            }
            if let Some(sent) = sent {
                let _ = sent.send(Ok(()));
            }
            if let (Some(id), Some(ack)) = (id, ack) {
                acknowledgements.push_back((
                    id,
                    Instant::now() + ack_timeout.unwrap_or(CONNECT_TIMEOUT),
                    ack,
                ));
            }
            true
        }
    }
}

fn parse_event(json_text: &str) -> Option<(String, Vec<Value>)> {
    let mut values = serde_json::from_str::<Vec<Value>>(json_text).ok()?;
    let name = values.first()?.as_str()?.to_owned();
    values.remove(0);
    Some((name, values))
}

fn parse_binary_event(packet: &str) -> Option<(String, usize)> {
    let (_, json) = packet.split_once('-')?;
    let values = serde_json::from_str::<Vec<Value>>(json).ok()?;
    Some((values.first()?.as_str()?.to_owned(), 1))
}

fn handle_ack(frame: &str, acknowledgements: &mut VecDeque<PendingAck>) {
    let Some(tail) = frame.strip_prefix("43") else {
        return;
    };
    let digit_count = tail.bytes().take_while(u8::is_ascii_digit).count();
    let Ok(id) = tail[..digit_count].parse::<u64>() else {
        return;
    };
    let Ok(values) = serde_json::from_str::<Vec<Value>>(&tail[digit_count..]) else {
        return;
    };
    if let Some(index) = acknowledgements
        .iter()
        .position(|(pending, _, _)| *pending == id)
    {
        if let Some((_, _, sender)) = acknowledgements.remove(index) {
            let _ = sender.send(Ok(SocketPayload::Json(values)));
        }
    }
}

fn fail_acks(acks: VecDeque<PendingAck>) {
    for (_, _, sender) in acks {
        let _ = sender.send(Err(Error::SocketAckClosed));
    }
}

async fn next_ack_deadline(acknowledgements: &VecDeque<PendingAck>) {
    let deadline = acknowledgements
        .iter()
        .map(|(_, deadline, _)| *deadline)
        .min()
        .unwrap_or_else(|| Instant::now() + CONNECT_TIMEOUT);
    sleep_until(deadline).await;
}

fn expire_acks(acknowledgements: &mut VecDeque<PendingAck>) {
    let now = Instant::now();
    let mut index = 0;
    while index < acknowledgements.len() {
        if acknowledgements[index].1 <= now {
            if let Some((_, _, sender)) = acknowledgements.remove(index) {
                let _ = sender.send(Err(Error::SocketAckTimeout));
            }
        } else if acknowledgements[index].2.is_closed() {
            acknowledgements.remove(index);
        } else {
            index += 1;
        }
    }
}

fn drain_pending(pending: &mut VecDeque<Command>) {
    while let Some(command) = pending.pop_front() {
        match command {
            Command::Emit { ack, sent, .. } => {
                if let Some(sent) = sent {
                    let _ = sent.send(Err(Error::SocketAckClosed));
                }
                if let Some(ack) = ack {
                    let _ = ack.send(Err(Error::SocketAckClosed));
                }
            }
        }
    }
}

fn drain_command_queue(commands: &mut mpsc::Receiver<Command>) {
    while let Ok(command) = commands.try_recv() {
        fail_command(command);
    }
}

fn fail_command(command: Command) {
    match command {
        Command::Emit { ack, sent, .. } => {
            if let Some(sent) = sent {
                let _ = sent.send(Err(Error::SocketAckClosed));
            }
            if let Some(ack) = ack {
                let _ = ack.send(Err(Error::SocketAckClosed));
            }
        }
    }
}

fn command_is_cancelled(command: &Command) -> bool {
    matches!(command, Command::Emit { ack: Some(sender), .. } if sender.is_closed())
}

async fn send_error(events: &mpsc::Sender<SocketEvent>, message: &str) {
    let _ = events
        .send(SocketEvent::supervisor(
            "error",
            SocketPayload::Json(vec![Value::String(message.to_owned())]),
        ))
        .await;
}

async fn send_escalation(events: &mpsc::Sender<SocketEvent>, attempts: u32) {
    let _ = events
        .send(SocketEvent::supervisor(
            "reconnect:escalated",
            SocketPayload::Json(vec![json!({ "attempts": attempts })]),
        ))
        .await;
}
