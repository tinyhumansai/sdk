//! Recall.ai calendar connection and upcoming meetings.

use super::AgentIntegrationsApi;
use crate::{Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecallCalendarStatus {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecallCalendarConnectResponse {
    #[serde(default, alias = "url")]
    pub connect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecallCalendarDisconnectResponse {
    #[serde(default)]
    pub disconnected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecallMeeting {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub meeting_url: Option<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub bot_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecallMeetingsResponse {
    #[serde(default)]
    pub meetings: Vec<RecallMeeting>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecallOauthCompleteResponse {
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub email: Option<String>,
}

impl AgentIntegrationsApi<'_> {
    /// Start the Recall.ai Calendar V1 OAuth flow for Google Calendar.
    pub async fn connect_recall_calendar(&self) -> Result<RecallCalendarConnectResponse, Error> {
        self.http
            .send_typed(
                Method::POST,
                "/agent-integrations/recall-calendar/connect",
                &[],
                None,
                true,
            )
            .await
    }

    /// Disconnect the user's Google Calendar from Recall.
    pub async fn disconnect_recall_calendar(
        &self,
    ) -> Result<RecallCalendarDisconnectResponse, Error> {
        self.http
            .send_typed(
                Method::POST,
                "/agent-integrations/recall-calendar/disconnect",
                &[],
                None,
                true,
            )
            .await
    }

    /// List upcoming meetings from the connected calendar.
    pub async fn list_recall_calendar_meetings(&self) -> Result<RecallMeetingsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/recall-calendar/meetings",
                &[],
                None,
                true,
            )
            .await
    }

    /// OAuth landing page (public).
    pub async fn recall_calendar_oauth_complete(
        &self,
        query: &[QueryParam],
    ) -> Result<RecallOauthCompleteResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/recall-calendar/oauth-complete",
                query,
                None,
                true,
            )
            .await
    }

    /// Get the user's Recall calendar connection status.
    pub async fn get_recall_calendar_status(&self) -> Result<RecallCalendarStatus, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/recall-calendar/status",
                &[],
                None,
                true,
            )
            .await
    }
}
