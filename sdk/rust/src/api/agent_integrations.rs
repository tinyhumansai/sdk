//! Agent integrations: Apify, Composio, crypto swaps/bridges, financial APIs,
//! Google Places, media generation, Parallel web research, Recall calendar,
//! Tenor GIFs, and Twilio voice.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/agent-integrations/*` routes.
pub struct AgentIntegrationsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AgentIntegrationsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    pub async fn list_files(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/file-storage/files",
                query,
                None,
                true,
            )
            .await
    }
    pub async fn get_file(&self, file_id: &str) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn download_file(&self, file_id: &str) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                &format!(
                    "/agent-integrations/file-storage/files/{}/download",
                    enc(file_id)
                ),
                &[],
                None,
                false,
            )
            .await
    }
    pub async fn public_file(&self, file_id: &str) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                &format!("/agent-integrations/file-storage/public/{}", enc(file_id)),
                &[],
                None,
                false,
            )
            .await
    }
    pub async fn file_storage_usage(&self) -> Result<Value, Error> {
        self.http
            .send(
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
    ) -> Result<Value, Error> {
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
        self.http
            .post_multipart("/agent-integrations/file-storage/files", form)
            .await
    }
    pub async fn update_file_visibility(
        &self,
        file_id: &str,
        visibility: &str,
    ) -> Result<Value, Error> {
        let body = serde_json::json!({"visibility": visibility});
        self.http
            .send(
                Method::PATCH,
                &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
                &[],
                Some(&body),
                true,
            )
            .await
    }
    pub async fn delete_file(&self, file_id: &str) -> Result<Value, Error> {
        self.http
            .send(
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
    ) -> Result<Value, Error> {
        let body = serde_json::json!({"expiresInSeconds": expires_in_seconds});
        self.http
            .send(
                Method::POST,
                &format!(
                    "/agent-integrations/file-storage/files/{}/link",
                    enc(file_id)
                ),
                &[],
                Some(&body),
                true,
            )
            .await
    }
    pub async fn history_rewards_status(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/history-rewards/status",
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn claim_history_reward(&self) -> Result<Value, Error> {
        self.http
            .send(
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
    ) -> Result<Value, Error> {
        let form = reqwest::multipart::Form::new()
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes).file_name(file_name.to_owned()),
            )
            .text("agent", agent.to_owned());
        self.http
            .post_multipart("/agent-integrations/history-rewards/uploads", form)
            .await
    }

    // --- Apify ---

    /// Run an Apify actor.
    pub async fn run_apify_actor(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/apify/run",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Get status of an Apify actor run.
    pub async fn get_apify_run(&self, run_id: &str) -> Result<Value, Error> {
        let path = format!("/agent-integrations/apify/runs/{}", enc(run_id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Get results from a completed Apify actor run.
    pub async fn get_apify_run_results(
        &self,
        run_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/agent-integrations/apify/runs/{}/results", enc(run_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    // --- Composio ---

    /// Start a Composio OAuth connection flow.
    pub async fn authorize_composio(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/composio/authorize",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// List the user's Composio connections.
    pub async fn list_composio_connections(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/composio/connections",
                &[],
                None,
                true,
            )
            .await
    }

    /// Delete a Composio connection.
    pub async fn delete_composio_connection(&self, connection_id: &str) -> Result<Value, Error> {
        let path = format!(
            "/agent-integrations/composio/connections/{}",
            enc(connection_id)
        );
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Execute a Composio tool on behalf of the user.
    pub async fn execute_composio_tool(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/composio/execute",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// List Composio toolkits available to users.
    pub async fn list_composio_toolkits(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/composio/toolkits",
                &[],
                None,
                true,
            )
            .await
    }

    /// List Composio tools as OpenAI function-call schemas.
    pub async fn list_composio_tools(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/composio/tools",
                query,
                None,
                true,
            )
            .await
    }

    /// List the user's currently enabled Composio triggers.
    pub async fn list_composio_triggers(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/composio/triggers",
                query,
                None,
                true,
            )
            .await
    }

    /// Enable a Composio trigger on one of the user's connections.
    pub async fn create_composio_trigger(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/composio/triggers",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// List triggers available for a toolkit.
    pub async fn list_composio_available_triggers(
        &self,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/composio/triggers/available",
                query,
                None,
                true,
            )
            .await
    }

    /// Disable (delete) a Composio trigger owned by the user.
    pub async fn delete_composio_trigger(&self, trigger_id: &str) -> Result<Value, Error> {
        let path = format!("/agent-integrations/composio/triggers/{}", enc(trigger_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    // --- Crypto ---

    /// Build a cross-chain bridge transaction.
    pub async fn crypto_bridge(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/crypto/bridge",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// List supported chains for cross-chain swaps and bridges.
    pub async fn list_crypto_routes(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/crypto/routes",
                &[],
                None,
                true,
            )
            .await
    }

    /// Build a single-chain swap transaction.
    pub async fn crypto_swap(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/crypto/swap",
                &[],
                Some(body),
                true,
            )
            .await
    }

    // --- Financial APIs ---

    /// Commodity / futures price series (WTI, BRENT, NATURAL_GAS) via Alpha Vantage.
    pub async fn financial_apis_commodity(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/financial-apis/commodity",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Daily OHLCV series for a digital currency via Alpha Vantage DIGITAL_CURRENCY_DAILY.
    pub async fn financial_apis_crypto_series(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/financial-apis/crypto-series",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Realtime FX or crypto exchange rate via Alpha Vantage CURRENCY_EXCHANGE_RATE.
    pub async fn financial_apis_exchange_rate(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/financial-apis/exchange-rate",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Realtime options chain for a symbol via Alpha Vantage REALTIME_OPTIONS.
    pub async fn financial_apis_options(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/financial-apis/options",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Latest quote for a stock or index symbol via Alpha Vantage GLOBAL_QUOTE.
    pub async fn financial_apis_quote(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/financial-apis/quote",
                &[],
                Some(body),
                true,
            )
            .await
    }

    // --- Google Places ---

    /// Get place details via Google Places API.
    pub async fn google_places_details(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/google-places/details",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Search places via Google Places API.
    pub async fn google_places_search(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/google-places/search",
                &[],
                Some(body),
                true,
            )
            .await
    }

    // --- Media Generation ---

    /// Generate or edit an image via GMI (Seedream / SeedEdit).
    pub async fn media_generation_images(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/media-generation/images",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// List curated media-generation models.
    pub async fn list_media_generation_models(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/media-generation/models",
                query,
                None,
                true,
            )
            .await
    }

    /// Poll a media-generation request.
    pub async fn get_media_generation_request(&self, request_id: &str) -> Result<Value, Error> {
        let path = format!(
            "/agent-integrations/media-generation/requests/{}",
            enc(request_id)
        );
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Generate a video via GMI (Seedance / Veo).
    pub async fn media_generation_videos(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/media-generation/videos",
                &[],
                Some(body),
                true,
            )
            .await
    }

    // --- Parallel ---

    /// Chat with the web via Parallel Chat Completions.
    pub async fn parallel_chat(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/parallel/chat",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Generate a web dataset via Parallel FindAll.
    pub async fn parallel_dataset(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/parallel/dataset",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Get dataset (FindAll) run status.
    pub async fn get_parallel_dataset(&self, findall_id: &str) -> Result<Value, Error> {
        let path = format!("/agent-integrations/parallel/dataset/{}", enc(findall_id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Get dataset (FindAll) matched candidates snapshot.
    pub async fn get_parallel_dataset_result(&self, findall_id: &str) -> Result<Value, Error> {
        let path = format!(
            "/agent-integrations/parallel/dataset/{}/result",
            enc(findall_id)
        );
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Enrich web data with a structured output schema (synchronous).
    pub async fn parallel_enrich(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/parallel/enrich",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Extract content from URLs via Parallel API.
    pub async fn parallel_extract(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/parallel/extract",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Start a deep research task (Parallel Task API).
    pub async fn parallel_research(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/parallel/research",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Get deep research run status.
    pub async fn get_parallel_research(&self, run_id: &str) -> Result<Value, Error> {
        let path = format!("/agent-integrations/parallel/research/{}", enc(run_id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Block on a deep research run until completion.
    pub async fn get_parallel_research_result(
        &self,
        run_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!(
            "/agent-integrations/parallel/research/{}/result",
            enc(run_id)
        );
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Web search via Parallel API.
    pub async fn parallel_search(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/parallel/search",
                &[],
                Some(body),
                true,
            )
            .await
    }

    // --- Pricing ---

    /// Get plan-aware pricing for all agent integrations.
    pub async fn get_pricing(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/agent-integrations/pricing", &[], None, true)
            .await
    }

    // --- Recall Calendar ---

    /// Start the Recall.ai Calendar V1 OAuth flow for Google Calendar.
    pub async fn connect_recall_calendar(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/recall-calendar/connect",
                &[],
                None,
                true,
            )
            .await
    }

    /// Disconnect the user's Google Calendar from Recall.
    pub async fn disconnect_recall_calendar(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/recall-calendar/disconnect",
                &[],
                None,
                true,
            )
            .await
    }

    /// List upcoming meetings from the connected calendar.
    pub async fn list_recall_calendar_meetings(&self) -> Result<Value, Error> {
        self.http
            .send(
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
    ) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/agent-integrations/recall-calendar/oauth-complete",
                query,
                None,
                true,
            )
            .await
    }

    /// Get the user's Recall calendar connection status.
    pub async fn get_recall_calendar_status(&self) -> Result<Value, Error> {
        self.http
            .send(
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
    pub async fn tenor_search(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/tenor/search",
                &[],
                Some(body),
                true,
            )
            .await
    }

    // --- Twilio ---

    /// Make a call via Twilio.
    pub async fn twilio_call(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/agent-integrations/twilio/call",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Twilio webhook for incoming calls.
    pub async fn twilio_incoming_call_webhook(
        &self,
        user_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!(
            "/agent-integrations/twilio/webhooks/incoming-call/{}",
            enc(user_id)
        );
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }

    /// Twilio webhook for call status updates.
    pub async fn twilio_status_webhook(&self, user_id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!(
            "/agent-integrations/twilio/webhooks/status/{}",
            enc(user_id)
        );
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }
}
