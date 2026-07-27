use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok(data: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": data}))
}

#[tokio::test]
async fn create_channel_link_token_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/channels/telegram/link-token"))
        .respond_with(ok(json!({"token": "lt_1"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .auth()
        .create_channel_link_token("telegram")
        .await
        .unwrap();
    assert_eq!(result, json!({"token": "lt_1"}));
}

#[tokio::test]
async fn send_email_link_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/email/send-link"))
        .and(body_json(json!({"email": "a@b.com"})))
        .respond_with(ok(json!({"sent": true})))
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
        .respond_with(ok(json!({"ok": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().verify_email("tok_123").await.unwrap();
    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn consume_login_token_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/login-token/consume"))
        .and(body_json(json!({"token": "one_time"})))
        .respond_with(ok(json!({"session": "s_1"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .auth()
        .consume_login_token(&json!({"token": "one_time"}))
        .await
        .unwrap();
    assert_eq!(result, json!({"session": "s_1"}));
}

#[tokio::test]
async fn me_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/me"))
        .respond_with(ok(json!({"id": "user_1"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().me().await.unwrap();
    assert_eq!(result, json!({"id": "user_1"}));
}

#[tokio::test]
async fn list_integrations_gets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/integrations"))
        .respond_with(ok(json!([{"id": "int_1"}])))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().list_integrations().await.unwrap();
    assert_eq!(result, json!([{"id": "int_1"}]));
}

#[tokio::test]
async fn delete_integration_deletes() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/auth/integrations/int_1"))
        .respond_with(ok(json!({"deleted": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().delete_integration("int_1").await.unwrap();
    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn create_integration_token_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/integrations/int_1/tokens"))
        .and(body_json(json!({"scope": "read"})))
        .respond_with(ok(json!({"token": "at_1"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .auth()
        .create_integration_token("int_1", &json!({"scope": "read"}))
        .await
        .unwrap();
    assert_eq!(result, json!({"token": "at_1"}));
}

#[tokio::test]
async fn oauth_callback_gets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/google/callback"))
        .respond_with(ok(json!({"redirect": "/"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.auth().oauth_callback("google").await.unwrap();
    assert_eq!(result, json!({"redirect": "/"}));
}

#[tokio::test]
async fn oauth_connect_gets_with_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/google/connect"))
        .and(query_param("redirect", "/home"))
        .respond_with(ok(json!({"url": "https://oauth"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .auth()
        .oauth_connect("google", &[("redirect", Some("/home".to_string()))])
        .await
        .unwrap();
    assert_eq!(result, json!({"url": "https://oauth"}));
}

#[tokio::test]
async fn oauth_login_gets_with_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/auth/google/login"))
        .and(query_param("redirect", "/home"))
        .respond_with(ok(json!({"url": "https://login"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .auth()
        .oauth_login("google", &[("redirect", Some("/home".to_string()))])
        .await
        .unwrap();
    assert_eq!(result, json!({"url": "https://login"}));
}
