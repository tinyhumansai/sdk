//! Invite codes: campaign management (admin), personal codes, redemption, and status checks.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/invite/*` routes.
pub struct InviteApi<'a> {
    http: &'a HttpClient,
}

impl<'a> InviteApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Create a campaign invite code (admin only).
    pub async fn create_campaign_invite(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/invite/campaign", &[], Some(body), true)
            .await
    }

    /// List all campaign invite codes (admin only).
    pub async fn list_campaign_invites(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/invite/campaign", &[], None, true)
            .await
    }

    /// Deactivate a campaign invite code (admin only).
    pub async fn delete_campaign_invite(&self, code_id: &str) -> Result<Value, Error> {
        let path = format!("/invite/campaign/{}", enc(code_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// List the current user's invite codes with usage info.
    pub async fn list_my_codes(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/invite/my-codes", &[], None, true)
            .await
    }

    /// Redeem an invite code.
    pub async fn redeem_invite(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/invite/redeem", &[], Some(body), true)
            .await
    }

    /// Check if an invite code is valid and available.
    pub async fn get_invite_status(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/invite/status", query, None, true)
            .await
    }
}
