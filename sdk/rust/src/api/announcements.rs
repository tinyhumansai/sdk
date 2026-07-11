//! Active in-app announcements for the signed-in user.

use reqwest::Method;
use serde_json::Value;

use crate::{Error, HttpClient};

/// Typed client for the `/announcements/*` routes.
pub struct AnnouncementsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AnnouncementsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Get the latest active announcement for the signed-in user.
    pub async fn get_latest_announcements(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/announcements/latest", &[], None, true)
            .await
    }
}
