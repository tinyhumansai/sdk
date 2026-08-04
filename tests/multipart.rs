//! Regression tests for the `Content-Type` on `RawClient::post_multipart`.
//!
//! The shared `headers()` builder sets a fixed `application/json`. On a
//! multipart upload that must NOT leak onto the request: reqwest's
//! `.multipart(form)` owns `multipart/form-data; boundary=…`, and if the JSON
//! type wins the backend JSON-parses the multipart body and 500s with
//! `Unexpected token '-', "--<boundary>"... is not valid JSON`. These tests
//! inspect the actual outgoing headers so a regression fails loudly.

use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn post_multipart_sends_multipart_content_type_not_json() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/file-storage/files"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "f_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri()).with_token(Some("t".into()));
    let form = reqwest::multipart::Form::new()
        .part(
            "file",
            reqwest::multipart::Part::bytes(b"hello".to_vec())
                .file_name("t.txt")
                .mime_str("text/plain")
                .unwrap(),
        )
        .text("visibility", "public");

    let result = client
        .raw()
        .post_multipart("/agent-integrations/file-storage/files", form)
        .await
        .expect("multipart upload should succeed");
    assert_eq!(result, json!({"id": "f_1"}));

    // Inspect what actually went on the wire.
    let requests = server
        .received_requests()
        .await
        .expect("wiremock records requests");
    let req = requests
        .iter()
        .find(|r| r.url.path() == "/agent-integrations/file-storage/files")
        .expect("the upload request was recorded");
    let content_type = req
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        content_type.starts_with("multipart/form-data"),
        "multipart upload must send a multipart/form-data content-type, got: {content_type:?}"
    );
    assert!(
        !content_type.contains("application/json"),
        "multipart upload must NOT carry application/json, got: {content_type:?}"
    );
}

#[tokio::test]
async fn raw_json_post_still_sends_application_json() {
    // No-regression guard: the JSON path is untouched and still labels its body
    // `application/json` (set by reqwest's `.json()` and our shared header).
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/some/json/route"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    client
        .raw()
        .post("/some/json/route", &json!({"name": "x"}))
        .await
        .expect("json post should succeed");

    let requests = server
        .received_requests()
        .await
        .expect("wiremock records requests");
    let req = requests
        .iter()
        .find(|r| r.url.path() == "/some/json/route")
        .expect("the json request was recorded");
    let content_type = req
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        content_type.starts_with("application/json"),
        "json post must send application/json, got: {content_type:?}"
    );
}
