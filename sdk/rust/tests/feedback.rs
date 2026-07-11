use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_feedback_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/feedback"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"id": "fb_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .feedback()
        .create_feedback(&json!({"type": "bug", "title": "t", "body": "b"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "fb_1"}));
}

#[tokio::test]
async fn list_feedback_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/feedback"))
        .and(query_param("sort", "top"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": []})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .feedback()
        .list_feedback(&[("sort", Some("top".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!([]));
}

#[tokio::test]
async fn get_feedback_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/feedback/fb_9"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"id": "fb_9"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.feedback().get_feedback("fb_9").await.unwrap();

    assert_eq!(result, json!({"id": "fb_9"}));
}

#[tokio::test]
async fn update_feedback_status_patches() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/feedback/fb_9/status"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"status": "planned"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .feedback()
        .update_feedback_status("fb_9", &json!({"status": "planned"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "planned"}));
}

#[tokio::test]
async fn vote_feedback_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/feedback/fb_9/vote"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"value": 1}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .feedback()
        .vote_feedback("fb_9", &json!({"value": 1}))
        .await
        .unwrap();

    assert_eq!(result, json!({"value": 1}));
}
