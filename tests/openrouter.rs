//! Direct OpenRouter proxy: catalogs, the four request shapes, and the async
//! video flow.
//!
//! The distinction these pin is which routes carry the `{ success, data }`
//! envelope and which return the upstream provider's payload verbatim. Chat,
//! completions, messages and embeddings are passthroughs — unwrapping them
//! would strip the caller's actual response — while the catalogs and the media
//! routes are enveloped like every other agent integration.

use serde_json::json;
use tinyhumans_sdk::api::agent_integrations::{
    OpenRouterMediaModelsResponse, OpenRouterModelsResponse, OpenRouterVideoJob,
};
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_models_is_typed_and_paginated() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/openrouter/models"))
        .and(query_param("author", "anthropic"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "object": "list",
                "data": [{
                    "id": "anthropic/claude-sonnet-4.5",
                    "display_name": "Claude Sonnet 4.5",
                    "context_length": 1000000,
                    "input_modalities": ["text", "image"],
                    "supports_tools": true,
                    "supports_thinking": true,
                    "pricing": {"input_per_1m": 3.0, "output_per_1m": 15.0, "cached_input_per_1m": 0.3}
                }],
                "total": 1, "limit": 100, "offset": 0
            }
        })))
        .mount(&server)
        .await;

    let response: OpenRouterModelsResponse = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .list_openrouter_models(&[("author", Some("anthropic".into()))])
        .await
        .unwrap();

    assert_eq!(response.total, 1);
    // The id is the bare slug, which is what callers send back as `model`.
    assert_eq!(response.data[0].id, "anthropic/claude-sonnet-4.5");
    assert_eq!(response.data[0].pricing.cached_input_per_1m, Some(0.3));
    assert!(response.data[0].supports_tools);
}

#[tokio::test]
async fn chat_completion_returns_the_upstream_payload_unwrapped() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/openrouter/chat/completions"))
        .and(body_json(json!({
            "model": "anthropic/claude-sonnet-4.5",
            "messages": [{"role": "user", "content": "hi"}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "gen-1",
            "choices": [{"message": {"role": "assistant", "content": "hello"}}],
            "usage": {"prompt_tokens": 1, "completion_tokens": 1, "total_tokens": 2}
        })))
        .mount(&server)
        .await;

    let response = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .openrouter_chat_completion(&json!({
            "model": "anthropic/claude-sonnet-4.5",
            "messages": [{"role": "user", "content": "hi"}]
        }))
        .await
        .unwrap();

    // A passthrough route has no envelope to unwrap; the OpenAI-shaped body is
    // handed back as-is.
    assert_eq!(response["id"], "gen-1");
    assert_eq!(response["choices"][0]["message"]["content"], "hello");
}

#[tokio::test]
async fn arbitrary_openrouter_parameters_are_forwarded() {
    let server = MockServer::start().await;
    // The request type is `impl Serialize` precisely so provider-specific knobs
    // reach upstream instead of being dropped by a closed struct.
    Mock::given(method("POST"))
        .and(path("/agent-integrations/openrouter/chat/completions"))
        .and(body_json(json!({
            "model": "anthropic/claude-sonnet-4.5",
            "messages": [],
            "reasoning": {"effort": "high"},
            "transforms": ["middle-out"],
            "provider": {"order": ["anthropic"]}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "gen-2"})))
        .mount(&server)
        .await;

    let response = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .openrouter_chat_completion(&json!({
            "model": "anthropic/claude-sonnet-4.5",
            "messages": [],
            "reasoning": {"effort": "high"},
            "transforms": ["middle-out"],
            "provider": {"order": ["anthropic"]}
        }))
        .await
        .unwrap();
    assert_eq!(response["id"], "gen-2");
}

#[tokio::test]
async fn messages_speaks_the_anthropic_shape() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/openrouter/messages"))
        .and(body_json(json!({
            "model": "anthropic/claude-sonnet-4.5",
            "system": "be terse",
            "messages": [{"role": "user", "content": "hi"}],
            "max_tokens": 64
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "msg_1",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "hello"}],
            "usage": {"input_tokens": 12, "output_tokens": 15}
        })))
        .mount(&server)
        .await;

    let response = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .openrouter_message(&json!({
            "model": "anthropic/claude-sonnet-4.5",
            "system": "be terse",
            "messages": [{"role": "user", "content": "hi"}],
            "max_tokens": 64
        }))
        .await
        .unwrap();

    // Anthropic's envelope, not OpenAI's: content blocks and input/output tokens.
    assert_eq!(response["type"], "message");
    assert_eq!(response["content"][0]["text"], "hello");
    assert_eq!(response["usage"]["input_tokens"], 12);
}

#[tokio::test]
async fn embedding_models_come_from_their_own_catalog() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/openrouter/embeddings/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "object": "list",
                "data": [{
                    "id": "openai/text-embedding-3-small",
                    "display_name": "Text Embedding 3 Small",
                    "pricing": {"input_per_1m": 0.02, "output_per_1m": 0.0}
                }],
                "total": 1, "limit": 100, "offset": 0
            }
        })))
        .mount(&server)
        .await;

    let response: OpenRouterModelsResponse = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .list_openrouter_embedding_models(&[])
        .await
        .unwrap();
    assert_eq!(response.data[0].id, "openai/text-embedding-3-small");
    assert_eq!(response.data[0].pricing.output_per_1m, 0.0);
}

#[tokio::test]
async fn embeddings_returns_the_upstream_payload() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/openrouter/embeddings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object": "list",
            "data": [{"object": "embedding", "index": 0, "embedding": [0.1, 0.2]}],
            "usage": {"prompt_tokens": 8, "total_tokens": 8}
        })))
        .mount(&server)
        .await;

    let response = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .openrouter_embeddings(&json!({
            "model": "openai/text-embedding-3-small",
            "input": "hello"
        }))
        .await
        .unwrap();
    assert_eq!(response["data"][0]["index"], 0);
    assert_eq!(response["usage"]["prompt_tokens"], 8);
}

#[tokio::test]
async fn image_models_carry_no_generation_price() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/openrouter/images/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "object": "list",
                "data": [{"id": "bytedance-seed/seedream-4.5", "display_name": "Seedream 4.5"}],
                "total": 1, "limit": 100, "offset": 0
            }
        })))
        .mount(&server)
        .await;

    let response: OpenRouterMediaModelsResponse = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .list_openrouter_image_models(&[])
        .await
        .unwrap();
    // Images bill at the cost the generation response reports, so the catalog
    // publishes no price to quote up front.
    assert_eq!(response.data[0].price_per_generation, None);
}

#[tokio::test]
async fn video_models_publish_a_flat_generation_price() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/openrouter/videos/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "object": "list",
                "data": [{
                    "id": "google/veo-3.1",
                    "display_name": "Veo 3.1",
                    "price_per_generation": 0.5
                }],
                "total": 1, "limit": 100, "offset": 0
            }
        })))
        .mount(&server)
        .await;

    let response: OpenRouterMediaModelsResponse = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .list_openrouter_video_models(&[])
        .await
        .unwrap();
    assert_eq!(response.data[0].price_per_generation, Some(0.5));
}

#[tokio::test]
async fn image_generation_unwraps_the_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/openrouter/images"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "created": 1,
                "data": [{"b64_json": "aGk="}],
                "usage": {"cost": 0.04}
            }
        })))
        .mount(&server)
        .await;

    let response = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .openrouter_create_image(&json!({
            "model": "bytedance-seed/seedream-4.5",
            "prompt": "a red panda"
        }))
        .await
        .unwrap();
    assert_eq!(response["data"][0]["b64_json"], "aGk=");
}

#[tokio::test]
async fn the_video_flow_submits_polls_and_downloads() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/agent-integrations/openrouter/videos"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {"id": "job-abc", "status": "pending", "polling_url": "/api/v1/videos/job-abc"}
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/openrouter/videos/job-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {"id": "job-abc", "status": "completed", "generation_id": "gen-xyz"}
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(
            "/agent-integrations/openrouter/videos/job-abc/content",
        ))
        .and(query_param("index", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes([0_u8, 1, 2, 255]))
        .mount(&server)
        .await;

    let api = TinyHumansClient::new(server.uri());

    let job: OpenRouterVideoJob = api
        .agent_integrations()
        .openrouter_create_video(&json!({"model": "google/veo-3.1", "prompt": "a mountain"}))
        .await
        .unwrap();
    assert_eq!(job.id, "job-abc");
    assert_eq!(job.status, "pending");

    let polled = api
        .agent_integrations()
        .get_openrouter_video("job-abc")
        .await
        .unwrap();
    assert_eq!(polled.status, "completed");
    assert_eq!(polled.generation_id.as_deref(), Some("gen-xyz"));

    let bytes = api
        .agent_integrations()
        .openrouter_video_content("job-abc", Some(1))
        .await
        .unwrap();
    assert_eq!(bytes, vec![0, 1, 2, 255]);
}

#[tokio::test]
async fn video_content_omits_the_index_when_unset() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/agent-integrations/openrouter/videos/job-abc/content",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_bytes([7_u8]))
        .mount(&server)
        .await;

    let bytes = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .openrouter_video_content("job-abc", None)
        .await
        .unwrap();
    assert_eq!(bytes, vec![7]);
}

#[tokio::test]
async fn video_job_ids_are_path_encoded() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/agent-integrations/openrouter/videos/job%2Fabc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true, "data": {"id": "job/abc", "status": "pending"}
        })))
        .mount(&server)
        .await;

    let job = TinyHumansClient::new(server.uri())
        .agent_integrations()
        .get_openrouter_video("job/abc")
        .await
        .unwrap();
    assert_eq!(job.id, "job/abc");
}
