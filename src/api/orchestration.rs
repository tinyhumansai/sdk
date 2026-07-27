use super::types::{ContinueRunRequest, OrchestrationEventRequest, SubmitWorldDiffRequest};
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

pub struct OrchestrationApi<'a> {
    http: &'a HttpClient,
}
impl<'a> OrchestrationApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }
    async fn body<T: Serialize>(&self, path: &str, body: &T) -> Result<Value, Error> {
        let body = serde_json::to_value(body).expect("request is serializable");
        self.http
            .send(Method::POST, path, &[], Some(&body), true)
            .await
    }
    pub async fn events(&self, request: &OrchestrationEventRequest) -> Result<Value, Error> {
        self.body("/orchestration/v1/events", request).await
    }
    pub async fn run(&self, request: &RunRequest) -> Result<Value, Error> {
        self.body("/orchestration/v1/run", request).await
    }
    pub async fn continue_run(&self, request: &ContinueRunRequest) -> Result<Value, Error> {
        self.body("/orchestration/v1/run/continue", request).await
    }
    pub async fn list_sessions(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/orchestration/v1/sessions", query, None, true)
            .await
    }
    pub async fn session_messages(&self, id: &str, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/orchestration/v1/sessions/{}/messages", enc(id)),
                query,
                None,
                true,
            )
            .await
    }
    pub async fn session_state(&self, id: &str) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/orchestration/v1/sessions/{}/state", enc(id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn world_diff(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/orchestration/v1/world-diff",
                query,
                None,
                true,
            )
            .await
    }
    pub async fn submit_world_diff(
        &self,
        request: &SubmitWorldDiffRequest,
    ) -> Result<Value, Error> {
        self.body("/orchestration/v1/world-diff", request).await
    }
}
