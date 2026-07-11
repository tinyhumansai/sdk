from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["PaymentsApi"]


class PaymentsApi(ApiNamespace):
    """Payments: Stripe subscriptions, Coinbase Commerce charges, credit balance
    and top-ups, transaction history, the customer portal, and auto-recharge."""

    def create_coinbase_charge(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a Coinbase Commerce charge."""
        return self._http.post("/payments/coinbase/charge", body=body, **kwargs)

    def get_coinbase_charge(
        self,
        gateway_transaction_id: str,
        query: dict[str, Any] | None = None,
        **kwargs: Any,
    ) -> Json:
        """Get charge status."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/payments/coinbase/charge/{enc(gateway_transaction_id)}",
            query=merged,
            **kwargs,
        )

    def get_auto_recharge(self, **kwargs: Any) -> Json:
        """Get Stripe auto-recharge settings for top-up credits."""
        return self._http.get("/payments/credits/auto-recharge", **kwargs)

    def update_auto_recharge(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Update Stripe auto-recharge settings for top-up credits."""
        return self._http.patch("/payments/credits/auto-recharge", body=body, **kwargs)

    def list_auto_recharge_cards(self, **kwargs: Any) -> Json:
        """List saved Stripe cards for auto recharge."""
        return self._http.get("/payments/credits/auto-recharge/cards", **kwargs)

    def create_auto_recharge_card_setup_intent(self, **kwargs: Any) -> Json:
        """Create a Stripe SetupIntent for adding a saved card."""
        return self._http.post(
            "/payments/credits/auto-recharge/cards/setup-intent", **kwargs
        )

    def update_auto_recharge_card(
        self,
        payment_method_id: str,
        body: dict[str, Any] | None = None,
        **kwargs: Any,
    ) -> Json:
        """Update a saved Stripe card for auto recharge."""
        return self._http.patch(
            f"/payments/credits/auto-recharge/cards/{enc(payment_method_id)}",
            body=body,
            **kwargs,
        )

    def delete_auto_recharge_card(
        self, payment_method_id: str, **kwargs: Any
    ) -> Json:
        """Delete a saved Stripe card for auto recharge."""
        return self._http.delete(
            f"/payments/credits/auto-recharge/cards/{enc(payment_method_id)}",
            **kwargs,
        )

    def get_credit_balance(self, **kwargs: Any) -> Json:
        """Get the current user's credit balance."""
        return self._http.get("/payments/credits/balance", **kwargs)

    def create_credit_top_up(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a credit top-up payment."""
        return self._http.post("/payments/credits/top-up", body=body, **kwargs)

    def get_credit_top_up_cancel(self, **kwargs: Any) -> Json:
        """Handle canceled credit top-up (callback)."""
        return self._http.get("/payments/credits/top-up/cancel", **kwargs)

    def get_credit_top_up_success(
        self, query: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Handle successful credit top-up (callback)."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/payments/credits/top-up/success", query=merged, **kwargs
        )

    def list_credit_transactions(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Get paginated credit transaction history."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/payments/credits/transactions", query=merged, **kwargs
        )

    def get_stripe_checkout_return(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Public Stripe Checkout redirect target (browser hand-off)."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            "/payments/stripe/checkout/return", query=merged, **kwargs
        )

    def get_current_plan(self, **kwargs: Any) -> Json:
        """Get current subscription plan for authenticated user."""
        return self._http.get("/payments/stripe/currentPlan", **kwargs)

    def get_stripe_plans(self, **kwargs: Any) -> Json:
        """Get all available subscription plans."""
        return self._http.get("/payments/stripe/plans", **kwargs)

    def create_stripe_portal_session(self, **kwargs: Any) -> Json:
        """Create a Stripe Customer Portal session."""
        return self._http.post("/payments/stripe/portal", **kwargs)

    def get_stripe_portal_return(self, **kwargs: Any) -> Json:
        """Stripe Customer Portal return page."""
        return self._http.get("/payments/stripe/portal/return", **kwargs)

    def purchase_stripe_plan(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a Stripe Checkout Session for subscription purchase."""
        return self._http.post("/payments/stripe/purchasePlan", body=body, **kwargs)
