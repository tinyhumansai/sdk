//! Typed Medulla harness and workflow events over Socket.IO.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::api::medulla::{EventEnvelope, WorkflowDescriptor};
use crate::socket::{SocketConnection, SocketEvent};
use crate::Error;

/// An agent identity a harness can execute for Medulla.
///
/// `id` is the only routing requirement. Known provenance fields are typed and
/// newer descriptor fields are retained in `extra`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDescriptor {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Client → server roster advertisement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterAgents {
    pub agents: Vec<AgentDescriptor>,
}

/// Client → server workflow catalog advertisement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterWorkflows {
    pub workflows: Vec<WorkflowDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

/// Open harness-v2 envelope streamed during a delegated task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessEnvelope {
    pub kind: String,
    #[serde(flatten)]
    pub fields: Map<String, Value>,
}

/// Client → server task progress frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEnvelope {
    pub task_id: String,
    pub envelope: HarnessEnvelope,
}

/// Token usage reported when a delegated task settles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// Client → server terminal delegated-task result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResult {
    pub task_id: String,
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<TaskUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
}

/// Client → server capability-probe response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesResult {
    pub probe_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Value>,
}

/// Workflow catalog operation requested by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRequestOp {
    Get,
    NodeKinds,
    Runs,
    Copilot,
}

/// Server → client request for workflow data or a copilot authoring turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRequest {
    pub request_id: String,
    pub op: WorkflowRequestOp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

/// Client → server workflow round-trip result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowResult {
    pub request_id: String,
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Server → client delegated-task start frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRun {
    pub task_id: String,
    pub cycle_id: String,
    pub instruction: String,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
    pub timeout_ms: u64,
}

/// Server → client follow-up input for a running task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSend {
    pub task_id: String,
    pub input: String,
}

/// Server → client delegated-task cancellation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskAbort {
    pub task_id: String,
}

/// Server → client capability probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesRequest {
    pub probe_id: String,
    pub agent_id: String,
}

/// Every typed Medulla event emitted by the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum MedullaServerEvent {
    TaskRun(TaskRun),
    TaskSend(TaskSend),
    TaskAbort(TaskAbort),
    CapabilitiesRequest(CapabilitiesRequest),
    WorkflowRequest(WorkflowRequest),
    /// Same envelope delivered by the reconnecting HTTP SSE session stream.
    SessionEvent(EventEnvelope),
}

impl SocketEvent {
    /// Decode a Medulla event, or return `None` for any other socket family.
    ///
    /// Generic consumers still receive every non-Medulla event unchanged.
    pub fn decode_medulla(&self) -> Result<Option<MedullaServerEvent>, Error> {
        let decoded = match self.name.as_str() {
            "medulla:task_run" => MedullaServerEvent::TaskRun(self.decode()?),
            "medulla:task_send" => MedullaServerEvent::TaskSend(self.decode()?),
            "medulla:task_abort" => MedullaServerEvent::TaskAbort(self.decode()?),
            "medulla:capabilities_request" => {
                MedullaServerEvent::CapabilitiesRequest(self.decode()?)
            }
            "medulla:workflow_request" => MedullaServerEvent::WorkflowRequest(self.decode()?),
            "medulla:event" => MedullaServerEvent::SessionEvent(self.decode()?),
            _ => return Ok(None),
        };
        Ok(Some(decoded))
    }
}

impl SocketConnection {
    /// Advertise the agents this connection can execute.
    pub async fn register_medulla_agents(&self, agents: Vec<AgentDescriptor>) -> Result<(), Error> {
        self.emit("medulla:register_agents", &RegisterAgents { agents })
            .await
    }

    /// Advertise saved workflows owned by this connection.
    pub async fn register_medulla_workflows(
        &self,
        workflows: Vec<WorkflowDescriptor>,
        agent_id: Option<String>,
    ) -> Result<(), Error> {
        self.emit(
            "medulla:register_workflows",
            &RegisterWorkflows {
                workflows,
                agent_id,
            },
        )
        .await
    }

    /// Stream one delegated-task envelope to the backend.
    pub async fn send_medulla_task_envelope(&self, payload: &TaskEnvelope) -> Result<(), Error> {
        self.emit("medulla:task_envelope", payload).await
    }

    /// Settle one delegated task.
    pub async fn send_medulla_task_result(&self, payload: &TaskResult) -> Result<(), Error> {
        self.emit("medulla:task_result", payload).await
    }

    /// Answer a Medulla capability probe.
    pub async fn send_medulla_capabilities(
        &self,
        payload: &CapabilitiesResult,
    ) -> Result<(), Error> {
        self.emit("medulla:capabilities_result", payload).await
    }

    /// Answer a Medulla workflow round trip.
    pub async fn send_medulla_workflow_result(
        &self,
        payload: &WorkflowResult,
    ) -> Result<(), Error> {
        self.emit("medulla:workflow_result", payload).await
    }
}
