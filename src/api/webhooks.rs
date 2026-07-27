//! Webhooks: inbound provider receivers plus user-managed webhook tunnels.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient};

/// Typed client for the `/webhooks/*` routes.
pub struct WebhooksApi<'a> {
    http: &'a HttpClient,
}

impl<'a> WebhooksApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Receive a Composio trigger webhook (HMAC-verified by the backend).
    pub async fn receive_composio_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/webhooks/composio", &[], Some(body), true)
            .await
    }

    /// Create a new webhook tunnel.
    pub async fn create_core_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/webhooks/core", &[], Some(body), true)
            .await
    }

    /// List the current user's webhook tunnels.
    pub async fn list_core_webhooks(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/webhooks/core", &[], None, true)
            .await
    }

    /// Get the remaining USD budget available for webhook tunnel bandwidth.
    pub async fn get_core_webhook_bandwidth(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/webhooks/core/bandwidth", &[], None, true)
            .await
    }

    /// Get a specific webhook tunnel by id.
    pub async fn get_core_webhook(&self, id: &str) -> Result<Value, Error> {
        let path = format!("/webhooks/core/{}", enc(id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Update a webhook tunnel.
    pub async fn update_core_webhook(&self, id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/webhooks/core/{}", enc(id));
        self.http
            .send(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// Delete a webhook tunnel.
    pub async fn delete_core_webhook(&self, id: &str) -> Result<Value, Error> {
        let path = format!("/webhooks/core/{}", enc(id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Receive a Discord interaction webhook (Ed25519-verified by the backend).
    pub async fn receive_discord_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/webhooks/discord", &[], Some(body), true)
            .await
    }

    /// Receive a GitHub webhook event (HMAC-SHA256-verified by the backend).
    pub async fn receive_github_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/webhooks/github", &[], Some(body), true)
            .await
    }

    /// Forward a payload to a tunnel via its public ingress endpoint.
    pub async fn forward_webhook_ingress(&self, uuid: &str) -> Result<Value, Error> {
        let path = format!("/webhooks/ingress/{}", enc(uuid));
        self.http.send(Method::POST, &path, &[], None, true).await
    }

    /// Forward a payload to a tunnel with a downstream path suffix.
    pub async fn forward_webhook_ingress_with_path(
        &self,
        uuid: &str,
        path: &str,
    ) -> Result<Value, Error> {
        let full = format!("/webhooks/ingress/{}/{}", enc(uuid), enc(path));
        self.http.send(Method::POST, &full, &[], None, true).await
    }

    /// Receive a Coinbase Commerce payment webhook.
    pub async fn receive_coinbase_payment_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/webhooks/payments/coinbase",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Receive a Stripe payment webhook.
    pub async fn receive_stripe_payment_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/webhooks/payments/stripe",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Receive a Sentry internal-integration webhook (HMAC-SHA256-verified).
    pub async fn receive_sentry_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/webhooks/sentry", &[], Some(body), true)
            .await
    }

    /// Receive a Telegram webhook update for the master bot.
    pub async fn receive_telegram_webhook(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/webhooks/telegram", &[], Some(body), true)
            .await
    }

    /// Receive a Telegram webhook update for a managed (per-user) bot.
    pub async fn receive_managed_telegram_webhook(
        &self,
        bot_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!("/webhooks/telegram/managed/{}", enc(bot_id));
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }
}
