use serde_json::json;
use tinyhumans_sdk::api::types::ClaimReferralRequest;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn claim_referral_posts_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/referral/claim"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"claimed": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .referral()
        .claim_referral(&ClaimReferralRequest {
            code: "abc".into(),
            device_fingerprint: None,
        })
        .await
        .unwrap();

    assert_eq!(result, json!({"claimed": true}));
}

#[tokio::test]
async fn get_referral_stats_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/referral/stats"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"earnings": 10}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.referral().get_referral_stats().await.unwrap();

    assert_eq!(result, json!({"earnings": 10}));
}
