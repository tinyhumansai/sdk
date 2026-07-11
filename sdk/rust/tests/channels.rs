use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn send_message_posts_to_channel() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/telegram/messages"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"messageId": 7}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .send_message("telegram", &json!({"text": "hi"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"messageId": 7}));
}

#[tokio::test]
async fn delete_message_uses_path_params() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/channels/discord/messages/123"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .delete_message("discord", "123")
        .await
        .unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn list_threads_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/channels/telegram/threads"))
        .and(query_param("active", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": []})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .list_threads("telegram", &[("active", Some("true".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!([]));
}

#[tokio::test]
async fn update_thread_patches() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/channels/telegram/threads/th_1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"status": "closed"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .update_thread("telegram", "th_1", &json!({"action": "close"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "closed"}));
}

#[tokio::test]
async fn send_typing_posts_without_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/telegram/typing"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.channels().send_typing("telegram").await.unwrap();

    assert_eq!(result, json!({"ok": true}));
}
