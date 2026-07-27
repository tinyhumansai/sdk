use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn steering_deserializes_active_directive_and_history() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/orchestration/v1/steering"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "active": {
                    "directive": "Prioritize the pending reply.",
                    "consumedCycles": 2,
                    "maxCycles": 20
                },
                "history": [{
                    "directive": "Review unread sessions.",
                    "createdAt": "2026-07-27T12:00:00Z"
                }]
            }
        })))
        .mount(&server)
        .await;

    let response = TinyHumansClient::new(server.uri())
        .orchestration()
        .steering()
        .await
        .unwrap();

    assert_eq!(response.active.unwrap().consumed_cycles, 2);
    assert_eq!(response.history[0].directive, "Review unread sessions.");
}
