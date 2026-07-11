use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_investor_page_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/investors/acme"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"slug": "acme"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.investors().get_investor_page("acme").await.unwrap();

    assert_eq!(result, json!({"slug": "acme"}));
}

#[tokio::test]
async fn track_investor_event_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/investors/acme/events"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"tracked": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .investors()
        .track_investor_event("acme", &json!({"eventType": "PAGE_VIEW"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"tracked": true}));
}
