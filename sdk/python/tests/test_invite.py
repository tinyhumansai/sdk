from __future__ import annotations

from tinyhumans.api.invite import InviteApi
from helpers import RecordingHttp


def test_create_campaign_invite_posts_body() -> None:
    http = RecordingHttp({"data": {"code": "ABC"}})
    api = InviteApi(http)

    result = api.create_campaign_invite({"maxUses": 5})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/invite/campaign"
    assert call["body"] == {"maxUses": 5}
    assert result == {"code": "ABC"}


def test_list_campaign_invites() -> None:
    http = RecordingHttp({"data": []})
    api = InviteApi(http)

    api.list_campaign_invites()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/invite/campaign"


def test_delete_campaign_invite_encodes_path() -> None:
    http = RecordingHttp({"data": {}})
    api = InviteApi(http)

    api.delete_campaign_invite("code id/1")

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/invite/campaign/code%20id%2F1"


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
