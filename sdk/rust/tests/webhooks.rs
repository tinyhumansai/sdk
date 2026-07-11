use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn receive_composio_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/composio"))
        .and(body_json(json!({"event": "x"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_composio_webhook(&json!({"event": "x"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn create_core_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/core"))
        .and(body_json(json!({"name": "my-hook"})))
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
async fn list_core_webhooks_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/core"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": [{"id": "wh_1"}]})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.webhooks().list_core_webhooks().await.unwrap();

    assert_eq!(result, json!([{"id": "wh_1"}]));
}

#[tokio::test]
async fn get_core_webhook_bandwidth_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/core/bandwidth"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"remaining": 5}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .get_core_webhook_bandwidth()
        .await
        .unwrap();

    assert_eq!(result, json!({"remaining": 5}));
}

#[tokio::test]
async fn get_core_webhook_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/core/wh_42"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "wh_42"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.webhooks().get_core_webhook("wh_42").await.unwrap();

    assert_eq!(result, json!({"id": "wh_42"}));
}

#[tokio::test]
async fn update_core_webhook_patches_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/webhooks/core/wh_42"))
        .and(body_json(json!({"isActive": false})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"isActive": false}})),
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
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .delete_core_webhook("wh_42")
        .await
        .unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn receive_discord_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/discord"))
        .and(body_json(json!({"type": 1})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_discord_webhook(&json!({"type": 1}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn receive_github_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/github"))
        .and(body_json(json!({"action": "opened"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_github_webhook(&json!({"action": "opened"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn forward_webhook_ingress_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/ingress/uuid_9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"forwarded": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .forward_webhook_ingress("uuid_9")
        .await
        .unwrap();

    assert_eq!(result, json!({"forwarded": true}));
}

#[tokio::test]
async fn forward_ingress_with_path_uses_both_params() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/ingress/uuid_1/deploy"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"forwarded": true}})),
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

#[tokio::test]
async fn receive_coinbase_payment_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/payments/coinbase"))
        .and(body_json(json!({"event": "charge"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_coinbase_payment_webhook(&json!({"event": "charge"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn receive_stripe_payment_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/payments/stripe"))
        .and(body_json(json!({"type": "invoice"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_stripe_payment_webhook(&json!({"type": "invoice"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn receive_sentry_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/sentry"))
        .and(body_json(json!({"action": "triggered"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_sentry_webhook(&json!({"action": "triggered"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn receive_telegram_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/telegram"))
        .and(body_json(json!({"update_id": 1})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_telegram_webhook(&json!({"update_id": 1}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn receive_managed_telegram_webhook_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/telegram/managed/bot_9"))
        .and(body_json(json!({"update_id": 2})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .receive_managed_telegram_webhook("bot_9", &json!({"update_id": 2}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}
