use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_coinbase_charge_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/coinbase/charge"))
        .and(body_json(json!({"amount": 10})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "charge_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .create_coinbase_charge(&json!({"amount": 10}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "charge_1"}));
}

#[tokio::test]
async fn get_coinbase_charge_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/coinbase/charge/gtx_9"))
        .and(query_param("sync", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "paid"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .get_coinbase_charge("gtx_9", &[("sync", Some("true".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "paid"}));
}

#[tokio::test]
async fn get_auto_recharge_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/auto-recharge"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"enabled": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().get_auto_recharge().await.unwrap();

    assert_eq!(result, json!({"enabled": true}));
}

#[tokio::test]
async fn update_auto_recharge_patches_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/payments/credits/auto-recharge"))
        .and(body_json(json!({"enabled": false})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"enabled": false}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .update_auto_recharge(&json!({"enabled": false}))
        .await
        .unwrap();

    assert_eq!(result, json!({"enabled": false}));
}

#[tokio::test]
async fn list_auto_recharge_cards_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/auto-recharge/cards"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"cards": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().list_auto_recharge_cards().await.unwrap();

    assert_eq!(result, json!({"cards": []}));
}

#[tokio::test]
async fn create_auto_recharge_card_setup_intent_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/credits/auto-recharge/cards/setup-intent"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"client_secret": "cs_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .create_auto_recharge_card_setup_intent()
        .await
        .unwrap();

    assert_eq!(result, json!({"client_secret": "cs_1"}));
}

#[tokio::test]
async fn update_auto_recharge_card_patches_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/payments/credits/auto-recharge/cards/pm_9"))
        .and(body_json(json!({"default": true})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "pm_9"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .update_auto_recharge_card("pm_9", &json!({"default": true}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "pm_9"}));
}

#[tokio::test]
async fn delete_auto_recharge_card_deletes() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/payments/credits/auto-recharge/cards/pm_9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .delete_auto_recharge_card("pm_9")
        .await
        .unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn get_credit_balance_unwraps() {
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
async fn create_credit_top_up_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/credits/top-up"))
        .and(body_json(json!({"amount": 20})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"url": "https://pay"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .create_credit_top_up(&json!({"amount": 20}))
        .await
        .unwrap();

    assert_eq!(result, json!({"url": "https://pay"}));
}

#[tokio::test]
async fn get_credit_top_up_cancel_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/top-up/cancel"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"canceled": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().get_credit_top_up_cancel().await.unwrap();

    assert_eq!(result, json!({"canceled": true}));
}

#[tokio::test]
async fn get_credit_top_up_success_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/top-up/success"))
        .and(query_param("session_id", "sess_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .get_credit_top_up_success(&[("session_id", Some("sess_1".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn list_credit_transactions_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/credits/transactions"))
        .and(query_param("limit", "10"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": []})),
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

#[tokio::test]
async fn get_stripe_checkout_return_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/stripe/checkout/return"))
        .and(query_param("session_id", "cs_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "complete"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .get_stripe_checkout_return(&[("session_id", Some("cs_1".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "complete"}));
}

#[tokio::test]
async fn get_current_plan_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/stripe/currentPlan"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"plan": "pro"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().get_current_plan().await.unwrap();

    assert_eq!(result, json!({"plan": "pro"}));
}

#[tokio::test]
async fn get_stripe_plans_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/stripe/plans"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"plans": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().get_stripe_plans().await.unwrap();

    assert_eq!(result, json!({"plans": []}));
}

#[tokio::test]
async fn create_stripe_portal_session_posts() {
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
async fn get_stripe_portal_return_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payments/stripe/portal/return"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.payments().get_stripe_portal_return().await.unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn purchase_stripe_plan_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payments/stripe/purchasePlan"))
        .and(body_json(json!({"plan": "pro"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"url": "https://checkout"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .payments()
        .purchase_stripe_plan(&json!({"plan": "pro"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"url": "https://checkout"}));
}
