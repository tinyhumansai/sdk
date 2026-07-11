from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["AnnouncementsApi"]


class AnnouncementsApi(ApiNamespace):
    """Active in-app announcements for the signed-in user."""

    def get_latest_announcements(self, **kwargs: Any) -> Json:
        """Get the latest active announcement for the signed-in user."""
        return self._http.get("/announcements/latest", **kwargs)
