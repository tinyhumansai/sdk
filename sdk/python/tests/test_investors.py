from __future__ import annotations

from tinyhumans.api.investors import InvestorsApi
from helpers import RecordingHttp


def test_get_investor_page_encodes_slug() -> None:
    http = RecordingHttp({"data": {"slug": "seed"}})
    api = InvestorsApi(http)

    result = api.get_investor_page("seed round")

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/investors/seed%20round"
    assert result == {"slug": "seed"}


def test_track_investor_event_posts_body() -> None:
    http = RecordingHttp({"data": {"ok": True}})
    api = InvestorsApi(http)

    body = {"eventType": "PAGE_VIEW", "page": "hero"}
    api.track_investor_event("deck-1", body)

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/investors/deck-1/events"
    assert call["body"] == body
