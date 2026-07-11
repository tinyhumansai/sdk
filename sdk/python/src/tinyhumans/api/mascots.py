from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["MascotsApi"]


class MascotsApi(ApiNamespace):
    """Mascots: library, manifests, Rive assets, the interactive demo, and meeting dispatch."""

    def list_mascots(self, **kwargs: Any) -> Json:
        """List available mascots."""
        return self._http.get("/mascots", **kwargs)

    def get_demo(self, **kwargs: Any) -> Json:
        """Interactive mascot library demo page."""
        return self._http.get("/mascots/demo", **kwargs)

    def join_meeting(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Send the mascot bot into a live meeting."""
        return self._http.post("/mascots/join-meeting", body=body, **kwargs)

    def list_meetings(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List the authenticated user's mascot-bot meeting history."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/mascots/meetings", query=merged, **kwargs)

    def get_mascot(self, id: str, **kwargs: Any) -> Json:
        """Get a mascot manifest."""
        return self._http.get(f"/mascots/{enc(id)}", **kwargs)

    def get_mascot_riv(self, id: str, **kwargs: Any) -> Json:
        """Download the Rive animation file for a mascot."""
        return self._http.get(f"/mascots/{enc(id)}/riv", **kwargs)
