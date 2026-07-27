from __future__ import annotations

from typing import Any

from ..http import Json
from ..types import FileUpload, HistoryUpload
from ._base import ApiNamespace, enc

__all__ = ["AgentIntegrationsApi"]


class AgentIntegrationsApi(ApiNamespace):
    """Agent integrations: Apify, Composio, crypto swaps/bridges, financial APIs,
    Google Places, media generation, Parallel web research, Recall calendar,
    Tenor GIFs, and Twilio voice."""

    # --- File storage and history rewards ---

    def upload_file(self, body: FileUpload, **kwargs: Any) -> Json:
        return self._http.post(
            "/agent-integrations/file-storage/files", body=body, **kwargs
        )

    def list_files(self, query: dict[str, Any] | None = None, **kwargs: Any) -> Json:
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/agent-integrations/file-storage/files", query=merged, **kwargs
        )

    def get_file_storage_usage(self, **kwargs: Any) -> Json:
        return self._http.get("/agent-integrations/file-storage/usage", **kwargs)

    def get_file(self, file_id: str, **kwargs: Any) -> Json:
        return self._http.get(
            f"/agent-integrations/file-storage/files/{enc(file_id)}", **kwargs
        )

    def update_file_visibility(
        self, file_id: str, visibility: str, **kwargs: Any
    ) -> Json:
        return self._http.patch(
            f"/agent-integrations/file-storage/files/{enc(file_id)}",
            body={"visibility": visibility},
            **kwargs,
        )

    def delete_file(self, file_id: str, **kwargs: Any) -> Json:
        return self._http.delete(
            f"/agent-integrations/file-storage/files/{enc(file_id)}", **kwargs
        )

    def download_file(self, file_id: str, **kwargs: Any) -> Json:
        return self._http.get(
            f"/agent-integrations/file-storage/files/{enc(file_id)}/download",
            **kwargs,
        )

    def create_file_link(
        self, file_id: str, expires_in_seconds: int = 3600, **kwargs: Any
    ) -> Json:
        return self._http.post(
            f"/agent-integrations/file-storage/files/{enc(file_id)}/link",
            body={"expiresInSeconds": expires_in_seconds},
            **kwargs,
        )

    def get_public_file(self, file_id: str, **kwargs: Any) -> Json:
        return self._http.get(
            f"/agent-integrations/file-storage/public/{enc(file_id)}", **kwargs
        )

    def upload_history(self, body: HistoryUpload, **kwargs: Any) -> Json:
        return self._http.post(
            "/agent-integrations/history-rewards/uploads", body=body, **kwargs
        )

    def claim_history_reward(self, **kwargs: Any) -> Json:
        return self._http.post("/agent-integrations/history-rewards/claim", **kwargs)

    def get_history_reward_status(self, **kwargs: Any) -> Json:
        return self._http.get("/agent-integrations/history-rewards/status", **kwargs)

    # --- Apify ---

    def run_apify_actor(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Run an Apify actor."""
        return self._http.post("/agent-integrations/apify/run", body=body, **kwargs)

    def get_apify_run(self, run_id: str, **kwargs: Any) -> Json:
        """Get status of an Apify actor run."""
        return self._http.get(f"/agent-integrations/apify/runs/{enc(run_id)}", **kwargs)

    def get_apify_run_results(
        self, run_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Get results from a completed Apify actor run."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/agent-integrations/apify/runs/{enc(run_id)}/results",
            query=merged,
            **kwargs,
        )

    # --- Composio ---

    def authorize_composio(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Start a Composio OAuth connection flow."""
        return self._http.post(
            "/agent-integrations/composio/authorize", body=body, **kwargs
        )

    def list_composio_connections(self, **kwargs: Any) -> Json:
        """List the user's Composio connections."""
        return self._http.get("/agent-integrations/composio/connections", **kwargs)

    def delete_composio_connection(self, connection_id: str, **kwargs: Any) -> Json:
        """Delete a Composio connection."""
        return self._http.delete(
            f"/agent-integrations/composio/connections/{enc(connection_id)}", **kwargs
        )

    def execute_composio_tool(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Execute a Composio tool on behalf of the user."""
        return self._http.post(
            "/agent-integrations/composio/execute", body=body, **kwargs
        )

    def list_composio_toolkits(self, **kwargs: Any) -> Json:
        """List Composio toolkits available to users."""
        return self._http.get("/agent-integrations/composio/toolkits", **kwargs)

    def list_composio_tools(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List Composio tools as OpenAI function-call schemas."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/agent-integrations/composio/tools", query=merged, **kwargs
        )

    def list_composio_triggers(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List the user's currently enabled Composio triggers."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/agent-integrations/composio/triggers", query=merged, **kwargs
        )

    def create_composio_trigger(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Enable a Composio trigger on one of the user's connections."""
        return self._http.post(
            "/agent-integrations/composio/triggers", body=body, **kwargs
        )

    def list_composio_available_triggers(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List triggers available for a toolkit."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/agent-integrations/composio/triggers/available", query=merged, **kwargs
        )

    def delete_composio_trigger(self, trigger_id: str, **kwargs: Any) -> Json:
        """Disable (delete) a Composio trigger owned by the user."""
        return self._http.delete(
            f"/agent-integrations/composio/triggers/{enc(trigger_id)}", **kwargs
        )

    # --- Crypto ---

    def crypto_bridge(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Build a cross-chain bridge transaction."""
        return self._http.post("/agent-integrations/crypto/bridge", body=body, **kwargs)

    def list_crypto_routes(self, **kwargs: Any) -> Json:
        """List supported chains for cross-chain swaps and bridges."""
        return self._http.get("/agent-integrations/crypto/routes", **kwargs)

    def crypto_swap(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Build a single-chain swap transaction."""
        return self._http.post("/agent-integrations/crypto/swap", body=body, **kwargs)

    # --- Financial APIs ---

    def financial_apis_commodity(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Commodity / futures price series (WTI, BRENT, NATURAL_GAS) via Alpha Vantage."""
        return self._http.post(
            "/agent-integrations/financial-apis/commodity", body=body, **kwargs
        )

    def financial_apis_crypto_series(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Daily OHLCV series for a digital currency via Alpha Vantage DIGITAL_CURRENCY_DAILY."""
        return self._http.post(
            "/agent-integrations/financial-apis/crypto-series", body=body, **kwargs
        )

    def financial_apis_exchange_rate(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Realtime FX or crypto exchange rate via Alpha Vantage CURRENCY_EXCHANGE_RATE."""
        return self._http.post(
            "/agent-integrations/financial-apis/exchange-rate", body=body, **kwargs
        )

    def financial_apis_options(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Realtime options chain for a symbol via Alpha Vantage REALTIME_OPTIONS."""
        return self._http.post(
            "/agent-integrations/financial-apis/options", body=body, **kwargs
        )

    def financial_apis_quote(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Latest quote for a stock or index symbol via Alpha Vantage GLOBAL_QUOTE."""
        return self._http.post(
            "/agent-integrations/financial-apis/quote", body=body, **kwargs
        )

    # --- Google Places ---

    def google_places_details(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Get place details via Google Places API."""
        return self._http.post(
            "/agent-integrations/google-places/details", body=body, **kwargs
        )

    def google_places_search(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Search places via Google Places API."""
        return self._http.post(
            "/agent-integrations/google-places/search", body=body, **kwargs
        )

    # --- Media Generation ---

    def media_generation_images(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Generate or edit an image via GMI (Seedream / SeedEdit)."""
        return self._http.post(
            "/agent-integrations/media-generation/images", body=body, **kwargs
        )

    def list_media_generation_models(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List curated media-generation models."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/agent-integrations/media-generation/models", query=merged, **kwargs
        )

    def get_media_generation_request(self, request_id: str, **kwargs: Any) -> Json:
        """Poll a media-generation request."""
        return self._http.get(
            f"/agent-integrations/media-generation/requests/{enc(request_id)}", **kwargs
        )

    def media_generation_videos(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Generate a video via GMI (Seedance / Veo)."""
        return self._http.post(
            "/agent-integrations/media-generation/videos", body=body, **kwargs
        )

    # --- Parallel ---

    def parallel_chat(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Chat with the web via Parallel Chat Completions."""
        return self._http.post("/agent-integrations/parallel/chat", body=body, **kwargs)

    def parallel_dataset(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Generate a web dataset via Parallel FindAll."""
        return self._http.post(
            "/agent-integrations/parallel/dataset", body=body, **kwargs
        )

    def get_parallel_dataset(self, findall_id: str, **kwargs: Any) -> Json:
        """Get dataset (FindAll) run status."""
        return self._http.get(
            f"/agent-integrations/parallel/dataset/{enc(findall_id)}", **kwargs
        )

    def get_parallel_dataset_result(self, findall_id: str, **kwargs: Any) -> Json:
        """Get dataset (FindAll) matched candidates snapshot."""
        return self._http.get(
            f"/agent-integrations/parallel/dataset/{enc(findall_id)}/result", **kwargs
        )

    def parallel_enrich(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Enrich web data with a structured output schema (synchronous)."""
        return self._http.post(
            "/agent-integrations/parallel/enrich", body=body, **kwargs
        )

    def parallel_extract(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Extract content from URLs via Parallel API."""
        return self._http.post(
            "/agent-integrations/parallel/extract", body=body, **kwargs
        )

    def parallel_research(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Start a deep research task (Parallel Task API)."""
        return self._http.post(
            "/agent-integrations/parallel/research", body=body, **kwargs
        )

    def get_parallel_research(self, run_id: str, **kwargs: Any) -> Json:
        """Get deep research run status."""
        return self._http.get(
            f"/agent-integrations/parallel/research/{enc(run_id)}", **kwargs
        )

    def get_parallel_research_result(
        self, run_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Block on a deep research run until completion."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/agent-integrations/parallel/research/{enc(run_id)}/result",
            query=merged,
            **kwargs,
        )

    def parallel_search(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Web search via Parallel API."""
        return self._http.post(
            "/agent-integrations/parallel/search", body=body, **kwargs
        )

    # --- Pricing ---

    def get_pricing(self, **kwargs: Any) -> Json:
        """Get plan-aware pricing for all agent integrations."""
        return self._http.get("/agent-integrations/pricing", **kwargs)

    # --- Recall Calendar ---

    def connect_recall_calendar(self, **kwargs: Any) -> Json:
        """Start the Recall.ai Calendar V1 OAuth flow for Google Calendar."""
        return self._http.post("/agent-integrations/recall-calendar/connect", **kwargs)

    def disconnect_recall_calendar(self, **kwargs: Any) -> Json:
        """Disconnect the user's Google Calendar from Recall."""
        return self._http.post(
            "/agent-integrations/recall-calendar/disconnect", **kwargs
        )

    def list_recall_calendar_meetings(self, **kwargs: Any) -> Json:
        """List upcoming meetings from the connected calendar."""
        return self._http.get("/agent-integrations/recall-calendar/meetings", **kwargs)

    def recall_calendar_oauth_complete(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """OAuth landing page (public)."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/agent-integrations/recall-calendar/oauth-complete", query=merged, **kwargs
        )

    def get_recall_calendar_status(self, **kwargs: Any) -> Json:
        """Get the user's Recall calendar connection status."""
        return self._http.get("/agent-integrations/recall-calendar/status", **kwargs)

    # --- Tenor ---

    def tenor_search(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Search for GIFs via the Tenor API."""
        return self._http.post("/agent-integrations/tenor/search", body=body, **kwargs)

    # --- Twilio ---

    def twilio_call(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Make a call via Twilio."""
        return self._http.post("/agent-integrations/twilio/call", body=body, **kwargs)

    def twilio_incoming_call_webhook(
        self, user_id: str, body: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Twilio webhook for incoming calls."""
        return self._http.post(
            f"/agent-integrations/twilio/webhooks/incoming-call/{enc(user_id)}",
            body=body,
            **kwargs,
        )

    def twilio_status_webhook(
        self, user_id: str, body: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Twilio webhook for call status updates."""
        return self._http.post(
            f"/agent-integrations/twilio/webhooks/status/{enc(user_id)}",
            body=body,
            **kwargs,
        )
