//! Composio connections, tools, and triggers.

use super::AgentIntegrationsApi;
use crate::{enc, Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioAuthorizeRequest {
    pub toolkit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioAuthorizeResponse {
    #[serde(default, alias = "redirectUrl")]
    pub connect_url: String,
    #[serde(default)]
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioToolkit {
    pub slug: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolkitsResponse {
    #[serde(default)]
    pub toolkits: Vec<String>,
    #[serde(default)]
    pub catalog: Vec<ComposioToolkit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioConnection {
    pub id: String,
    #[serde(default)]
    pub toolkit: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: Option<String>,
    /// The backend's explanation for a non-`ACTIVE` `status`, when Composio
    /// supplies one (e.g. why a token refresh failed).
    #[serde(default)]
    pub status_reason: Option<String>,
    /// `true` when the connection was explicitly disabled rather than expired.
    #[serde(default)]
    pub is_disabled: Option<bool>,
    #[serde(default)]
    pub account_email: Option<String>,
    #[serde(default)]
    pub workspace: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

/// The backend has emitted both a bare array and `{ connections: [...] }`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ComposioConnectionsResponse {
    List(Vec<ComposioConnection>),
    Object {
        #[serde(default)]
        connections: Vec<ComposioConnection>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default, rename = "memoryChunksDeleted")]
    pub memory_chunks_deleted: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioExecuteRequest {
    pub tool: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioExecuteResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub successful: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
    #[serde(default)]
    pub markdown_formatted: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolFunction {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Option<Value>,
    #[serde(default)]
    pub output_parameters: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolSchema {
    #[serde(rename = "type", default)]
    pub kind: String,
    pub function: ComposioToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolsResponse {
    #[serde(default)]
    pub tools: Vec<ComposioToolSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioTriggerRequest {
    pub connection_id: String,
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioTrigger {
    #[serde(default, alias = "id")]
    pub trigger_id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub connection_id: Option<String>,
    #[serde(default)]
    pub trigger_config: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioTriggersResponse {
    #[serde(default, alias = "activeTriggers")]
    pub triggers: Vec<ComposioTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioAvailableTriggersResponse {
    #[serde(default, alias = "availableTriggers")]
    pub triggers: Vec<ComposioTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioGithubRepo {
    pub owner: String,
    pub repo: String,
    pub full_name: String,
    #[serde(default)]
    pub private: Option<bool>,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioGithubReposResponse {
    #[serde(default)]
    pub connection_id: Option<String>,
    #[serde(default)]
    pub repositories: Vec<ComposioGithubRepo>,
}

impl AgentIntegrationsApi<'_> {
    /// Start a Composio OAuth connection flow.
    pub async fn authorize_composio(
        &self,
        request: &impl Serialize,
    ) -> Result<ComposioAuthorizeResponse, Error> {
        self.post("/agent-integrations/composio/authorize", request)
            .await
    }

    /// List the user's Composio connections.
    pub async fn list_composio_connections(&self) -> Result<ComposioConnectionsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/connections",
                &[],
                None,
                true,
            )
            .await
    }

    /// Delete a Composio connection.
    pub async fn delete_composio_connection(
        &self,
        connection_id: &str,
    ) -> Result<ComposioDeleteResponse, Error> {
        let path = format!(
            "/agent-integrations/composio/connections/{}",
            enc(connection_id)
        );
        self.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Execute a Composio tool on behalf of the user.
    pub async fn execute_composio_tool(
        &self,
        request: &impl Serialize,
    ) -> Result<ComposioExecuteResponse, Error> {
        self.post("/agent-integrations/composio/execute", request)
            .await
    }

    /// List Composio toolkits available to users.
    pub async fn list_composio_toolkits(&self) -> Result<ComposioToolkitsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/toolkits",
                &[],
                None,
                true,
            )
            .await
    }

    /// List Composio tools as OpenAI function-call schemas.
    pub async fn list_composio_tools(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioToolsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/tools",
                query,
                None,
                true,
            )
            .await
    }

    /// List the user's currently enabled Composio triggers.
    pub async fn list_composio_triggers(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioTriggersResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/triggers",
                query,
                None,
                true,
            )
            .await
    }

    /// Enable a Composio trigger on one of the user's connections.
    pub async fn create_composio_trigger(
        &self,
        request: &impl Serialize,
    ) -> Result<ComposioTrigger, Error> {
        self.post("/agent-integrations/composio/triggers", request)
            .await
    }

    /// List triggers available for a toolkit.
    pub async fn list_composio_available_triggers(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioAvailableTriggersResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/triggers/available",
                query,
                None,
                true,
            )
            .await
    }

    /// Disable (delete) a Composio trigger owned by the user.
    pub async fn delete_composio_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<ComposioDeleteResponse, Error> {
        let path = format!("/agent-integrations/composio/triggers/{}", enc(trigger_id));
        self.send(Method::DELETE, &path, &[], None, true).await
    }

    /// List repositories visible through an authorized GitHub connection.
    pub async fn list_composio_github_repos(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioGithubReposResponse, Error> {
        self.send(
            Method::GET,
            "/agent-integrations/composio/github/repos",
            query,
            None,
            true,
        )
        .await
    }
}
