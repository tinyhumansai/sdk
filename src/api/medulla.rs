use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::types::DynamicResponse;
use crate::{enc, Error, HttpClient, QueryParam};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceProfile {
    pub workspace: String,
    pub medulla_md: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_delegation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workspace_profiles: Vec<WorkspaceProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Open,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskSourceRequest {
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

pub struct MedullaApi<'a> {
    http: &'a HttpClient,
}

impl<'a> MedullaApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }
    async fn body<T: Serialize>(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
        body: &T,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(body).expect("request is serializable");
        self.http
            .send(method, path, query, Some(&body), true)
            .await
            .map(Into::into)
    }
    pub async fn roster(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/medulla/v1/roster", &[], None, true)
            .await
            .map(Into::into)
    }
    pub async fn workflows(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/medulla/v1/workflows", &[], None, true)
            .await
            .map(Into::into)
    }
    pub async fn routing_strategy(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/medulla/v1/routing/strategy", &[], None, true)
            .await
            .map(Into::into)
    }
    pub async fn set_routing_strategy(&self, strategy: &str) -> Result<DynamicResponse, Error> {
        self.body(
            Method::PUT,
            "/medulla/v1/routing/strategy",
            &[],
            &serde_json::json!({"strategy": strategy}),
        )
        .await
    }
    pub async fn list_sessions(&self, query: &[QueryParam]) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/medulla/v1/sessions", query, None, true)
            .await
            .map(Into::into)
    }
    pub async fn create_session(
        &self,
        request: &CreateSessionRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body(Method::POST, "/medulla/v1/sessions", &[], request)
            .await
    }
    pub async fn get_session(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/medulla/v1/sessions/{}", enc(id)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn delete_session(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::DELETE,
                &format!("/medulla/v1/sessions/{}", enc(id)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn abort_session(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::POST,
                &format!("/medulla/v1/sessions/{}/abort", enc(id)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn session_events(
        &self,
        id: &str,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/medulla/v1/sessions/{}/events", enc(id)),
                query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn session_messages(
        &self,
        id: &str,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/medulla/v1/sessions/{}/messages", enc(id)),
                query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn send_session_message(
        &self,
        id: &str,
        body: &str,
        sync: Option<bool>,
    ) -> Result<DynamicResponse, Error> {
        self.body(
            Method::POST,
            &format!("/medulla/v1/sessions/{}/messages", enc(id)),
            &[("sync", sync.map(|v| v.to_string()))],
            &serde_json::json!({"body": body}),
        )
        .await
    }
    pub async fn session_stream(
        &self,
        id: &str,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/medulla/v1/sessions/{}/stream", enc(id)),
                query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn list_tasks(&self, query: &[QueryParam]) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/medulla/v1/tasks", query, None, true)
            .await
            .map(Into::into)
    }
    pub async fn create_task(&self, request: &CreateTaskRequest) -> Result<DynamicResponse, Error> {
        self.body(Method::POST, "/medulla/v1/tasks", &[], request)
            .await
    }
    pub async fn update_task(
        &self,
        id: &str,
        request: &UpdateTaskRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body(
            Method::PATCH,
            &format!("/medulla/v1/tasks/{}", enc(id)),
            &[],
            request,
        )
        .await
    }
    pub async fn delete_task(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::DELETE,
                &format!("/medulla/v1/tasks/{}", enc(id)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn list_task_sources(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/medulla/v1/tasks/sources", &[], None, true)
            .await
            .map(Into::into)
    }
    pub async fn create_task_source(
        &self,
        request: &CreateTaskSourceRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body(Method::POST, "/medulla/v1/tasks/sources", &[], request)
            .await
    }
    pub async fn delete_task_source(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::DELETE,
                &format!("/medulla/v1/tasks/sources/{}", enc(id)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn sync_task_source(&self, id: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::POST,
                &format!("/medulla/v1/tasks/sources/{}/sync", enc(id)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
}
