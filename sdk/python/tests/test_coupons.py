from __future__ import annotations

from tinyhumans.api.coupons import CouponsApi
from helpers import RecordingHttp


def test_create_coupon_posts_body() -> None:
    http = RecordingHttp({"data": {"code": "ABCD-EFGH"}})
    api = CouponsApi(http)

    result = api.create_coupon({"amountUsd": 10})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/coupons/admin"
    assert call["body"] == {"amountUsd": 10}
    assert result == {"code": "ABCD-EFGH"}


def test_list_coupons_passes_query() -> None:
    http = RecordingHttp({"data": []})
    api = CouponsApi(http)

    api.list_coupons({"isActive": True, "page": 2})

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/coupons/admin"
    assert http.last["query"] == {"isActive": True, "page": 2}


def test_delete_coupon_encodes_path() -> None:
    http = RecordingHttp({"data": {}})
    api = CouponsApi(http)

    api.delete_coupon("cpn 1/x")

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/coupons/admin/cpn%201%2Fx"


def test_list_my_coupons() -> None:
    http = RecordingHttp({"data": []})
    api = CouponsApi(http)

    api.list_my_coupons()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/coupons/me"


def test_redeem_coupon_posts_body() -> None:
    http = RecordingHttp({"data": {"ok": True}})
    api = CouponsApi(http)

    api.redeem_coupon({"code": "ABCD-EFGH"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/coupons/redeem"
    assert http.last["body"] == {"code": "ABCD-EFGH"}
