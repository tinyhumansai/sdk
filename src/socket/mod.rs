//! Authenticated Socket.IO transport for live backend events.
//!
//! The backend uses Socket.IO v4 framing on `/socket.io/`; it is not a raw
//! WebSocket endpoint. [`SocketConnection`] owns the protocol connection,
//! automatically reconnects after network failures, exposes all event names
//! through [`SocketConnection::next_event`], and supports JSON, binary, and
//! acknowledgement-based emits. Typed Medulla helpers live in [`medulla`].

use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::FutureExt;
use rust_socketio::asynchronous::{Client, ClientBuilder};
use rust_socketio::Payload;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

use crate::Error;

pub mod events;
pub mod medulla;

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
    fn from_socket_io(payload: Payload) -> Self {
        match payload {
            Payload::Text(values) => Self::Json(values),
            Payload::Binary(bytes) => Self::Binary(bytes.to_vec()),
            #[allow(deprecated)]
            Payload::String(text) => Self::Json(vec![
                serde_json::from_str(&text).unwrap_or(Value::String(text))
            ]),
        }
    }

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

/// One incoming Socket.IO event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketEvent {
    pub name: String,
    pub payload: SocketPayload,
}

impl SocketEvent {
    /// Construct an event. Useful when forwarding or testing event decoders.
    pub fn new(name: impl Into<String>, payload: SocketPayload) -> Self {
        Self {
            name: name.into(),
            payload,
        }
    }

    /// Decode the event's first JSON argument.
    pub fn decode<T: DeserializeOwned>(&self) -> Result<T, Error> {
        self.payload.decode(&self.name)
    }
}

/// A connected, authenticated Socket.IO client plus its incoming event queue.
pub struct SocketConnection {
    client: Client,
    events: mpsc::UnboundedReceiver<SocketEvent>,
}

impl fmt::Debug for SocketConnection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SocketConnection")
            .finish_non_exhaustive()
    }
}

impl SocketConnection {
    pub(crate) async fn connect(base_url: String, token: String) -> Result<Self, Error> {
        let (sender, events) = mpsc::unbounded_channel();

        let any_sender = sender.clone();
        let open_sender = sender.clone();
        let close_sender = sender.clone();
        let error_sender = sender;
        let client = ClientBuilder::new(base_url)
            .auth(serde_json::json!({ "token": token }))
            .opening_header("x-sdk-client", "tinyhumans-rust")
            .on_any(move |event, payload, _| {
                let sender = any_sender.clone();
                async move {
                    let _ = sender.send(SocketEvent::new(
                        event.to_string(),
                        SocketPayload::from_socket_io(payload),
                    ));
                }
                .boxed()
            })
            .on("open", move |payload, _| {
                let sender = open_sender.clone();
                async move {
                    let _ = sender.send(SocketEvent::new(
                        "open",
                        SocketPayload::from_socket_io(payload),
                    ));
                }
                .boxed()
            })
            .on("close", move |payload, _| {
                let sender = close_sender.clone();
                async move {
                    let _ = sender.send(SocketEvent::new(
                        "close",
                        SocketPayload::from_socket_io(payload),
                    ));
                }
                .boxed()
            })
            .on("error", move |payload, _| {
                let sender = error_sender.clone();
                async move {
                    let _ = sender.send(SocketEvent::new(
                        "error",
                        SocketPayload::from_socket_io(payload),
                    ));
                }
                .boxed()
            })
            .connect()
            .await?;

        Ok(Self { client, events })
    }

    /// Wait for the next incoming event.
    ///
    /// Returns `None` only after every transport callback has gone away. Network
    /// interruptions normally produce `close`/`error` events and reconnect.
    pub async fn next_event(&mut self) -> Option<SocketEvent> {
        self.events.recv().await
    }

    /// Emit any serializable JSON payload to any public backend socket event.
    pub async fn emit<T: Serialize>(&self, event: &str, payload: &T) -> Result<(), Error> {
        self.client
            .emit(event, serde_json::to_value(payload)?)
            .await?;
        Ok(())
    }

    /// Emit a binary Socket.IO packet.
    pub async fn emit_binary(&self, event: &str, bytes: Vec<u8>) -> Result<(), Error> {
        self.client.emit(event, bytes).await?;
        Ok(())
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
        let sender = Arc::new(Mutex::new(Some(sender)));
        let callback_sender = Arc::clone(&sender);

        self.client
            .emit_with_ack(
                event,
                serde_json::to_value(payload)?,
                timeout,
                move |payload, _| {
                    let sender = Arc::clone(&callback_sender);
                    async move {
                        if let Some(sender) = sender.lock().expect("ack sender poisoned").take() {
                            let _ = sender.send(SocketPayload::from_socket_io(payload));
                        }
                    }
                    .boxed()
                },
            )
            .await?;

        let payload = tokio::time::timeout(timeout, receiver)
            .await
            .map_err(|_| Error::SocketAckTimeout)?
            .map_err(|_| Error::SocketAckClosed)?;
        payload.decode(event)
    }

    /// Close the connection intentionally.
    pub async fn disconnect(&self) -> Result<(), Error> {
        self.client.disconnect().await?;
        Ok(())
    }
}
