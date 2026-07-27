//! Mascots: library, manifests, Rive assets, the interactive demo, and meeting dispatch.

use reqwest::Method;
use serde_json::Value;

use super::types::JoinMeetingRequest;
use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/mascots/*` routes.
pub struct MascotsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> MascotsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List available mascots.
    pub async fn list_mascots(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/mascots", &[], None, true)
            .await
    }

    /// Interactive mascot library demo page.
    pub async fn get_demo(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/mascots/demo", &[], None, true)
            .await
    }

    /// Send the mascot bot into a live meeting.
    pub async fn join_meeting(&self, request: &JoinMeetingRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("join meeting request is serializable");
        self.http
            .send(
                Method::POST,
                "/mascots/join-meeting",
                &[],
                Some(&body),
                true,
            )
            .await
    }

    /// List the authenticated user's mascot-bot meeting history.
    pub async fn list_meetings(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/mascots/meetings", query, None, true)
            .await
    }

    /// Get a mascot manifest.
    pub async fn get_mascot(&self, id: &str) -> Result<Value, Error> {
        let path = format!("/mascots/{}", enc(id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Download the Rive animation file for a mascot.
    pub async fn get_mascot_riv(&self, id: &str) -> Result<Value, Error> {
        let path = format!("/mascots/{}/riv", enc(id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }
}
