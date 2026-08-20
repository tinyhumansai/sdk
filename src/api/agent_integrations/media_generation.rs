//! GMI image and video generation.

use super::AgentIntegrationsApi;
use crate::{enc, Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_images: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoGenerationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    #[serde(rename = "type", default)]
    pub kind: String,
    pub url: String,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub media: Vec<MediaItem>,
    #[serde(default)]
    pub cost_usd: f64,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaModel {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub capabilities: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MediaModelsResponse {
    #[serde(default)]
    pub models: Vec<MediaModel>,
}

impl AgentIntegrationsApi<'_> {
    /// Generate or edit an image via GMI (Seedream / SeedEdit).
    pub async fn media_generation_images(
        &self,
        request: &impl Serialize,
    ) -> Result<MediaResponse, Error> {
        self.post("/agent-integrations/media-generation/images", request)
            .await
    }

    /// List curated media-generation models.
    pub async fn list_media_generation_models(
        &self,
        query: &[QueryParam],
    ) -> Result<MediaModelsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/media-generation/models",
                query,
                None,
                true,
            )
            .await
    }

    /// Poll a media-generation request.
    pub async fn get_media_generation_request(
        &self,
        request_id: &str,
    ) -> Result<MediaResponse, Error> {
        let path = format!(
            "/agent-integrations/media-generation/requests/{}",
            enc(request_id)
        );
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Generate a video via GMI (Seedance / Veo).
    pub async fn media_generation_videos(
        &self,
        request: &impl Serialize,
    ) -> Result<MediaResponse, Error> {
        self.post("/agent-integrations/media-generation/videos", request)
            .await
    }
}
