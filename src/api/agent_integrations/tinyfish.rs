//! TinyFish search, fetch, and agent runs.

use super::AgentIntegrationsApi;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishSearchRequest {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_thumbnail: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TinyFishSearchResult {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub snippet: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishSearchResponse {
    #[serde(default)]
    pub results: Vec<TinyFishSearchResult>,
    #[serde(default)]
    pub total_results: Option<u64>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishFetchRequest {
    pub urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_links: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TinyFishFetchedPage {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(default, rename = "imageLinks")]
    pub image_links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishFetchResponse {
    #[serde(default)]
    pub results: Vec<TinyFishFetchedPage>,
    #[serde(default)]
    pub errors: Vec<Value>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishAgentRunRequest {
    pub url: String,
    pub goal: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_config: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_vault: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credential_item_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishAgentRunResponse {
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
}

impl AgentIntegrationsApi<'_> {
    pub async fn tinyfish_search(
        &self,
        request: &impl Serialize,
    ) -> Result<TinyFishSearchResponse, Error> {
        self.post("/agent-integrations/tinyfish/search", request)
            .await
    }

    pub async fn tinyfish_fetch(
        &self,
        request: &impl Serialize,
    ) -> Result<TinyFishFetchResponse, Error> {
        self.post("/agent-integrations/tinyfish/fetch", request)
            .await
    }

    pub async fn tinyfish_agent_run(
        &self,
        request: &impl Serialize,
    ) -> Result<TinyFishAgentRunResponse, Error> {
        self.post("/agent-integrations/tinyfish/agent/run", request)
            .await
    }
}
