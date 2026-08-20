//! Google Places search and details.

use super::AgentIntegrationsApi;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GooglePlacesSearchRequest {
    pub query: String,
    #[serde(
        default,
        rename = "maxResults",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_results: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GooglePlaceDetailsRequest {
    pub place_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GooglePlace {
    #[serde(default)]
    pub place_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub location: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GooglePlacesResponse {
    #[serde(default, alias = "places")]
    pub results: Vec<GooglePlace>,
    #[serde(default)]
    pub place: Option<GooglePlace>,
    #[serde(default, rename = "costUsd")]
    pub cost_usd: f64,
}

impl AgentIntegrationsApi<'_> {
    /// Get place details via Google Places API.
    pub async fn google_places_details(
        &self,
        request: &impl Serialize,
    ) -> Result<GooglePlacesResponse, Error> {
        self.post("/agent-integrations/google-places/details", request)
            .await
    }

    /// Search places via Google Places API.
    pub async fn google_places_search(
        &self,
        request: &impl Serialize,
    ) -> Result<GooglePlacesResponse, Error> {
        self.post("/agent-integrations/google-places/search", request)
            .await
    }
}
