from __future__ import annotations

from typing import Any

from ..http import Json
from ..types import ApiKeyCreate
from ._base import ApiNamespace, enc

__all__ = ["ApiKeysApi"]


class ApiKeysApi(ApiNamespace):
    """User-managed API keys."""

    def list(self, **kwargs: Any) -> Json:
        return self._http.get("/api-keys", **kwargs)

    def create(self, body: ApiKeyCreate, **kwargs: Any) -> Json:
        return self._http.post("/api-keys", body=body, **kwargs)

    def revoke(self, key_id: str, **kwargs: Any) -> Json:
        return self._http.delete(f"/api-keys/{enc(key_id)}", **kwargs)
