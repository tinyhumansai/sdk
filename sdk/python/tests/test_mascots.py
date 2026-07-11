from __future__ import annotations

from tinyhumans.api.mascots import MascotsApi
from helpers import RecordingHttp


def test_list_mascots() -> None:
    http = RecordingHttp({"data": []})
    api = MascotsApi(http)

    api.list_mascots()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/mascots"


def test_get_demo() -> None:
    http = RecordingHttp({"body": "<html></html>"})
    api = MascotsApi(http)

    result = api.get_demo()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/mascots/demo"
    assert result == "<html></html>"


def test_join_meeting_posts_body() -> None:
    http = RecordingHttp({"data": {"joined": True}})
    api = MascotsApi(http)

    api.join_meeting({"meetUrl": "https://meet.example/xyz"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/mascots/join-meeting"
    assert http.last["body"] == {"meetUrl": "https://meet.example/xyz"}


def test_list_meetings_passes_query() -> None:
    http = RecordingHttp({"data": []})
    api = MascotsApi(http)

    api.list_meetings({"limit": 10})

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/mascots/meetings"
    assert http.last["query"] == {"limit": 10}


def test_get_mascot_encodes_path() -> None:
    http = RecordingHttp({"data": {"id": "m 1"}})
    api = MascotsApi(http)

    api.get_mascot("m 1")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/mascots/m%201"


def test_get_mascot_riv_encodes_path() -> None:
    http = RecordingHttp({"data": {}})
    api = MascotsApi(http)

    api.get_mascot_riv("m 1")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/mascots/m%201/riv"
