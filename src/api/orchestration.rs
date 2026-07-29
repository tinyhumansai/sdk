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
    /// Client-side tools offered to the orchestrator. Leave empty for a
    /// tool-less run, which returns a final reply rather than a tool loop.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<RunOptions>,
}

/// A client-side tool definition offered to a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDef {
    /// Name exposed to the orchestrator.
    pub name: String,
    /// Human-readable purpose used for tool selection.
    pub description: String,
    /// JSON-Schema object describing the tool parameters.
    pub parameters: Value,
}

/// A tool call requested by the orchestrator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    /// Identifier used to correlate the eventual result.
    pub id: String,
    /// Requested tool name.
    pub name: String,
    /// JSON arguments supplied by the orchestrator.
    #[serde(default)]
    pub args: Value,
}

/// A tool result fed back via `run/continue`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult {
    /// Identifier of the tool call being answered.
    pub id: String,
    /// Whether the tool completed successfully.
    pub ok: bool,
    /// Successful JSON result, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Failure detail, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Orchestration behaviour overrides (`options.config`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunConfig {
    /// Maximum orchestration passes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_passes: Option<u32>,
    /// Maximum execution steps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<u32>,
    /// Maximum delegation depth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<u32>,
    /// Context window used for prompt budgeting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_window_tokens: Option<u32>,
    /// Verification policy or mode understood by the backend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<String>,
}

/// Resource ceilings for a run (`options.limits`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunLimits {
    /// Maximum tasks that may execute concurrently.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<u32>,
    /// Aggregate token ceiling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// Absolute or relative deadline in milliseconds, per backend contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_ms: Option<u64>,
    /// Maximum child tasks created by one delegate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tasks_per_delegate: Option<u32>,
    /// Maximum nested delegation depth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<u32>,
}

/// The `options` object of a run request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunOptions {
    /// Prompt sections replaced for this run, keyed by backend section name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_overrides: Option<std::collections::BTreeMap<String, String>>,
    /// Optional orchestration behaviour overrides.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<RunConfig>,
    /// Optional resource ceilings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<RunLimits>,
    /// Authored workspace profiles for the directories this cycle works over.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_profiles: Option<Vec<super::medulla::WorkspaceProfile>>,
}

/// Final reply from a tool-less run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunReply {
    /// Final assistant response.
    pub reply: String,
    /// Number of orchestration passes consumed.
    #[serde(default)]
    pub pass_count: Option<u32>,
    /// Backend-produced compact history records.
    #[serde(default)]
    pub compressed_history: Vec<Value>,
    /// Escalations recorded during the run.
    #[serde(default)]
    pub escalations: Vec<Value>,
    /// Session that owns the run.
    #[serde(default)]
    pub session_id: Option<String>,
    /// Final cycle identifier.
    #[serde(default)]
    pub cycle_id: Option<String>,
}

/// A single step of the client tool-loop, discriminated by `stop`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "stop", rename_all = "snake_case")]
pub enum LoopEvent {
    /// The orchestrator wants the client to run tools and continue.
    ToolUse {
        /// Cycle awaiting tool results.
        #[serde(rename = "cycleId")]
        cycle_id: String,
        /// Session that owns the cycle.
        #[serde(rename = "sessionId")]
        session_id: String,
        /// Calls the client must execute before continuing.
        #[serde(rename = "toolCalls", default)]
        tool_calls: Vec<ToolCall>,
    },
    /// The run finished with a final reply.
    End {
        /// Completed cycle identifier.
        #[serde(rename = "cycleId")]
        cycle_id: String,
        /// Session that owns the completed run.
        #[serde(rename = "sessionId")]
        session_id: String,
        /// Final assistant response.
        reply: String,
        /// Number of orchestration passes consumed.
        #[serde(rename = "passCount", default)]
        pass_count: Option<u32>,
        /// Backend-produced compact history records.
        #[serde(rename = "compressedHistory", default)]
        compressed_history: Vec<Value>,
        /// Escalations recorded during the run.
        #[serde(default)]
        escalations: Vec<Value>,
    },
    /// Long-poll returned without progress; poll `run/continue` again.
    Pending {
        /// Cycle still being polled.
        #[serde(rename = "cycleId")]
        cycle_id: String,
        /// Session that owns the cycle.
        #[serde(rename = "sessionId")]
        session_id: String,
    },
    /// The run errored.
    Error {
        /// Failed cycle identifier.
        #[serde(rename = "cycleId")]
        cycle_id: String,
        /// Session that owns the failed cycle.
        #[serde(rename = "sessionId")]
        session_id: String,
        /// Structured backend error payload.
        error: Value,
    },
}

/// Outcome of [`OrchestrationApi::run`].
///
/// The backend returns different shapes depending on whether tools were
/// offered, discriminated by the presence of a `stop` field.
#[derive(Debug, Clone, PartialEq)]
pub enum RunResult {
    /// Completed tool-less run.
    Reply(Box<RunReply>),
    /// Next state in a client-managed tool loop.
    Loop(Box<LoopEvent>),
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
    async fn body<B: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
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
    /// Start a one-shot orchestration run.
    ///
    /// Without tools the backend returns a final [`RunReply`]; with tools it
    /// returns the first [`LoopEvent`] of a client-driven tool loop. The two are
    /// told apart by the presence of a `stop` field.
    pub async fn run(&self, request: &RunRequest) -> Result<RunResult, Error> {
        let value: Value = self.body("/orchestration/v1/run", request).await?;
        if value.get("stop").is_some() {
            Ok(RunResult::Loop(Box::new(serde_json::from_value(value)?)))
        } else {
            Ok(RunResult::Reply(Box::new(serde_json::from_value(value)?)))
        }
    }

    /// Continue a tool-loop run.
    ///
    /// Pass an empty `tool_results` to poll a run that is still pending.
    pub async fn continue_run(&self, request: &ContinueRunRequest) -> Result<LoopEvent, Error> {
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
