//! Short-link resolution.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient};

/// Typed client for the `/r/*` short-link routes.
pub struct RedirectApi<'a> {
    http: &'a HttpClient,
}

impl<'a> RedirectApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Redirect a short link code to its original URL (returns a 302 redirect).
    pub async fn resolve_redirect(&self, code: &str) -> Result<Value, Error> {
        let path = format!("/r/{}", enc(code));
        self.http.send(Method::GET, &path, &[], None, true).await
    }
}
