from __future__ import annotations

from tinyhumans.api.channels import ChannelsApi
from helpers import RecordingHttp


def test_send_message_posts_body() -> None:
    http = RecordingHttp({"data": {"messageId": 1}})
    api = ChannelsApi(http)

    result = api.send_message("telegram", {"text": "hi"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/channels/telegram/messages"
    assert call["body"] == {"text": "hi"}
    assert result == {"messageId": 1}


def test_delete_message_encodes_ids() -> None:
    http = RecordingHttp({"data": {}})
    api = ChannelsApi(http)

    api.delete_message("discord", 123)

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/channels/discord/messages/123"


def test_add_reaction_posts_body() -> None:
    http = RecordingHttp({"data": {}})
    api = ChannelsApi(http)

    api.add_reaction("telegram", {"messageId": 1, "emoji": "👍"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/channels/telegram/reactions"


def test_create_thread_posts_body() -> None:
    http = RecordingHttp({"data": {}})
    api = ChannelsApi(http)

    api.create_thread("telegram", {"title": "t"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/channels/telegram/threads"


def test_list_threads_passes_query() -> None:
    http = RecordingHttp({"data": []})
    api = ChannelsApi(http)

    api.list_threads("telegram", {"active": True})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/channels/telegram/threads"
    assert call["query"] == {"active": True}


def test_update_thread_patches() -> None:
    http = RecordingHttp({"data": {}})
    api = ChannelsApi(http)

    api.update_thread("telegram", "th_1", {"action": "close"})

    assert http.last["method"] == "PATCH"
    assert http.last["path"] == "/channels/telegram/threads/th_1"
    assert http.last["body"] == {"action": "close"}


def test_send_typing_optional_body() -> None:
    http = RecordingHttp({"data": {}})
    api = ChannelsApi(http)

    api.send_typing("telegram")

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/channels/telegram/typing"
    assert http.last["body"] is None
