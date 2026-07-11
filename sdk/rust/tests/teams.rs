use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_teams_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": [{"id": "team_1"}]})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().list_teams().await.unwrap();

    assert_eq!(result, json!([{"id": "team_1"}]));
}

#[tokio::test]
async fn join_team_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/join"))
        .and(body_json(json!({"code": "abc"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"joined": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .join_team(&json!({"code": "abc"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"joined": true}));
}

#[tokio::test]
async fn get_team_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams/team_42"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "team_42"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().get_team("team_42").await.unwrap();

    assert_eq!(result, json!({"id": "team_42"}));
}

#[tokio::test]
async fn update_team_uses_put() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/teams/team_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"name": "New"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .update_team("team_1", &json!({"name": "New"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"name": "New"}));
}

#[tokio::test]
async fn revoke_invite_uses_delete_with_two_path_params() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/teams/team_1/invites/inv_9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"revoked": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .revoke_invite("team_1", "inv_9")
        .await
        .unwrap();

    assert_eq!(result, json!({"revoked": true}));
}

#[tokio::test]
async fn leave_team_posts_no_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_1/leave"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"left": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().leave_team("team_1").await.unwrap();

    assert_eq!(result, json!({"left": true}));
}

#[tokio::test]
async fn update_member_role_uses_put() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/teams/team_1/members/user_2/role"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"role": "admin"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .update_member_role("team_1", "user_2", &json!({"role": "admin"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"role": "admin"}));
}
