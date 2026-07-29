//! Durable Medulla sessions, the worker roster, and the operator task program.
//!
//! Response models live in [`super::medulla_types`] and are re-exported here.
//! They replace the [`super::types::DynamicResponse`] this namespace used to
//! return; see that module for why hand-written models are sound for routes the
//! deployed OpenAPI document does not give response schemas for.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::medulla_types::{
    TaskPayload, TaskSourcePayload, TaskSourceSyncPayload, TaskSourcesPayload, TasksPayload,
};
use crate::{enc, Error, HttpClient, QueryParam};

pub use super::medulla_types::{
    AbortResult, Deleted, EventEnvelope, EventKind, GithubIssueState, Message, Role, Roster,
    RosterBudget, RosterWorker, RoutingStrategy, SendResult, SessionArchived, SessionCreated,
    SessionDetail, SessionStatus, SessionSummary, Task, TaskRecurrence, TaskRecurrenceFrequency,
    TaskSource, TaskSourceRef, TaskSourceSyncResult,
};

/// One workspace root's authored `MEDULLA.md`, attached to a session mint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceProfile {
    /// The workspace/repo path this profile describes.
    pub workspace: String,
    /// Verbatim `MEDULLA.md` contents; the backend distils it.
    pub medulla_md: String,
}

/// Request body for `POST /medulla/v1/sessions`.
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

/// Request body for `POST /medulla/v1/tasks`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// Initial recurrence rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<TaskRecurrence>,
}

/// Lifecycle state of an operator-owned task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    /// Ready for work.
    Open,
    /// Work has started.
    InProgress,
    /// Work completed successfully.
    Done,
    /// Work was intentionally abandoned.
    Cancelled,
}

/// Patch body for `PATCH /medulla/v1/tasks/{id}`.
///
/// Every field is optional and omitted fields are left untouched. `recurrence`
/// is deliberately doubly-optional: `None` omits the key, `Some(None)` sends
/// JSON `null` to clear an existing rule, and `Some(Some(rule))` replaces it.
/// A single `Option` could not express the clear.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Option<TaskRecurrence>>,
}

/// Request body for `POST /medulla/v1/tasks/sources`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskSourceRequest {
    /// GitHub repository in `owner/name` form.
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GithubIssueState>,
    /// Labels that issues must match.
    ///
    /// An `Option` rather than a bare `Vec` so an explicitly empty list — "match
    /// no labels" — stays distinguishable from an omitted field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Provider token to store; write-only, never returned on a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed client for the `/medulla/v1/*` routes.
pub struct MedullaApi<'a> {
    http: &'a HttpClient,
}

impl<'a> MedullaApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Send a JSON body and decode the unwrapped payload.
    async fn body<B: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
        body: &B,
    ) -> Result<T, Error> {
        let body = serde_json::to_value(body).expect("request is serializable");
        self.http
            .send_typed(method, path, query, Some(&body), true)
            .await
    }

    /// Send a request without a body and decode the unwrapped payload.
    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
    ) -> Result<T, Error> {
        self.http.send_typed(method, path, query, None, true).await
    }

    // --- Roster and routing ----------------------------------------------

    /// Read the connected worker roster.
    pub async fn roster(&self) -> Result<Vec<RosterWorker>, Error> {
        let roster: Roster = self.get(Method::GET, "/medulla/v1/roster", &[]).await?;
        Ok(roster.workers)
    }

    /// Read the backend's configured worker routing strategy.
    pub async fn routing_strategy(&self) -> Result<RoutingStrategy, Error> {
        self.get(Method::GET, "/medulla/v1/routing/strategy", &[])
            .await
    }

    /// Persist the operator's worker routing strategy.
    pub async fn set_routing_strategy(&self, strategy: &str) -> Result<RoutingStrategy, Error> {
        self.body(
            Method::PUT,
            "/medulla/v1/routing/strategy",
            &[],
            &serde_json::json!({ "strategy": strategy }),
        )
        .await
    }

    // --- Sessions ---------------------------------------------------------

    /// List durable sessions.
    pub async fn list_sessions(&self, query: &[QueryParam]) -> Result<Vec<SessionSummary>, Error> {
        self.get(Method::GET, "/medulla/v1/sessions", query).await
    }

    /// Create a durable session.
    pub async fn create_session(
        &self,
        request: &CreateSessionRequest,
    ) -> Result<SessionCreated, Error> {
        self.body(Method::POST, "/medulla/v1/sessions", &[], request)
            .await
    }

    /// Fetch one session's state.
    pub async fn get_session(&self, id: &str) -> Result<SessionDetail, Error> {
        self.get(
            Method::GET,
            &format!("/medulla/v1/sessions/{}", enc(id)),
            &[],
        )
        .await
    }

    /// Archive a session.
    pub async fn delete_session(&self, id: &str) -> Result<SessionArchived, Error> {
        self.get(
            Method::DELETE,
            &format!("/medulla/v1/sessions/{}", enc(id)),
            &[],
        )
        .await
    }

    /// Abort the session's running cycle.
    pub async fn abort_session(&self, id: &str) -> Result<AbortResult, Error> {
        self.get(
            Method::POST,
            &format!("/medulla/v1/sessions/{}/abort", enc(id)),
            &[],
        )
        .await
    }

    /// Replay a session's persisted events.
    pub async fn session_events(
        &self,
        id: &str,
        query: &[QueryParam],
    ) -> Result<Vec<EventEnvelope>, Error> {
        self.get(
            Method::GET,
            &format!("/medulla/v1/sessions/{}/events", enc(id)),
            query,
        )
        .await
    }

    /// Replay a session's messages.
    pub async fn session_messages(
        &self,
        id: &str,
        query: &[QueryParam],
    ) -> Result<Vec<Message>, Error> {
        self.get(
            Method::GET,
            &format!("/medulla/v1/sessions/{}/messages", enc(id)),
            query,
        )
        .await
    }

    /// Post a message to a session.
    ///
    /// `sync` selects the blocking form. The backend spells this flag `1`/`0`,
    /// not `true`/`false`, so it is serialized explicitly here rather than via
    /// `bool::to_string`.
    pub async fn send_session_message(
        &self,
        id: &str,
        body: &str,
        sync: Option<bool>,
    ) -> Result<SendResult, Error> {
        let sync = sync.map(|sync| if sync { "1".to_owned() } else { "0".to_owned() });
        self.body(
            Method::POST,
            &format!("/medulla/v1/sessions/{}/messages", enc(id)),
            &[("sync", sync)],
            &serde_json::json!({ "body": body }),
        )
        .await
    }

    // --- Task program -----------------------------------------------------

    /// List the operator-owned task ledger.
    pub async fn list_tasks(&self, query: &[QueryParam]) -> Result<Vec<Task>, Error> {
        let payload: TasksPayload = self.get(Method::GET, "/medulla/v1/tasks", query).await?;
        Ok(payload.tasks)
    }

    /// Create an operator-owned task.
    pub async fn create_task(&self, request: &CreateTaskRequest) -> Result<Task, Error> {
        let payload: TaskPayload = self
            .body(Method::POST, "/medulla/v1/tasks", &[], request)
            .await?;
        Ok(payload.task)
    }

    /// Update an operator-owned task.
    pub async fn update_task(&self, id: &str, request: &UpdateTaskRequest) -> Result<Task, Error> {
        let payload: TaskPayload = self
            .body(
                Method::PATCH,
                &format!("/medulla/v1/tasks/{}", enc(id)),
                &[],
                request,
            )
            .await?;
        Ok(payload.task)
    }

    /// Delete an operator-owned task.
    pub async fn delete_task(&self, id: &str) -> Result<bool, Error> {
        let deleted: Deleted = self
            .get(
                Method::DELETE,
                &format!("/medulla/v1/tasks/{}", enc(id)),
                &[],
            )
            .await?;
        Ok(deleted.deleted)
    }

    /// List configured GitHub task sources.
    pub async fn list_task_sources(&self) -> Result<Vec<TaskSource>, Error> {
        let payload: TaskSourcesPayload = self
            .get(Method::GET, "/medulla/v1/tasks/sources", &[])
            .await?;
        Ok(payload.sources)
    }

    /// Configure a GitHub task source.
    pub async fn create_task_source(
        &self,
        request: &CreateTaskSourceRequest,
    ) -> Result<TaskSource, Error> {
        let payload: TaskSourcePayload = self
            .body(Method::POST, "/medulla/v1/tasks/sources", &[], request)
            .await?;
        Ok(payload.source)
    }

    /// Remove a configured GitHub task source.
    pub async fn delete_task_source(&self, id: &str) -> Result<bool, Error> {
        let deleted: Deleted = self
            .get(
                Method::DELETE,
                &format!("/medulla/v1/tasks/sources/{}", enc(id)),
                &[],
            )
            .await?;
        Ok(deleted.deleted)
    }

    /// Synchronize one GitHub source into the task ledger.
    pub async fn sync_task_source(&self, id: &str) -> Result<TaskSourceSyncResult, Error> {
        let payload: TaskSourceSyncPayload = self
            .get(
                Method::POST,
                &format!("/medulla/v1/tasks/sources/{}/sync", enc(id)),
                &[],
            )
            .await?;
        Ok(payload.result)
    }
}
