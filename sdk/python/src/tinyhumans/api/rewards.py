from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["RewardsApi"]


class RewardsApi(ApiNamespace):
    """Backend-backed rewards snapshot and connected-account management."""

    def unlink_discord(self, **kwargs: Any) -> Json:
        """Disconnect (unlink) the authenticated user's Discord account."""
        return self._http.delete("/rewards/discord", **kwargs)

    def get_my_rewards(self, **kwargs: Any) -> Json:
        """Get the authenticated user's backend-backed rewards snapshot."""
        return self._http.get("/rewards/me", **kwargs)

    def claim(self, reward_type: str, **kwargs: Any) -> Json:
        """Claim a reward or achievement."""
        return self._http.post(
            "/rewards/claim", body={"rewardType": reward_type}, **kwargs
        )
