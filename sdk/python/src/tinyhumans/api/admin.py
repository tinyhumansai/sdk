from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["AdminApi"]


class AdminApi(ApiNamespace):
    """Admin-only routes: analytics, announcements, investors, coupons, mascots,
    user credit/subscription management, and audit logs."""

    # Analytics

    def get_analytics_activity(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Get admin activity analytics."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/activity", query=merged, **kwargs)

    def get_analytics_channels(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Messaging channel analytics — volume, day-by-day series, top users."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/channels", query=merged, **kwargs)

    def get_analytics_dashboard(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Dashboard summary — DAU, MAU, events, API performance, feature usage."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/dashboard", query=merged, **kwargs)

    def get_analytics_backend_events(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Get backend event analytics."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/events/backend", query=merged, **kwargs)

    def get_analytics_financials_details(self, **kwargs: Any) -> Json:
        """Recent paying users (subscriptions) and topups."""
        return self._http.get("/admin/analytics/financials/details", **kwargs)

    def get_analytics_home(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Home dashboard metrics — signups, active users, prompts, tokens."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/home", query=merged, **kwargs)

    def get_analytics_inference_usage(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Inference usage analytics — prompts, tokens, cost, revenue, top users."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/inference/usage", query=merged, **kwargs)

    def get_analytics_leaderboard(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Active-user leaderboard ranked by active days, prompts, tokens, or cost."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/leaderboard", query=merged, **kwargs)

    def get_analytics_provider_credits(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Provider credit/limit health snapshot."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/analytics/providers/credits", query=merged, **kwargs)

    # Announcements

    def list_announcements(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List announcements."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/announcements", query=merged, **kwargs)

    def create_announcement(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create an announcement."""
        return self._http.post("/admin/announcements", body=body, **kwargs)

    def get_announcement(self, announcement_id: str, **kwargs: Any) -> Json:
        """Get an announcement by id."""
        return self._http.get(f"/admin/announcements/{enc(announcement_id)}", **kwargs)

    def update_announcement(
        self, announcement_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Update an announcement."""
        return self._http.patch(
            f"/admin/announcements/{enc(announcement_id)}", body=body, **kwargs
        )

    def delete_announcement(self, announcement_id: str, **kwargs: Any) -> Json:
        """Delete an announcement."""
        return self._http.delete(f"/admin/announcements/{enc(announcement_id)}", **kwargs)

    # Management

    def list_audit_logs(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List admin audit-log entries."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/admin/audit-logs", query=merged, **kwargs)

    # Coupons

    def create_coupon(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a coupon."""
        return self._http.post("/admin/coupons", body=body, **kwargs)

    def bulk_create_coupons(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Bulk-create coupons."""
        return self._http.post("/admin/coupons/bulk", body=body, **kwargs)

    def update_coupon(self, coupon_id: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Update a coupon's expiry/active flag."""
        return self._http.patch(f"/admin/coupons/{enc(coupon_id)}", body=body, **kwargs)

    def delete_coupon(self, coupon_id: str, **kwargs: Any) -> Json:
        """Deactivate a coupon."""
        return self._http.delete(f"/admin/coupons/{enc(coupon_id)}", **kwargs)

    # Investors

    def create_investor(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create an investor."""
        return self._http.post("/admin/investors", body=body, **kwargs)

    def get_investor(self, investor_id: str, **kwargs: Any) -> Json:
        """Get an investor by id."""
        return self._http.get(f"/admin/investors/{enc(investor_id)}", **kwargs)

    def update_investor(
        self, investor_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Update an investor."""
        return self._http.put(f"/admin/investors/{enc(investor_id)}", body=body, **kwargs)

    def delete_investor(self, investor_id: str, **kwargs: Any) -> Json:
        """Delete an investor."""
        return self._http.delete(f"/admin/investors/{enc(investor_id)}", **kwargs)

    def get_investor_analytics(
        self, investor_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Get event analytics for an investor."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/admin/investors/{enc(investor_id)}/analytics", query=merged, **kwargs
        )

    def list_investor_events(
        self, investor_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List events for an investor, paginated."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/admin/investors/{enc(investor_id)}/events", query=merged, **kwargs
        )

    # Mascots

    def list_mascots(self, **kwargs: Any) -> Json:
        """List all mascots with provenance."""
        return self._http.get("/admin/mascots", **kwargs)

    def create_mascot(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Upload a custom Rive mascot."""
        return self._http.post("/admin/mascots", body=body, **kwargs)

    def update_mascot(self, id: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Update a custom mascot's metadata or binary."""
        return self._http.put(f"/admin/mascots/{enc(id)}", body=body, **kwargs)

    def delete_mascot(self, id: str, **kwargs: Any) -> Json:
        """Delete a custom mascot."""
        return self._http.delete(f"/admin/mascots/{enc(id)}", **kwargs)

    # Users

    def bulk_grant_user_credits(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Add credits to multiple users."""
        return self._http.post("/admin/users/credits/bulk", body=body, **kwargs)

    def get_admin_user(self, user_id: str, **kwargs: Any) -> Json:
        """Get a user's profile and credit balance."""
        return self._http.get(f"/admin/users/{enc(user_id)}", **kwargs)

    def get_user_channel_analytics(
        self, user_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Per-user messaging channel breakdown across telegram/discord/web."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/admin/users/{enc(user_id)}/analytics/channels", query=merged, **kwargs
        )

    def get_user_usage_analytics(
        self, user_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Per-user usage analytics — prompts, tokens, cost, tools."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/admin/users/{enc(user_id)}/analytics/usage", query=merged, **kwargs
        )

    def grant_user_credits(
        self, user_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Add or deduct a user's credits."""
        return self._http.post(
            f"/admin/users/{enc(user_id)}/credits", body=body, **kwargs
        )

    def list_user_credit_transactions(
        self, user_id: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List a user's credit transactions."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/admin/users/{enc(user_id)}/credits/transactions", query=merged, **kwargs
        )

    def grant_user_subscription(
        self, user_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Grant a paid subscription plan."""
        return self._http.post(
            f"/admin/users/{enc(user_id)}/subscription", body=body, **kwargs
        )

    def cancel_user_subscription(
        self, user_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Cancel a user's subscription and demote to free."""
        return self._http.delete(
            f"/admin/users/{enc(user_id)}/subscription", body=body, **kwargs
        )
