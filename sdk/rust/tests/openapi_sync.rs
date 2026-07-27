use serde_json::json;
use tinyhumans_sdk::api::api_keys::{ApiKeyScope, CreateApiKeyRequest};
use tinyhumans_sdk::api::medulla::{CreateTaskRequest, TaskStatus};
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"ok": true}}))
}

#[tokio::test]
async fn typed_api_key_request_uses_openapi_field_names() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api-keys"))
        .and(body_json(
            json!({"name":"CI","scopes":["inference"],"allowedIps":["10.0.0.0/8"]}),
        ))
        .respond_with(ok())
        .mount(&server)
        .await;
    let request = CreateApiKeyRequest {
        name: "CI".into(),
        scopes: vec![ApiKeyScope::Inference],
        allowed_ips: vec!["10.0.0.0/8".into()],
        expires_at: None,
    };
    TinyHumansClient::new(server.uri())
        .api_keys()
        .create(&request)
        .await
        .unwrap();
}

#[tokio::test]
async fn medulla_task_status_serializes_in_camel_case() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/medulla/v1/tasks"))
        .and(body_json(json!({"title":"Ship","status":"inProgress"})))
        .respond_with(ok())
        .mount(&server)
        .await;
    let request = CreateTaskRequest {
        title: "Ship".into(),
        description: None,
        status: Some(TaskStatus::InProgress),
        recurrence: None,
    };
    TinyHumansClient::new(server.uri())
        .medulla()
        .create_task(&request)
        .await
        .unwrap();
}

#[tokio::test]
async fn path_segments_are_encoded_on_new_namespaces() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/orchestration/v1/sessions/a%2Fb/messages"))
        .respond_with(ok())
        .mount(&server)
        .await;
    TinyHumansClient::new(server.uri())
        .orchestration()
        .session_messages("a/b", &[])
        .await
        .unwrap();
}
