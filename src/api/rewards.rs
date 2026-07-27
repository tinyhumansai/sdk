//! Backend-backed rewards snapshot and connected-account management.

use reqwest::Method;
use serde_json::Value;

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
    pub async fn unlink_discord(&self) -> Result<Value, Error> {
        self.http
            .send(Method::DELETE, "/rewards/discord", &[], None, true)
            .await
    }

    /// Get the authenticated user's backend-backed rewards snapshot.
    pub async fn get_my_rewards(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/rewards/me", &[], None, true)
            .await
    }

    pub async fn claim(&self, reward_type: &str) -> Result<Value, Error> {
        let body = serde_json::json!({"rewardType": reward_type});
        self.http
            .send(Method::POST, "/rewards/claim", &[], Some(&body), true)
            .await
    }
}
