from __future__ import annotations

from typing import Any

from ..http import Json
from ..types import OrchestrationContinue, OrchestrationEventIngest, OrchestrationRun
from ._base import ApiNamespace, enc

__all__ = ["OrchestrationApi"]


class OrchestrationApi(ApiNamespace):
    """Device counterpart events, world state, and orchestration cycles."""

    def ingest_event(self, body: OrchestrationEventIngest, **kwargs: Any) -> Json:
        return self._http.post("/orchestration/v1/events", body=body, **kwargs)

    def update_world_diff(self, body: dict[str, Any], **kwargs: Any) -> Json:
        return self._http.post("/orchestration/v1/world-diff", body=body, **kwargs)

    def get_world_diff(self, session: str, **kwargs: Any) -> Json:
        return self._http.get(
            "/orchestration/v1/world-diff", query={"session": session}, **kwargs
        )

    def list_sessions(self, **kwargs: Any) -> Json:
        return self._http.get("/orchestration/v1/sessions", **kwargs)

    def list_messages(
        self, id: str, *, after: int | None = None, **kwargs: Any
    ) -> Json:
        return self._http.get(
            f"/orchestration/v1/sessions/{enc(id)}/messages",
            query={"after": after} if after is not None else None,
            **kwargs,
        )

    def get_session_state(self, id: str, **kwargs: Any) -> Json:
        return self._http.get(f"/orchestration/v1/sessions/{enc(id)}/state", **kwargs)

    def run(self, body: OrchestrationRun, **kwargs: Any) -> Json:
        return self._http.post("/orchestration/v1/run", body=body, **kwargs)

    def continue_run(self, body: OrchestrationContinue, **kwargs: Any) -> Json:
        return self._http.post("/orchestration/v1/run/continue", body=body, **kwargs)
