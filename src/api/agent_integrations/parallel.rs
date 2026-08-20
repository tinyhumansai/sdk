//! Parallel web research: chat, search, extract, enrich, datasets, tasks.

use super::AgentIntegrationsApi;
use crate::{enc, Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelChatModel {
    Speed,
    Lite,
    Base,
    Core,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParallelMessage {
    pub role: ParallelRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelChatRequest {
    pub model: ParallelChatModel,
    pub messages: Vec<ParallelMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelChatResponse {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub usage: Option<Value>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ParallelSearchMode {
    OneShot,
    Agentic,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelSearchRequest {
    pub objective: String,
    pub search_queries: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ParallelSearchMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpts: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelSearchResult {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub excerpts: Vec<String>,
    #[serde(default)]
    pub publish_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelSearchResponse {
    #[serde(default)]
    pub results: Vec<ParallelSearchResult>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelExtractRequest {
    pub urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ParallelExtractResult {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub excerpts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelExtractResponse {
    #[serde(default)]
    pub results: Vec<ParallelExtractResult>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelProcessor {
    Lite,
    Base,
    Core,
    Ultra,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelResearchRequest {
    pub input: Value,
    pub processor: ParallelProcessor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelEnrichRequest {
    pub input: Value,
    pub processor: ParallelProcessor,
    pub output_schema: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelRunResponse {
    #[serde(default, alias = "id")]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParallelMatchCondition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelDatasetGenerator {
    Preview,
    Base,
    Core,
    Pro,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelDatasetRequest {
    pub objective: String,
    pub entity_type: String,
    pub match_conditions: Vec<ParallelMatchCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<ParallelDatasetGenerator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelDatasetResponse {
    #[serde(default, alias = "id")]
    pub findall_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub matches: Vec<Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
}

impl AgentIntegrationsApi<'_> {
    /// Chat with the web via Parallel Chat Completions.
    pub async fn parallel_chat(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelChatResponse, Error> {
        self.post("/agent-integrations/parallel/chat", request)
            .await
    }

    /// Generate a web dataset via Parallel FindAll.
    pub async fn parallel_dataset(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelDatasetResponse, Error> {
        self.post("/agent-integrations/parallel/dataset", request)
            .await
    }

    /// Get dataset (FindAll) run status.
    pub async fn get_parallel_dataset(
        &self,
        findall_id: &str,
    ) -> Result<ParallelDatasetResponse, Error> {
        let path = format!("/agent-integrations/parallel/dataset/{}", enc(findall_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Get dataset (FindAll) matched candidates snapshot.
    pub async fn get_parallel_dataset_result(
        &self,
        findall_id: &str,
    ) -> Result<ParallelDatasetResponse, Error> {
        let path = format!(
            "/agent-integrations/parallel/dataset/{}/result",
            enc(findall_id)
        );
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Enrich web data with a structured output schema (synchronous).
    pub async fn parallel_enrich(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelRunResponse, Error> {
        self.post("/agent-integrations/parallel/enrich", request)
            .await
    }

    /// Extract content from URLs via Parallel API.
    pub async fn parallel_extract(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelExtractResponse, Error> {
        self.post("/agent-integrations/parallel/extract", request)
            .await
    }

    /// Start a deep research task (Parallel Task API).
    pub async fn parallel_research(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelRunResponse, Error> {
        self.post("/agent-integrations/parallel/research", request)
            .await
    }

    /// Get deep research run status.
    pub async fn get_parallel_research(&self, run_id: &str) -> Result<ParallelRunResponse, Error> {
        let path = format!("/agent-integrations/parallel/research/{}", enc(run_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Block on a deep research run until completion.
    pub async fn get_parallel_research_result(
        &self,
        run_id: &str,
        query: &[QueryParam],
    ) -> Result<ParallelRunResponse, Error> {
        let path = format!(
            "/agent-integrations/parallel/research/{}/result",
            enc(run_id)
        );
        self.send(Method::GET, &path, query, None, true).await
    }

    /// Web search via Parallel API.
    pub async fn parallel_search(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelSearchResponse, Error> {
        self.post("/agent-integrations/parallel/search", request)
            .await
    }
}
