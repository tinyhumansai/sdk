from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["InviteApi"]


class InviteApi(ApiNamespace):
    """Invite codes: campaign management (admin), personal codes, redemption, and status checks."""

    def create_campaign_invite(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a campaign invite code (admin only)."""
        return self._http.post("/invite/campaign", body=body, **kwargs)

    def list_campaign_invites(self, **kwargs: Any) -> Json:
        """List all campaign invite codes (admin only)."""
        return self._http.get("/invite/campaign", **kwargs)

    def delete_campaign_invite(self, code_id: str, **kwargs: Any) -> Json:
        """Deactivate a campaign invite code (admin only)."""
        return self._http.delete(f"/invite/campaign/{enc(code_id)}", **kwargs)

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
