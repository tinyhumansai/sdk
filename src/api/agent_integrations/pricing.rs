//! Plan-aware pricing for every agent integration.

use super::AgentIntegrationsApi;
use crate::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Metering state is backend-extensible; stable totals are typed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationPricingResponse {
    #[serde(default)]
    pub integrations: Map<String, Value>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl AgentIntegrationsApi<'_> {
    /// Get plan-aware pricing for all agent integrations.
    pub async fn get_pricing(&self) -> Result<IntegrationPricingResponse, Error> {
        self.http
            .send_typed(Method::GET, "/agent-integrations/pricing", &[], None, true)
            .await
    }
}
