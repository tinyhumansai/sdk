//! Referral link claiming and earnings/referral-list statistics.

use reqwest::Method;

use super::types::{ClaimReferralRequest, DynamicResponse};
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
    pub async fn claim_referral(
        &self,
        request: &ClaimReferralRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("referral request is serializable");
        self.http
            .send_typed(Method::POST, "/referral/claim", &[], Some(&body), true)
            .await
    }

    /// Fetch referral link, earnings summary, and referral list.
    pub async fn get_referral_stats(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/referral/stats", &[], None, true)
            .await
    }
}
