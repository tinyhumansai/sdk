//! Public investor deck/landing page access plus engagement event tracking.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient};

/// Typed client for the `/investors/*` routes.
pub struct InvestorsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> InvestorsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Get an investor deck by slug.
    pub async fn get_investor_page(&self, slug: &str) -> Result<Value, Error> {
        let path = format!("/investors/{}", enc(slug));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Track an investor page event.
    pub async fn track_investor_event(&self, slug: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/investors/{}/events", enc(slug));
        self.http.send(Method::POST, &path, &[], Some(body), true).await
    }
}
