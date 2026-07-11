from __future__ import annotations

from tinyhumans.api.redirect import RedirectApi
from helpers import RecordingHttp


def test_resolve_redirect_encodes_code() -> None:
    http = RecordingHttp({"data": {"url": "https://example.com"}})
    api = RedirectApi(http)

    result = api.resolve_redirect("ab/cd")

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/r/ab%2Fcd"
    assert result == {"url": "https://example.com"}
