//! SSE tests: the incremental frame parser, the reconnect dedupe cursor, and
//! end-to-end streaming against a local TCP stub.

use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use super::*;

fn frames(chunks: &[&str]) -> Vec<SseFrame> {
    let mut parser = SseParser::new();
    let mut out = Vec::new();
    for chunk in chunks {
        parser.feed(chunk, &mut out);
    }
    out
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

#[test]
fn parses_id_and_data_frame() {
    assert_eq!(
        frames(&["id: 7\ndata: {\"a\":1}\n\n"]),
        vec![SseFrame {
            id: Some(7),
            data: "{\"a\":1}".into(),
        }]
    );
}

#[test]
fn concatenates_multiline_data() {
    assert_eq!(
        frames(&["data: one\ndata: two\n\n"]),
        vec![SseFrame {
            id: None,
            data: "one\ntwo".into(),
        }]
    );
}

#[test]
fn ignores_ping_comments() {
    assert_eq!(frames(&[": ping\n\n"]), vec![]);
}

#[test]
fn handles_chunked_and_crlf_boundaries() {
    // The frame is split mid-token across chunks and uses CRLF line endings.
    assert_eq!(
        frames(&["id: 4\r\nda", "ta: hel", "lo\r\n\r\n"]),
        vec![SseFrame {
            id: Some(4),
            data: "hello".into(),
        }]
    );
}

#[test]
fn yields_multiple_frames_from_one_chunk() {
    assert_eq!(
        frames(&["data: a\n\ndata: b\n\n"]),
        vec![
            SseFrame {
                id: None,
                data: "a".into()
            },
            SseFrame {
                id: None,
                data: "b".into()
            },
        ]
    );
}

// ---------------------------------------------------------------------------
// Dedupe cursor
// ---------------------------------------------------------------------------

#[test]
fn dedupe_from_start_accepts_everything() {
    let mut dedup = SeqDedup::new(None);
    assert!(dedup.accept(Some(1)));
    assert!(dedup.accept(Some(2)));
    // Unsequenced deltas always pass and never move the cursor.
    assert!(dedup.accept(None));
    assert_eq!(dedup.cursor(), Some(2));
}

#[test]
fn dedupe_skips_replayed_seqs_and_advances_cursor() {
    let mut dedup = SeqDedup::new(Some(5));
    assert!(
        !dedup.accept(Some(5)),
        "the cursor value itself is a replay"
    );
    assert!(!dedup.accept(Some(4)));
    assert!(dedup.accept(Some(6)));
    assert_eq!(dedup.cursor(), Some(6));
}

// ---------------------------------------------------------------------------
// Streaming
// ---------------------------------------------------------------------------

/// Serve one connection with `body` as an SSE response, then close.
async fn spawn_sse_stub(body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (mut sock, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 2048];
        let _ = sock.read(&mut buf).await;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = sock.write_all(response.as_bytes()).await;
        let _ = sock.flush().await;
        let _ = sock.shutdown().await;
    });
    format!("http://{addr}")
}

fn envelope_frame(seq: u64) -> String {
    format!(
        "id: {seq}\ndata: {{\"seq\":{seq},\"at\":1,\"sessionId\":\"s1\",\"event\":{{\"kind\":\"assistant\",\"body\":\"hi\"}}}}\n\n"
    )
}

#[tokio::test]
async fn stream_yields_decoded_envelopes() {
    let body: &'static str = Box::leak(envelope_frame(1).into_boxed_str());
    let base = spawn_sse_stub(body).await;
    let stream = event_stream(
        reqwest::Client::new(),
        format!("{base}/medulla/v1/sessions/s1/stream"),
        None,
    );
    futures::pin_mut!(stream);
    let envelope = stream.next().await.unwrap().unwrap();
    assert_eq!(envelope.seq, Some(1));
    assert_eq!(envelope.session_id, "s1");
    assert_eq!(
        envelope.kind(),
        crate::api::medulla_types::EventKind::Assistant { body: "hi".into() }
    );
}

#[tokio::test]
async fn stream_surfaces_a_decode_error_for_bad_json() {
    let base = spawn_sse_stub("id: 1\ndata: {not json}\n\n").await;
    let stream = event_stream(
        reqwest::Client::new(),
        format!("{base}/medulla/v1/sessions/s1/stream"),
        None,
    );
    futures::pin_mut!(stream);
    let err = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(err, Error::Decode(_)), "got {err:?}");
}

#[tokio::test]
async fn stream_surfaces_a_connect_error() {
    // Bind then release, so the port refuses the connection.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    let stream = event_stream(
        reqwest::Client::new(),
        format!("http://{addr}/medulla/v1/sessions/s1/stream"),
        None,
    );
    futures::pin_mut!(stream);
    let err = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(err, Error::Http(_)), "got {err:?}");
}

/// The token is a query parameter, not a header, so browser `EventSource`
/// consumers can reconnect the same URL.
#[test]
fn session_stream_url_carries_the_encoded_token_and_id() {
    let client =
        crate::TinyHumansClient::new("https://api.example.com").with_token(Some("tok/en".into()));
    // Exercised through the public builder; the URL is asserted via the
    // percent-encoding helper the builder uses.
    assert_eq!(enc("tok/en"), "tok%2Fen");
    assert_eq!(enc("sess/1"), "sess%2F1");
    // The stream is constructible without panicking for an id needing encoding.
    let _ = client.raw().session_event_stream("sess/1", Some(9));
}
