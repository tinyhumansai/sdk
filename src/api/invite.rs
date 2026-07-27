//! Personal invite codes, redemption, and status checks.

use reqwest::Method;
use serde_json::Value;

use super::types::CodeRequest;
use crate::{Error, HttpClient, QueryParam};

/// Typed client for the `/invite/*` routes.
pub struct InviteApi<'a> {
    http: &'a HttpClient,
}

impl<'a> InviteApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List the current user's invite codes with usage info.
    pub async fn list_my_codes(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/invite/my-codes", &[], None, true)
            .await
    }

    /// Redeem an invite code.
    pub async fn redeem_invite(&self, request: &CodeRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("invite request is serializable");
        self.http
            .send(Method::POST, "/invite/redeem", &[], Some(&body), true)
            .await
    }

    /// Check if an invite code is valid and available.
    pub async fn get_invite_status(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/invite/status", query, None, true)
            .await
    }
}
