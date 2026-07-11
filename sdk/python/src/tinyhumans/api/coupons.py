from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["CouponsApi"]


class CouponsApi(ApiNamespace):
    """Coupons: admin creation and management, the user's redeemed coupons, and redemption."""

    def create_coupon(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a coupon (admin only)."""
        return self._http.post("/coupons/admin", body=body, **kwargs)

    def list_coupons(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List all coupons (admin only)."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get("/coupons/admin", query=merged, **kwargs)

    def delete_coupon(self, coupon_id: str, **kwargs: Any) -> Json:
        """Deactivate a coupon (admin only)."""
        return self._http.delete(f"/coupons/admin/{enc(coupon_id)}", **kwargs)

    def list_my_coupons(self, **kwargs: Any) -> Json:
        """List the current user's redeemed coupons."""
        return self._http.get("/coupons/me", **kwargs)

    def redeem_coupon(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Redeem a coupon code."""
        return self._http.post("/coupons/redeem", body=body, **kwargs)
