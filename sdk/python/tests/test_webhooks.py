from __future__ import annotations

from tinyhumans.api.webhooks import WebhooksApi
from helpers import RecordingHttp


def test_receive_composio_webhook_posts_body() -> None:
    http = RecordingHttp({"data": {"ok": True}})
    api = WebhooksApi(http)

    result = api.receive_composio_webhook({"event": "trigger"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/webhooks/composio"
    assert call["body"] == {"event": "trigger"}
    assert result == {"ok": True}


def test_create_core_webhook_posts_body() -> None:
    http = RecordingHttp({"data": {"id": "wh_1"}})
    api = WebhooksApi(http)

    api.create_core_webhook({"name": "hook"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/webhooks/core"
    assert http.last["body"] == {"name": "hook"}


def test_list_core_webhooks() -> None:
    http = RecordingHttp({"data": []})
    api = WebhooksApi(http)

    api.list_core_webhooks()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/webhooks/core"


def test_get_core_webhook_bandwidth() -> None:
    http = RecordingHttp({"data": {"remainingBudgetUsd": 1.5}})
    api = WebhooksApi(http)

    api.get_core_webhook_bandwidth()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/webhooks/core/bandwidth"


def test_get_core_webhook_encodes_path() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.get_core_webhook("a/b")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/webhooks/core/a%2Fb"


def test_update_core_webhook_patches() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.update_core_webhook("wh_1", {"isActive": False})

    assert http.last["method"] == "PATCH"
    assert http.last["path"] == "/webhooks/core/wh_1"
    assert http.last["body"] == {"isActive": False}


def test_delete_core_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.delete_core_webhook("wh_1")

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/webhooks/core/wh_1"


def test_receive_discord_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_discord_webhook({"type": 1})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/webhooks/discord"


def test_receive_github_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_github_webhook({"action": "opened"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/webhooks/github"


def test_forward_webhook_ingress_optional_body() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.forward_webhook_ingress("uuid-1")

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/webhooks/ingress/uuid-1"
    assert http.last["body"] is None


def test_forward_webhook_ingress_with_path_encodes() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.forward_webhook_ingress_with_path("uuid-1", "a/b", {"x": 1})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/webhooks/ingress/uuid-1/a%2Fb"
    assert http.last["body"] == {"x": 1}


def test_receive_coinbase_payment_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_coinbase_payment_webhook({})

    assert http.last["path"] == "/webhooks/payments/coinbase"


def test_receive_stripe_payment_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_stripe_payment_webhook({})

    assert http.last["path"] == "/webhooks/payments/stripe"


def test_receive_sentry_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_sentry_webhook({})

    assert http.last["path"] == "/webhooks/sentry"


def test_receive_telegram_webhook() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_telegram_webhook({"update_id": 1})

    assert http.last["path"] == "/webhooks/telegram"


def test_receive_managed_telegram_webhook_encodes_bot_id() -> None:
    http = RecordingHttp({"data": {}})
    api = WebhooksApi(http)

    api.receive_managed_telegram_webhook(42, {"update_id": 1})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/webhooks/telegram/managed/42"
    assert http.last["body"] == {"update_id": 1}
