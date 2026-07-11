from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["InvestorsApi"]


class InvestorsApi(ApiNamespace):
    """Public investor deck/landing page access plus engagement event tracking."""

    def get_investor_page(self, slug: str, **kwargs: Any) -> Json:
        """Get an investor deck by slug."""
        return self._http.get(f"/investors/{enc(slug)}", **kwargs)

    def track_investor_event(
        self, slug: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Track an investor page event."""
        return self._http.post(f"/investors/{enc(slug)}/events", body=body, **kwargs)
