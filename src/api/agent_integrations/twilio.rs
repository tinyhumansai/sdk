//! Twilio outbound voice calls.

use super::AgentIntegrationsApi;
use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwilioCallRequest {
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twiml: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TwilioCallResponse {
    #[serde(default)]
    pub call_sid: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub cost_usd: f64,
}

impl AgentIntegrationsApi<'_> {
    /// Make a call via Twilio.
    pub async fn twilio_call(&self, request: &impl Serialize) -> Result<TwilioCallResponse, Error> {
        self.post("/agent-integrations/twilio/call", request).await
    }
}
