from __future__ import annotations

from tinyhumans.api.invite import InviteApi
from helpers import RecordingHttp


def test_list_my_codes() -> None:
    http = RecordingHttp({"data": []})
    api = InviteApi(http)

    api.list_my_codes()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/invite/my-codes"


def test_redeem_invite_posts_body() -> None:
    http = RecordingHttp({"data": {"ok": True}})
    api = InviteApi(http)

    api.redeem_invite({"code": "XYZ"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/invite/redeem"
    assert http.last["body"] == {"code": "XYZ"}


def test_get_invite_status_passes_query() -> None:
    http = RecordingHttp({"data": {"valid": True}})
    api = InviteApi(http)

    api.get_invite_status({"code": "XYZ"})

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/invite/status"
    assert http.last["query"] == {"code": "XYZ"}
