from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["HealthApi"]


class HealthApi(ApiNamespace):
    """Service health check."""

    def check(self, **kwargs: Any) -> Json:
        """Health check endpoint. Returns server uptime and status information."""
        return self._http.get("/", **kwargs)
