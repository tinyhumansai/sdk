from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["ChannelsApi"]


class ChannelsApi(ApiNamespace):
    """Telegram and Discord channel integration: messages, reactions, threads, typing."""

    def send_message(self, channel: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Send a rich message to a user's linked channel."""
        return self._http.post(f"/channels/{enc(channel)}/messages", body=body, **kwargs)

    def delete_message(
        self, channel: str, message_id: str | int, **kwargs: Any
    ) -> Json:
        """Delete a message from a user's linked channel."""
        return self._http.delete(
            f"/channels/{enc(channel)}/messages/{enc(message_id)}", **kwargs
        )

    def add_reaction(self, channel: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """React to a message on a user's linked channel."""
        return self._http.post(
            f"/channels/{enc(channel)}/reactions", body=body, **kwargs
        )

    def create_thread(self, channel: str, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a new conversation thread."""
        return self._http.post(f"/channels/{enc(channel)}/threads", body=body, **kwargs)

    def list_threads(
        self, channel: str, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List conversation threads."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        return self._http.get(
            f"/channels/{enc(channel)}/threads", query=merged, **kwargs
        )

    def update_thread(
        self, channel: str, thread_id: str, body: dict[str, Any], **kwargs: Any
    ) -> Json:
        """Update a thread's status (close or reopen)."""
        return self._http.patch(
            f"/channels/{enc(channel)}/threads/{enc(thread_id)}", body=body, **kwargs
        )

    def send_typing(
        self, channel: str, body: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """Broadcast a typing indicator on a user's linked channel."""
        return self._http.post(f"/channels/{enc(channel)}/typing", body=body, **kwargs)
