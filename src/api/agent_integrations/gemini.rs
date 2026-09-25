//! Gemini API: grounded `generateContent` (Google Search / Google Maps) and the
//! metered Gemini Live relay (conversation, live transcription, live tools).
//!
//! Billed at Google's paid-tier rates plus a 10% premium.
//!
//! # Live relay protocol
//!
//! [`AgentIntegrationsApi::gemini_create_live_session`] returns a single-use
//! ticket and a `wsUrl`. Open a plain WebSocket to `wsUrl` within 60 seconds
//! and speak the Gemini Live protocol: send `realtimeInput`, `clientContent`
//! and `toolResponse` frames; receive Google's server messages verbatim
//! (`setupComplete`, `serverContent`, `toolCall`, `usageMetadata`, ...). The
//! session setup is fixed when the ticket is minted, so any `setup` frame the
//! client sends is ignored. Usage is metered server-side per turn. The relay
//! closes with one of the [`GEMINI_LIVE_CLOSE_UNAUTHORIZED`] family of codes.

use super::AgentIntegrationsApi;
use crate::{enc, Error};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Relay close code: the ticket is missing, expired, or already used.
pub const GEMINI_LIVE_CLOSE_UNAUTHORIZED: u16 = 4401;
/// Relay close code: the balance no longer covers the per-session reserve.
pub const GEMINI_LIVE_CLOSE_INSUFFICIENT_CREDITS: u16 = 4402;
/// Relay close code: idle timeout or maximum session duration reached.
pub const GEMINI_LIVE_CLOSE_TIMEOUT: u16 = 4408;
/// Relay close code: the upstream Gemini connection failed.
pub const GEMINI_LIVE_CLOSE_UPSTREAM_ERROR: u16 = 1011;

/// One turn of a Gemini conversation. `parts` are Gemini `Part` objects
/// (`{"text": ...}`, `{"inlineData": ...}`, `{"functionResponse": ...}`, ...).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GeminiContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default)]
    pub parts: Vec<Value>,
}

impl GeminiContent {
    /// A single-part text turn from the user.
    pub fn user_text(text: impl Into<String>) -> Self {
        Self {
            role: Some("user".into()),
            parts: vec![serde_json::json!({ "text": text.into() })],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiTimeRangeFilter {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiGoogleSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_range_filter: Option<GeminiTimeRangeFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiGoogleMaps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_widget: Option<bool>,
}

/// A Gemini tool. Set exactly one field; the backend accepts Google Search,
/// Google Maps (not in Live sessions) and function declarations only.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_search: Option<GeminiGoogleSearch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_maps: Option<GeminiGoogleMaps>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<Value>>,
}

impl GeminiTool {
    pub fn google_search() -> Self {
        Self {
            google_search: Some(GeminiGoogleSearch::default()),
            ..Self::default()
        }
    }

    pub fn google_maps() -> Self {
        Self {
            google_maps: Some(GeminiGoogleMaps::default()),
            ..Self::default()
        }
    }

    pub fn functions(declarations: Vec<Value>) -> Self {
        Self {
            function_declarations: Some(declarations),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct GeminiLatLng {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiRetrievalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lat_lng: Option<GeminiLatLng>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiToolConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_config: Option<GeminiRetrievalConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<Value>,
}

/// Body of `POST /agent-integrations/gemini/models/{model}/generate-content`,
/// Gemini's native `GenerateContentRequest` minus cached content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiGenerateContentRequest {
    pub contents: Vec<GeminiContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<GeminiToolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiModalityTokenCount {
    #[serde(default)]
    pub modality: Option<String>,
    #[serde(default)]
    pub token_count: Option<u64>,
}

/// Gemini `usageMetadata`. Live messages report output as `response*`;
/// `generateContent` reports it as `candidates*`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiUsageMetadata {
    #[serde(default)]
    pub prompt_token_count: Option<u64>,
    #[serde(default)]
    pub cached_content_token_count: Option<u64>,
    #[serde(default, alias = "responseTokenCount")]
    pub candidates_token_count: Option<u64>,
    #[serde(default)]
    pub tool_use_prompt_token_count: Option<u64>,
    #[serde(default)]
    pub thoughts_token_count: Option<u64>,
    #[serde(default)]
    pub total_token_count: Option<u64>,
    #[serde(default)]
    pub prompt_tokens_details: Vec<GeminiModalityTokenCount>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiCandidate {
    #[serde(default)]
    pub content: Option<GeminiContent>,
    #[serde(default)]
    pub finish_reason: Option<String>,
    /// `webSearchQueries`, `groundingChunks`, `groundingSupports`,
    /// `searchEntryPoint`, `googleMapsWidgetContextToken`, ...
    #[serde(default)]
    pub grounding_metadata: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Google's `GenerateContentResponse` plus the amount charged.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiGenerateContentResponse {
    #[serde(default)]
    pub candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    pub usage_metadata: Option<GeminiUsageMetadata>,
    #[serde(default)]
    pub model_version: Option<String>,
    #[serde(default)]
    pub response_id: Option<String>,
    #[serde(default)]
    pub cost_usd: f64,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl GeminiGenerateContentResponse {
    /// Concatenated text parts of the first candidate.
    pub fn text(&self) -> String {
        self.candidates
            .first()
            .and_then(|c| c.content.as_ref())
            .map(|content| {
                content
                    .parts
                    .iter()
                    .filter_map(|p| p.get("text").and_then(Value::as_str))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A Live conversation: native-audio or text dialogue, optionally with
/// Google Search and function calling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiLiveConversation {
    /// `gemini-3.8-live` or `gemini-2.5-flash-native-audio-preview-12-2025`.
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_minutes: Option<u32>,
    /// `["AUDIO"]` (default) or `["TEXT"]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_modalities: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speech_config: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_transcription: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realtime_input_config: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_window_compression: Option<Value>,
    /// Pass `{"handle": ...}` from a `sessionResumptionUpdate` to resume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_resumption: Option<Value>,
    /// Google Search and/or function declarations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
}

/// Live streaming speech-to-text on `gemini-3.5-transcribe-live`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiLiveTranscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_minutes: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary: Option<Vec<String>>,
    /// `VERBATIM` or `SMART`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription_mode: Option<String>,
}

/// Body of `POST /agent-integrations/gemini/live/sessions`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum GeminiLiveSessionRequest {
    Conversation(Box<GeminiLiveConversation>),
    Transcribe(GeminiLiveTranscription),
}

/// A single-use relay ticket. Connect a WebSocket to `ws_url` before
/// `ticket_expires_at`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiLiveTicket {
    pub session_id: String,
    pub ticket: String,
    pub ws_url: String,
    pub ticket_expires_at: String,
    pub model: String,
    pub mode: String,
    pub max_minutes: u32,
    /// Balance reserved for each open session.
    #[serde(default)]
    pub reserve_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiLiveUsageTotals {
    #[serde(default)]
    pub prompt_tokens: u64,
    #[serde(default)]
    pub cached_tokens: u64,
    #[serde(default)]
    pub output_tokens: u64,
    #[serde(default)]
    pub thoughts_tokens: u64,
    #[serde(default)]
    pub tool_use_prompt_tokens: u64,
    #[serde(default)]
    pub search_queries: u64,
}

/// Status and metered usage of a Live session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeminiLiveSession {
    pub session_id: String,
    pub mode: String,
    pub model: String,
    /// `PENDING`, `ACTIVE`, `CLOSED`, `EXPIRED` or `FAILED`.
    pub status: String,
    #[serde(default)]
    pub max_minutes: u32,
    #[serde(default)]
    pub ticket_expires_at: Option<String>,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub closed_at: Option<String>,
    #[serde(default)]
    pub close_reason: Option<String>,
    #[serde(default)]
    pub turn_count: u64,
    #[serde(default)]
    pub charged_usd: f64,
    #[serde(default)]
    pub usage_totals: GeminiLiveUsageTotals,
}

impl AgentIntegrationsApi<'_> {
    /// Gemini `generateContent` with Google Search / Google Maps grounding or
    /// function calling. Billed on usage plus grounding fees.
    pub async fn gemini_generate_content(
        &self,
        model: &str,
        request: &GeminiGenerateContentRequest,
    ) -> Result<GeminiGenerateContentResponse, Error> {
        self.post(
            &format!(
                "/agent-integrations/gemini/models/{}/generate-content",
                enc(model)
            ),
            request,
        )
        .await
    }

    /// Open a metered Gemini Live session and get its single-use relay ticket.
    pub async fn gemini_create_live_session(
        &self,
        request: &GeminiLiveSessionRequest,
    ) -> Result<GeminiLiveTicket, Error> {
        self.post("/agent-integrations/gemini/live/sessions", request)
            .await
    }

    /// Status and metered usage of a Live session owned by the caller.
    pub async fn gemini_live_session(&self, session_id: &str) -> Result<GeminiLiveSession, Error> {
        self.send(
            Method::GET,
            &format!(
                "/agent-integrations/gemini/live/sessions/{}",
                enc(session_id)
            ),
            &[],
            None,
            true,
        )
        .await
    }
}
