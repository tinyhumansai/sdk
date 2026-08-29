//! Agent integrations, one module per provider.
//!
//! Every provider module contributes its request/response DTOs and an
//! `impl` block on [`AgentIntegrationsApi`], so a provider's wire types and
//! its methods live together. Each module is re-exported here, and
//! [`super::agent_integration_types`] re-exports every DTO under its
//! historical path.

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{Error, HttpClient, QueryParam};

pub mod apify;
pub mod composio;
pub mod crypto;
pub mod file_storage;
pub mod financial_apis;
pub mod google_places;
pub mod history_rewards;
pub mod media_generation;
pub mod openrouter;
pub mod parallel;
pub mod pricing;
pub mod recall_calendar;
pub mod tenor;
pub mod tinyfish;
pub mod twilio;

pub use apify::*;
pub use composio::*;
pub use crypto::*;
pub use file_storage::*;
pub use financial_apis::*;
pub use google_places::*;
pub use history_rewards::*;
pub use media_generation::*;
pub use openrouter::*;
pub use parallel::*;
pub use pricing::*;
pub use recall_calendar::*;
pub use tenor::*;
pub use tinyfish::*;
pub use twilio::*;

/// Typed client for the `/agent-integrations/*` routes.
pub struct AgentIntegrationsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AgentIntegrationsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    async fn send<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
        body: Option<&Value>,
        authenticated: bool,
    ) -> Result<T, Error> {
        self.http
            .send_typed(method, path, query, body, authenticated)
            .await
    }

    async fn post<Request: Serialize, Response: DeserializeOwned>(
        &self,
        path: &str,
        request: &Request,
    ) -> Result<Response, Error> {
        let body = serde_json::to_value(request)?;
        self.send(Method::POST, path, &[], Some(&body), true).await
    }

    /// POST to a route that returns the upstream provider's payload verbatim
    /// rather than the `{ success, data }` envelope, so the response must not
    /// be unwrapped.
    ///
    /// Rejects `stream: true` up front: [`HttpClient::send`] awaits
    /// `response.text()` and buffers the whole body before returning, so a
    /// streamed SSE response would come back as one opaque, non-JSON string
    /// instead of incremental events. The SDK does not yet expose an
    /// incremental transport for this surface, so failing fast here is safer
    /// than silently handing back a payload nothing can consume as intended.
    async fn passthrough<Request: Serialize>(
        &self,
        path: &str,
        request: &Request,
    ) -> Result<crate::api::types::DynamicResponse, Error> {
        let body = serde_json::to_value(request)?;
        if matches!(body.get("stream"), Some(Value::Bool(true))) {
            return Err(Error::StreamingNotSupported(path.to_owned()));
        }
        self.http
            .send(Method::POST, path, &[], Some(&body), false)
            .await
            .map(Into::into)
    }

    async fn bytes_query(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
    ) -> Result<Vec<u8>, Error> {
        self.http.send_bytes_query(method, path, query).await
    }
}
