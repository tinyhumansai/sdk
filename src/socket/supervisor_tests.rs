use super::*;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::test]
async fn supervisor_error_events_are_tagged_as_internal() {
    let (events, mut receiver) = mpsc::channel(1);

    send_error(&events, "websocket upgrade failed").await;

    let event = receiver.recv().await.unwrap();
    assert_eq!(event.name, "error");
    assert_eq!(event.source, super::super::SocketEventSource::Supervisor);
}

#[tokio::test]
async fn every_websocket_redirect_hop_has_its_own_connect_deadline() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = [0; 1024];
        let _ = stream.read(&mut request).await.unwrap();
        tokio::time::sleep(Duration::from_millis(150)).await;
    });
    let (_shutdown_tx, mut shutdown_rx) = watch::channel(false);
    let mut endpoint = format!("ws://{address}/socket.io/?EIO=4&transport=websocket");
    let started = Instant::now();
    let error =
        connect_with_redirects_within(&mut endpoint, &mut shutdown_rx, Duration::from_millis(25))
            .await
            .unwrap_err();
    assert!(matches!(error, ConnectFailure::Retryable(message) if message.contains("timed out")));
    assert!(started.elapsed() >= Duration::from_millis(25));
    server.await.unwrap();
}

#[tokio::test]
async fn cross_origin_redirect_is_rejected_before_the_target_handshake() {
    let redirect = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redirect_address = redirect.local_addr().unwrap();
    let target = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let target_address = target.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = redirect.accept().await.unwrap();
        let mut request = [0; 1024];
        let _ = stream.read(&mut request).await.unwrap();
        let response = format!(
            "HTTP/1.1 302 Found\r\nLocation: ws://{target_address}/socket.io/?EIO=4&transport=websocket\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        );
        tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes())
            .await
            .unwrap();
    });
    let (_shutdown_tx, mut shutdown_rx) = watch::channel(false);
    let mut endpoint = format!("ws://{redirect_address}/socket.io/?EIO=4&transport=websocket");
    let result =
        connect_with_redirects_within(&mut endpoint, &mut shutdown_rx, Duration::from_secs(1))
            .await;
    assert!(
        matches!(result, Err(ConnectFailure::Policy(message)) if message.contains("redirect changed origin"))
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(50), target.accept())
            .await
            .is_err()
    );
    server.await.unwrap();
}

#[test]
fn socket_event_parser_preserves_all_json_arguments() {
    assert_eq!(
        parse_event(r#"["device:update",{"value":1},"extra"]"#),
        Some((
            "device:update".to_owned(),
            vec![json!({"value": 1}), json!("extra")]
        ))
    );
}

#[tokio::test]
async fn teardown_drain_fails_queued_ack_waiters() {
    let (commands, mut receiver) = mpsc::channel(1);
    let (ack, result) = oneshot::channel();
    commands
        .send(Command::Emit {
            event: "device:hello".to_owned(),
            payload: OutgoingPayload::Json(json!({"ok": true})),
            ack: Some(ack),
            ack_timeout: Some(Duration::from_secs(1)),
            sent: None,
        })
        .await
        .unwrap();

    drain_command_queue(&mut receiver);

    assert!(matches!(result.await.unwrap(), Err(Error::SocketAckClosed)));
}

#[tokio::test(start_paused = true)]
async fn queued_emits_do_not_shorten_the_reconnect_backoff() {
    let (commands, mut command_rx) = mpsc::channel(4);
    let (shutdown, mut shutdown_rx) = watch::channel(false);
    let mut pending = VecDeque::new();
    let retry = tokio::spawn(async move {
        let stopped = sleep_or_disconnect(
            Duration::from_secs(10),
            &mut command_rx,
            &mut pending,
            &mut shutdown_rx,
        )
        .await;
        (stopped, pending)
    });

    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(5)).await;
    commands
        .send(Command::Emit {
            event: "device:hello".to_owned(),
            payload: OutgoingPayload::Json(json!({"ok": true})),
            ack: None,
            ack_timeout: None,
            sent: None,
        })
        .await
        .unwrap();
    tokio::task::yield_now().await;

    assert!(!retry.is_finished(), "an emit must not end the retry delay");
    tokio::time::advance(Duration::from_secs(5)).await;
    let (stopped, pending) = retry.await.unwrap();
    assert!(!stopped);
    assert_eq!(pending.len(), 1);
    drop(shutdown);
}

#[tokio::test(start_paused = true)]
async fn retry_exhaustion_sets_terminal_flags() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let loop_active = Arc::new(AtomicBool::new(false));
    let stopped_on_failure = Arc::new(AtomicBool::new(false));
    let (_command_tx, command_rx) = mpsc::channel(1);
    let (events_tx, _events_rx) = mpsc::channel(16);
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    let settings = Settings {
        base_url: format!("http://{address}"),
        initial_token: "token".to_owned(),
        token_provider: Arc::new(|| Some("token".to_owned())),
        loop_active: Arc::clone(&loop_active),
        stopped_on_failure: Arc::clone(&stopped_on_failure),
        retry: RetryConfig {
            max_attempts: 2,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(2),
        },
    };

    run(settings, command_rx, events_tx, shutdown_rx).await;

    assert!(!loop_active.load(Ordering::Acquire));
    assert!(stopped_on_failure.load(Ordering::Acquire));
}

#[tokio::test(start_paused = true)]
async fn supervisor_emits_one_escalation_event_at_attempt_five() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let loop_active = Arc::new(AtomicBool::new(false));
    let stopped_on_failure = Arc::new(AtomicBool::new(false));
    let (_command_tx, command_rx) = mpsc::channel(1);
    let (events_tx, mut events_rx) = mpsc::channel(16);
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    let settings = Settings {
        base_url: format!("http://{address}"),
        initial_token: "token".to_owned(),
        token_provider: Arc::new(|| Some("token".to_owned())),
        loop_active: Arc::clone(&loop_active),
        stopped_on_failure: Arc::clone(&stopped_on_failure),
        retry: RetryConfig {
            max_attempts: 6,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(4),
        },
    };

    run(settings, command_rx, events_tx, shutdown_rx).await;

    let mut escalation_events = 0;
    while let Ok(event) = events_rx.try_recv() {
        if event.name == "reconnect:escalated" {
            escalation_events += 1;
            assert_eq!(
                event.payload.decode::<Value>(&event.name).unwrap()["attempts"],
                5
            );
        }
    }
    assert_eq!(escalation_events, 1);
    assert!(!loop_active.load(Ordering::Acquire));
    assert!(stopped_on_failure.load(Ordering::Acquire));
}

#[test]
fn retries_escalate_once_at_attempt_five_and_stop_at_ten_with_capped_backoff() {
    let mut retry = RetryState::default();
    let expected = [1, 2, 4, 8, 16, 30, 30, 30, 30, 30];

    for (index, seconds) in expected.into_iter().enumerate() {
        let decision = retry.failed_attempt();
        assert_eq!(decision.delay, Duration::from_secs(seconds));
        assert_eq!(decision.escalate, index == 4);
        assert_eq!(decision.exhausted, index == 9);
    }
}
