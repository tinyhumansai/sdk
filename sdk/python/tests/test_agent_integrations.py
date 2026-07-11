from __future__ import annotations

from tinyhumans.api.agent_integrations import AgentIntegrationsApi
from helpers import RecordingHttp


def make() -> tuple[AgentIntegrationsApi, RecordingHttp]:
    http = RecordingHttp({"data": {"ok": True}})
    return AgentIntegrationsApi(http), http


# --- Apify ---

def test_run_apify_actor() -> None:
    api, http = make()
    api.run_apify_actor({"actorId": "a1", "input": {}})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/apify/run"
    assert http.last["body"] == {"actorId": "a1", "input": {}}


def test_get_apify_run() -> None:
    api, http = make()
    api.get_apify_run("run 1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/apify/runs/run%201"


def test_get_apify_run_results() -> None:
    api, http = make()
    api.get_apify_run_results("r1", {"limit": 10})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/apify/runs/r1/results"
    assert http.last["query"] == {"limit": 10}


# --- Composio ---

def test_authorize_composio() -> None:
    api, http = make()
    api.authorize_composio({"toolkit": "gmail"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/composio/authorize"
    assert http.last["body"] == {"toolkit": "gmail"}


def test_list_composio_connections() -> None:
    api, http = make()
    api.list_composio_connections()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/composio/connections"


def test_delete_composio_connection() -> None:
    api, http = make()
    api.delete_composio_connection("conn 1")
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/agent-integrations/composio/connections/conn%201"


def test_execute_composio_tool() -> None:
    api, http = make()
    api.execute_composio_tool({"tool": "GMAIL_SEND_EMAIL"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/composio/execute"
    assert http.last["body"] == {"tool": "GMAIL_SEND_EMAIL"}


def test_list_composio_toolkits() -> None:
    api, http = make()
    api.list_composio_toolkits()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/composio/toolkits"


def test_refresh_composio_toolkits() -> None:
    api, http = make()
    api.refresh_composio_toolkits({"full": True})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/composio/toolkits/refresh"
    assert http.last["query"] == {"full": True}


def test_list_composio_tools() -> None:
    api, http = make()
    api.list_composio_tools({"toolkits": "gmail"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/composio/tools"
    assert http.last["query"] == {"toolkits": "gmail"}


def test_list_composio_triggers() -> None:
    api, http = make()
    api.list_composio_triggers()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/composio/triggers"


def test_create_composio_trigger() -> None:
    api, http = make()
    api.create_composio_trigger({"connectionId": "c1", "slug": "s1"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/composio/triggers"
    assert http.last["body"] == {"connectionId": "c1", "slug": "s1"}


def test_list_composio_available_triggers() -> None:
    api, http = make()
    api.list_composio_available_triggers({"toolkit": "gmail"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/composio/triggers/available"
    assert http.last["query"] == {"toolkit": "gmail"}


def test_delete_composio_trigger() -> None:
    api, http = make()
    api.delete_composio_trigger("trig 1")
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/agent-integrations/composio/triggers/trig%201"


# --- Crypto ---

def test_crypto_bridge() -> None:
    api, http = make()
    api.crypto_bridge({"srcChainId": 1})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/crypto/bridge"
    assert http.last["body"] == {"srcChainId": 1}


def test_list_crypto_routes() -> None:
    api, http = make()
    api.list_crypto_routes()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/crypto/routes"


def test_crypto_swap() -> None:
    api, http = make()
    api.crypto_swap({"chainId": 1})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/crypto/swap"
    assert http.last["body"] == {"chainId": 1}


# --- Financial APIs ---

def test_financial_apis_commodity() -> None:
    api, http = make()
    api.financial_apis_commodity({"commodity": "WTI"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/financial-apis/commodity"
    assert http.last["body"] == {"commodity": "WTI"}


def test_financial_apis_crypto_series() -> None:
    api, http = make()
    api.financial_apis_crypto_series({"symbol": "BTC"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/financial-apis/crypto-series"


def test_financial_apis_exchange_rate() -> None:
    api, http = make()
    api.financial_apis_exchange_rate({"fromCurrency": "BTC", "toCurrency": "USD"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/financial-apis/exchange-rate"


def test_financial_apis_options() -> None:
    api, http = make()
    api.financial_apis_options({"symbol": "AAPL"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/financial-apis/options"


def test_financial_apis_quote() -> None:
    api, http = make()
    api.financial_apis_quote({"symbol": "AAPL"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/financial-apis/quote"


# --- Google Places ---

def test_google_places_details() -> None:
    api, http = make()
    api.google_places_details({"placeId": "p1"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/google-places/details"


def test_google_places_search() -> None:
    api, http = make()
    api.google_places_search({"query": "coffee"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/google-places/search"


# --- Media Generation ---

def test_media_generation_images() -> None:
    api, http = make()
    api.media_generation_images({"prompt": "cat"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/media-generation/images"
    assert http.last["body"] == {"prompt": "cat"}


def test_list_media_generation_models() -> None:
    api, http = make()
    api.list_media_generation_models({"includeUpstream": True})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/media-generation/models"
    assert http.last["query"] == {"includeUpstream": True}


def test_get_media_generation_request() -> None:
    api, http = make()
    api.get_media_generation_request("req 1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/media-generation/requests/req%201"


def test_media_generation_videos() -> None:
    api, http = make()
    api.media_generation_videos({"prompt": "cat"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/media-generation/videos"


# --- Parallel ---

def test_parallel_chat() -> None:
    api, http = make()
    api.parallel_chat({"model": "speed", "messages": []})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/parallel/chat"


def test_parallel_dataset() -> None:
    api, http = make()
    api.parallel_dataset({"objective": "x"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/parallel/dataset"


def test_get_parallel_dataset() -> None:
    api, http = make()
    api.get_parallel_dataset("fa 1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/parallel/dataset/fa%201"


def test_get_parallel_dataset_result() -> None:
    api, http = make()
    api.get_parallel_dataset_result("fa 1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/parallel/dataset/fa%201/result"


def test_parallel_enrich() -> None:
    api, http = make()
    api.parallel_enrich({"input": "x", "processor": "lite", "outputSchema": {}})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/parallel/enrich"


def test_parallel_extract() -> None:
    api, http = make()
    api.parallel_extract({"urls": ["https://x.com"]})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/parallel/extract"


def test_parallel_research() -> None:
    api, http = make()
    api.parallel_research({"input": "x", "processor": "lite"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/parallel/research"


def test_get_parallel_research() -> None:
    api, http = make()
    api.get_parallel_research("run 1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/parallel/research/run%201"


def test_get_parallel_research_result() -> None:
    api, http = make()
    api.get_parallel_research_result("run 1", {"timeoutSeconds": 30})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/parallel/research/run%201/result"
    assert http.last["query"] == {"timeoutSeconds": 30}


def test_parallel_search() -> None:
    api, http = make()
    api.parallel_search({"objective": "x", "searchQueries": []})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/parallel/search"


# --- Pricing ---

def test_get_pricing() -> None:
    api, http = make()
    api.get_pricing()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/pricing"


# --- Recall Calendar ---

def test_connect_recall_calendar() -> None:
    api, http = make()
    api.connect_recall_calendar()
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/recall-calendar/connect"


def test_disconnect_recall_calendar() -> None:
    api, http = make()
    api.disconnect_recall_calendar()
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/recall-calendar/disconnect"


def test_list_recall_calendar_meetings() -> None:
    api, http = make()
    api.list_recall_calendar_meetings()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/recall-calendar/meetings"


def test_recall_calendar_oauth_complete() -> None:
    api, http = make()
    api.recall_calendar_oauth_complete({"t": "tok", "status": "success"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/recall-calendar/oauth-complete"
    assert http.last["query"] == {"t": "tok", "status": "success"}


def test_get_recall_calendar_status() -> None:
    api, http = make()
    api.get_recall_calendar_status()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/agent-integrations/recall-calendar/status"


# --- Tenor ---

def test_tenor_search() -> None:
    api, http = make()
    api.tenor_search({"query": "cat"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/tenor/search"
    assert http.last["body"] == {"query": "cat"}


# --- Twilio ---

def test_twilio_call() -> None:
    api, http = make()
    api.twilio_call({"to": "+15551234567"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/twilio/call"
    assert http.last["body"] == {"to": "+15551234567"}


def test_twilio_incoming_call_webhook() -> None:
    api, http = make()
    api.twilio_incoming_call_webhook("user 1", {"From": "+1"})
    assert http.last["method"] == "POST"
    assert (
        http.last["path"]
        == "/agent-integrations/twilio/webhooks/incoming-call/user%201"
    )
    assert http.last["body"] == {"From": "+1"}


def test_twilio_status_webhook() -> None:
    api, http = make()
    api.twilio_status_webhook("user 1")
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/agent-integrations/twilio/webhooks/status/user%201"
    assert http.last["body"] is None
