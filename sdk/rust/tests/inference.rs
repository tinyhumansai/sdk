use serde_json::json;
use tinyhumans_sdk::TinyHumansClient;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Every inference endpoint returns native OpenAI-format bodies that are NOT
// wrapped in the `{ success, data }` envelope. Each test responds with a plain
// body and asserts it is returned unchanged.

#[tokio::test]
async fn list_models_returns_body_as_is() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/openai/v1/models"))
        .and(query_param("with_display", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"object": "list", "data": [{"id": "gpt-x"}]})),
        )
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .inference()
        .list_models(&[("with_display", Some("true".to_string()))])
        .await
        .unwrap();
    assert_eq!(result, json!({"object": "list", "data": [{"id": "gpt-x"}]}));
}

#[tokio::test]
async fn create_chat_completion_not_unwrapped() {
    let server = MockServer::start().await;
    // A body that happens to contain `success`/`data` keys must NOT be unwrapped.
    let body = json!({
        "id": "chatcmpl_1",
        "object": "chat.completion",
        "success": true,
        "data": {"ignored": true},
        "choices": [{"message": {"role": "assistant", "content": "hi"}}]
    });
    Mock::given(method("POST"))
        .and(path("/openai/v1/chat/completions"))
        .and(body_json(json!({"model": "gpt-x", "messages": []})))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .inference()
        .create_chat_completion(&json!({"model": "gpt-x", "messages": []}))
        .await
        .unwrap();
    assert_eq!(result, body);
}

#[tokio::test]
async fn create_completion_returns_body_as_is() {
    let server = MockServer::start().await;
    let body = json!({"id": "cmpl_1", "object": "text_completion"});
    Mock::given(method("POST"))
        .and(path("/openai/v1/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .inference()
        .create_completion(&json!({"model": "gpt-x", "prompt": "hi"}))
        .await
        .unwrap();
    assert_eq!(result, body);
}

#[tokio::test]
async fn create_transcription_returns_body_as_is() {
    let server = MockServer::start().await;
    let body = json!({"text": "hello world"});
    Mock::given(method("POST"))
        .and(path("/openai/v1/audio/transcriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .inference()
        .create_transcription(&json!({"model": "whisper-1", "file": "audio"}))
        .await
        .unwrap();
    assert_eq!(result, body);
}

#[tokio::test]
async fn create_speech_returns_body_as_is() {
    let server = MockServer::start().await;
    // Non-JSON audio payloads fall back to a JSON string value.
    let body = json!({"audioUrl": "cdn/tts.mp3"});
    Mock::given(method("POST"))
        .and(path("/openai/v1/audio/speech"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .inference()
        .create_speech(&json!({"model": "eleven", "input": "hi", "voice": "rachel"}))
        .await
        .unwrap();
    assert_eq!(result, body);
}

#[tokio::test]
async fn create_embeddings_returns_body_as_is() {
    let server = MockServer::start().await;
    let body = json!({"object": "list", "data": [{"embedding": [0.1, 0.2]}]});
    Mock::given(method("POST"))
        .and(path("/openai/v1/embeddings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = TinyHumansClient::new(server.uri());
    let result = client
        .inference()
        .create_embeddings(&json!({"model": "voyage-3", "input": "hi"}))
        .await
        .unwrap();
    assert_eq!(result, body);
}
