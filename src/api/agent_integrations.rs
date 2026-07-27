//! Agent integrations: Apify, Composio, crypto swaps/bridges, financial APIs,
//! Google Places, media generation, Parallel web research, Recall calendar,
//! Tenor GIFs, and Twilio voice.

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub use super::agent_integration_types::*;
use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/agent-integrations/*` routes.
pub struct AgentIntegrationsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AgentIntegrationsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    async fn send<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
        body: Option<&Value>,
        authenticated: bool,
    ) -> Result<T, Error> {
        self.http
            .send_typed(method, path, query, body, authenticated)
            .await
    }

    async fn post<Request: Serialize, Response: DeserializeOwned>(
        &self,
        path: &str,
        request: &Request,
    ) -> Result<Response, Error> {
        let body = serde_json::to_value(request)?;
        self.send(Method::POST, path, &[], Some(&body), true).await
    }

    pub async fn list_files(&self, query: &[QueryParam]) -> Result<ListFilesResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/file-storage/files",
                query,
                None,
                true,
            )
            .await
    }
    pub async fn get_file(&self, file_id: &str) -> Result<FileMetadata, Error> {
        self.http
            .send_typed(
                Method::GET,
                &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn download_file(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        self.http
            .send_bytes(
                Method::GET,
                &format!(
                    "/agent-integrations/file-storage/files/{}/download",
                    enc(file_id)
                ),
            )
            .await
    }
    pub async fn public_file(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        self.http
            .send_bytes(
                Method::GET,
                &format!("/agent-integrations/file-storage/public/{}", enc(file_id)),
            )
            .await
    }
    pub async fn file_storage_usage(&self) -> Result<StorageUsage, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/file-storage/usage",
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn upload_file(
        &self,
        file_name: &str,
        bytes: Vec<u8>,
        visibility: Option<&str>,
        ttl_days: Option<u32>,
    ) -> Result<UploadFileResponse, Error> {
        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(bytes).file_name(file_name.to_owned()),
        );
        if let Some(value) = visibility {
            form = form.text("visibility", value.to_owned());
        }
        if let Some(value) = ttl_days {
            form = form.text("ttlDays", value.to_string());
        }
        let value = self
            .http
            .post_multipart("/agent-integrations/file-storage/files", form)
            .await?;
        Ok(serde_json::from_value(value)?)
    }
    pub async fn update_file_visibility(
        &self,
        file_id: &str,
        visibility: &str,
    ) -> Result<FileMetadata, Error> {
        let body = serde_json::to_value(UpdateFileVisibilityRequest {
            visibility: visibility.to_owned(),
        })?;
        self.send(
            Method::PATCH,
            &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
            &[],
            Some(&body),
            true,
        )
        .await
    }
    pub async fn delete_file(&self, file_id: &str) -> Result<DeleteFileResponse, Error> {
        self.http
            .send_typed(
                Method::DELETE,
                &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn create_file_link(
        &self,
        file_id: &str,
        expires_in_seconds: Option<u32>,
    ) -> Result<FileLinkResponse, Error> {
        self.post(
            &format!(
                "/agent-integrations/file-storage/files/{}/link",
                enc(file_id)
            ),
            &CreateFileLinkRequest { expires_in_seconds },
        )
        .await
    }
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

    // --- Apify ---

    /// Run an Apify actor.
    pub async fn run_apify_actor(
        &self,
        request: &impl Serialize,
    ) -> Result<ApifyRunResponse, Error> {
        self.post("/agent-integrations/apify/run", request).await
    }

    /// Get status of an Apify actor run.
    pub async fn get_apify_run(&self, run_id: &str) -> Result<ApifyRunResponse, Error> {
        let path = format!("/agent-integrations/apify/runs/{}", enc(run_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Get results from a completed Apify actor run.
    pub async fn get_apify_run_results(
        &self,
        run_id: &str,
        query: &[QueryParam],
    ) -> Result<ApifyResultsResponse, Error> {
        let path = format!("/agent-integrations/apify/runs/{}/results", enc(run_id));
        self.send(Method::GET, &path, query, None, true).await
    }

    // --- Composio ---

    /// Start a Composio OAuth connection flow.
    pub async fn authorize_composio(
        &self,
        request: &impl Serialize,
    ) -> Result<ComposioAuthorizeResponse, Error> {
        self.post("/agent-integrations/composio/authorize", request)
            .await
    }

    /// List the user's Composio connections.
    pub async fn list_composio_connections(&self) -> Result<ComposioConnectionsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/connections",
                &[],
                None,
                true,
            )
            .await
    }

    /// Delete a Composio connection.
    pub async fn delete_composio_connection(
        &self,
        connection_id: &str,
    ) -> Result<ComposioDeleteResponse, Error> {
        let path = format!(
            "/agent-integrations/composio/connections/{}",
            enc(connection_id)
        );
        self.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Execute a Composio tool on behalf of the user.
    pub async fn execute_composio_tool(
        &self,
        request: &impl Serialize,
    ) -> Result<ComposioExecuteResponse, Error> {
        self.post("/agent-integrations/composio/execute", request)
            .await
    }

    /// List Composio toolkits available to users.
    pub async fn list_composio_toolkits(&self) -> Result<ComposioToolkitsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/toolkits",
                &[],
                None,
                true,
            )
            .await
    }

    /// List Composio tools as OpenAI function-call schemas.
    pub async fn list_composio_tools(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioToolsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/tools",
                query,
                None,
                true,
            )
            .await
    }

    /// List the user's currently enabled Composio triggers.
    pub async fn list_composio_triggers(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioTriggersResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/triggers",
                query,
                None,
                true,
            )
            .await
    }

    /// Enable a Composio trigger on one of the user's connections.
    pub async fn create_composio_trigger(
        &self,
        request: &impl Serialize,
    ) -> Result<ComposioTrigger, Error> {
        self.post("/agent-integrations/composio/triggers", request)
            .await
    }

    /// List triggers available for a toolkit.
    pub async fn list_composio_available_triggers(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioAvailableTriggersResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/composio/triggers/available",
                query,
                None,
                true,
            )
            .await
    }

    /// Disable (delete) a Composio trigger owned by the user.
    pub async fn delete_composio_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<ComposioDeleteResponse, Error> {
        let path = format!("/agent-integrations/composio/triggers/{}", enc(trigger_id));
        self.send(Method::DELETE, &path, &[], None, true).await
    }

    /// List repositories visible through an authorized GitHub connection.
    pub async fn list_composio_github_repos(
        &self,
        query: &[QueryParam],
    ) -> Result<ComposioGithubReposResponse, Error> {
        self.send(
            Method::GET,
            "/agent-integrations/composio/github/repos",
            query,
            None,
            true,
        )
        .await
    }

    // --- Crypto ---

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

    // --- Financial APIs ---

    /// Commodity / futures price series (WTI, BRENT, NATURAL_GAS) via Alpha Vantage.
    pub async fn financial_apis_commodity(
        &self,
        request: &impl Serialize,
    ) -> Result<FinancialSeriesResponse, Error> {
        self.post("/agent-integrations/financial-apis/commodity", request)
            .await
    }

    /// Daily OHLCV series for a digital currency via Alpha Vantage DIGITAL_CURRENCY_DAILY.
    pub async fn financial_apis_crypto_series(
        &self,
        request: &impl Serialize,
    ) -> Result<FinancialSeriesResponse, Error> {
        self.post("/agent-integrations/financial-apis/crypto-series", request)
            .await
    }

    /// Realtime FX or crypto exchange rate via Alpha Vantage CURRENCY_EXCHANGE_RATE.
    pub async fn financial_apis_exchange_rate(
        &self,
        request: &impl Serialize,
    ) -> Result<ExchangeRateResponse, Error> {
        self.post("/agent-integrations/financial-apis/exchange-rate", request)
            .await
    }

    /// Realtime options chain for a symbol via Alpha Vantage REALTIME_OPTIONS.
    pub async fn financial_apis_options(
        &self,
        request: &impl Serialize,
    ) -> Result<OptionsResponse, Error> {
        self.post("/agent-integrations/financial-apis/options", request)
            .await
    }

    /// Latest quote for a stock or index symbol via Alpha Vantage GLOBAL_QUOTE.
    pub async fn financial_apis_quote(
        &self,
        request: &impl Serialize,
    ) -> Result<QuoteResponse, Error> {
        self.post("/agent-integrations/financial-apis/quote", request)
            .await
    }

    // --- Google Places ---

    /// Get place details via Google Places API.
    pub async fn google_places_details(
        &self,
        request: &impl Serialize,
    ) -> Result<GooglePlacesResponse, Error> {
        self.post("/agent-integrations/google-places/details", request)
            .await
    }

    /// Search places via Google Places API.
    pub async fn google_places_search(
        &self,
        request: &impl Serialize,
    ) -> Result<GooglePlacesResponse, Error> {
        self.post("/agent-integrations/google-places/search", request)
            .await
    }

    // --- Media Generation ---

    /// Generate or edit an image via GMI (Seedream / SeedEdit).
    pub async fn media_generation_images(
        &self,
        request: &impl Serialize,
    ) -> Result<MediaResponse, Error> {
        self.post("/agent-integrations/media-generation/images", request)
            .await
    }

    /// List curated media-generation models.
    pub async fn list_media_generation_models(
        &self,
        query: &[QueryParam],
    ) -> Result<MediaModelsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/media-generation/models",
                query,
                None,
                true,
            )
            .await
    }

    /// Poll a media-generation request.
    pub async fn get_media_generation_request(
        &self,
        request_id: &str,
    ) -> Result<MediaResponse, Error> {
        let path = format!(
            "/agent-integrations/media-generation/requests/{}",
            enc(request_id)
        );
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Generate a video via GMI (Seedance / Veo).
    pub async fn media_generation_videos(
        &self,
        request: &impl Serialize,
    ) -> Result<MediaResponse, Error> {
        self.post("/agent-integrations/media-generation/videos", request)
            .await
    }

    // --- Parallel ---

    /// Chat with the web via Parallel Chat Completions.
    pub async fn parallel_chat(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelChatResponse, Error> {
        self.post("/agent-integrations/parallel/chat", request)
            .await
    }

    /// Generate a web dataset via Parallel FindAll.
    pub async fn parallel_dataset(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelDatasetResponse, Error> {
        self.post("/agent-integrations/parallel/dataset", request)
            .await
    }

    /// Get dataset (FindAll) run status.
    pub async fn get_parallel_dataset(
        &self,
        findall_id: &str,
    ) -> Result<ParallelDatasetResponse, Error> {
        let path = format!("/agent-integrations/parallel/dataset/{}", enc(findall_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Get dataset (FindAll) matched candidates snapshot.
    pub async fn get_parallel_dataset_result(
        &self,
        findall_id: &str,
    ) -> Result<ParallelDatasetResponse, Error> {
        let path = format!(
            "/agent-integrations/parallel/dataset/{}/result",
            enc(findall_id)
        );
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Enrich web data with a structured output schema (synchronous).
    pub async fn parallel_enrich(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelRunResponse, Error> {
        self.post("/agent-integrations/parallel/enrich", request)
            .await
    }

    /// Extract content from URLs via Parallel API.
    pub async fn parallel_extract(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelExtractResponse, Error> {
        self.post("/agent-integrations/parallel/extract", request)
            .await
    }

    /// Start a deep research task (Parallel Task API).
    pub async fn parallel_research(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelRunResponse, Error> {
        self.post("/agent-integrations/parallel/research", request)
            .await
    }

    /// Get deep research run status.
    pub async fn get_parallel_research(&self, run_id: &str) -> Result<ParallelRunResponse, Error> {
        let path = format!("/agent-integrations/parallel/research/{}", enc(run_id));
        self.send(Method::GET, &path, &[], None, true).await
    }

    /// Block on a deep research run until completion.
    pub async fn get_parallel_research_result(
        &self,
        run_id: &str,
        query: &[QueryParam],
    ) -> Result<ParallelRunResponse, Error> {
        let path = format!(
            "/agent-integrations/parallel/research/{}/result",
            enc(run_id)
        );
        self.send(Method::GET, &path, query, None, true).await
    }

    /// Web search via Parallel API.
    pub async fn parallel_search(
        &self,
        request: &impl Serialize,
    ) -> Result<ParallelSearchResponse, Error> {
        self.post("/agent-integrations/parallel/search", request)
            .await
    }

    // --- TinyFish ---

    pub async fn tinyfish_search(
        &self,
        request: &impl Serialize,
    ) -> Result<TinyFishSearchResponse, Error> {
        self.post("/agent-integrations/tinyfish/search", request)
            .await
    }

    pub async fn tinyfish_fetch(
        &self,
        request: &impl Serialize,
    ) -> Result<TinyFishFetchResponse, Error> {
        self.post("/agent-integrations/tinyfish/fetch", request)
            .await
    }

    pub async fn tinyfish_agent_run(
        &self,
        request: &impl Serialize,
    ) -> Result<TinyFishAgentRunResponse, Error> {
        self.post("/agent-integrations/tinyfish/agent/run", request)
            .await
    }

    // --- Pricing ---

    /// Get plan-aware pricing for all agent integrations.
    pub async fn get_pricing(&self) -> Result<IntegrationPricingResponse, Error> {
        self.http
            .send_typed(Method::GET, "/agent-integrations/pricing", &[], None, true)
            .await
    }

    // --- Recall Calendar ---

    /// Start the Recall.ai Calendar V1 OAuth flow for Google Calendar.
    pub async fn connect_recall_calendar(&self) -> Result<RecallCalendarConnectResponse, Error> {
        self.http
            .send_typed(
                Method::POST,
                "/agent-integrations/recall-calendar/connect",
                &[],
                None,
                true,
            )
            .await
    }

    /// Disconnect the user's Google Calendar from Recall.
    pub async fn disconnect_recall_calendar(
        &self,
    ) -> Result<RecallCalendarDisconnectResponse, Error> {
        self.http
            .send_typed(
                Method::POST,
                "/agent-integrations/recall-calendar/disconnect",
                &[],
                None,
                true,
            )
            .await
    }

    /// List upcoming meetings from the connected calendar.
    pub async fn list_recall_calendar_meetings(&self) -> Result<RecallMeetingsResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/recall-calendar/meetings",
                &[],
                None,
                true,
            )
            .await
    }

    /// OAuth landing page (public).
    pub async fn recall_calendar_oauth_complete(
        &self,
        query: &[QueryParam],
    ) -> Result<RecallOauthCompleteResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/recall-calendar/oauth-complete",
                query,
                None,
                true,
            )
            .await
    }

    /// Get the user's Recall calendar connection status.
    pub async fn get_recall_calendar_status(&self) -> Result<RecallCalendarStatus, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/recall-calendar/status",
                &[],
                None,
                true,
            )
            .await
    }

    // --- Tenor ---

    /// Search for GIFs via the Tenor API.
    pub async fn tenor_search(
        &self,
        request: &impl Serialize,
    ) -> Result<TenorSearchResponse, Error> {
        self.post("/agent-integrations/tenor/search", request).await
    }

    // --- Twilio ---

    /// Make a call via Twilio.
    pub async fn twilio_call(&self, request: &impl Serialize) -> Result<TwilioCallResponse, Error> {
        self.post("/agent-integrations/twilio/call", request).await
    }
}
