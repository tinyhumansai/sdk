from __future__ import annotations

from typing import Any, Literal

from ..http import Json
from ..types import (
    MedullaSessionCreate,
    MedullaTaskCreate,
    MedullaTaskSourceCreate,
    MedullaTaskUpdate,
)
from ._base import ApiNamespace, enc

__all__ = ["MedullaApi"]


class MedullaApi(ApiNamespace):
    """Hosted Medulla sessions, routing, tasks, and task sources."""

    def create_session(self, body: MedullaSessionCreate, **kwargs: Any) -> Json:
        return self._http.post("/medulla/v1/sessions", body=body, **kwargs)

    def list_sessions(self, **kwargs: Any) -> Json:
        return self._http.get("/medulla/v1/sessions", **kwargs)

    def get_session(self, id: str, **kwargs: Any) -> Json:
        return self._http.get(f"/medulla/v1/sessions/{enc(id)}", **kwargs)

    def delete_session(self, id: str, **kwargs: Any) -> Json:
        return self._http.delete(f"/medulla/v1/sessions/{enc(id)}", **kwargs)

    def send_message(
        self, id: str, body: str, *, sync: bool | None = None, **kwargs: Any
    ) -> Json:
        query = {"sync": "1"} if sync else None
        return self._http.post(
            f"/medulla/v1/sessions/{enc(id)}/messages",
            body={"body": body},
            query=query,
            **kwargs,
        )

    def list_messages(
        self, id: str, *, after: int | None = None, **kwargs: Any
    ) -> Json:
        return self._http.get(
            f"/medulla/v1/sessions/{enc(id)}/messages",
            query={"after": after} if after is not None else None,
            **kwargs,
        )

    def list_events(self, id: str, *, after: int | None = None, **kwargs: Any) -> Json:
        return self._http.get(
            f"/medulla/v1/sessions/{enc(id)}/events",
            query={"after": after} if after is not None else None,
            **kwargs,
        )

    def abort_session(self, id: str, **kwargs: Any) -> Json:
        return self._http.post(f"/medulla/v1/sessions/{enc(id)}/abort", **kwargs)

    def stream_session(
        self,
        id: str,
        *,
        token: str | None = None,
        last_event_id: int | None = None,
        after: int | None = None,
        **kwargs: Any,
    ) -> Json:
        query = {
            key: value
            for key, value in {
                "token": token,
                "lastEventId": last_event_id,
                "after": after,
            }.items()
            if value is not None
        }
        return self._http.get(
            f"/medulla/v1/sessions/{enc(id)}/stream", query=query or None, **kwargs
        )

    def get_roster(self, **kwargs: Any) -> Json:
        return self._http.get("/medulla/v1/roster", **kwargs)

    def get_routing_strategy(self, **kwargs: Any) -> Json:
        return self._http.get("/medulla/v1/routing/strategy", **kwargs)

    def set_routing_strategy(
        self,
        strategy: Literal["manual", "balanced", "cpuFirst", "memoryFirst"],
        **kwargs: Any,
    ) -> Json:
        return self._http.put(
            "/medulla/v1/routing/strategy", body={"strategy": strategy}, **kwargs
        )

    def list_tasks(self, **kwargs: Any) -> Json:
        return self._http.get("/medulla/v1/tasks", **kwargs)

    def create_task(self, body: MedullaTaskCreate, **kwargs: Any) -> Json:
        return self._http.post("/medulla/v1/tasks", body=body, **kwargs)

    def list_task_sources(self, **kwargs: Any) -> Json:
        return self._http.get("/medulla/v1/tasks/sources", **kwargs)

    def create_task_source(self, body: MedullaTaskSourceCreate, **kwargs: Any) -> Json:
        return self._http.post("/medulla/v1/tasks/sources", body=body, **kwargs)

    def sync_task_source(self, id: str, **kwargs: Any) -> Json:
        return self._http.post(f"/medulla/v1/tasks/sources/{enc(id)}/sync", **kwargs)

    def delete_task_source(self, id: str, **kwargs: Any) -> Json:
        return self._http.delete(f"/medulla/v1/tasks/sources/{enc(id)}", **kwargs)

    def update_task(self, id: str, body: MedullaTaskUpdate, **kwargs: Any) -> Json:
        return self._http.patch(f"/medulla/v1/tasks/{enc(id)}", body=body, **kwargs)

    def delete_task(self, id: str, **kwargs: Any) -> Json:
        return self._http.delete(f"/medulla/v1/tasks/{enc(id)}", **kwargs)
