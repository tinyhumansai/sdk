from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace, enc

__all__ = ["RedirectApi"]


class RedirectApi(ApiNamespace):
    """Short-link resolution."""

    def resolve_redirect(self, code: str, **kwargs: Any) -> Json:
        """Redirect a short link code to its original URL (returns a 302 redirect)."""
        return self._http.get(f"/r/{enc(code)}", **kwargs)
