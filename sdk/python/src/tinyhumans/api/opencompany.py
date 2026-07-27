from __future__ import annotations

from typing import Any

from ..http import Json
from ..types import CustomDomain, OpenCompanyCreate
from ._base import ApiNamespace, enc

__all__ = ["OpenCompanyApi"]


class OpenCompanyApi(ApiNamespace):
    def list_instances(self, **kwargs: Any) -> Json:
        return self._http.get("/opencompany/instances", **kwargs)

    def create_instance(self, body: OpenCompanyCreate, **kwargs: Any) -> Json:
        return self._http.post("/opencompany/instances", body=body, **kwargs)

    def suspend(self, slug: str, **kwargs: Any) -> Json:
        return self._http.post(f"/opencompany/instances/{enc(slug)}/suspend", **kwargs)

    def resume(self, slug: str, **kwargs: Any) -> Json:
        return self._http.post(f"/opencompany/instances/{enc(slug)}/resume", **kwargs)

    def delete_instance(
        self, slug: str, *, purge_data: bool | None = None, **kwargs: Any
    ) -> Json:
        query = (
            {"purge_data": str(purge_data).lower()} if purge_data is not None else None
        )
        return self._http.delete(
            f"/opencompany/instances/{enc(slug)}", query=query, **kwargs
        )

    def set_custom_domain(self, slug: str, body: CustomDomain, **kwargs: Any) -> Json:
        return self._http.put(
            f"/opencompany/instances/{enc(slug)}/custom-domain", body=body, **kwargs
        )

    def delete_custom_domain(self, slug: str, **kwargs: Any) -> Json:
        return self._http.delete(
            f"/opencompany/instances/{enc(slug)}/custom-domain", **kwargs
        )

    def verify_custom_domain(self, slug: str, **kwargs: Any) -> Json:
        return self._http.post(
            f"/opencompany/instances/{enc(slug)}/custom-domain/verify", **kwargs
        )
