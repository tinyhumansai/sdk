//! Active in-app announcements for the signed-in user.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{Error, HttpClient};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnnouncementSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnnouncementCallToAction {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub body: String,
    pub severity: AnnouncementSeverity,
    pub cta: Option<AnnouncementCallToAction>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
}

/// Typed client for the `/announcements/*` routes.
pub struct AnnouncementsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AnnouncementsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Get the latest active announcement for the signed-in user.
    pub async fn get_latest_announcements(&self) -> Result<Option<Announcement>, Error> {
        let value = self
            .http
            .send(Method::GET, "/announcements/latest", &[], None, true)
            .await?;
        serde_json::from_value(value).map_err(Error::Decode)
    }
}
