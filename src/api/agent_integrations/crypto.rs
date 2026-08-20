//! Cross-chain crypto swaps and bridges.

use super::AgentIntegrationsApi;
use crate::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CryptoSwapRequest {
    pub chain_id: u64,
    pub token_in: String,
    pub token_in_amount: String,
    pub token_out: String,
    pub token_out_recipient: String,
    pub sender_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slippage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CryptoBridgeRequest {
    pub src_chain_id: u64,
    pub src_chain_token_in: String,
    pub src_chain_token_in_amount: String,
    pub dst_chain_id: u64,
    pub dst_chain_token_out: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dst_chain_token_out_amount: Option<String>,
    pub dst_chain_token_out_recipient: String,
    pub src_chain_order_authority_address: String,
    pub dst_chain_order_authority_address: String,
}

/// deBridge responses contain a provider-specific transaction object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CryptoTransactionResponse {
    #[serde(default)]
    pub transaction: Value,
    #[serde(default)]
    pub order: Option<Value>,
    #[serde(default)]
    pub cost_usd: f64,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CryptoRoutesResponse {
    #[serde(default)]
    pub chains: Vec<Value>,
    #[serde(default)]
    pub routes: Vec<Value>,
}

impl AgentIntegrationsApi<'_> {
    /// Build a cross-chain bridge transaction.
    pub async fn crypto_bridge(
        &self,
        request: &impl Serialize,
    ) -> Result<CryptoTransactionResponse, Error> {
        self.post("/agent-integrations/crypto/bridge", request)
            .await
    }

    /// List supported chains for cross-chain swaps and bridges.
    pub async fn list_crypto_routes(&self) -> Result<CryptoRoutesResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/crypto/routes",
                &[],
                None,
                true,
            )
            .await
    }

    /// Build a single-chain swap transaction.
    pub async fn crypto_swap(
        &self,
        request: &impl Serialize,
    ) -> Result<CryptoTransactionResponse, Error> {
        self.post("/agent-integrations/crypto/swap", request).await
    }
}
