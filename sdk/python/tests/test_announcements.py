from __future__ import annotations

from tinyhumans.api.announcements import AnnouncementsApi
from helpers import RecordingHttp


def test_get_latest_announcements() -> None:
    http = RecordingHttp({"data": {"id": "ann_1", "title": "Hello"}})
    api = AnnouncementsApi(http)

    result = api.get_latest_announcements()

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/announcements/latest"
    assert result == {"id": "ann_1", "title": "Hello"}
