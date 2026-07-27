use super::types::DynamicResponse;
use crate::{enc, Error, HttpClient, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}

pub struct OpenCompanyApi<'a> {
    http: &'a HttpClient,
}
impl<'a> OpenCompanyApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }
    async fn body(
        &self,
        method: Method,
        path: &str,
        body: Value,
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(method, path, &[], Some(&body), true)
            .await
            .map(Into::into)
    }
    pub async fn list_instances(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/opencompany/instances", &[], None, true)
            .await
            .map(Into::into)
    }
    pub async fn create_instance(
        &self,
        request: &CreateInstanceRequest,
    ) -> Result<DynamicResponse, Error> {
        self.body(
            Method::POST,
            "/opencompany/instances",
            serde_json::to_value(request).expect("request is serializable"),
        )
        .await
    }
    pub async fn delete_instance(
        &self,
        slug: &str,
        purge_data: Option<bool>,
    ) -> Result<DynamicResponse, Error> {
        let query: [QueryParam; 1] = [("purge_data", purge_data.map(|v| v.to_string()))];
        self.http
            .send(
                Method::DELETE,
                &format!("/opencompany/instances/{}", enc(slug)),
                &query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn suspend(&self, slug: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::POST,
                &format!("/opencompany/instances/{}/suspend", enc(slug)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn resume(&self, slug: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::POST,
                &format!("/opencompany/instances/{}/resume", enc(slug)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn set_custom_domain(
        &self,
        slug: &str,
        domain: &str,
    ) -> Result<DynamicResponse, Error> {
        self.body(
            Method::PUT,
            &format!("/opencompany/instances/{}/custom-domain", enc(slug)),
            serde_json::json!({"domain": domain}),
        )
        .await
    }
    pub async fn remove_custom_domain(&self, slug: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::DELETE,
                &format!("/opencompany/instances/{}/custom-domain", enc(slug)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
    pub async fn verify_custom_domain(&self, slug: &str) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::POST,
                &format!("/opencompany/instances/{}/custom-domain/verify", enc(slug)),
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }
}
