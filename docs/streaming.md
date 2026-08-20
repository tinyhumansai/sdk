# Streaming

The SDK supports both live transports exposed by the backend:

- Reconnecting HTTP Server-Sent Events for a durable Medulla session.
- Authenticated Socket.IO for channels, agent media, orchestration, Medulla
  harnesses and workflows, WebRTC signaling, meeting bots, and webhook tunnels.

## Medulla session SSE

`session_event_stream` replays from `Last-Event-ID`, reconnects after a closed
response, and de-duplicates persisted sequence numbers. Advisory token deltas
without a sequence are passed through immediately.

```rust
use futures::StreamExt;
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new("https://api.tinyhumans.ai")
    .with_token(std::env::var("TINYHUMANS_TOKEN").ok());
let stream = client.raw().session_event_stream("session-id", None);
tokio::pin!(stream);

while let Some(envelope) = stream.next().await {
    println!("{:?}", envelope?.kind());
}
# Ok(())
# }
```

## Socket.IO

`connect_socket` authenticates with the same bearer token and reconnects after
transport failures. `next_event` receives the complete backend event catalog.
Use `emit`, `emit_binary`, and `emit_with_ack` for non-Medulla event families;
the constants in `socket::events` prevent event-name typos.

Medulla has typed helpers for all six client-to-server harness events and a
typed decoder for all six server-to-client task, capability, workflow, and
session-stream events:

```rust
use tinyhumans_sdk::socket::medulla::MedullaServerEvent;
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new("https://api.tinyhumans.ai")
    .with_token(std::env::var("TINYHUMANS_TOKEN").ok());
let mut socket = client.connect_socket().await?;

while let Some(event) = socket.next_event().await {
    match event.decode_medulla()? {
        Some(MedullaServerEvent::TaskRun(task)) => {
            println!("run {}: {}", task.task_id, task.instruction);
        }
        Some(MedullaServerEvent::WorkflowRequest(request)) => {
            println!("workflow request: {:?}", request.op);
        }
        Some(MedullaServerEvent::SessionEvent(envelope)) => {
            println!("session event: {:?}", envelope.kind());
        }
        _ => {}
    }
}
# Ok(())
# }
```

Socket.IO binary packets are surfaced as `SocketPayload::Binary`; JSON events
are `SocketPayload::Json`. This allows agent audio/video chunks and tunnel
frames to be forwarded without string conversion.
