use super::*;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[test]
fn endpoint_policy_allows_loopback_http_and_remote_tls() {
    assert!(validate_endpoint("http://127.0.0.1:7788").is_ok());
    assert!(validate_endpoint("http://[::1]:7788").is_ok());
    assert!(validate_endpoint("https://api.example.test").is_ok());
    assert!(validate_endpoint("wss://api.example.test").is_ok());
}

#[test]
fn endpoint_policy_rejects_remote_plaintext_and_unknown_schemes() {
    assert!(matches!(
        validate_endpoint("http://api.example.test"),
        Err(Error::UnsafeSocketEndpoint)
    ));
    assert!(matches!(
        validate_endpoint("ftp://localhost"),
        Err(Error::UnsafeSocketEndpoint)
    ));
    assert!(matches!(
        validate_endpoint("not a url"),
        Err(Error::UnsafeSocketEndpoint)
    ));
}

#[test]
fn invalid_token_detection_matches_terminal_connect_rejections_only() {
    assert!(is_invalid_token_text(r#"{"message":"Invalid token"}"#));
    assert!(is_invalid_token_text("session token expired"));
    assert!(!is_invalid_token_text(
        r#"{"message":"gateway unavailable"}"#
    ));
}

#[test]
fn token_provider_is_queried_for_each_connection_attempt() {
    let attempts = std::sync::atomic::AtomicUsize::new(0);
    let provider = || {
        let attempt = attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Some(format!("token-{attempt}"))
    };

    assert_eq!(next_token(&provider).as_deref(), Some("token-0"));
    assert_eq!(next_token(&provider).as_deref(), Some("token-1"));
    assert_eq!(attempts.load(std::sync::atomic::Ordering::SeqCst), 2);
}

#[test]
fn redirect_policy_follows_same_origin_and_rejects_cross_origin_credentials() {
    assert_eq!(
        redirect_target(
            "wss://api.example.test/socket.io/?EIO=4",
            "/edge/socket.io/?EIO=4"
        )
        .unwrap(),
        "wss://api.example.test/edge/socket.io/?EIO=4"
    );
    assert!(matches!(
        redirect_target(
            "wss://api.example.test/socket.io/?EIO=4",
            "wss://other.example.test/socket.io/?EIO=4"
        ),
        Err(Error::CrossOriginSocketRedirect)
    ));
}

#[test]
fn backend_url_is_converted_to_engine_io_websocket_endpoint() {
    assert_eq!(
        websocket_url("https://api.example.test/base/"),
        "wss://api.example.test/base/socket.io/?EIO=4&transport=websocket"
    );
    assert_eq!(
        websocket_url("http://127.0.0.1:7788"),
        "ws://127.0.0.1:7788/socket.io/?EIO=4&transport=websocket"
    );
}

#[tokio::test]
async fn connect_event_exposes_socket_io_sid_and_emitter_drains_events_during_ack() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        socket
            .send(Message::Text(
                r#"0{"sid":"engine-sid","pingInterval":60000,"pingTimeout":60000}"#.into(),
            ))
            .await
            .unwrap();
        let Some(Ok(Message::Text(auth))) = socket.next().await else {
            panic!("expected Socket.IO auth packet");
        };
        assert!(auth.starts_with("40"));
        socket
            .send(Message::Text(r#"40{"sid":"socket-io-sid"}"#.into()))
            .await
            .unwrap();

        let Some(Ok(Message::Text(emit))) = socket.next().await else {
            panic!("expected acknowledgement emit");
        };
        let tail = emit.strip_prefix("42").unwrap();
        let ack_id = tail.bytes().take_while(u8::is_ascii_digit).count();
        let id = &tail[..ack_id];
        socket
            .send(Message::Text(format!(
                "42{}",
                json!(["error", {"msg": "backend failure"}])
            )))
            .await
            .unwrap();
        for index in 0..257 {
            socket
                .send(Message::Text(format!("42{}", json!(["burst", index]))))
                .await
                .unwrap();
        }
        socket
            .send(Message::Text(format!("43{id}[{{\"ok\":true}}]")))
            .await
            .unwrap();
    });

    let mut connection = SocketConnection::connect(
        format!("http://{address}"),
        Arc::new(|| Some("token".to_owned())),
    )
    .await
    .unwrap();
    let _open = connection.next_event().await.unwrap();
    let connected = connection.next_event().await.unwrap();
    assert_eq!(connected.name, "connect");
    assert_eq!(connected.source, SocketEventSource::Supervisor);
    assert_eq!(connected.decode::<Value>().unwrap()["sid"], "socket-io-sid");

    let emitter = connection.emitter();
    let ack = tokio::spawn(async move {
        emitter
            .emit_with_ack::<_, Value>("device:hello", &json!({"ok": true}), Duration::from_secs(3))
            .await
    });
    let mut burst_events = 0;
    let mut backend_error_seen = false;
    while burst_events < 257 {
        let event = tokio::time::timeout(Duration::from_secs(3), connection.next_event())
            .await
            .unwrap()
            .unwrap();
        if event.name == "burst" {
            burst_events += 1;
        } else if event.name == "error" {
            assert_eq!(event.source, SocketEventSource::Backend);
            assert_eq!(event.decode::<Value>().unwrap()["msg"], "backend failure");
            backend_error_seen = true;
        }
    }
    assert!(backend_error_seen);
    assert_eq!(ack.await.unwrap().unwrap(), json!({"ok": true}));
    connection.disconnect().await.unwrap();
    server.await.unwrap();
}
