use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn text_part_form() -> reqwest::multipart::Form {
    let part = reqwest::multipart::Part::bytes(b"hello upload".to_vec())
        .file_name("note.txt")
        .mime_str("text/plain")
        .expect("static mime parses");
    reqwest::multipart::Form::new().part("file", part)
}

async fn mock_upload_server() -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/file-storage/files"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"fileId": "f_1"}})),
        )
        .mount(&server)
        .await;
    server
}

/// A multipart upload must carry exactly ONE `Content-Type`, the multipart one.
///
/// Regression test for the duplicate-header bug: `headers()` sets
/// `Content-Type: application/json` for JSON routes, and `RequestBuilder::header`
/// — which `.multipart()` uses — **appends** rather than replaces. Inheriting the
/// JSON header therefore put two `Content-Type` lines on the wire. Node/Express
/// keeps the first per RFC 7230 §3.2.2, so the backend fed the multipart body to
/// `express.json()` and 500'd on the `--<boundary>` delimiter
/// (tinyhumansai/backend#1179).
#[tokio::test]
async fn multipart_upload_sends_only_the_multipart_content_type() {
    let server = mock_upload_server().await;
    let client = TinyHumansClient::new(server.uri()).with_token(Some("t".into()));

    client
        .raw()
        .post_multipart("/agent-integrations/file-storage/files", text_part_form())
        .await
        .expect("upload succeeds");

    let requests = server.received_requests().await.expect("requests recorded");
    let received = requests.first().expect("one request recorded");
    let content_types: Vec<&str> = received
        .headers
        .get_all(CONTENT_TYPE)
        .iter()
        .map(|v| v.to_str().expect("header is valid utf-8"))
        .collect();

    assert_eq!(
        content_types.len(),
        1,
        "expected exactly one Content-Type, got {content_types:?}"
    );
    assert!(
        content_types[0].starts_with("multipart/form-data; boundary="),
        "expected the multipart type with a boundary, got {:?}",
        content_types[0]
    );
}

/// The credential and static headers must survive dropping the JSON
/// `Content-Type` — the fix must not strip anything else.
#[tokio::test]
async fn multipart_upload_still_sends_auth_and_static_headers() {
    let server = mock_upload_server().await;
    let client = TinyHumansClient::new(server.uri())
        .with_token(Some("t".into()))
        .with_api_key(Some("k".into()));

    client
        .raw()
        .post_multipart("/agent-integrations/file-storage/files", text_part_form())
        .await
        .expect("upload succeeds");

    let requests = server.received_requests().await.expect("requests recorded");
    let headers = &requests.first().expect("one request recorded").headers;

    assert_eq!(headers.get("authorization").unwrap(), "Bearer t");
    assert_eq!(headers.get("x-api-key").unwrap(), "k");
    assert_eq!(headers.get("accept").unwrap(), "application/json");
    assert_eq!(headers.get("x-sdk-client").unwrap(), "tinyhumans-rust");
}

/// JSON routes are unaffected: they still send a single
/// `Content-Type: application/json`.
#[tokio::test]
async fn json_post_still_sends_a_single_json_content_type() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/some/new/route"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    client
        .raw()
        .post("/some/new/route", &json!({"name": "x"}))
        .await
        .expect("post succeeds");

    let requests = server.received_requests().await.expect("requests recorded");
    let content_types: Vec<&str> = requests
        .first()
        .expect("one request recorded")
        .headers
        .get_all(CONTENT_TYPE)
        .iter()
        .map(|v| v.to_str().expect("header is valid utf-8"))
        .collect();

    assert_eq!(content_types, vec!["application/json"]);
}
