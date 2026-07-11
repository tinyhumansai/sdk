from __future__ import annotations

from tinyhumans.api.auth import AuthApi
from helpers import RecordingHttp


def test_send_email_link_posts_json_body() -> None:
    http = RecordingHttp({"data": {"sent": True}})
    api = AuthApi(http)

    result = api.send_email_link({"email": "a@b.com"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/auth/email/send-link"
    assert call["body"] == {"email": "a@b.com"}
    assert result == {"sent": True}


def test_verify_email_passes_token_query() -> None:
    http = RecordingHttp({"data": {"ok": True}})
    api = AuthApi(http)

    api.verify_email("tok_123")

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/auth/email/verify"
    assert call["query"] == {"token": "tok_123"}


def test_me_returns_unwrapped_user() -> None:
    http = RecordingHttp({"data": {"id": "user_1"}})
    api = AuthApi(http)

    assert api.me() == {"id": "user_1"}


def test_channel_link_token_encodes_path() -> None:
    http = RecordingHttp({"data": {"token": "lt_1"}})
    api = AuthApi(http)

    api.create_channel_link_token("telegram")

    assert http.last["path"] == "/auth/channels/telegram/link-token"
    assert http.last["method"] == "POST"


def test_oauth_connect_query() -> None:
    http = RecordingHttp({"data": {}})
    api = AuthApi(http)

    api.oauth_connect("google", {"skillId": "skill_9"})

    assert http.last["path"] == "/auth/google/connect"
    assert http.last["query"] == {"skillId": "skill_9"}
