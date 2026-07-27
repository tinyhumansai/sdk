use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn exposes_only_user_coupon_routes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/coupons/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success":true,"data":[]})))
        .mount(&server)
        .await;
    assert_eq!(
        TinyHumansClient::new(server.uri())
            .coupons()
            .list_my_coupons()
            .await
            .unwrap(),
        json!([])
    );
}
