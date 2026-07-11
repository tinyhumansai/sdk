use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn run_apify_actor_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/apify/run"))
        .and(body_json(json!({"actorId": "abc", "input": {}})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"runId": "r1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .run_apify_actor(&json!({"actorId": "abc", "input": {}}))
        .await
        .unwrap();

    assert_eq!(result, json!({"runId": "r1"}));
}

#[tokio::test]
async fn get_apify_run_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/apify/runs/run_42"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "SUCCEEDED"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_apify_run("run_42")
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "SUCCEEDED"}));
}

#[tokio::test]
async fn get_apify_run_results_sends_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/apify/runs/run_42/results"))
        .and(query_param("limit", "10"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"items": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_apify_run_results("run_42", &[("limit", Some("10".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"items": []}));
}

#[tokio::test]
async fn list_composio_connections_unwraps_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/composio/connections"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": [{"id": "c1"}]})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_composio_connections()
        .await
        .unwrap();

    assert_eq!(result, json!([{"id": "c1"}]));
}

#[tokio::test]
async fn delete_composio_connection_uses_delete() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/agent-integrations/composio/connections/conn_9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .delete_composio_connection("conn_9")
        .await
        .unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

#[tokio::test]
async fn refresh_composio_toolkits_posts_with_query() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/composio/toolkits/refresh"))
        .and(query_param("full", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"refreshed": 3}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .refresh_composio_toolkits(&[("full", Some("true".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"refreshed": 3}));
}

#[tokio::test]
async fn get_pricing_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/pricing"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"apify": 1}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client.agent_integrations().get_pricing().await.unwrap();

    assert_eq!(result, json!({"apify": 1}));
}

#[tokio::test]
async fn connect_recall_calendar_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/recall-calendar/connect"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"url": "https://x"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .connect_recall_calendar()
        .await
        .unwrap();

    assert_eq!(result, json!({"url": "https://x"}));
}

#[tokio::test]
async fn twilio_incoming_call_webhook_uses_path_and_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/twilio/webhooks/incoming-call/user_7"))
        .and(body_json(json!({"CallSid": "CA1"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .twilio_incoming_call_webhook("user_7", &json!({"CallSid": "CA1"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}
