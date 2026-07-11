use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_teams_unwraps() {
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
async fn get_my_usage_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams/me/usage"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"spend": 10}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().get_my_usage().await.unwrap();

    assert_eq!(result, json!({"spend": 10}));
}

#[tokio::test]
async fn get_team_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams/team_9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "team_9"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().get_team("team_9").await.unwrap();

    assert_eq!(result, json!({"id": "team_9"}));
}

#[tokio::test]
async fn update_team_puts_body() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/teams/team_9"))
        .and(body_json(json!({"name": "New"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"name": "New"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .update_team("team_9", &json!({"name": "New"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"name": "New"}));
}

#[tokio::test]
async fn get_billing_plan_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams/team_9/billing/plan"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"plan": "pro"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().get_billing_plan("team_9").await.unwrap();

    assert_eq!(result, json!({"plan": "pro"}));
}

#[tokio::test]
async fn create_billing_portal_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_9/billing/portal"))
        .and(body_json(json!({"return_url": "https://r"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"url": "https://portal"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .create_billing_portal("team_9", &json!({"return_url": "https://r"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"url": "https://portal"}));
}

#[tokio::test]
async fn purchase_plan_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_9/billing/purchase"))
        .and(body_json(json!({"plan": "pro"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"url": "https://checkout"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .purchase_plan("team_9", &json!({"plan": "pro"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"url": "https://checkout"}));
}

#[tokio::test]
async fn create_invite_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_9/invites"))
        .and(body_json(json!({"role": "member"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"id": "inv_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .create_invite("team_9", &json!({"role": "member"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"id": "inv_1"}));
}

#[tokio::test]
async fn list_invites_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams/team_9/invites"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"invites": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().list_invites("team_9").await.unwrap();

    assert_eq!(result, json!({"invites": []}));
}

#[tokio::test]
async fn send_email_invite_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_9/invites/email"))
        .and(body_json(json!({"email": "a@b.com"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"sent": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .send_email_invite("team_9", &json!({"email": "a@b.com"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"sent": true}));
}

#[tokio::test]
async fn revoke_invite_uses_delete_with_two_path_params() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/teams/team_9/invites/inv_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"revoked": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .revoke_invite("team_9", "inv_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"revoked": true}));
}

#[tokio::test]
async fn leave_team_posts_no_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_9/leave"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"left": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().leave_team("team_9").await.unwrap();

    assert_eq!(result, json!({"left": true}));
}

#[tokio::test]
async fn list_members_unwraps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/teams/team_9/members"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"members": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().list_members("team_9").await.unwrap();

    assert_eq!(result, json!({"members": []}));
}

#[tokio::test]
async fn remove_member_deletes_full_path() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/teams/team_9/members/user_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"removed": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .remove_member("team_9", "user_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"removed": true}));
}

#[tokio::test]
async fn update_member_role_puts_body() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/teams/team_9/members/user_1/role"))
        .and(body_json(json!({"role": "admin"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"role": "admin"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .teams()
        .update_member_role("team_9", "user_1", &json!({"role": "admin"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"role": "admin"}));
}

#[tokio::test]
async fn switch_team_posts_no_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/teams/team_9/switch"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"active": "team_9"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.teams().switch_team("team_9").await.unwrap();

    assert_eq!(result, json!({"active": "team_9"}));
}
