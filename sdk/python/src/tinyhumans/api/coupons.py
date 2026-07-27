from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["CouponsApi"]


class CouponsApi(ApiNamespace):
    """Coupon redemption and the current user's redeemed coupons."""

    def list_my_coupons(self, **kwargs: Any) -> Json:
        """List the current user's redeemed coupons."""
        return self._http.get("/coupons/me", **kwargs)

    def redeem_coupon(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Redeem a coupon code."""
        return self._http.post("/coupons/redeem", body=body, **kwargs)
