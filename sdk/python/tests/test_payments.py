from __future__ import annotations

from tinyhumans.api.payments import PaymentsApi
from helpers import RecordingHttp


def test_create_coinbase_charge() -> None:
    http = RecordingHttp({"data": {"id": "chg_1"}})
    api = PaymentsApi(http)

    result = api.create_coinbase_charge({"plan": "PRO", "interval": "annual"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/payments/coinbase/charge"
    assert call["body"] == {"plan": "PRO", "interval": "annual"}
    assert result == {"id": "chg_1"}


def test_get_coinbase_charge() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_coinbase_charge("tx a/b", {"sync": True})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/payments/coinbase/charge/tx%20a%2Fb"
    assert call["query"] == {"sync": True}


def test_get_auto_recharge() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_auto_recharge()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/credits/auto-recharge"


def test_update_auto_recharge() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.update_auto_recharge({"enabled": True, "thresholdUsd": 5})

    call = http.last
    assert call["method"] == "PATCH"
    assert call["path"] == "/payments/credits/auto-recharge"
    assert call["body"] == {"enabled": True, "thresholdUsd": 5}


def test_list_auto_recharge_cards() -> None:
    http = RecordingHttp({"data": []})
    api = PaymentsApi(http)

    api.list_auto_recharge_cards()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/credits/auto-recharge/cards"


def test_create_auto_recharge_card_setup_intent() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.create_auto_recharge_card_setup_intent()

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/payments/credits/auto-recharge/cards/setup-intent"


def test_update_auto_recharge_card() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.update_auto_recharge_card("pm_1", {"isDefault": True})

    call = http.last
    assert call["method"] == "PATCH"
    assert call["path"] == "/payments/credits/auto-recharge/cards/pm_1"
    assert call["body"] == {"isDefault": True}


def test_delete_auto_recharge_card() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.delete_auto_recharge_card("pm_1")

    assert http.last["method"] == "DELETE"
    assert http.last["path"] == "/payments/credits/auto-recharge/cards/pm_1"


def test_get_credit_balance() -> None:
    http = RecordingHttp({"data": {"balance": 10}})
    api = PaymentsApi(http)

    assert api.get_credit_balance() == {"balance": 10}
    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/credits/balance"


def test_create_credit_top_up() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.create_credit_top_up({"amountUsd": 20, "gateway": "stripe"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/payments/credits/top-up"
    assert call["body"] == {"amountUsd": 20, "gateway": "stripe"}


def test_get_credit_top_up_cancel() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_credit_top_up_cancel()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/credits/top-up/cancel"


def test_get_credit_top_up_success() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_credit_top_up_success({"session_id": "cs_1"})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/payments/credits/top-up/success"
    assert call["query"] == {"session_id": "cs_1"}


def test_list_credit_transactions() -> None:
    http = RecordingHttp({"data": []})
    api = PaymentsApi(http)

    api.list_credit_transactions({"limit": 10, "offset": 0})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/payments/credits/transactions"
    assert call["query"] == {"limit": 10, "offset": 0}


def test_get_stripe_checkout_return() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_stripe_checkout_return({"status": "cancel"})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/payments/stripe/checkout/return"
    assert call["query"] == {"status": "cancel"}


def test_get_current_plan() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_current_plan()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/stripe/currentPlan"


def test_get_stripe_plans() -> None:
    http = RecordingHttp({"data": []})
    api = PaymentsApi(http)

    api.get_stripe_plans()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/stripe/plans"


def test_create_stripe_portal_session() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.create_stripe_portal_session()

    assert http.last["method"] == "POST"
    assert http.last["path"] == "/payments/stripe/portal"


def test_get_stripe_portal_return() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.get_stripe_portal_return()

    assert http.last["method"] == "GET"
    assert http.last["path"] == "/payments/stripe/portal/return"


def test_purchase_stripe_plan() -> None:
    http = RecordingHttp({"data": {}})
    api = PaymentsApi(http)

    api.purchase_stripe_plan({"plan": "PRO_MONTHLY", "couponCode": "SAVE"})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/payments/stripe/purchasePlan"
    assert call["body"] == {"plan": "PRO_MONTHLY", "couponCode": "SAVE"}
