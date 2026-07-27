use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::types::DynamicResponse;
use crate::{enc, Error, HttpClient};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyScope {
    Read,
    Write,
    Inference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ApiKeyScope>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_ips: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

pub struct ApiKeysApi<'a> {
    http: &'a HttpClient,
}

impl<'a> ApiKeysApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    pub async fn list(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/api-keys", &[], None, true)
            .await
    }

    pub async fn create(&self, request: &CreateApiKeyRequest) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("API key request is serializable");
        self.http
            .send_typed(Method::POST, "/api-keys", &[], Some(&body), true)
            .await
    }

    pub async fn revoke(&self, key_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/api-keys/{}", enc(key_id));
        self.http
            .send_typed(Method::DELETE, &path, &[], None, true)
            .await
    }
}
