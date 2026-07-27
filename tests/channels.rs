use serde_json::json;
use tinyhumans_sdk::api::channels::{
    AddReactionRequest, ChannelIdentifier, CreateThreadRequest, SendMessageRequest, ThreadAction,
    UpdateThreadRequest,
};
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok(data: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": data}))
}

#[tokio::test]
async fn send_message_posts_to_channel() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/telegram/messages"))
        .and(body_json(json!({"text": "hi"})))
        .respond_with(ok(json!({"messageId": 7})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .send_message(
            "telegram",
            &SendMessageRequest {
                text: Some("hi".into()),
                parse_mode: None,
                photo_url: None,
                sticker_file_id: None,
                animation_url: None,
                channel_id: None,
                buttons: vec![],
                reply_to_message_id: None,
                thread_id: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(result, json!({"messageId": 7}));
}

#[tokio::test]
async fn delete_message_uses_path_params() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/channels/discord/messages/123"))
        .respond_with(ok(json!({"deleted": true})))
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
async fn add_reaction_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/discord/reactions"))
        .and(body_json(json!({"messageId": "m_1", "emoji": "star"})))
        .respond_with(ok(json!({"ok": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .add_reaction(
            "discord",
            &AddReactionRequest {
                message_id: ChannelIdentifier::String("m_1".into()),
                emoji: "star".into(),
                chat_id: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn create_thread_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/telegram/threads"))
        .and(body_json(json!({"title": "t"})))
        .respond_with(ok(json!({"id": "th_1"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .create_thread("telegram", &CreateThreadRequest { title: "t".into() })
        .await
        .unwrap();
    assert_eq!(result, json!({"id": "th_1"}));
}

#[tokio::test]
async fn list_threads_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/channels/telegram/threads"))
        .and(query_param("active", "true"))
        .respond_with(ok(json!([])))
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
        .and(body_json(json!({"action": "close"})))
        .respond_with(ok(json!({"status": "closed"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .channels()
        .update_thread(
            "telegram",
            "th_1",
            &UpdateThreadRequest {
                action: ThreadAction::Close,
            },
        )
        .await
        .unwrap();
    assert_eq!(result, json!({"status": "closed"}));
}

#[tokio::test]
async fn send_typing_posts_without_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/telegram/typing"))
        .respond_with(ok(json!({"ok": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.channels().send_typing("telegram").await.unwrap();
    assert_eq!(result, json!({"ok": true}));
}
