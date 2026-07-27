//! Service health check.

use reqwest::Method;

use super::types::DynamicResponse;
use crate::{Error, HttpClient};

/// Typed client for the health-check route.
pub struct HealthApi<'a> {
    http: &'a HttpClient,
}

impl<'a> HealthApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Health check endpoint. Returns server uptime and status information.
    pub async fn check(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/", &[], None, true)
            .await
            .map(Into::into)
    }
}
