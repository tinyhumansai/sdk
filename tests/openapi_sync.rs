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
        // `create_task` now decodes a typed `Task`, so the stub returns the
        // route's real `{"task": {...}}` payload rather than a placeholder.
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": { "task": {
                "id": "task-1",
                "title": "Ship",
                "description": "",
                "status": "inProgress",
                "createdAt": "2026-07-29T00:00:00Z",
                "updatedAt": "2026-07-29T00:00:00Z",
            }},
        })))
        .mount(&server)
        .await;
    let request = CreateTaskRequest {
        title: "Ship".into(),
        description: None,
        status: Some(TaskStatus::InProgress),
        recurrence: None,
    };
    let task = TinyHumansClient::new(server.uri())
        .medulla()
        .create_task(&request)
        .await
        .unwrap();
    assert_eq!(task.status, TaskStatus::InProgress);
}

#[tokio::test]
async fn medulla_workflows_returns_advertised_catalog() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/medulla/v1/workflows"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "workflows": [{
                    "id": "release",
                    "name": "Release",
                    "nodeCount": 4,
                    "enabled": true,
                    "agentId": "agent-1"
                }]
            }
        })))
        .mount(&server)
        .await;

    let workflows = TinyHumansClient::new(server.uri())
        .medulla()
        .workflows()
        .await
        .unwrap();

    assert_eq!(workflows.len(), 1);
    assert_eq!(workflows[0].id, "release");
    assert_eq!(workflows[0].node_count, 4);
    assert_eq!(workflows[0].agent_id.as_deref(), Some("agent-1"));
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

    assert_eq!(manifest["source"]["operationCount"], 197);
    assert_eq!(manifest["source"]["supplementalOperationCount"], 15);
    assert_eq!(manifest["source"]["excludedAdminOperationCount"], 32);
    assert_eq!(manifest["source"]["excludedWebhookOperationCount"], 12);
    assert_eq!(rust_routes.len(), 197);
    assert_eq!(rust_routes, manifest_routes);
    assert!(rust_routes
        .iter()
        .all(|(_, path)| !path.split('/').any(|segment| segment == "admin")));
    // Webhook *receivers* stay out of the public surface: they are provider
    // callbacks authenticated by signature, so an SDK caller invoking one would
    // be forging provider traffic. The user-owned tunnel CRUD under
    // `/webhooks/core` is ordinary bearer-authenticated user-facing API and is
    // the one permitted exception under this prefix.
    assert!(rust_routes.iter().all(|(_, path)| {
        !path.split('/').any(|segment| segment == "webhooks")
            || *path == "/webhooks/core"
            || path.starts_with("/webhooks/core/")
    }));
}
