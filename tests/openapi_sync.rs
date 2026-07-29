use std::collections::BTreeSet;

use serde_json::json;
use tinyhumans_sdk::api::api_keys::{ApiKeyScope, CreateApiKeyRequest};
use tinyhumans_sdk::api::medulla::{CreateTaskRequest, TaskStatus};
use tinyhumans_sdk::generated_public_routes::PUBLIC_ROUTES;
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

#[tokio::test]
async fn medulla_workflows_reads_the_advert_parity_route() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/medulla/v1/workflows"))
        .respond_with(ok())
        .mount(&server)
        .await;
    TinyHumansClient::new(server.uri())
        .medulla()
        .workflows()
        .await
        .unwrap();
}

#[test]
fn generated_rust_routes_match_the_public_manifest() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../api/tinyhumans.backend.json")).unwrap();
    let manifest_routes = manifest["namespaces"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|namespace| namespace["routes"].as_array().unwrap())
        .map(|route| {
            let route = route.as_str().unwrap();
            route.split_once(' ').unwrap()
        })
        .collect::<BTreeSet<_>>();
    let rust_routes = PUBLIC_ROUTES.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(manifest["source"]["operationCount"], 188);
    assert_eq!(manifest["source"]["supplementalOperationCount"], 6);
    assert_eq!(manifest["source"]["excludedAdminOperationCount"], 35);
    assert_eq!(manifest["source"]["excludedWebhookOperationCount"], 18);
    assert_eq!(rust_routes.len(), 188);
    assert_eq!(rust_routes, manifest_routes);
    assert!(rust_routes
        .iter()
        .all(|(_, path)| !path.split('/').any(|segment| segment == "admin")));
    assert!(rust_routes
        .iter()
        .all(|(_, path)| !path.split('/').any(|segment| segment == "webhooks")));
}
