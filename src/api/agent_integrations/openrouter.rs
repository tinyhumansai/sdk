//! Direct OpenRouter proxy.
//!
//! Unlike the curated `/openai/*` inference surface, these routes take **bare
//! OpenRouter slugs** (`anthropic/claude-sonnet-4.5`) and pass the request
//! straight upstream, so any parameter OpenRouter accepts is forwarded. The
//! request bodies are therefore taken as `impl Serialize` rather than closed
//! structs — pinning them here would silently drop provider knobs the SDK does
//! not yet know about — and the passthrough responses come back as
//! [`DynamicResponse`], matching how [`crate::api::inference`] treats the
//! OpenAI-compatible surface.
//!
//! Four request shapes share the one provider:
//!
//! - OpenAI chat and text completions.
//! - Anthropic-format messages, which the Anthropic SDK can call directly.
//! - Embeddings, which resolve against a **separate** upstream catalog — an
//!   embedding slug is not valid on the chat routes, or vice versa.
//! - Image and video generation, billed per generation rather than per token
//!   and each with its own catalog again.
//!
//! Video generation is asynchronous: [`AgentIntegrationsApi::openrouter_create_video`]
//! returns a job to poll with [`AgentIntegrationsApi::get_openrouter_video`],
//! then fetch with [`AgentIntegrationsApi::openrouter_video_content`]. It is
//! billed on submit, so polling and download cost nothing, and both are
//! restricted to the account that submitted the job.

use super::AgentIntegrationsApi;
use crate::{api::types::DynamicResponse, enc, Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Per-1M-token pricing as charged, for one catalog model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenRouterModelPricing {
    #[serde(default)]
    pub input_per_1m: f64,
    #[serde(default)]
    pub output_per_1m: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_input_per_1m: Option<f64>,
}

/// One chat or embedding model. `id` is the bare slug to send back as `model`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenRouterModel {
    pub id: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u64>,
    #[serde(default)]
    pub input_modalities: Vec<String>,
    #[serde(default)]
    pub supports_tools: bool,
    #[serde(default)]
    pub supports_thinking: bool,
    #[serde(default)]
    pub pricing: OpenRouterModelPricing,
}

/// Paginated catalog listing. `total` counts matches before pagination.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenRouterModelsResponse {
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub data: Vec<OpenRouterModel>,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub limit: u64,
    #[serde(default)]
    pub offset: u64,
}

/// One image or video model.
///
/// Media models are not token-priced, so this carries no per-1M block. Video
/// models publish a flat `price_per_generation`; image models publish none at
/// all, because an image is billed at the exact cost the generation response
/// reports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenRouterMediaModel {
    pub id: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_per_generation: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenRouterMediaModelsResponse {
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub data: Vec<OpenRouterMediaModel>,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub limit: u64,
    #[serde(default)]
    pub offset: u64,
}

/// An accepted video generation job. Poll `id` until `status` is terminal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenRouterVideoJob {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polling_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl AgentIntegrationsApi<'_> {
    /// List chat models available through the direct OpenRouter proxy.
    ///
    /// Supports `search`, `modality`, `tools`, `max_output_price`, `author`,
    /// `limit` and `offset`. Embedding, image and video models are listed by
    /// their own methods — they are separate upstream catalogs.
    pub async fn list_openrouter_models(
        &self,
        query: &[QueryParam],
    ) -> Result<OpenRouterModelsResponse, Error> {
        self.send(
            Method::GET,
            "/agent-integrations/openrouter/models",
            query,
            None,
            true,
        )
        .await
    }

    /// Create a chat completion straight against OpenRouter.
    ///
    /// `stream: true` is rejected with [`Error::StreamingNotSupported`]: this
    /// method's transport buffers the full response body (there is no
    /// incremental transport exposed for this route yet, including through
    /// [`crate::TinyHumansClient::raw`], which buffers the same way), so a
    /// streamed request would come back as one opaque non-JSON string rather
    /// than the events a streaming caller wants. Omit `stream` (or set it to
    /// `false`) to get the buffered JSON response.
    pub async fn openrouter_chat_completion(
        &self,
        request: &impl Serialize,
    ) -> Result<DynamicResponse, Error> {
        self.passthrough("/agent-integrations/openrouter/chat/completions", request)
            .await
    }

    /// Create a text completion straight against OpenRouter.
    ///
    /// `stream: true` is rejected the same as on
    /// [`Self::openrouter_chat_completion`] — no incremental transport is
    /// exposed for it yet.
    pub async fn openrouter_completion(
        &self,
        request: &impl Serialize,
    ) -> Result<DynamicResponse, Error> {
        self.passthrough("/agent-integrations/openrouter/completions", request)
            .await
    }

    /// Create a message in **Anthropic's** format.
    ///
    /// Request and response are Anthropic-shaped, not OpenAI-shaped: `system`
    /// is a top-level field and the reply is a `type: "message"` envelope with
    /// a `content` block array. `stream: true` is rejected the same as on
    /// [`Self::openrouter_chat_completion`] — no incremental transport is
    /// exposed for it yet.
    pub async fn openrouter_message(
        &self,
        request: &impl Serialize,
    ) -> Result<DynamicResponse, Error> {
        self.passthrough("/agent-integrations/openrouter/messages", request)
            .await
    }

    /// List embedding models. A separate catalog from the chat models.
    pub async fn list_openrouter_embedding_models(
        &self,
        query: &[QueryParam],
    ) -> Result<OpenRouterModelsResponse, Error> {
        self.send(
            Method::GET,
            "/agent-integrations/openrouter/embeddings/models",
            query,
            None,
            true,
        )
        .await
    }

    /// Create embeddings. `model` must be an embedding slug, not a chat slug.
    pub async fn openrouter_embeddings(
        &self,
        request: &impl Serialize,
    ) -> Result<DynamicResponse, Error> {
        self.passthrough("/agent-integrations/openrouter/embeddings", request)
            .await
    }

    /// List image-generation models. These publish no price: an image is
    /// billed at the exact cost the generation response reports.
    pub async fn list_openrouter_image_models(
        &self,
        query: &[QueryParam],
    ) -> Result<OpenRouterMediaModelsResponse, Error> {
        self.send(
            Method::GET,
            "/agent-integrations/openrouter/images/models",
            query,
            None,
            true,
        )
        .await
    }

    /// Generate an image. Synchronous — the response carries the image data.
    pub async fn openrouter_create_image(
        &self,
        request: &impl Serialize,
    ) -> Result<DynamicResponse, Error> {
        self.post("/agent-integrations/openrouter/images", request)
            .await
    }

    /// List video-generation models, with their flat per-generation price.
    pub async fn list_openrouter_video_models(
        &self,
        query: &[QueryParam],
    ) -> Result<OpenRouterMediaModelsResponse, Error> {
        self.send(
            Method::GET,
            "/agent-integrations/openrouter/videos/models",
            query,
            None,
            true,
        )
        .await
    }

    /// Submit a video generation job.
    ///
    /// Asynchronous, and billed here on submit rather than on completion, so
    /// polling and download are free.
    pub async fn openrouter_create_video(
        &self,
        request: &impl Serialize,
    ) -> Result<OpenRouterVideoJob, Error> {
        self.post("/agent-integrations/openrouter/videos", request)
            .await
    }

    /// Poll a video generation job. Only the submitting account can read it;
    /// another account's job id returns 404 rather than 403.
    pub async fn get_openrouter_video(&self, job_id: &str) -> Result<OpenRouterVideoJob, Error> {
        let path = format!("/agent-integrations/openrouter/videos/{}", enc(job_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Download the rendered video. `index` selects one output when the job
    /// produced several.
    pub async fn openrouter_video_content(
        &self,
        job_id: &str,
        index: Option<u32>,
    ) -> Result<Vec<u8>, Error> {
        let path = format!(
            "/agent-integrations/openrouter/videos/{}/content",
            enc(job_id)
        );
        let query = [("index", index.map(|i| i.to_string()))];
        self.bytes_query(Method::GET, &path, &query).await
    }
}
