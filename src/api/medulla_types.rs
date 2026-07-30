//! Response models for the `/medulla/v1` routes.
//!
//! The deployed OpenAPI document lists every route in this namespace but does
//! not document their response schemas, which is why these operations returned
//! [`super::types::DynamicResponse`] until now. The models here are ported from
//! the Medulla TUI's own client, where they have run against the live backend.
//!
//! They are deliberately forward-compatible: every optional field carries
//! `#[serde(default)]`, unknown fields are ignored rather than rejected, and the
//! open enums ([`SessionStatus`], [`Role`], [`EventKind`]) keep a catch-all
//! variant. A backend that grows a field or an enum case therefore does not
//! break a deployed SDK, which is the property `DynamicResponse` was protecting.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Sessions
// ---------------------------------------------------------------------------

/// Session lifecycle status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Session is currently processing or ready for input.
    Active,
    /// Session exists but has no active cycle.
    Idle,
    /// Session was archived and is no longer active.
    Archived,
    /// A status not yet modelled by this SDK.
    #[serde(other)]
    Other,
}

/// Message author role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Message authored by the user.
    User,
    /// Message authored by the assistant.
    Assistant,
    /// A role not yet modelled by this SDK.
    #[serde(other)]
    Other,
}

/// Result of creating a session (`POST /medulla/v1/sessions`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionCreated {
    /// Identifier assigned to the new session.
    pub session_id: String,
}

/// Item in the session list (`GET /medulla/v1/sessions`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    /// Stable session identifier.
    pub session_id: String,
    /// Optional human-facing session title.
    #[serde(default)]
    pub title: Option<String>,
    /// Unix timestamp of the most recent activity.
    #[serde(default)]
    pub last_active_at: Option<i64>,
    /// Current lifecycle state.
    pub status: SessionStatus,
    /// Most recent persisted event sequence.
    #[serde(default)]
    pub last_seq: Option<i64>,
}

/// Detailed session state (`GET /medulla/v1/sessions/{id}`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetail {
    /// Stable session identifier.
    pub session_id: String,
    /// Current lifecycle state.
    pub status: SessionStatus,
    /// Identifier of the most recently started cycle.
    #[serde(default)]
    pub last_cycle_id: Option<String>,
    /// Most recent persisted message sequence.
    #[serde(default)]
    pub last_seq: Option<i64>,
    /// Most recent event-stream sequence.
    #[serde(default)]
    pub event_seq: Option<i64>,
}

/// Result of archiving a session (`DELETE /medulla/v1/sessions/{id}`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionArchived {
    /// Identifier of the archived session.
    pub session_id: String,
    /// Resulting lifecycle state.
    pub status: SessionStatus,
}

/// Result of `POST /medulla/v1/sessions/{id}/messages`.
///
/// The async (202) response carries `cycleId`/`seq`; the synchronous
/// (`?sync=1`) response additionally carries `reply`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResult {
    /// Cognitive cycle created for the message.
    pub cycle_id: String,
    /// Persisted message sequence.
    pub seq: i64,
    /// Final assistant response when synchronous mode was requested.
    #[serde(default)]
    pub reply: Option<String>,
}

/// A replayed message (`GET /medulla/v1/sessions/{id}/messages`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Monotonic message sequence within the session.
    pub seq: i64,
    /// Author of the message.
    pub role: Role,
    /// Message text.
    pub body: String,
    /// Optional Unix timestamp supplied by the backend.
    #[serde(default)]
    pub ts: Option<i64>,
    /// Cycle that produced or consumed the message.
    #[serde(default)]
    pub cycle_id: Option<String>,
}

/// Result of `POST /medulla/v1/sessions/{id}/abort`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortResult {
    /// Identifier of the targeted session.
    pub session_id: String,
    /// Whether an active cycle was aborted.
    pub aborted: bool,
}

// ---------------------------------------------------------------------------
// Event stream
// ---------------------------------------------------------------------------

/// Envelope wrapping every event on the session stream.
///
/// `event` retains the raw JSON payload; [`EventEnvelope::kind`] parses it into
/// a typed [`EventKind`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope {
    /// Optional persisted sequence; streaming-only events may omit it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    /// Event timestamp supplied by the backend.
    pub at: u64,
    /// Session that owns the event.
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// Cycle associated with the event, when applicable.
    #[serde(rename = "cycleId", default, skip_serializing_if = "Option::is_none")]
    pub cycle_id: Option<String>,
    /// Raw event payload; shape depends on `event.kind`.
    pub event: Value,
}

impl EventEnvelope {
    /// Parse the raw `event` payload into a typed [`EventKind`].
    pub fn kind(&self) -> EventKind {
        EventKind::from_value(&self.event)
    }
}

/// Typed event payload parsed from [`EventEnvelope::event`].
///
/// `Unknown` preserves the raw value, so an event kind this SDK does not model
/// still reaches the caller intact.
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind {
    /// A user message was recorded.
    User {
        /// Recorded user text.
        body: String,
    },
    /// The assistant produced a final message.
    Assistant {
        /// Final assistant text.
        body: String,
    },
    /// A cognitive cycle started.
    CycleStart {
        /// Backend cycle identifier.
        cycle_id: Option<String>,
    },
    /// A cognitive cycle ended.
    CycleEnd {
        /// Backend cycle identifier.
        cycle_id: Option<String>,
        /// Number of orchestration passes consumed.
        pass_count: Option<u64>,
        /// Wall-clock duration reported by the backend.
        duration_ms: Option<u64>,
        /// Whether the cycle terminated with an error.
        error: Option<bool>,
    },
    /// An error occurred during a cycle.
    Error {
        /// Component that reported the error.
        source: String,
        /// Human-readable error detail.
        message: String,
    },
    /// Streaming assistant token delta (unpersisted, no seq).
    AssistantDelta {
        /// Newly emitted assistant text.
        delta: String,
    },
    /// Streaming reasoning token delta (unpersisted, no seq).
    ReasoningDelta {
        /// Newly emitted reasoning text.
        delta: String,
    },
    /// Streaming tool-call delta (unpersisted); raw payload preserved.
    ToolCallDelta {
        /// Raw tool-call delta payload.
        value: Value,
    },
    /// An event kind not modelled by this SDK; raw payload preserved.
    Unknown(Value),
}

impl EventKind {
    /// Parse a raw event object (`{ "kind": ..., ... }`) into a typed kind.
    pub fn from_value(value: &Value) -> EventKind {
        let kind = value
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let text = |key: &str| {
            value
                .get(key)
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned()
        };
        let opt_text = |key: &str| value.get(key).and_then(Value::as_str).map(str::to_owned);
        let opt_u64 = |key: &str| value.get(key).and_then(Value::as_u64);
        match kind {
            "user" => EventKind::User { body: text("body") },
            "assistant" => EventKind::Assistant { body: text("body") },
            "cycle_start" => EventKind::CycleStart {
                cycle_id: opt_text("cycleId"),
            },
            "cycle_end" => EventKind::CycleEnd {
                cycle_id: opt_text("cycleId"),
                pass_count: opt_u64("passCount"),
                duration_ms: opt_u64("durationMs"),
                error: value.get("error").and_then(Value::as_bool),
            },
            "error" => EventKind::Error {
                source: text("source"),
                message: text("message"),
            },
            "assistant_delta" => EventKind::AssistantDelta {
                delta: text("delta"),
            },
            "reasoning_delta" => EventKind::ReasoningDelta {
                delta: text("delta"),
            },
            "tool_call_delta" => EventKind::ToolCallDelta {
                value: value.clone(),
            },
            _ => EventKind::Unknown(value.clone()),
        }
    }
}

// ---------------------------------------------------------------------------
// Worker roster
// ---------------------------------------------------------------------------

/// Client-safe harness budget advertised by a connected worker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterBudget {
    /// Provider that meters the budget.
    pub provider: String,
    /// Provider-defined accounting window.
    pub window: String,
    /// Tokens still available in the current window, when known.
    #[serde(default)]
    pub remaining_tokens: Option<u64>,
    /// Total token allowance for the current window, when known.
    #[serde(default)]
    pub limit_tokens: Option<u64>,
    /// Unix timestamp after which a depleted budget becomes usable.
    #[serde(default)]
    pub cooldown_until: Option<u64>,
    /// Origin of the budget observation.
    pub source: String,
}

/// Connected worker returned by `GET /medulla/v1/roster`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterWorker {
    /// Stable worker identifier in the manager registry.
    pub registry_id: String,
    /// Operator-facing worker label.
    pub label: String,
    /// Operator-facing description of the worker.
    pub description: String,
    /// Current manager-reported availability.
    pub availability: String,
    /// Harness implementation serving the worker.
    #[serde(default)]
    pub harness: Option<String>,
    /// Network address advertised by the worker.
    #[serde(default)]
    pub address: Option<String>,
    /// Provider-specific worker handle.
    #[serde(default)]
    pub handle: Option<String>,
    /// Peer identifier used for wallet-backed routing.
    #[serde(default)]
    pub wallet_peer_id: Option<String>,
    /// Logical CPU capacity visible to the worker.
    #[serde(default)]
    pub cpu_cores: Option<u32>,
    /// Total host memory in bytes.
    #[serde(default)]
    pub total_mem_bytes: Option<u64>,
    /// Currently available host memory in bytes.
    #[serde(default)]
    pub available_mem_bytes: Option<u64>,
    /// Primary IPv4 address selected by the manager.
    #[serde(default)]
    pub primary_ipv4: Option<String>,
    /// Routing and discovery labels attached to the worker.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Whether the worker is currently selected for dispatch.
    pub selected: bool,
    /// Provider-specific capability document.
    #[serde(default)]
    pub capabilities: Option<Value>,
    /// Client-safe budgets reported for this worker.
    #[serde(default)]
    pub budgets: Vec<RosterBudget>,
}

/// Response payload of `GET /medulla/v1/roster`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Roster {
    /// Workers currently known to the manager.
    #[serde(default)]
    pub workers: Vec<RosterWorker>,
}

// ---------------------------------------------------------------------------
// Task program
// ---------------------------------------------------------------------------

/// Recurrence frequency for an operator-owned task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskRecurrenceFrequency {
    /// Repeat every day.
    Daily,
    /// Repeat every week.
    Weekly,
    /// Repeat every month.
    Monthly,
    /// Repeat after a custom number of days.
    EveryDays,
}

/// Optional recurrence rule attached to a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRecurrence {
    /// Schedule family used to calculate the next occurrence.
    pub frequency: TaskRecurrenceFrequency,
    /// Custom day interval used with [`TaskRecurrenceFrequency::EveryDays`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_days: Option<u32>,
    /// RFC 3339 timestamp for the next scheduled occurrence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_at: Option<String>,
}

/// External source identity attached to a synchronized task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSourceRef {
    /// Source provider name.
    pub provider: String,
    /// Stable item identifier assigned by the provider.
    pub source_id: String,
    /// Browser URL for the source item.
    #[serde(default)]
    pub url: Option<String>,
}

/// An operator-owned task in the backend program ledger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// Stable backend task identifier.
    pub id: String,
    /// Short operator-facing title.
    pub title: String,
    /// Detailed task description.
    pub description: String,
    /// Current task lifecycle state.
    pub status: super::medulla::TaskStatus,
    /// Recurrence rule, if the task repeats.
    #[serde(default)]
    pub recurrence: Option<TaskRecurrence>,
    /// External item that produced this task.
    #[serde(default)]
    pub source: Option<TaskSourceRef>,
    /// RFC 3339 creation timestamp.
    pub created_at: String,
    /// RFC 3339 timestamp of the latest update.
    pub updated_at: String,
    /// RFC 3339 timestamp of the latest source synchronization.
    #[serde(default)]
    pub last_synced_at: Option<String>,
    /// Backend-owned dispatch state.
    #[serde(default)]
    pub dispatch: Value,
}

/// GitHub issue state selected by a task source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GithubIssueState {
    /// Synchronize open issues.
    Open,
    /// Synchronize closed issues.
    Closed,
    /// Synchronize issues in either state.
    All,
}

/// A configured GitHub task source. Tokens never appear on this response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSource {
    /// Stable backend source identifier.
    pub id: String,
    /// Source provider name.
    pub provider: String,
    /// Whether periodic synchronization is enabled.
    pub enabled: bool,
    /// GitHub repository in `owner/name` form.
    pub repository: String,
    /// Issue state included by the source.
    pub state: GithubIssueState,
    /// Labels that issues must match.
    #[serde(default)]
    pub labels: Vec<String>,
    /// Additional provider-defined search filter.
    #[serde(default)]
    pub filter: Option<String>,
    /// Whether a provider token is configured; the token itself is never returned.
    pub has_token: bool,
    /// RFC 3339 creation timestamp.
    pub created_at: String,
    /// RFC 3339 timestamp of the latest update.
    pub updated_at: String,
}

/// Result of synchronizing one GitHub source into the task ledger.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TaskSourceSyncResult {
    /// Number of tasks created.
    pub added: u64,
    /// Number of existing tasks updated.
    pub updated: u64,
    /// Number of source items already current.
    pub unchanged: u64,
    /// Non-fatal provider errors encountered during synchronization.
    #[serde(default)]
    pub errors: Vec<String>,
}

/// Result shared by the task and task-source delete endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Deleted {
    /// Whether the requested resource existed and was removed.
    pub deleted: bool,
}

// ---------------------------------------------------------------------------
// Envelope wrappers
// ---------------------------------------------------------------------------
//
// A few task-program routes nest their payload one level deeper than the
// `{success, data}` envelope the transport already unwraps — `data` is
// `{"tasks": [...]}` rather than the array itself. These wrappers keep that
// detail inside the SDK so callers receive the payload directly.

#[derive(Debug, Deserialize)]
pub(crate) struct TasksPayload {
    #[serde(default)]
    pub(crate) tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TaskPayload {
    pub(crate) task: Task,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TaskSourcesPayload {
    #[serde(default)]
    pub(crate) sources: Vec<TaskSource>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TaskSourcePayload {
    pub(crate) source: TaskSource,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TaskSourceSyncPayload {
    pub(crate) result: TaskSourceSyncResult,
}

/// Worker routing strategy configured on the backend
/// (`GET`/`PUT /medulla/v1/routing/strategy`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RoutingStrategy {
    /// Backend-defined strategy name, in camelCase.
    ///
    /// Left as an open string: the set of strategies is owned by the host that
    /// acts on it, and an unrecognized value should degrade to "no preference"
    /// in that host rather than fail to decode here.
    #[serde(default)]
    pub strategy: Option<String>,
}
