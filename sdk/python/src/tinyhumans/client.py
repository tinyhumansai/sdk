from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from .http import HttpClient, Json


@dataclass(slots=True)
class NamespaceClient:
    http: HttpClient
    base_path: str

    def get(self, path: str = "/", **kwargs: Any) -> Json:
        return self.http.get(self._path(path), **kwargs)

    def post(self, path: str = "/", body: Any = None, **kwargs: Any) -> Json:
        return self.http.post(self._path(path), body=body, **kwargs)

    def put(self, path: str = "/", body: Any = None, **kwargs: Any) -> Json:
        return self.http.put(self._path(path), body=body, **kwargs)

    def patch(self, path: str = "/", body: Any = None, **kwargs: Any) -> Json:
        return self.http.patch(self._path(path), body=body, **kwargs)

    def delete(self, path: str = "/", **kwargs: Any) -> Json:
        return self.http.delete(self._path(path), **kwargs)

    def _path(self, path: str) -> str:
        suffix = "" if path == "/" else path if path.startswith("/") else f"/{path}"
        return f"{self.base_path}{suffix}" or "/"


class TinyHumansClient:
    def __init__(
        self,
        *,
        base_url: str,
        token: str | None = None,
        api_key: str | None = None,
        admin_service_token: str | None = None,
        headers: dict[str, str] | None = None,
        unwrap_envelope: bool = True,
        timeout: float = 30.0,
    ) -> None:
        self.raw = HttpClient(
            base_url=base_url,
            token=token,
            api_key=api_key,
            admin_service_token=admin_service_token,
            headers=headers,
            unwrap_envelope=unwrap_envelope,
            timeout=timeout,
        )
        self.api_keys = NamespaceClient(self.raw, "/api-keys")
        self.auth = NamespaceClient(self.raw, "/auth")
        self.inference = NamespaceClient(self.raw, "/openai")
        self.agent_integrations = NamespaceClient(self.raw, "/agent-integrations")
        self.payments = NamespaceClient(self.raw, "/payments")
        self.feedback = NamespaceClient(self.raw, "/feedback")
        self.teams = NamespaceClient(self.raw, "/teams")
        self.channels = NamespaceClient(self.raw, "/channels")
        self.mascots = NamespaceClient(self.raw, "/mascots")
        self.admin = NamespaceClient(self.raw, "/admin")
        self.announcements = NamespaceClient(self.raw, "/announcements")
        self.coupons = NamespaceClient(self.raw, "/coupons")
        self.invite = NamespaceClient(self.raw, "/invite")
        self.investors = NamespaceClient(self.raw, "/investors")
        self.referral = NamespaceClient(self.raw, "/referral")
        self.rewards = NamespaceClient(self.raw, "/rewards")
        self.webhooks = NamespaceClient(self.raw, "/webhooks")

    def health(self) -> Json:
        return self.raw.get("/")

    def swagger(self) -> Json:
        return self.raw.get("/swagger.json", unwrap_envelope=False)
