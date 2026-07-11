use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_coupon_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/coupons/admin"))
        .and(body_json(json!({"amountUsd": 25})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"code": "ABCD-EFGH"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .coupons()
        .create_coupon(&json!({"amountUsd": 25}))
        .await
        .unwrap();

    assert_eq!(result, json!({"code": "ABCD-EFGH"}));
}

#[tokio::test]
async fn list_coupons_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/coupons/admin"))
        .and(query_param("redeemed", "false"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": []})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .coupons()
        .list_coupons(&[("redeemed", Some("false".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!([]));
}

#[tokio::test]
async fn delete_coupon_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/coupons/admin/coupon_7"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.coupons().delete_coupon("coupon_7").await.unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn redeem_coupon_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/coupons/redeem"))
        .and(body_json(json!({"code": "ABCD-EFGH"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"credited": 25}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .coupons()
        .redeem_coupon(&json!({"code": "ABCD-EFGH"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"credited": 25}));
}

#[tokio::test]
async fn list_my_coupons_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/coupons/me"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": [{"code": "X"}]})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.coupons().list_my_coupons().await.unwrap();

    assert_eq!(result, json!([{"code": "X"}]));
}
