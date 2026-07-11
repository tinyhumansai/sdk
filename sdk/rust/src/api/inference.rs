//! OpenAI-compatible inference: models, completions, embeddings, speech, transcription.
//!
//! These endpoints return native OpenAI-format bodies and are NOT wrapped in the
//! TinyHumans `{ success, data }` envelope, so every method passes
//! `unwrap = false` and returns the response body as-is.

use reqwest::Method;
use serde_json::Value;

use crate::{Error, HttpClient, QueryParam};

/// Typed client for the `/openai/v1/*` inference routes.
pub struct InferenceApi<'a> {
    http: &'a HttpClient,
}

impl<'a> InferenceApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List available models in OpenAI format.
    pub async fn list_models(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/openai/v1/models", query, None, false)
            .await
    }

    /// Create a chat completion.
    pub async fn create_chat_completion(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/openai/v1/chat/completions",
                &[],
                Some(body),
                false,
            )
            .await
    }

    /// Create a text completion.
    pub async fn create_completion(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/openai/v1/completions",
                &[],
                Some(body),
                false,
            )
            .await
    }

    /// Transcribe audio to text.
    pub async fn create_transcription(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/openai/v1/audio/transcriptions",
                &[],
                Some(body),
                false,
            )
            .await
    }

    /// Synthesize speech from text via ElevenLabs (mp3).
    pub async fn create_speech(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/openai/v1/audio/speech",
                &[],
                Some(body),
                false,
            )
            .await
    }

    /// Create text embeddings via Voyage AI.
    pub async fn create_embeddings(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/openai/v1/embeddings",
                &[],
                Some(body),
                false,
            )
            .await
    }
}
