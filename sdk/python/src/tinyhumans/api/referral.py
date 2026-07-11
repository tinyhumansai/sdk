from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["ReferralApi"]


class ReferralApi(ApiNamespace):
    """Referral link claiming and earnings/referral-list statistics."""

    def claim_referral(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Claim a referral link for the authenticated user."""
        return self._http.post("/referral/claim", body=body, **kwargs)

    def get_referral_stats(self, **kwargs: Any) -> Json:
        """Fetch referral link, earnings summary, and referral list."""
        return self._http.get("/referral/stats", **kwargs)
