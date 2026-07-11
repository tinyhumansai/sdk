use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn send_email_link_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/email/send-link"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"sent": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .auth()
        .send_email_link(&json!({"email": "a@b.com"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"sent": true}));
}

#[tokio::test]
async fn verify_email_sends_token_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/email/verify"))
        .and(query_param("token", "tok_123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().verify_email("tok_123").await.unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn me_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/me"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "user_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().me().await.unwrap();

    assert_eq!(result, json!({"id": "user_1"}));
}
