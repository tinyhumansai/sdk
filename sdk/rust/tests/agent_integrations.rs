use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// --- Apify ---

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

// --- Composio ---

#[tokio::test]
async fn authorize_composio_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/composio/authorize"))
        .and(body_json(json!({"toolkit": "github"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"redirectUrl": "https://x"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .authorize_composio(&json!({"toolkit": "github"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"redirectUrl": "https://x"}));
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
async fn execute_composio_tool_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/composio/execute"))
        .and(body_json(json!({"tool": "GITHUB_STAR"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"output": "ok"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .execute_composio_tool(&json!({"tool": "GITHUB_STAR"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"output": "ok"}));
}

#[tokio::test]
async fn list_composio_toolkits_gets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/composio/toolkits"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"toolkits": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_composio_toolkits()
        .await
        .unwrap();

    assert_eq!(result, json!({"toolkits": []}));
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
async fn list_composio_tools_gets_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/composio/tools"))
        .and(query_param("toolkit", "github"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"tools": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_composio_tools(&[("toolkit", Some("github".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"tools": []}));
}

#[tokio::test]
async fn list_composio_triggers_gets_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/composio/triggers"))
        .and(query_param("connectionId", "conn_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"triggers": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_composio_triggers(&[("connectionId", Some("conn_1".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"triggers": []}));
}

#[tokio::test]
async fn create_composio_trigger_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/composio/triggers"))
        .and(body_json(json!({"slug": "NEW_ISSUE"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"triggerId": "trg_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .create_composio_trigger(&json!({"slug": "NEW_ISSUE"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"triggerId": "trg_1"}));
}

#[tokio::test]
async fn list_composio_available_triggers_gets_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/composio/triggers/available"))
        .and(query_param("toolkit", "github"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"available": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_composio_available_triggers(&[("toolkit", Some("github".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"available": []}));
}

#[tokio::test]
async fn delete_composio_trigger_uses_delete() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/agent-integrations/composio/triggers/trg_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"deleted": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .delete_composio_trigger("trg_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"deleted": true}));
}

// --- Crypto ---

#[tokio::test]
async fn crypto_bridge_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/crypto/bridge"))
        .and(body_json(json!({"from": "eth", "to": "base"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"tx": "0xabc"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .crypto_bridge(&json!({"from": "eth", "to": "base"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"tx": "0xabc"}));
}

#[tokio::test]
async fn list_crypto_routes_gets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/crypto/routes"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"chains": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_crypto_routes()
        .await
        .unwrap();

    assert_eq!(result, json!({"chains": []}));
}

#[tokio::test]
async fn crypto_swap_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/crypto/swap"))
        .and(body_json(json!({"tokenIn": "USDC"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"tx": "0xdef"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .crypto_swap(&json!({"tokenIn": "USDC"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"tx": "0xdef"}));
}

// --- Financial APIs ---

#[tokio::test]
async fn financial_apis_commodity_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/financial-apis/commodity"))
        .and(body_json(json!({"symbol": "WTI"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"symbol": "WTI"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .financial_apis_commodity(&json!({"symbol": "WTI"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"symbol": "WTI"}));
}

#[tokio::test]
async fn financial_apis_crypto_series_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/financial-apis/crypto-series"))
        .and(body_json(json!({"symbol": "BTC"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"symbol": "BTC"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .financial_apis_crypto_series(&json!({"symbol": "BTC"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"symbol": "BTC"}));
}

#[tokio::test]
async fn financial_apis_exchange_rate_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/financial-apis/exchange-rate"))
        .and(body_json(json!({"from": "USD", "to": "EUR"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"rate": 1.1}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .financial_apis_exchange_rate(&json!({"from": "USD", "to": "EUR"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"rate": 1.1}));
}

#[tokio::test]
async fn financial_apis_options_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/financial-apis/options"))
        .and(body_json(json!({"symbol": "AAPL"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"symbol": "AAPL"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .financial_apis_options(&json!({"symbol": "AAPL"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"symbol": "AAPL"}));
}

#[tokio::test]
async fn financial_apis_quote_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/financial-apis/quote"))
        .and(body_json(json!({"symbol": "MSFT"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"price": 100}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .financial_apis_quote(&json!({"symbol": "MSFT"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"price": 100}));
}

// --- Google Places ---

#[tokio::test]
async fn google_places_details_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/google-places/details"))
        .and(body_json(json!({"placeId": "p_1"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"name": "Cafe"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .google_places_details(&json!({"placeId": "p_1"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"name": "Cafe"}));
}

#[tokio::test]
async fn google_places_search_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/google-places/search"))
        .and(body_json(json!({"query": "coffee"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"results": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .google_places_search(&json!({"query": "coffee"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"results": []}));
}

// --- Media Generation ---

#[tokio::test]
async fn media_generation_images_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/media-generation/images"))
        .and(body_json(json!({"prompt": "a cat"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"requestId": "req_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .media_generation_images(&json!({"prompt": "a cat"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"requestId": "req_1"}));
}

#[tokio::test]
async fn list_media_generation_models_gets_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/media-generation/models"))
        .and(query_param("kind", "image"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"models": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_media_generation_models(&[("kind", Some("image".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"models": []}));
}

#[tokio::test]
async fn get_media_generation_request_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/media-generation/requests/req_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "done"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_media_generation_request("req_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "done"}));
}

#[tokio::test]
async fn media_generation_videos_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/media-generation/videos"))
        .and(body_json(json!({"prompt": "a dog"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"requestId": "req_2"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .media_generation_videos(&json!({"prompt": "a dog"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"requestId": "req_2"}));
}

// --- Parallel ---

#[tokio::test]
async fn parallel_chat_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/parallel/chat"))
        .and(body_json(json!({"messages": []})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"reply": "hi"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .parallel_chat(&json!({"messages": []}))
        .await
        .unwrap();

    assert_eq!(result, json!({"reply": "hi"}));
}

#[tokio::test]
async fn parallel_dataset_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/parallel/dataset"))
        .and(body_json(json!({"query": "companies"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"findallId": "fa_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .parallel_dataset(&json!({"query": "companies"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"findallId": "fa_1"}));
}

#[tokio::test]
async fn get_parallel_dataset_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/parallel/dataset/fa_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "running"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_parallel_dataset("fa_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "running"}));
}

#[tokio::test]
async fn get_parallel_dataset_result_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/parallel/dataset/fa_1/result"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"candidates": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_parallel_dataset_result("fa_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"candidates": []}));
}

#[tokio::test]
async fn parallel_enrich_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/parallel/enrich"))
        .and(body_json(json!({"input": "acme"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"enriched": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .parallel_enrich(&json!({"input": "acme"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"enriched": true}));
}

#[tokio::test]
async fn parallel_extract_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/parallel/extract"))
        .and(body_json(json!({"urls": ["https://x"]})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"content": "text"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .parallel_extract(&json!({"urls": ["https://x"]}))
        .await
        .unwrap();

    assert_eq!(result, json!({"content": "text"}));
}

#[tokio::test]
async fn parallel_research_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/parallel/research"))
        .and(body_json(json!({"topic": "ai"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"runId": "run_1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .parallel_research(&json!({"topic": "ai"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"runId": "run_1"}));
}

#[tokio::test]
async fn get_parallel_research_uses_path_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/parallel/research/run_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"status": "queued"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_parallel_research("run_1")
        .await
        .unwrap();

    assert_eq!(result, json!({"status": "queued"}));
}

#[tokio::test]
async fn get_parallel_research_result_uses_path_and_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/parallel/research/run_1/result"))
        .and(query_param("timeout", "60"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"report": "done"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_parallel_research_result("run_1", &[("timeout", Some("60".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"report": "done"}));
}

#[tokio::test]
async fn parallel_search_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/parallel/search"))
        .and(body_json(json!({"query": "news"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"results": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .parallel_search(&json!({"query": "news"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"results": []}));
}

// --- Pricing ---

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

// --- Recall Calendar ---

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
async fn disconnect_recall_calendar_posts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/recall-calendar/disconnect"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"disconnected": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .disconnect_recall_calendar()
        .await
        .unwrap();

    assert_eq!(result, json!({"disconnected": true}));
}

#[tokio::test]
async fn list_recall_calendar_meetings_gets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/recall-calendar/meetings"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"meetings": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .list_recall_calendar_meetings()
        .await
        .unwrap();

    assert_eq!(result, json!({"meetings": []}));
}

#[tokio::test]
async fn recall_calendar_oauth_complete_gets_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/recall-calendar/oauth-complete"))
        .and(query_param("code", "code_1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"ok": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .recall_calendar_oauth_complete(&[("code", Some("code_1".to_string()))])
        .await
        .unwrap();

    assert_eq!(result, json!({"ok": true}));
}

#[tokio::test]
async fn get_recall_calendar_status_gets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/recall-calendar/status"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"connected": false}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .get_recall_calendar_status()
        .await
        .unwrap();

    assert_eq!(result, json!({"connected": false}));
}

// --- Tenor ---

#[tokio::test]
async fn tenor_search_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/tenor/search"))
        .and(body_json(json!({"q": "cat"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"gifs": []}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .tenor_search(&json!({"q": "cat"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"gifs": []}));
}

// --- Twilio ---

#[tokio::test]
async fn twilio_call_posts_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/twilio/call"))
        .and(body_json(json!({"to": "+15550001111"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"callSid": "CA1"}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .twilio_call(&json!({"to": "+15550001111"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"callSid": "CA1"}));
}

#[tokio::test]
async fn twilio_incoming_call_webhook_uses_path_and_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/agent-integrations/twilio/webhooks/incoming-call/user_7",
        ))
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

#[tokio::test]
async fn twilio_status_webhook_uses_path_and_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/twilio/webhooks/status/user_7"))
        .and(body_json(json!({"CallStatus": "completed"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success": true, "data": {"handled": true}})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .agent_integrations()
        .twilio_status_webhook("user_7", &json!({"CallStatus": "completed"}))
        .await
        .unwrap();

    assert_eq!(result, json!({"handled": true}));
}
