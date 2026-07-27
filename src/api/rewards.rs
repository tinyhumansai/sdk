//! Backend-backed rewards snapshot and connected-account management.

use reqwest::Method;

use super::types::DynamicResponse;
use crate::{Error, HttpClient};

/// Typed client for the `/rewards/*` routes.
pub struct RewardsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> RewardsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Disconnect (unlink) the authenticated user's Discord account.
    pub async fn unlink_discord(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::DELETE, "/rewards/discord", &[], None, true)
            .await
            .map(Into::into)
    }

    /// Get the authenticated user's backend-backed rewards snapshot.
    pub async fn get_my_rewards(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/rewards/me", &[], None, true)
            .await
            .map(Into::into)
    }

    pub async fn claim(&self, reward_type: &str) -> Result<DynamicResponse, Error> {
        let body = serde_json::json!({"rewardType": reward_type});
        self.http
            .send(Method::POST, "/rewards/claim", &[], Some(&body), true)
            .await
            .map(Into::into)
    }
}
