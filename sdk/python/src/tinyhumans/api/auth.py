from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["AuthApi"]


class AuthApi(ApiNamespace):
    """Auth and account state: magic-link login, OAuth, current user, and tokens."""

    def create_channel_link_token(self, channel: str, **kwargs: Any) -> Json:
        """Create a short-lived token to link a Telegram or Discord account."""
        return self._http.post(f"/auth/channels/{enc(channel)}/link-token", **kwargs)

    def send_email_link(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Send a magic link for email login."""
        return self._http.post("/auth/email/send-link", body=body, **kwargs)

    def verify_email(self, token: str, **kwargs: Any) -> Json:
        """Verify a magic link and complete email login (302 redirect)."""
        query = self._merge_query({"token": token}, kwargs.pop("query", None))
        return self._http.get("/auth/email/verify", query=query, **kwargs)

    def consume_login_token(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Consume a one-time login token, exchanging it for a session."""
        return self._http.post("/auth/login-token/consume", body=body, **kwargs)

    def me(self, **kwargs: Any) -> Json:
        """Get the currently authenticated user."""
        return self._http.get("/auth/me", **kwargs)

    def list_integrations(self, **kwargs: Any) -> Json:
        """List the user's connected third-party integrations."""
        return self._http.get("/auth/integrations", **kwargs)

    def delete_integration(self, integration_id: str, **kwargs: Any) -> Json:
        """Disconnect a third-party integration."""
        return self._http.delete(f"/auth/integrations/{enc(integration_id)}", **kwargs)

    def create_integration_token(
        self, integration_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Issue an access token for a connected integration."""
        return self._http.post(
            f"/auth/integrations/{enc(integration_id)}/tokens", body=body, **kwargs
        )

    def oauth_callback(self, provider: str, **kwargs: Any) -> Json:
        """OAuth provider callback endpoint (returns a redirect)."""
        return self._http.get(f"/auth/{enc(provider)}/callback", **kwargs)

    def oauth_connect(
        self, provider: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Start an OAuth connect flow for a provider."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(f"/auth/{enc(provider)}/connect", query=merged, **kwargs)

    def oauth_login(
        self, provider: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Start an OAuth login flow for a provider (returns a redirect)."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(f"/auth/{enc(provider)}/login", query=merged, **kwargs)
