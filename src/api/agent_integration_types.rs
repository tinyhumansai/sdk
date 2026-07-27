//! Request and response DTOs for the public agent-integration APIs.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApifyRunRequest {
    pub actor_id: String,
    pub input: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_mbytes: Option<u32>,
}

/// Apify response fields vary by actor; stable run metadata is typed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApifyRunResponse {
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApifyResultsResponse {
    #[serde(default)]
    pub items: Vec<Value>,
    #[serde(default)]
    pub count: Option<u64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioAuthorizeRequest {
    pub toolkit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioAuthorizeResponse {
    #[serde(default, alias = "redirectUrl")]
    pub connect_url: String,
    #[serde(default)]
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioToolkit {
    pub slug: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolkitsResponse {
    #[serde(default)]
    pub toolkits: Vec<String>,
    #[serde(default)]
    pub catalog: Vec<ComposioToolkit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioConnection {
    pub id: String,
    #[serde(default)]
    pub toolkit: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub account_email: Option<String>,
    #[serde(default)]
    pub workspace: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

/// The backend has emitted both a bare array and `{ connections: [...] }`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ComposioConnectionsResponse {
    List(Vec<ComposioConnection>),
    Object {
        #[serde(default)]
        connections: Vec<ComposioConnection>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default, rename = "memoryChunksDeleted")]
    pub memory_chunks_deleted: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioExecuteRequest {
    pub tool: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioExecuteResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub successful: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
    #[serde(default)]
    pub markdown_formatted: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolFunction {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Option<Value>,
    #[serde(default)]
    pub output_parameters: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolSchema {
    #[serde(rename = "type", default)]
    pub kind: String,
    pub function: ComposioToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioToolsResponse {
    #[serde(default)]
    pub tools: Vec<ComposioToolSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioTriggerRequest {
    pub connection_id: String,
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioTrigger {
    #[serde(default, alias = "id")]
    pub trigger_id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub connection_id: Option<String>,
    #[serde(default)]
    pub trigger_config: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioTriggersResponse {
    #[serde(default, alias = "activeTriggers")]
    pub triggers: Vec<ComposioTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComposioAvailableTriggersResponse {
    #[serde(default, alias = "availableTriggers")]
    pub triggers: Vec<ComposioTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioGithubRepo {
    pub owner: String,
    pub repo: String,
    pub full_name: String,
    #[serde(default)]
    pub private: Option<bool>,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposioGithubReposResponse {
    #[serde(default)]
    pub connection_id: Option<String>,
    #[serde(default)]
    pub repositories: Vec<ComposioGithubRepo>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolRequest {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OptionsRequest {
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_greeks: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateRequest {
    pub from_currency: String,
    pub to_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CryptoSeriesRequest {
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Commodity {
    Wti,
    Brent,
    NaturalGas,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeriesInterval {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommodityRequest {
    pub commodity: Commodity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<SeriesInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub symbol: String,
    pub price: f64,
    #[serde(default)]
    pub open: f64,
    #[serde(default)]
    pub high: f64,
    #[serde(default)]
    pub low: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub previous_close: f64,
    #[serde(default)]
    pub change: f64,
    #[serde(default)]
    pub change_percent: String,
    #[serde(default)]
    pub latest_trading_day: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    #[serde(default)]
    pub quote: Quote,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRate {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: f64,
    #[serde(default)]
    pub bid: Option<f64>,
    #[serde(default)]
    pub ask: Option<f64>,
    #[serde(default)]
    pub last_refreshed: String,
    #[serde(default)]
    pub time_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateResponse {
    pub rate: ExchangeRateValue,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExchangeRateValue {
    Detailed(ExchangeRate),
    Numeric(f64),
}

impl Default for ExchangeRateValue {
    fn default() -> Self {
        Self::Numeric(0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OptionsResponse {
    pub symbol: String,
    #[serde(default)]
    pub contracts: Vec<Value>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SeriesPoint {
    pub date: String,
    #[serde(default)]
    pub open: f64,
    #[serde(default)]
    pub high: f64,
    #[serde(default)]
    pub low: f64,
    #[serde(default)]
    pub close: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FinancialSeries {
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub commodity: String,
    #[serde(default)]
    pub interval: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub series: Vec<SeriesPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinancialSeriesResponse {
    #[serde(default)]
    pub series: FinancialSeries,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GooglePlacesSearchRequest {
    pub query: String,
    #[serde(
        default,
        rename = "maxResults",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_results: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GooglePlaceDetailsRequest {
    pub place_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GooglePlace {
    #[serde(default)]
    pub place_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub location: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GooglePlacesResponse {
    #[serde(default, alias = "places")]
    pub results: Vec<GooglePlace>,
    #[serde(default)]
    pub place: Option<GooglePlace>,
    #[serde(default, rename = "costUsd")]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_images: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoGenerationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    #[serde(rename = "type", default)]
    pub kind: String,
    pub url: String,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub media: Vec<MediaItem>,
    #[serde(default)]
    pub cost_usd: f64,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaModel {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub capabilities: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MediaModelsResponse {
    #[serde(default)]
    pub models: Vec<MediaModel>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenorSearchRequest {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(
        default,
        rename = "contentFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TenorGif {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub media: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TenorSearchResponse {
    #[serde(default)]
    pub results: Vec<TenorGif>,
    #[serde(default)]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwilioCallRequest {
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twiml: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TwilioCallResponse {
    #[serde(default)]
    pub call_sid: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub file_id: String,
    pub filename: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub visibility: String,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub public_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StorageUsage {
    #[serde(default, rename = "usedBytes")]
    pub used_bytes: u64,
    #[serde(default, rename = "limitBytes")]
    pub limit_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListFilesResponse {
    #[serde(default)]
    pub files: Vec<FileMetadata>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub usage: StorageUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    #[serde(flatten)]
    pub file: FileMetadata,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileLinkResponse {
    pub url: String,
    #[serde(default, rename = "expiresAt")]
    pub expires_at: Option<String>,
    #[serde(default, rename = "costUsd")]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteFileResponse {
    #[serde(default)]
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateFileVisibilityRequest {
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<u32>,
}

/// Metering state is backend-extensible; stable totals are typed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationPricingResponse {
    #[serde(default)]
    pub integrations: Map<String, Value>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelChatModel {
    Speed,
    Lite,
    Base,
    Core,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParallelMessage {
    pub role: ParallelRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelChatRequest {
    pub model: ParallelChatModel,
    pub messages: Vec<ParallelMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelChatResponse {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub usage: Option<Value>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ParallelSearchMode {
    OneShot,
    Agentic,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelSearchRequest {
    pub objective: String,
    pub search_queries: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ParallelSearchMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpts: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelSearchResult {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub excerpts: Vec<String>,
    #[serde(default)]
    pub publish_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelSearchResponse {
    #[serde(default)]
    pub results: Vec<ParallelSearchResult>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelExtractRequest {
    pub urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ParallelExtractResult {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub excerpts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelExtractResponse {
    #[serde(default)]
    pub results: Vec<ParallelExtractResult>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelProcessor {
    Lite,
    Base,
    Core,
    Ultra,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelResearchRequest {
    pub input: Value,
    pub processor: ParallelProcessor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelEnrichRequest {
    pub input: Value,
    pub processor: ParallelProcessor,
    pub output_schema: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelRunResponse {
    #[serde(default, alias = "id")]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParallelMatchCondition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParallelDatasetGenerator {
    Preview,
    Base,
    Core,
    Pro,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParallelDatasetRequest {
    pub objective: String,
    pub entity_type: String,
    pub match_conditions: Vec<ParallelMatchCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<ParallelDatasetGenerator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParallelDatasetResponse {
    #[serde(default, alias = "id")]
    pub findall_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub matches: Vec<Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishSearchRequest {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_thumbnail: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TinyFishSearchResult {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub snippet: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishSearchResponse {
    #[serde(default)]
    pub results: Vec<TinyFishSearchResult>,
    #[serde(default)]
    pub total_results: Option<u64>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishFetchRequest {
    pub urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_links: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TinyFishFetchedPage {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(default, rename = "imageLinks")]
    pub image_links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishFetchResponse {
    #[serde(default)]
    pub results: Vec<TinyFishFetchedPage>,
    #[serde(default)]
    pub errors: Vec<Value>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishAgentRunRequest {
    pub url: String,
    pub goal: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_config: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_vault: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credential_item_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TinyFishAgentRunResponse {
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
}
