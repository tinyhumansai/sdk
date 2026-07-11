from __future__ import annotations

from tinyhumans.api.rewards import RewardsApi
from helpers import RecordingHttp


def test_unlink_discord() -> None:
    http = RecordingHttp({"data": {"unlinked": True}})
    api = RewardsApi(http)

    result = api.unlink_discord()

    call = http.last
    assert call["method"] == "DELETE"
    assert call["path"] == "/rewards/discord"
    assert result == {"unlinked": True}


def test_get_my_rewards() -> None:
    http = RecordingHttp({"data": {"points": 10}})
    api = RewardsApi(http)

    assert api.get_my_rewards() == {"points": 10}
    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/rewards/me"
