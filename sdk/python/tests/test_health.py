from __future__ import annotations

from tinyhumans.api.health import HealthApi
from helpers import RecordingHttp


def test_check() -> None:
    http = RecordingHttp({"data": {"status": "ok"}})
    api = HealthApi(http)

    result = api.check()

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/"
    assert result == {"status": "ok"}
