//! Tenor GIF search.

use super::AgentIntegrationsApi;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenorSearchRequest {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(
        default,
        rename = "contentFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TenorGif {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub media: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TenorSearchResponse {
    #[serde(default)]
    pub results: Vec<TenorGif>,
    #[serde(default)]
    pub next: Option<String>,
}

impl AgentIntegrationsApi<'_> {
    /// Search for GIFs via the Tenor API.
    pub async fn tenor_search(
        &self,
        request: &impl Serialize,
    ) -> Result<TenorSearchResponse, Error> {
        self.post("/agent-integrations/tenor/search", request).await
    }
}
