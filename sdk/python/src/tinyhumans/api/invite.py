from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["InviteApi"]


class InviteApi(ApiNamespace):
    """Personal invite codes, redemption, and status checks."""

    def list_my_codes(self, **kwargs: Any) -> Json:
        """List the current user's invite codes with usage info."""
        return self._http.get("/invite/my-codes", **kwargs)

    def redeem_invite(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Redeem an invite code."""
        return self._http.post("/invite/redeem", body=body, **kwargs)

    def get_invite_status(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Check if an invite code is valid and available."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/invite/status", query=merged, **kwargs)
