use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok(data: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": data}))
}

#[tokio::test]
async fn create_campaign_invite_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/invite/campaign"))
        .and(body_json(json!({"maxUses": 10})))
        .respond_with(ok(json!({"code": "ABC"})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .invite()
        .create_campaign_invite(&json!({"maxUses": 10}))
        .await
        .unwrap();
    assert_eq!(result, json!({"code": "ABC"}));
}

#[tokio::test]
async fn list_campaign_invites_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/invite/campaign"))
        .respond_with(ok(json!([{"code": "ABC"}])))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.invite().list_campaign_invites().await.unwrap();
    assert_eq!(result, json!([{"code": "ABC"}]));
}

#[tokio::test]
async fn delete_campaign_invite_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/invite/campaign/code_42"))
        .respond_with(ok(json!({"deleted": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .invite()
        .delete_campaign_invite("code_42")
        .await
        .unwrap();
    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn list_my_codes_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/invite/my-codes"))
        .respond_with(ok(json!([{"code": "X"}])))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.invite().list_my_codes().await.unwrap();
    assert_eq!(result, json!([{"code": "X"}]));
}

#[tokio::test]
async fn redeem_invite_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/invite/redeem"))
        .and(body_json(json!({"code": "WELCOME"})))
        .respond_with(ok(json!({"redeemed": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .invite()
        .redeem_invite(&json!({"code": "WELCOME"}))
        .await
        .unwrap();
    assert_eq!(result, json!({"redeemed": true}));
}

#[tokio::test]
async fn get_invite_status_sends_code_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/invite/status"))
        .and(query_param("code", "WELCOME"))
        .respond_with(ok(json!({"valid": true})))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .invite()
        .get_invite_status(&[("code", Some("WELCOME".to_string()))])
        .await
        .unwrap();
    assert_eq!(result, json!({"valid": true}));
}
