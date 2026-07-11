use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_core_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/core"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "wh_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .create_core_webhook(&json!({"name": "my-hook"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "wh_1"}));
}

#[tokio::test]
async fn list_core_webhooks_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/core"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": [{"id": "wh_1"}]})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.webhooks().list_core_webhooks().await.unwrap();

    assert_eq!(result, json!([{"id": "wh_1"}]));
}

#[tokio::test]
async fn get_core_webhook_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/core/wh_42"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"id": "wh_42"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.webhooks().get_core_webhook("wh_42").await.unwrap();

    assert_eq!(result, json!({"id": "wh_42"}));
}

#[tokio::test]
async fn update_core_webhook_patches() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/webhooks/core/wh_42"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"isActive": false}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .update_core_webhook("wh_42", &json!({"isActive": false}))
        .await
        .unwrap();

    assert_eq!(result, json!({"isActive": false}));
}

#[tokio::test]
async fn delete_core_webhook_deletes() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/webhooks/core/wh_42"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.webhooks().delete_core_webhook("wh_42").await.unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn forward_ingress_with_path_uses_both_params() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/ingress/uuid_1/deploy"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"forwarded": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .forward_webhook_ingress_with_path("uuid_1", "deploy")
        .await
        .unwrap();

    assert_eq!(result, json!({"forwarded": true}));
}
