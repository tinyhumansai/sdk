from __future__ import annotations

from typing import Any


class RecordingHttp:
    """A duck-typed stand-in for HttpClient that records calls and returns
    canned responses (already unwrapped, as the real client would return)."""

    def __init__(self, responses: Any = None) -> None:
        # `responses` is a dict (repeated) or a list of dicts consumed in order.
        if responses is None:
            responses = [{}]
        self._responses = responses if isinstance(responses, list) else [responses]
        self.calls: list[dict[str, Any]] = []

    def request(
        self,
        method: str,
        path: str,
        *,
        query: dict[str, Any] | None = None,
        body: Any = None,
        headers: dict[str, str] | None = None,
        unwrap_envelope: bool | None = None,
    ) -> Any:
        self.calls.append(
            {
                "method": method.upper(),
                "path": path,
                "query": query,
                "body": body,
                "headers": headers,
                "unwrap_envelope": unwrap_envelope,
            }
        )
        index = min(len(self.calls) - 1, len(self._responses) - 1)
        spec = self._responses[index]
        if "body" in spec:
            return spec["body"]
        return spec.get("data")

    def get(self, path: str, **kwargs: Any) -> Any:
        return self.request("GET", path, **kwargs)

    def post(self, path: str, body: Any = None, **kwargs: Any) -> Any:
        return self.request("POST", path, body=body, **kwargs)

    def put(self, path: str, body: Any = None, **kwargs: Any) -> Any:
        return self.request("PUT", path, body=body, **kwargs)

    def patch(self, path: str, body: Any = None, **kwargs: Any) -> Any:
        return self.request("PATCH", path, body=body, **kwargs)

    def delete(self, path: str, **kwargs: Any) -> Any:
        return self.request("DELETE", path, **kwargs)

    @property
    def last(self) -> dict[str, Any]:
        return self.calls[-1]
