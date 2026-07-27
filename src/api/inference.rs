//! OpenAI-compatible inference: models, completions, embeddings, speech, transcription.
//!
//! These endpoints return native OpenAI-format bodies and are NOT wrapped in the
//! TinyHumans `{ success, data }` envelope, so every method passes
//! `unwrap = false` and returns the response body as-is.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::types::DynamicResponse;
use crate::{Error, HttpClient, QueryParam};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ChatTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatTool {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ChatToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatToolFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub parameters: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    Mode(ToolChoiceMode),
    Function {
        #[serde(rename = "type")]
        kind: String,
        function: ToolChoiceFunction,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolChoiceFunction {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SpeechModel {
    TtsV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpeechRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<SpeechModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_visemes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EmbeddingModel {
    EmbeddingV1,
    EmbeddingCodeV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum EmbeddingInput {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingInputType {
    Query,
    Document,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddingsRequest {
    pub model: EmbeddingModel,
    pub input: EmbeddingInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<EmbeddingInputType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranscriptionRequest {
    pub file_name: String,
    pub file: Vec<u8>,
    pub model: Option<String>,
    pub language: Option<String>,
    pub response_format: Option<String>,
    pub temperature: Option<f64>,
    pub vad_model: Option<String>,
    pub diarize: Option<bool>,
    pub timestamp_granularities: Vec<String>,
}

/// Typed client for the `/openai/v1/*` inference routes.
pub struct InferenceApi<'a> {
    http: &'a HttpClient,
}

impl<'a> InferenceApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List available models in OpenAI format.
    pub async fn list_models(&self, query: &[QueryParam]) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/openai/v1/models", query, None, false)
            .await
            .map(Into::into)
    }

    /// Create a chat completion.
    pub async fn create_chat_completion(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("chat request is serializable");
        self.http
            .send(
                Method::POST,
                "/openai/v1/chat/completions",
                &[],
                Some(&body),
                false,
            )
            .await
            .map(Into::into)
    }

    /// Create a text completion.
    pub async fn create_completion(
        &self,
        request: &CompletionRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("completion request is serializable");
        self.http
            .send(
                Method::POST,
                "/openai/v1/completions",
                &[],
                Some(&body),
                false,
            )
            .await
            .map(Into::into)
    }

    /// Transcribe audio to text.
    pub async fn create_transcription(
        &self,
        request: &TranscriptionRequest,
    ) -> Result<DynamicResponse, Error> {
        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(request.file.clone())
                .file_name(request.file_name.clone()),
        );
        if let Some(value) = &request.model {
            form = form.text("model", value.clone());
        }
        if let Some(value) = &request.language {
            form = form.text("language", value.clone());
        }
        if let Some(value) = &request.response_format {
            form = form.text("response_format", value.clone());
        }
        if let Some(value) = request.temperature {
            form = form.text("temperature", value.to_string());
        }
        if let Some(value) = &request.vad_model {
            form = form.text("vad_model", value.clone());
        }
        if let Some(value) = request.diarize {
            form = form.text("diarize", value.to_string());
        }
        for value in &request.timestamp_granularities {
            form = form.text("timestamp_granularities[]", value.clone());
        }
        self.http
            .post_multipart("/openai/v1/audio/transcriptions", form)
            .await
            .map(Into::into)
    }

    /// Synthesize speech from text via ElevenLabs (mp3).
    pub async fn create_speech(&self, request: &SpeechRequest) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("speech request is serializable");
        self.http
            .send(
                Method::POST,
                "/openai/v1/audio/speech",
                &[],
                Some(&body),
                false,
            )
            .await
            .map(Into::into)
    }

    /// Create text embeddings via Voyage AI.
    pub async fn create_embeddings(
        &self,
        request: &EmbeddingsRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("embeddings request is serializable");
        self.http
            .send(
                Method::POST,
                "/openai/v1/embeddings",
                &[],
                Some(&body),
                false,
            )
            .await
            .map(Into::into)
    }
}
