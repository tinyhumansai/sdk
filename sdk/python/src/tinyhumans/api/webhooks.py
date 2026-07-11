from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["WebhooksApi"]


class WebhooksApi(ApiNamespace):
    """Webhooks: inbound provider receivers plus user-managed webhook tunnels."""

    def receive_composio_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a Composio trigger webhook (HMAC-verified by the backend)."""
        return self._http.post("/webhooks/composio", body=body, **kwargs)

    def create_core_webhook(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a new webhook tunnel."""
        return self._http.post("/webhooks/core", body=body, **kwargs)

    def list_core_webhooks(self, **kwargs: Any) -> Json:
        """List the current user's webhook tunnels."""
        return self._http.get("/webhooks/core", **kwargs)

    def get_core_webhook_bandwidth(self, **kwargs: Any) -> Json:
        """Get the remaining USD budget available for webhook tunnel bandwidth."""
        return self._http.get("/webhooks/core/bandwidth", **kwargs)

    def get_core_webhook(self, id: str, **kwargs: Any) -> Json:
        """Get a specific webhook tunnel by id."""
        return self._http.get(f"/webhooks/core/{enc(id)}", **kwargs)

    def update_core_webhook(self, id: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Update a webhook tunnel."""
        return self._http.patch(f"/webhooks/core/{enc(id)}", body=body, **kwargs)

    def delete_core_webhook(self, id: str, **kwargs: Any) -> Json:
        """Delete a webhook tunnel."""
        return self._http.delete(f"/webhooks/core/{enc(id)}", **kwargs)

    def receive_discord_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a Discord interaction webhook (Ed25519-verified by the backend)."""
        return self._http.post("/webhooks/discord", body=body, **kwargs)

    def receive_github_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a GitHub webhook event (HMAC-SHA256-verified by the backend)."""
        return self._http.post("/webhooks/github", body=body, **kwargs)

    def forward_webhook_ingress(
        self, uuid: str, body: Any = None, **kwargs: Any
    ) -> Json:
        """Forward a payload to a tunnel via its public ingress endpoint."""
        return self._http.post(f"/webhooks/ingress/{enc(uuid)}", body=body, **kwargs)

    def forward_webhook_ingress_with_path(
        self, uuid: str, path: str, body: Any = None, **kwargs: Any
    ) -> Json:
        """Forward a payload to a tunnel with a downstream path suffix."""
        return self._http.post(
            f"/webhooks/ingress/{enc(uuid)}/{enc(path)}", body=body, **kwargs
        )

    def receive_coinbase_payment_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a Coinbase Commerce payment webhook."""
        return self._http.post("/webhooks/payments/coinbase", body=body, **kwargs)

    def receive_stripe_payment_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a Stripe payment webhook."""
        return self._http.post("/webhooks/payments/stripe", body=body, **kwargs)

    def receive_sentry_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a Sentry internal-integration webhook (HMAC-SHA256-verified)."""
        return self._http.post("/webhooks/sentry", body=body, **kwargs)

    def receive_telegram_webhook(self, body: Any, **kwargs: Any) -> Json:
        """Receive a Telegram webhook update for the master bot."""
        return self._http.post("/webhooks/telegram", body=body, **kwargs)

    def receive_managed_telegram_webhook(
        self, bot_id: str | int, body: Any, **kwargs: Any
    ) -> Json:
        """Receive a Telegram webhook update for a managed (per-user) bot."""
        return self._http.post(
            f"/webhooks/telegram/managed/{enc(bot_id)}", body=body, **kwargs
        )
