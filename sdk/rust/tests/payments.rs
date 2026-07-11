use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_coinbase_charge_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/coinbase/charge"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "chg_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .create_coinbase_charge(&json!({"plan": "PRO"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "chg_1"}));
}

#[tokio::test]
async fn get_coinbase_charge_path_and_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/coinbase/charge/tx_9"))
        .and(query_param("sync", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "PAID"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .get_coinbase_charge("tx_9", &[("sync", Some("true".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "PAID"}));
}

#[tokio::test]
async fn get_credit_balance_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/balance"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"balanceUsd": 12.5}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().get_credit_balance().await.unwrap();

    assert_eq!(result, json!({"balanceUsd": 12.5}));
}

#[tokio::test]
async fn update_auto_recharge_patches_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/payments/credits/auto-recharge"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"enabled": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .update_auto_recharge(&json!({"enabled": true}))
        .await
        .unwrap();

    assert_eq!(result, json!({"enabled": true}));
}

#[tokio::test]
async fn delete_auto_recharge_card_uses_delete() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/payments/credits/auto-recharge/cards/pm_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .delete_auto_recharge_card("pm_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn create_stripe_portal_session_posts_no_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/stripe/portal"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"url": "https://portal"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .create_stripe_portal_session()
        .await
        .unwrap();

    assert_eq!(result, json!({"url": "https://portal"}));
}

#[tokio::test]
async fn list_credit_transactions_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/transactions"))
        .and(query_param("limit", "10"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": []})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .list_credit_transactions(&[("limit", Some("10".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!([]));
}
