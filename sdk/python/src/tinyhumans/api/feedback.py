from __future__ import annotations

from typing import Any

from ..http import Json
from ..types import FeedbackIngest
from ._base import ApiNamespace, enc

__all__ = ["FeedbackApi"]


class FeedbackApi(ApiNamespace):
    """Public feedback board: submit, browse, read, vote, and comment."""

    def create_feedback(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Submit feedback or a bug report (LLM-moderated, rate-limited)."""
        return self._http.post("/feedback", body=body, **kwargs)

    def ingest(self, body: FeedbackIngest, **kwargs: Any) -> Json:
        """Ingest feedback from another TinyHumans product."""
        return self._http.post("/feedback/ingest", body=body, **kwargs)

    def list_feedback(self, query: dict[str, Any] | None = None, **kwargs: Any) -> Json:
        """List feedback on the public board."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/feedback", query=merged, **kwargs)

    def get_feedback(self, id: str, **kwargs: Any) -> Json:
        """Get a feedback item with its comments."""
        return self._http.get(f"/feedback/{enc(id)}", **kwargs)

    def comment_feedback(self, id: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Comment on a feedback item."""
        return self._http.post(f"/feedback/{enc(id)}/comments", body=body, **kwargs)

    def vote_feedback(self, id: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Up/down-vote a feedback item (value 0 retracts)."""
        return self._http.post(f"/feedback/{enc(id)}/vote", body=body, **kwargs)
