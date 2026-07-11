//! Referral link claiming and earnings/referral-list statistics.

use reqwest::Method;
use serde_json::Value;

use crate::{Error, HttpClient};

/// Typed client for the `/referral/*` routes.
pub struct ReferralApi<'a> {
    http: &'a HttpClient,
}

impl<'a> ReferralApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Claim a referral link for the authenticated user.
    pub async fn claim_referral(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/referral/claim", &[], Some(body), true)
            .await
    }

    /// Fetch referral link, earnings summary, and referral list.
    pub async fn get_referral_stats(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/referral/stats", &[], None, true)
            .await
    }
}
