//! Authenticated Socket.IO transport for live backend events.
//!
//! The backend uses Socket.IO v4 framing on `/socket.io/`; it is not a raw
//! WebSocket endpoint. [`SocketConnection`] owns the protocol connection and
//! its bounded reconnect supervisor, exposes event names through
//! [`SocketConnection::next_event`], and supports JSON, binary, and
//! acknowledgement-based emits. Credentials are sent only after a validated
//! same-origin WebSocket upgrade and Socket.IO CONNECT is acknowledged before
//! queued emits are sent.

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot, watch};
use url::Url;

use crate::Error;
mod supervisor;
use supervisor::{Command, OutgoingPayload, RetryConfig, Settings, TokenProvider};

pub mod events;

/// A Socket.IO event payload.
///
/// Socket.IO text packets may carry more than one JSON argument, so JSON is a
/// vector rather than a single value. Most TinyHumans events use the first
/// element. Binary packets remain raw bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketPayload {
    Json(Vec<Value>),
    Binary(Vec<u8>),
}

impl SocketPayload {
    /// Decode the first JSON argument as a concrete event type.
    pub fn decode<T: DeserializeOwned>(&self, event_name: &str) -> Result<T, Error> {
        let Self::Json(values) = self else {
            return Err(Error::UnexpectedSocketPayload(event_name.to_owned()));
        };
        let value = values
            .first()
            .cloned()
            .ok_or_else(|| Error::UnexpectedSocketPayload(event_name.to_owned()))?;
        Ok(serde_json::from_value(value)?)
    }
}

/// Source of an incoming Socket.IO event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketEventSource {
    /// An event received by name from the backend Socket.IO namespace.
    Backend,
    /// A lifecycle or transport event synthesized by the SDK supervisor.
    Supervisor,
}

/// One incoming Socket.IO event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketEvent {
    /// Identifies whether the event came from the backend namespace or the
    /// SDK's connection supervisor.
    pub source: SocketEventSource,
    pub name: String,
    pub payload: SocketPayload,
}

impl SocketEvent {
    /// Construct an event. Useful when forwarding or testing event decoders.
    pub fn new(name: impl Into<String>, payload: SocketPayload) -> Self {
        Self {
            source: SocketEventSource::Backend,
            name: name.into(),
            payload,
        }
    }

    /// Construct an SDK supervisor lifecycle or transport event.
    pub fn supervisor(name: impl Into<String>, payload: SocketPayload) -> Self {
        Self {
            source: SocketEventSource::Supervisor,
            name: name.into(),
            payload,
        }
    }

    /// Decode the event's first JSON argument.
    pub fn decode<T: DeserializeOwned>(&self) -> Result<T, Error> {
        self.payload.decode(&self.name)
    }
}

/// An authenticated Socket.IO connection supervisor and its incoming events.
///
/// Connection attempts run in the background. Emit requests are queued until
/// the server acknowledges the Socket.IO CONNECT packet. Reconnect attempts
/// refresh the token through the configured provider.
pub struct SocketConnection {
    commands: mpsc::Sender<Command>,
    events: mpsc::Receiver<SocketEvent>,
    loop_active: Arc<AtomicBool>,
    stopped_on_failure: Arc<AtomicBool>,
    shutdown: watch::Sender<bool>,
}

/// Cloneable outbound handle for a [`SocketConnection`].
///
/// The event stream remains owned by [`SocketConnection`], so callers can
/// receive inbound events while one or more emits wait for delivery or ACKs.
#[derive(Clone)]
pub struct SocketEmitter {
    commands: mpsc::Sender<Command>,
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;

impl fmt::Debug for SocketConnection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SocketConnection")
            .finish_non_exhaustive()
    }
}

impl SocketConnection {
    pub(crate) async fn connect(
        base_url: String,
        token_provider: TokenProvider,
    ) -> Result<Self, Error> {
        validate_endpoint(&base_url)?;
        let token = next_token(token_provider.as_ref()).ok_or(Error::MissingSocketToken)?;
        let (commands, command_rx) = mpsc::channel(128);
        let (event_tx, events) = mpsc::channel(256);
        let (shutdown, shutdown_rx) = watch::channel(false);
        let loop_active = Arc::new(AtomicBool::new(false));
        let stopped_on_failure = Arc::new(AtomicBool::new(false));
        tokio::spawn(supervisor::run(
            Settings {
                base_url,
                initial_token: token,
                token_provider: Arc::clone(&token_provider),
                loop_active: Arc::clone(&loop_active),
                stopped_on_failure: Arc::clone(&stopped_on_failure),
                retry: RetryConfig::default(),
            },
            command_rx,
            event_tx,
            shutdown_rx,
        ));

        Ok(Self {
            commands,
            events,
            loop_active,
            stopped_on_failure,
            shutdown,
        })
    }

    /// Wait for the next incoming event.
    ///
    /// Returns `None` after the supervisor has stopped. Network interruptions
    /// produce `close`/`error` events while bounded reconnect attempts continue.
    pub async fn next_event(&mut self) -> Option<SocketEvent> {
        self.events.recv().await
    }

    /// Emit any serializable JSON payload to any public backend socket event.
    pub async fn emit<T: Serialize>(&self, event: &str, payload: &T) -> Result<(), Error> {
        self.emitter().emit(event, payload).await
    }

    /// Emit a binary Socket.IO packet.
    pub async fn emit_binary(&self, event: &str, bytes: Vec<u8>) -> Result<(), Error> {
        self.emitter().emit_binary(event, bytes).await
    }

    /// Emit JSON and decode the server acknowledgement.
    pub async fn emit_with_ack<T, R>(
        &self,
        event: &str,
        payload: &T,
        timeout: Duration,
    ) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.emitter().emit_with_ack(event, payload, timeout).await
    }

    /// Clone a handle that sends outbound packets without owning the event stream.
    pub fn emitter(&self) -> SocketEmitter {
        SocketEmitter {
            commands: self.commands.clone(),
        }
    }

    /// Stop reconnecting and close the connection intentionally.
    ///
    /// Pending emits and acknowledgement waiters are failed during teardown.
    pub async fn disconnect(&self) -> Result<(), Error> {
        self.shutdown.send_replace(true);
        Ok(())
    }

    /// Whether the SDK's bounded reconnect loop is still active.
    pub fn loop_active(&self) -> bool {
        self.loop_active.load(Ordering::Acquire)
    }

    /// Whether the supervisor stopped after a terminal authentication or
    /// endpoint-policy failure, or after exhausting its retry attempts.
    pub fn stopped_on_failure(&self) -> bool {
        self.stopped_on_failure.load(Ordering::Acquire)
    }
}

impl SocketEmitter {
    /// Emit any serializable JSON payload to any public backend socket event.
    pub async fn emit<T: Serialize>(&self, event: &str, payload: &T) -> Result<(), Error> {
        let (sent, result) = oneshot::channel();
        self.commands
            .send(Command::Emit {
                event: event.to_owned(),
                payload: OutgoingPayload::Json(serde_json::to_value(payload)?),
                ack: None,
                ack_timeout: None,
                sent: Some(sent),
            })
            .await
            .map_err(|_| Error::SocketAckClosed)?;
        result.await.map_err(|_| Error::SocketAckClosed)?
    }

    /// Emit a binary Socket.IO packet.
    pub async fn emit_binary(&self, event: &str, bytes: Vec<u8>) -> Result<(), Error> {
        let (sent, result) = oneshot::channel();
        self.commands
            .send(Command::Emit {
                event: event.to_owned(),
                payload: OutgoingPayload::Binary(bytes),
                ack: None,
                ack_timeout: None,
                sent: Some(sent),
            })
            .await
            .map_err(|_| Error::SocketAckClosed)?;
        result.await.map_err(|_| Error::SocketAckClosed)?
    }

    /// Emit JSON and decode the server acknowledgement.
    pub async fn emit_with_ack<T, R>(
        &self,
        event: &str,
        payload: &T,
        timeout: Duration,
    ) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .send(Command::Emit {
                event: event.to_owned(),
                payload: OutgoingPayload::Json(serde_json::to_value(payload)?),
                ack: Some(sender),
                ack_timeout: Some(timeout),
                sent: None,
            })
            .await
            .map_err(|_| Error::SocketAckClosed)?;

        let payload = tokio::time::timeout(timeout, receiver)
            .await
            .map_err(|_| Error::SocketAckTimeout)?
            .map_err(|_| Error::SocketAckClosed)?;
        payload?.decode(event)
    }
}

fn next_token(provider: &(dyn Fn() -> Option<String> + Send + Sync)) -> Option<String> {
    provider().filter(|token| !token.trim().is_empty())
}

fn is_invalid_token_text(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("invalid token") || text.contains("token expired")
}

fn validate_endpoint(endpoint: &str) -> Result<Url, Error> {
    let url = Url::parse(endpoint).map_err(|_| Error::UnsafeSocketEndpoint)?;
    let secure = matches!(url.scheme(), "https" | "wss");
    let loopback_transport = matches!(url.scheme(), "http" | "ws");
    let loopback = url.host_str().is_some_and(|host| {
        host.eq_ignore_ascii_case("localhost")
            || host
                .trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    if !(secure || (loopback_transport && loopback))
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(Error::UnsafeSocketEndpoint);
    }
    Ok(url)
}

fn websocket_url(base_url: &str) -> String {
    let mut url = Url::parse(base_url).expect("validated socket endpoint URL");
    let scheme = match url.scheme() {
        "https" => "wss".to_owned(),
        "http" => "ws".to_owned(),
        "wss" | "ws" => url.scheme().to_owned(),
        _ => unreachable!("endpoint validation permits only HTTP(S)/WS(S)"),
    };
    url.set_scheme(&scheme)
        .expect("websocket schemes are compatible");
    let base_path = url.path().trim_end_matches('/');
    let path = if base_path.is_empty() {
        "/socket.io/".to_owned()
    } else {
        format!("{base_path}/socket.io/")
    };
    url.set_path(&path);
    url.set_query(Some("EIO=4&transport=websocket"));
    url.to_string()
}

fn redirect_target(current: &str, location: &str) -> Result<String, Error> {
    let current_url = Url::parse(current).map_err(|_| Error::UnsafeSocketEndpoint)?;
    let mut target = current_url
        .join(location)
        .map_err(|_| Error::UnsafeSocketEndpoint)?;
    let target_scheme = match target.scheme() {
        "http" => "ws".to_owned(),
        "https" => "wss".to_owned(),
        "ws" | "wss" => target.scheme().to_owned(),
        _ => return Err(Error::UnsafeSocketEndpoint),
    };
    target
        .set_scheme(&target_scheme)
        .map_err(|_| Error::UnsafeSocketEndpoint)?;
    validate_endpoint(target.as_str())?;
    if target.origin() != current_url.origin() {
        return Err(Error::CrossOriginSocketRedirect);
    }
    Ok(target.to_string())
}
