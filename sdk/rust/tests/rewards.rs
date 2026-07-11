use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn unlink_discord_sends_delete() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/rewards/discord"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"unlinked": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.rewards().unlink_discord().await.unwrap();

    assert_eq!(result, json!({"unlinked": true}));
}

#[tokio::test]
async fn get_my_rewards_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rewards/me"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"points": 42}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.rewards().get_my_rewards().await.unwrap();

    assert_eq!(result, json!({"points": 42}));
}
