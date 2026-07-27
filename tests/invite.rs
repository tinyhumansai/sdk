use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn exposes_personal_invite_routes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/invite/my-codes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success":true,"data":[]})))
        .mount(&server)
        .await;
    assert_eq!(
        TinyHumansClient::new(server.uri())
            .invite()
            .list_my_codes()
            .await
            .unwrap(),
        json!([])
    );
}
