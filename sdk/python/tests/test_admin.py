from __future__ import annotations

from tinyhumans.api.admin import AdminApi
from helpers import RecordingHttp


def _api() -> tuple[AdminApi, RecordingHttp]:
    http = RecordingHttp({"data": {"ok": True}})
    return AdminApi(http), http


# Analytics


def test_get_analytics_activity() -> None:
    api, http = _api()
    api.get_analytics_activity({"startDate": "2026-01-01"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/activity"
    assert http.last["query"] == {"startDate": "2026-01-01"}


def test_get_analytics_channels() -> None:
    api, http = _api()
    api.get_analytics_channels()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/channels"


def test_get_analytics_dashboard() -> None:
    api, http = _api()
    api.get_analytics_dashboard({"engagementThreshold": 5})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/dashboard"
    assert http.last["query"] == {"engagementThreshold": 5}


def test_get_analytics_backend_events() -> None:
    api, http = _api()
    api.get_analytics_backend_events()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/events/backend"


def test_get_analytics_financials_details() -> None:
    api, http = _api()
    api.get_analytics_financials_details()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/financials/details"


def test_get_analytics_home() -> None:
    api, http = _api()
    api.get_analytics_home()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/home"


def test_get_analytics_inference_usage() -> None:
    api, http = _api()
    api.get_analytics_inference_usage()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/inference/usage"


def test_get_analytics_leaderboard() -> None:
    api, http = _api()
    api.get_analytics_leaderboard({"sortBy": "prompts", "limit": 10})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/leaderboard"
    assert http.last["query"] == {"sortBy": "prompts", "limit": 10}


def test_get_analytics_provider_credits() -> None:
    api, http = _api()
    api.get_analytics_provider_credits()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/analytics/providers/credits"


# Announcements


def test_list_announcements() -> None:
    api, http = _api()
    api.list_announcements({"isActive": "true"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/announcements"
    assert http.last["query"] == {"isActive": "true"}


def test_create_announcement() -> None:
    api, http = _api()
    api.create_announcement({"title": "hi", "body": "there"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/announcements"
    assert http.last["body"] == {"title": "hi", "body": "there"}


def test_get_announcement() -> None:
    api, http = _api()
    api.get_announcement("a b")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/announcements/a%20b"


def test_update_announcement() -> None:
    api, http = _api()
    api.update_announcement("ann_1", {"isActive": False})
    assert http.last["method"] == "PATCH"
    assert http.last["path"] == "/admin/announcements/ann_1"
    assert http.last["body"] == {"isActive": False}


def test_delete_announcement() -> None:
    api, http = _api()
    api.delete_announcement("ann_1")
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/admin/announcements/ann_1"


# Management


def test_list_audit_logs() -> None:
    api, http = _api()
    api.list_audit_logs({"action": "grant"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/audit-logs"
    assert http.last["query"] == {"action": "grant"}


# Coupons


def test_create_coupon() -> None:
    api, http = _api()
    api.create_coupon({"amountUsd": 10})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/coupons"
    assert http.last["body"] == {"amountUsd": 10}


def test_bulk_create_coupons() -> None:
    api, http = _api()
    api.bulk_create_coupons({"count": 5, "amountUsd": 2})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/coupons/bulk"
    assert http.last["body"] == {"count": 5, "amountUsd": 2}


def test_update_coupon() -> None:
    api, http = _api()
    api.update_coupon("cpn_1", {"isActive": True})
    assert http.last["method"] == "PATCH"
    assert http.last["path"] == "/admin/coupons/cpn_1"
    assert http.last["body"] == {"isActive": True}


def test_delete_coupon() -> None:
    api, http = _api()
    api.delete_coupon("cpn_1")
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/admin/coupons/cpn_1"


# Investors


def test_create_investor() -> None:
    api, http = _api()
    api.create_investor({"name": "Acme"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/investors"
    assert http.last["body"] == {"name": "Acme"}


def test_get_investor() -> None:
    api, http = _api()
    api.get_investor("inv_1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/investors/inv_1"


def test_update_investor() -> None:
    api, http = _api()
    api.update_investor("inv_1", {"name": "New"})
    assert http.last["method"] == "PUT"
    assert http.last["path"] == "/admin/investors/inv_1"
    assert http.last["body"] == {"name": "New"}


def test_delete_investor() -> None:
    api, http = _api()
    api.delete_investor("inv_1")
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/admin/investors/inv_1"


def test_get_investor_analytics() -> None:
    api, http = _api()
    api.get_investor_analytics("inv_1", {"startDate": "2026-01-01"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/investors/inv_1/analytics"
    assert http.last["query"] == {"startDate": "2026-01-01"}


def test_list_investor_events() -> None:
    api, http = _api()
    api.list_investor_events("inv_1", {"page": 2})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/investors/inv_1/events"
    assert http.last["query"] == {"page": 2}


# Mascots


def test_list_mascots() -> None:
    api, http = _api()
    api.list_mascots()
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/mascots"


def test_create_mascot() -> None:
    api, http = _api()
    api.create_mascot({"id": "m1", "name": "Mo"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/mascots"
    assert http.last["body"] == {"id": "m1", "name": "Mo"}


def test_update_mascot() -> None:
    api, http = _api()
    api.update_mascot("m1", {"name": "Mo2"})
    assert http.last["method"] == "PUT"
    assert http.last["path"] == "/admin/mascots/m1"
    assert http.last["body"] == {"name": "Mo2"}


def test_delete_mascot() -> None:
    api, http = _api()
    api.delete_mascot("m1")
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/admin/mascots/m1"


# Users


def test_bulk_grant_user_credits() -> None:
    api, http = _api()
    api.bulk_grant_user_credits({"userIds": ["u1"], "credits": 5})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/users/credits/bulk"
    assert http.last["body"] == {"userIds": ["u1"], "credits": 5}


def test_get_admin_user() -> None:
    api, http = _api()
    api.get_admin_user("u1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/users/u1"


def test_get_user_channel_analytics() -> None:
    api, http = _api()
    api.get_user_channel_analytics("u1", {"granularity": "day"})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/users/u1/analytics/channels"
    assert http.last["query"] == {"granularity": "day"}


def test_get_user_usage_analytics() -> None:
    api, http = _api()
    api.get_user_usage_analytics("u1")
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/users/u1/analytics/usage"


def test_grant_user_credits() -> None:
    api, http = _api()
    api.grant_user_credits("u1", {"action": "ADD", "amountUsd": 5, "reason": "x"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/users/u1/credits"
    assert http.last["body"] == {"action": "ADD", "amountUsd": 5, "reason": "x"}


def test_list_user_credit_transactions() -> None:
    api, http = _api()
    api.list_user_credit_transactions("u1", {"page": 1})
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/admin/users/u1/credits/transactions"
    assert http.last["query"] == {"page": 1}


def test_grant_user_subscription() -> None:
    api, http = _api()
    api.grant_user_subscription("u1", {"plan": "pro", "reason": "vip"})
    assert http.last["method"] == "POST"
    assert http.last["path"] == "/admin/users/u1/subscription"
    assert http.last["body"] == {"plan": "pro", "reason": "vip"}


def test_cancel_user_subscription() -> None:
    api, http = _api()
    api.cancel_user_subscription("u1", {"reason": "done"})
    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/admin/users/u1/subscription"
    assert http.last["body"] == {"reason": "done"}
