use serde_json::json;
use tinyhumans_sdk::{Error, TinyHumansClient};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// User credential headers plus the static headers must reach the server.
#[tokio::test]
async fn sends_all_auth_and_static_headers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/me"))
        .and(header("authorization", "Bearer t"))
        .and(header("x-api-key", "k"))
        .and(header("accept", "application/json"))
        .and(header("x-sdk-client", "tinyhumans-rust"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "u_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri())
        .with_token(Some("t".into()))
        .with_api_key(Some("k".into()));

    let result = client.auth().me().await.unwrap();
    assert_eq!(result, json!({"id": "u_1"}));
}

#[tokio::test]
async fn raw_get_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/some/new/route"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.raw().get("/some/new/route").await.unwrap();
    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn raw_post_sends_body_and_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/some/new/route"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"created": 1}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .raw()
        .post("/some/new/route", &json!({"name": "x"}))
        .await
        .unwrap();
    assert_eq!(result, json!({"created": 1}));
}

#[tokio::test]
async fn error_status_is_returned_as_err() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/me"))
        .respond_with(
            ResponseTemplate::new(500).set_body_json(json!({"success": false, "error": "boom"})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let err = client.auth().me().await.unwrap_err();
    match err {
        Error::Status { status, body } => {
            assert_eq!(status, 500);
            assert_eq!(body, json!({"success": false, "error": "boom"}));
        }
        other => panic!("expected Error::Status, got {other:?}"),
    }
}

// swagger() uses unwrap=false, so a body with success/data keys is returned raw.
#[tokio::test]
async fn swagger_returns_body_without_unwrapping() {
    let server = MockServer::start().await;
    let body = json!({
        "openapi": "3.0.0",
        "success": true,
        "data": {"ignored": true}
    });
    Mock::given(method("GET"))
        .and(path("/swagger.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.swagger().await.unwrap();
    assert_eq!(result, body);
}

// A path param containing a space must be percent-encoded via `enc`.
#[tokio::test]
async fn path_param_is_percent_encoded() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/mascots/red%20fox"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "red fox"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.mascots().get_mascot("red fox").await.unwrap();
    assert_eq!(result, json!({"id": "red fox"}));
}
