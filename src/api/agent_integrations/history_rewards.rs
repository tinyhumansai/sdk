//! History upload rewards.

use super::AgentIntegrationsApi;
use crate::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRewardsStatus {
    #[serde(default)]
    pub eligible: bool,
    #[serde(default)]
    pub claimed: bool,
    #[serde(default)]
    pub uploaded_agents: Vec<String>,
    #[serde(default)]
    pub reward_amount_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRewardClaimResponse {
    #[serde(default)]
    pub claimed: bool,
    #[serde(default)]
    pub already_claimed: bool,
    #[serde(default)]
    pub amount_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HistoryUploadResponse {
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub accepted: bool,
    #[serde(default)]
    pub item_count: Option<u64>,
}

impl AgentIntegrationsApi<'_> {
    pub async fn history_rewards_status(&self) -> Result<HistoryRewardsStatus, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/history-rewards/status",
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn claim_history_reward(&self) -> Result<HistoryRewardClaimResponse, Error> {
        self.http
            .send_typed(
                Method::POST,
                "/agent-integrations/history-rewards/claim",
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn upload_history(
        &self,
        file_name: &str,
        bytes: Vec<u8>,
        agent: &str,
    ) -> Result<HistoryUploadResponse, Error> {
        let form = reqwest::multipart::Form::new()
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes).file_name(file_name.to_owned()),
            )
            .text("agent", agent.to_owned());
        let value = self
            .http
            .post_multipart("/agent-integrations/history-rewards/uploads", form)
            .await?;
        Ok(serde_json::from_value(value)?)
    }
}
