//! User-owned webhook tunnels: create, list, inspect, update, and delete a
//! tunnel, plus the remaining bandwidth budget.
//!
//! These are the `/webhooks/core*` routes — ordinary bearer-authenticated
//! user-facing operations. They are distinct from the webhook *receivers* also
//! served under `/webhooks` (Stripe, Telegram, Discord, GitHub, Composio,
//! Coinbase, Sentry, and the tunnel ingress paths), which providers call into,
//! authenticate by signature rather than user token, and the SDK deliberately
//! does not expose.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::types::DynamicResponse;
use crate::{enc, Error, HttpClient};

/// Body for creating a webhook tunnel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookTunnelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Body for updating a webhook tunnel. Every field is optional; omitted fields
/// are left unchanged by the backend.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookTunnelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// Typed client for the `/webhooks/core*` routes.
pub struct WebhooksApi<'a> {
    http: &'a HttpClient,
}

impl<'a> WebhooksApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List the authenticated user's webhook tunnels.
    pub async fn list_tunnels(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/webhooks/core", &[], None, true)
            .await
    }

    /// Create a new webhook tunnel.
    pub async fn create_tunnel(
        &self,
        request: &CreateWebhookTunnelRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("tunnel request is serializable");
        self.http
            .send_typed(Method::POST, "/webhooks/core", &[], Some(&body), true)
            .await
    }

    /// Fetch a specific webhook tunnel.
    pub async fn get_tunnel(&self, id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/webhooks/core/{}", enc(id));
        self.http
            .send_typed(Method::GET, &path, &[], None, true)
            .await
    }

    /// Update a webhook tunnel's name, description, or active state.
    pub async fn update_tunnel(
        &self,
        id: &str,
        request: &UpdateWebhookTunnelRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("tunnel update is serializable");
        let path = format!("/webhooks/core/{}", enc(id));
        self.http
            .send_typed(Method::PATCH, &path, &[], Some(&body), true)
            .await
    }

    /// Delete a webhook tunnel.
    pub async fn delete_tunnel(&self, id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/webhooks/core/{}", enc(id));
        self.http
            .send_typed(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// Remaining bandwidth budget available for webhook traffic.
    pub async fn get_bandwidth(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/webhooks/core/bandwidth", &[], None, true)
            .await
    }
}
