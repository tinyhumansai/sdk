use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_analytics_dashboard_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/admin/analytics/dashboard"))
        .and(query_param("startDate", "2026-01-01"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"dau": 42}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .get_analytics_dashboard(&[("startDate", Some("2026-01-01".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"dau": 42}));
}

#[tokio::test]
async fn get_analytics_financials_details_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/admin/analytics/financials/details"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"topups": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .get_analytics_financials_details()
        .await
        .unwrap();

    assert_eq!(result, json!({"topups": []}));
}

#[tokio::test]
async fn create_announcement_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/admin/announcements"))
        .and(body_json(json!({"title": "Hi", "body": "There"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "ann_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .create_announcement(&json!({"title": "Hi", "body": "There"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "ann_1"}));
}

#[tokio::test]
async fn get_announcement_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/admin/announcements/ann_9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "ann_9"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.admin().get_announcement("ann_9").await.unwrap();

    assert_eq!(result, json!({"id": "ann_9"}));
}

#[tokio::test]
async fn update_announcement_patches_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/admin/announcements/ann_2"))
        .and(body_json(json!({"isActive": false})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"updated": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .update_announcement("ann_2", &json!({"isActive": false}))
        .await
        .unwrap();

    assert_eq!(result, json!({"updated": true}));
}

#[tokio::test]
async fn delete_coupon_uses_delete_verb() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/admin/coupons/cpn_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deactivated": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.admin().delete_coupon("cpn_1").await.unwrap();

    assert_eq!(result, json!({"deactivated": true}));
}

#[tokio::test]
async fn update_investor_uses_put_verb() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/admin/investors/inv_1"))
        .and(body_json(json!({"name": "Acme"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "inv_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .update_investor("inv_1", &json!({"name": "Acme"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "inv_1"}));
}

#[tokio::test]
async fn list_investor_events_path_and_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/admin/investors/inv_5/events"))
        .and(query_param("page", "2"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"events": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .list_investor_events("inv_5", &[("page", Some("2".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"events": []}));
}

#[tokio::test]
async fn grant_user_credits_posts_to_nested_path() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/admin/users/user_1/credits"))
        .and(body_json(json!({"action": "ADD", "amountUsd": 5.0, "reason": "gift"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"balance": 5.0}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .grant_user_credits(
            "user_1",
            &json!({"action": "ADD", "amountUsd": 5.0, "reason": "gift"}),
        )
        .await
        .unwrap();

    assert_eq!(result, json!({"balance": 5.0}));
}

#[tokio::test]
async fn cancel_user_subscription_deletes_with_body() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/admin/users/user_7/subscription"))
        .and(body_json(json!({"reason": "requested"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"plan": "free"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .admin()
        .cancel_user_subscription("user_7", &json!({"reason": "requested"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"plan": "free"}));
}
