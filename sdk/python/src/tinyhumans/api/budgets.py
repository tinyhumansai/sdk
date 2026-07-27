from __future__ import annotations

from typing import Any

from ..http import Json
from ..types import BudgetSeatCreate, BudgetSeatUpdate
from ._base import ApiNamespace, enc

__all__ = ["BudgetsApi"]


class BudgetsApi(ApiNamespace):
    """Harness subscription seats and usage."""

    def get(self, **kwargs: Any) -> Json:
        return self._http.get("/budgets", **kwargs)

    def connect_seat(self, body: BudgetSeatCreate, **kwargs: Any) -> Json:
        return self._http.post("/budgets/seats", body=body, **kwargs)

    def update_seat(self, seat_id: str, body: BudgetSeatUpdate, **kwargs: Any) -> Json:
        return self._http.patch(f"/budgets/seats/{enc(seat_id)}", body=body, **kwargs)

    def disconnect_seat(self, seat_id: str, **kwargs: Any) -> Json:
        return self._http.delete(f"/budgets/seats/{enc(seat_id)}", **kwargs)
