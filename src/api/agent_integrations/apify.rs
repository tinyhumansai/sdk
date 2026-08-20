//! Apify actor runs.

use super::AgentIntegrationsApi;
use crate::{enc, Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApifyRunRequest {
    pub actor_id: String,
    pub input: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_mbytes: Option<u32>,
}

/// Apify response fields vary by actor; stable run metadata is typed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApifyRunResponse {
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApifyResultsResponse {
    #[serde(default)]
    pub items: Vec<Value>,
    #[serde(default)]
    pub count: Option<u64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl AgentIntegrationsApi<'_> {
    /// Run an Apify actor.
    pub async fn run_apify_actor(
        &self,
        request: &impl Serialize,
    ) -> Result<ApifyRunResponse, Error> {
        self.post("/agent-integrations/apify/run", request).await
    }

    /// Get status of an Apify actor run.
    pub async fn get_apify_run(&self, run_id: &str) -> Result<ApifyRunResponse, Error> {
        let path = format!("/agent-integrations/apify/runs/{}", enc(run_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Get results from a completed Apify actor run.
    pub async fn get_apify_run_results(
        &self,
        run_id: &str,
        query: &[QueryParam],
    ) -> Result<ApifyResultsResponse, Error> {
        let path = format!("/agent-integrations/apify/runs/{}/results", enc(run_id));
        self.send(Method::GET, &path, query, None, true).await
    }
}
