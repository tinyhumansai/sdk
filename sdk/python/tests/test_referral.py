from __future__ import annotations

from tinyhumans.api.referral import ReferralApi
from helpers import RecordingHttp


def test_claim_referral_posts_body() -> None:
    http = RecordingHttp({"data": {"claimed": True}})
    api = ReferralApi(http)

    body = {"code": "abc123"}
    result = api.claim_referral(body)

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/referral/claim"
    assert call["body"] == body
    assert result == {"claimed": True}


def test_get_referral_stats() -> None:
    http = RecordingHttp({"data": {"earnings": 5}})
    api = ReferralApi(http)

    assert api.get_referral_stats() == {"earnings": 5}
    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/referral/stats"
