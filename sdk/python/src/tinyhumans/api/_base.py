from __future__ import annotations

from typing import Any
from urllib.parse import quote

from ..http import HttpClient, Json

__all__ = ["ApiNamespace", "enc"]


def enc(value: Any) -> str:
    """Percent-encode a single path segment (parity with encodeURIComponent)."""
    return quote(str(value), safe="")


class ApiNamespace:
    """Base for typed namespace clients. Wraps a shared :class:`HttpClient`."""

    __slots__ = ("_http",)

    def __init__(self, http: HttpClient) -> None:
        self._http = http

    def _merge_query(
        self, query: dict[str, Any] | None, extra: dict[str, Any] | None
    ) -> dict[str, Any] | None:
        merged: dict[str, Any] = {}
        if query:
            merged.update({k: v for k, v in query.items() if v is not None})
        if extra:
            merged.update({k: v for k, v in extra.items() if v is not None})
        return merged or None
