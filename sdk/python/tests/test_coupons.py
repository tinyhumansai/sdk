from __future__ import annotations

from tinyhumans.api.coupons import CouponsApi
from helpers import RecordingHttp


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
