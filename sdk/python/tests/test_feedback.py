from __future__ import annotations

from tinyhumans.api.feedback import FeedbackApi
from helpers import RecordingHttp


def test_create_feedback_posts_body() -> None:
    http = RecordingHttp({"data": {"id": "f_1"}})
    api = FeedbackApi(http)

    result = api.create_feedback({"type": "bug", "title": "t", "body": "b"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/feedback"
    assert call["body"] == {"type": "bug", "title": "t", "body": "b"}
    assert result == {"id": "f_1"}


def test_list_feedback_passes_query() -> None:
    http = RecordingHttp({"data": []})
    api = FeedbackApi(http)

    api.list_feedback({"sort": "hot", "page": 1})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/feedback"
    assert call["query"] == {"sort": "hot", "page": 1}


def test_get_feedback_encodes_path() -> None:
    http = RecordingHttp({"data": {}})
    api = FeedbackApi(http)

    api.get_feedback("f/1")

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/feedback/f%2F1"


def test_comment_feedback_posts_body() -> None:
    http = RecordingHttp({"data": {}})
    api = FeedbackApi(http)

    api.comment_feedback("f_1", {"body": "nice"})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/feedback/f_1/comments"
    assert http.last["body"] == {"body": "nice"}


def test_update_feedback_status_patches() -> None:
    http = RecordingHttp({"data": {}})
    api = FeedbackApi(http)

    api.update_feedback_status("f_1", {"status": "planned"})

    assert http.last["method"] == "PATCH"
    assert http.last["path"] == "/feedback/f_1/status"
    assert http.last["body"] == {"status": "planned"}


def test_vote_feedback_posts_body() -> None:
    http = RecordingHttp({"data": {}})
    api = FeedbackApi(http)

    api.vote_feedback("f_1", {"value": 1})

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/feedback/f_1/vote"
    assert http.last["body"] == {"value": 1}
