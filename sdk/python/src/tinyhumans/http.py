from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any
from urllib.error import HTTPError
from urllib.parse import urlencode, urljoin
from urllib.request import Request, urlopen


Json = dict[str, Any] | list[Any] | str | int | float | bool | None


class TinyHumansError(RuntimeError):
    def __init__(self, status: int, body: Any, message: str | None = None) -> None:
        self.status = status
        self.body = body
        super().__init__(message or f"TinyHumans request failed with HTTP {status}")


@dataclass(slots=True)
class HttpClient:
    base_url: str
    token: str | None = None
    api_key: str | None = None
    admin_service_token: str | None = None
    headers: dict[str, str] | None = None
    unwrap_envelope: bool = True
    timeout: float = 30.0

    def request(
        self,
        method: str,
        path: str,
        *,
        query: dict[str, Any] | None = None,
        body: Any = None,
        headers: dict[str, str] | None = None,
        unwrap_envelope: bool | None = None,
    ) -> Json:
        url = self._url(path, query)
        request_headers = self._headers(headers)
        data: bytes | None = None
        if body is not None:
            request_headers.setdefault("content-type", "application/json")
            data = body.encode("utf-8") if isinstance(body, str) else json.dumps(body).encode("utf-8")

        request = Request(url, data=data, headers=request_headers, method=method.upper())
        try:
            with urlopen(request, timeout=self.timeout) as response:
                parsed = _read_body(response.read(), response.headers.get_content_type())
                if response.status == 204:
                    return None
                return self._unwrap(parsed, unwrap_envelope)
        except HTTPError as error:
            parsed = _read_body(error.read(), error.headers.get_content_type())
            message = parsed.get("error") if isinstance(parsed, dict) else None
            raise TinyHumansError(error.code, parsed, str(message) if message else None) from error

    def get(self, path: str, **kwargs: Any) -> Json:
        return self.request("GET", path, **kwargs)

    def post(self, path: str, body: Any = None, **kwargs: Any) -> Json:
        return self.request("POST", path, body=body, **kwargs)

    def put(self, path: str, body: Any = None, **kwargs: Any) -> Json:
        return self.request("PUT", path, body=body, **kwargs)

    def patch(self, path: str, body: Any = None, **kwargs: Any) -> Json:
        return self.request("PATCH", path, body=body, **kwargs)

    def delete(self, path: str, **kwargs: Any) -> Json:
        return self.request("DELETE", path, **kwargs)

    def _url(self, path: str, query: dict[str, Any] | None) -> str:
        base = self.base_url.rstrip("/") + "/"
        relative = path.lstrip("/")
        url = urljoin(base, relative)
        if query:
            pairs: list[tuple[str, str]] = []
            for key, value in query.items():
                values = value if isinstance(value, list) else [value]
                pairs.extend((key, str(item)) for item in values if item is not None)
            if pairs:
                url = f"{url}?{urlencode(pairs)}"
        return url

    def _headers(self, headers: dict[str, str] | None) -> dict[str, str]:
        merged = {
            "accept": "application/json",
            "x-sdk-client": "tinyhumans-python",
            **(self.headers or {}),
            **(headers or {}),
        }
        if self.token:
            merged["authorization"] = f"Bearer {self.token}"
        if self.api_key:
            merged["x-api-key"] = self.api_key
        if self.admin_service_token:
            merged["x-admin-service-token"] = self.admin_service_token
        return merged

    def _unwrap(self, body: Json, unwrap_envelope: bool | None) -> Json:
        should_unwrap = self.unwrap_envelope if unwrap_envelope is None else unwrap_envelope
        if should_unwrap and isinstance(body, dict) and body.get("success") is True and "data" in body:
            return body["data"]
        return body


def _read_body(data: bytes, content_type: str | None) -> Json:
    if not data:
        return None
    text = data.decode("utf-8")
    if content_type == "application/json" or text[:1] in ("{", "["):
        try:
            return json.loads(text)
        except json.JSONDecodeError:
            pass
    return text
