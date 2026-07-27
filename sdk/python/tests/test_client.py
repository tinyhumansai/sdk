from __future__ import annotations

from tinyhumans.client import TinyHumansClient
from tinyhumans.http import HttpClient
from tinyhumans.api.auth import AuthApi
from tinyhumans.api.health import HealthApi
from tinyhumans.api.teams import TeamsApi
from tinyhumans.api.webhooks import WebhooksApi
from helpers import RecordingHttp


NAMESPACES = [
    "agent_integrations",
    "announcements",
    "auth",
    "channels",
    "coupons",
    "feedback",
    "health",
    "inference",
    "invite",
    "mascots",
    "payments",
    "redirect",
    "referral",
    "rewards",
    "teams",
    "webhooks",
]


def _client() -> TinyHumansClient:
    return TinyHumansClient(base_url="https://api.tinyhumans.ai")


def test_raw_is_http_client() -> None:
    client = _client()
    assert isinstance(client.raw, HttpClient)
    assert client.raw.base_url == "https://api.tinyhumans.ai"


def test_all_namespaces_present() -> None:
    client = _client()
    assert len(NAMESPACES) == 16
    for name in NAMESPACES:
        assert hasattr(client, name), name
        assert getattr(client, name) is not None


def test_namespace_types() -> None:
    client = _client()
    assert isinstance(client.auth, AuthApi)
    assert isinstance(client.health, HealthApi)
    assert isinstance(client.teams, TeamsApi)
    assert isinstance(client.webhooks, WebhooksApi)


def test_constructor_forwards_options() -> None:
    client = TinyHumansClient(
        base_url="https://api.tinyhumans.ai",
        token="tok",
        api_key="key",
        headers={"x-custom": "c"},
        unwrap_envelope=False,
        timeout=12.5,
    )
    raw = client.raw
    assert raw.token == "tok"
    assert raw.api_key == "key"
    assert raw.headers == {"x-custom": "c"}
    assert raw.unwrap_envelope is False
    assert raw.timeout == 12.5


def test_swagger_requests_unwrapped() -> None:
    client = _client()
    client.raw = RecordingHttp({"body": {"openapi": "3.0.0"}})

    result = client.swagger()

    call = client.raw.last
    assert call["method"] == "GET"
    assert call["path"] == "/swagger.json"
    assert call["unwrap_envelope"] is False
    assert result == {"openapi": "3.0.0"}
