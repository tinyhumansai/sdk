from __future__ import annotations

from tinyhumans.api.teams import TeamsApi
from helpers import RecordingHttp


def test_list_teams() -> None:
    http = RecordingHttp({"data": []})
    api = TeamsApi(http)

    assert api.list_teams() == []
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/teams"


def test_join_team() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.join_team({"code": "inv_1"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/teams/join"
    assert call["body"] == {"code": "inv_1"}


def test_get_my_usage() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.get_my_usage()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/teams/me/usage"


def test_get_team() -> None:
    http = RecordingHttp({"data": {"id": "t_1"}})
    api = TeamsApi(http)

    api.get_team("t 1/x")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/teams/t%201%2Fx"


def test_update_team() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.update_team("t_1", {"name": "New"})

    call = http.last
    assert call["method"] == "PUT"
    assert call["path"] == "/teams/t_1"
    assert call["body"] == {"name": "New"}


def test_get_billing_plan() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.get_billing_plan("t_1")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/teams/t_1/billing/plan"


def test_create_billing_portal() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.create_billing_portal("t_1", {"returnUrl": "https://x"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/teams/t_1/billing/portal"
    assert call["body"] == {"returnUrl": "https://x"}


def test_purchase_plan() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.purchase_plan("t_1", {"plan": "PRO_YEARLY"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/teams/t_1/billing/purchase"
    assert call["body"] == {"plan": "PRO_YEARLY"}


def test_create_invite() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.create_invite("t_1", {"maxUses": 5})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/teams/t_1/invites"
    assert call["body"] == {"maxUses": 5}


def test_list_invites() -> None:
    http = RecordingHttp({"data": []})
    api = TeamsApi(http)

    api.list_invites("t_1")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/teams/t_1/invites"


def test_send_email_invite() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.send_email_invite("t_1", {"email": "a@b.com"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/teams/t_1/invites/email"
    assert call["body"] == {"email": "a@b.com"}


def test_revoke_invite() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.revoke_invite("t_1", "inv_1")

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/teams/t_1/invites/inv_1"


def test_leave_team() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.leave_team("t_1")

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/teams/t_1/leave"


def test_list_members() -> None:
    http = RecordingHttp({"data": []})
    api = TeamsApi(http)

    api.list_members("t_1")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/teams/t_1/members"


def test_remove_member() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.remove_member("t_1", "u_1")

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/teams/t_1/members/u_1"


def test_update_member_role() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.update_member_role("t_1", "u_1", {"role": "admin"})

    call = http.last
    assert call["method"] == "PUT"
    assert call["path"] == "/teams/t_1/members/u_1/role"
    assert call["body"] == {"role": "admin"}


def test_switch_team() -> None:
    http = RecordingHttp({"data": {}})
    api = TeamsApi(http)

    api.switch_team("t_1")

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/teams/t_1/switch"
