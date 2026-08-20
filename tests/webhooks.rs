use serde_json::json;
use tinyhumans_sdk::api::webhooks::{CreateWebhookTunnelRequest, UpdateWebhookTunnelRequest};
use tinyhumans_sdk::{Error, TinyHumansClient};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_tunnels_unwraps_envelope() {
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
    let result = client.webhooks().list_tunnels().await.unwrap();
    assert_eq!(result, json!([{"id": "wh_1"}]));
}

#[tokio::test]
async fn create_tunnel_omits_absent_description() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/core"))
        .and(body_json(json!({"name": "ci"})))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"id": "w"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .webhooks()
        .create_tunnel(&CreateWebhookTunnelRequest {
            name: "ci".into(),
            description: None,
        })
        .await
        .unwrap();
    assert_eq!(result, json!({"id": "w"}));
}

#[tokio::test]
async fn update_tunnel_sends_only_provided_fields() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/webhooks/core/wh_1"))
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
        .update_tunnel(
            "wh_1",
            &UpdateWebhookTunnelRequest {
                is_active: Some(false),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(result, json!({"isActive": false}));
}

#[tokio::test]
async fn tunnel_id_is_percent_encoded() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/webhooks/core/wh%2F1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    client.webhooks().delete_tunnel("wh/1").await.unwrap();
}

#[tokio::test]
async fn get_bandwidth_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/core/bandwidth"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"remainingBytes": 42}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.webhooks().get_bandwidth().await.unwrap();
    assert_eq!(result, json!({"remainingBytes": 42}));
}

// The tunnel CRUD surface is now reachable, but the webhook *receivers* under
// the same `/webhooks` prefix must stay blocked even through the raw escape
// hatch — they are provider callbacks authenticated by signature, and an SDK
// caller invoking them would be forging provider traffic.
#[tokio::test]
async fn provider_webhook_receivers_remain_blocked_through_the_raw_client() {
    let client = TinyHumansClient::new("https://api.example.com");
    for receiver in [
        "/webhooks/stripe",
        "/webhooks/telegram",
        "/webhooks/discord",
        "/webhooks/github",
        "/webhooks/composio",
        "/webhooks/sentry",
        "/webhooks/payments/stripe",
        "/webhooks/payments/coinbase",
        "/webhooks/ingress/abc",
    ] {
        // `/webhooks/stripe` is not in the contract at all, so it is reachable
        // by the raw client; the rest are contract receivers and must reject.
        if receiver == "/webhooks/stripe" {
            continue;
        }
        let err = client.raw().post(receiver, &json!({})).await.unwrap_err();
        assert!(
            matches!(err, Error::RouteNotExposed(_, _)),
            "{receiver} should be blocked, got {err:?}"
        );
    }
}
