from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["TeamsApi"]


class TeamsApi(ApiNamespace):
    """Team-scoped membership and access: listing teams, usage insights, invites,
    joining/leaving/switching teams, member management, and billing plan helpers."""

    def list_teams(self, **kwargs: Any) -> Json:
        """List all teams the authenticated user belongs to, with their role in each."""
        return self._http.get("/teams", **kwargs)

    def join_team(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Join a team using an invite code."""
        return self._http.post("/teams/join", body=body, **kwargs)

    def get_my_usage(self, **kwargs: Any) -> Json:
        """Get usage and spend insights plus remaining budget for the current cycle."""
        return self._http.get("/teams/me/usage", **kwargs)

    def get_team(self, team_id: str, **kwargs: Any) -> Json:
        """Get details for a single team."""
        return self._http.get(f"/teams/{enc(team_id)}", **kwargs)

    def get_billing_plan(self, team_id: str, **kwargs: Any) -> Json:
        """Get the team's current subscription plan."""
        return self._http.get(f"/teams/{enc(team_id)}/billing/plan", **kwargs)

    def create_billing_portal(
        self, team_id: str, body: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Create a Stripe billing portal session for the team."""
        return self._http.post(
            f"/teams/{enc(team_id)}/billing/portal", body=body, **kwargs
        )

    def purchase_plan(
        self, team_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Purchase a subscription plan for the team (creates a checkout session)."""
        return self._http.post(
            f"/teams/{enc(team_id)}/billing/purchase", body=body, **kwargs
        )

    def create_invite(
        self, team_id: str, body: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Create a shareable team invite."""
        return self._http.post(
            f"/teams/{enc(team_id)}/invites", body=body, **kwargs
        )

    def list_invites(self, team_id: str, **kwargs: Any) -> Json:
        """List a team's invites."""
        return self._http.get(f"/teams/{enc(team_id)}/invites", **kwargs)

    def send_email_invite(
        self, team_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Send a team invite via email."""
        return self._http.post(
            f"/teams/{enc(team_id)}/invites/email", body=body, **kwargs
        )

    def revoke_invite(
        self, team_id: str, invite_id: str, **kwargs: Any
    ) -> Json:
        """Revoke a team invite."""
        return self._http.delete(
            f"/teams/{enc(team_id)}/invites/{enc(invite_id)}", **kwargs
        )

    def leave_team(self, team_id: str, **kwargs: Any) -> Json:
        """Leave a team."""
        return self._http.post(f"/teams/{enc(team_id)}/leave", **kwargs)

    def list_members(self, team_id: str, **kwargs: Any) -> Json:
        """List a team's members."""
        return self._http.get(f"/teams/{enc(team_id)}/members", **kwargs)

    def switch_team(self, team_id: str, **kwargs: Any) -> Json:
        """Switch the user's active team."""
        return self._http.post(f"/teams/{enc(team_id)}/switch", **kwargs)
