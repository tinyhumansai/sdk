use super::types::{
    ContinueRunRequest, DynamicResponse, OrchestrationEventRequest, SubmitWorldDiffRequest,
};
use crate::{enc, Error, HttpClient, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunRequest {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

/// Current hosted steering directive and its recent predecessors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SteeringResponse {
    pub active: Option<ActiveSteeringDirective>,
    #[serde(default)]
    pub history: Vec<SteeringHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveSteeringDirective {
    pub directive: String,
    pub consumed_cycles: u32,
    pub max_cycles: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SteeringHistoryEntry {
    pub directive: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

pub struct OrchestrationApi<'a> {
    http: &'a HttpClient,
}
impl<'a> OrchestrationApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }
    async fn body<T: Serialize>(&self, path: &str, body: &T) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(body).expect("request is serializable");
        self.http
            .send_typed(Method::POST, path, &[], Some(&body), true)
            .await
    }
    pub async fn events(
        &self,
        request: &OrchestrationEventRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body("/orchestration/v1/events", request).await
    }
    pub async fn run(&self, request: &RunRequest) -> Result<DynamicResponse, Error> {
        self.body("/orchestration/v1/run", request).await
    }
    pub async fn continue_run(
        &self,
        request: &ContinueRunRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body("/orchestration/v1/run/continue", request).await
    }
    pub async fn list_sessions(&self, query: &[QueryParam]) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/orchestration/v1/sessions", query, None, true)
            .await
    }
    pub async fn session_messages(
        &self,
        id: &str,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                &format!("/orchestration/v1/sessions/{}/messages", enc(id)),
                query,
                None,
                true,
            )
            .await
    }
    pub async fn session_state(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                &format!("/orchestration/v1/sessions/{}/state", enc(id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn world_diff(&self, query: &[QueryParam]) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/orchestration/v1/world-diff",
                query,
                None,
                true,
            )
            .await
    }
    /// Get the active hosted steering directive and recent directive history.
    pub async fn steering(&self) -> Result<SteeringResponse, Error> {
        let value = self
            .http
            .send_typed(Method::GET, "/orchestration/v1/steering", &[], None, true)
            .await?;
        serde_json::from_value(value).map_err(Error::Decode)
    }
    pub async fn submit_world_diff(
        &self,
        request: &SubmitWorldDiffRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body("/orchestration/v1/world-diff", request).await
    }
}
