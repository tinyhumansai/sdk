import { describe, expect, it } from "vitest";

import { PaymentsApi } from "../src/api/payments.js";
import { mockClient } from "./helpers.js";

describe("PaymentsApi", () => {
  it("creates a Coinbase charge with a JSON body", async () => {
    const { http, last } = mockClient({ data: { hostedUrl: "https://pay" } });
    const api = new PaymentsApi(http);

    const result = await api.createCoinbaseCharge({ plan: "PRO", interval: "annual" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/payments/coinbase/charge");
    expect(call.body).toEqual({ plan: "PRO", interval: "annual" });
    expect(result).toEqual({ hostedUrl: "https://pay" });
  });

  it("gets a Coinbase charge, encoding the path and passing the sync query", async () => {
    const { http, last } = mockClient({ data: { status: "pending" } });
    const api = new PaymentsApi(http);

    await api.getCoinbaseCharge("charge/id 1", { sync: true });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/coinbase/charge/charge%2Fid%201");
    expect(call.query.sync).toEqual(["true"]);
  });

  it("gets auto-recharge settings", async () => {
    const { http, last } = mockClient({ data: { enabled: false } });
    const api = new PaymentsApi(http);

    await api.getAutoRecharge();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/credits/auto-recharge");
  });

  it("updates auto-recharge settings with a PATCH body", async () => {
    const { http, last } = mockClient({ data: { enabled: true } });
    const api = new PaymentsApi(http);

    await api.updateAutoRecharge({ enabled: true, thresholdUsd: 5, rechargeAmountUsd: 20 });

    const call = last();
    expect(call.method).toBe("PATCH");
    expect(call.path).toBe("/payments/credits/auto-recharge");
    expect(call.body).toEqual({ enabled: true, thresholdUsd: 5, rechargeAmountUsd: 20 });
  });

  it("lists saved auto-recharge cards", async () => {
    const { http, last } = mockClient({ data: { cards: [] } });
    const api = new PaymentsApi(http);

    await api.listAutoRechargeCards();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/credits/auto-recharge/cards");
  });

  it("creates an auto-recharge card SetupIntent", async () => {
    const { http, last } = mockClient({ data: { clientSecret: "seti_1" } });
    const api = new PaymentsApi(http);

    await api.createAutoRechargeCardSetupIntent();

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/payments/credits/auto-recharge/cards/setup-intent");
  });

  it("updates a saved card, encoding the path and sending the body", async () => {
    const { http, last } = mockClient({ data: { cards: [] } });
    const api = new PaymentsApi(http);

    await api.updateAutoRechargeCard("pm 1", { isDefault: true });

    const call = last();
    expect(call.method).toBe("PATCH");
    expect(call.path).toBe("/payments/credits/auto-recharge/cards/pm%201");
    expect(call.body).toEqual({ isDefault: true });
  });

  it("deletes a saved card, encoding the path", async () => {
    const { http, last } = mockClient({ data: { cards: [] } });
    const api = new PaymentsApi(http);

    await api.deleteAutoRechargeCard("pm 1");

    const call = last();
    expect(call.method).toBe("DELETE");
    expect(call.path).toBe("/payments/credits/auto-recharge/cards/pm%201");
  });

  it("unwraps the credit balance envelope", async () => {
    const { http, last } = mockClient({ data: { promotionBalanceUsd: 12.5 } });
    const api = new PaymentsApi(http);

    const result = await api.getCreditBalance();

    expect(last().path).toBe("/payments/credits/balance");
    expect(result).toEqual({ promotionBalanceUsd: 12.5 });
  });

  it("creates a credit top-up with a JSON body", async () => {
    const { http, last } = mockClient({ data: { url: "https://pay" } });
    const api = new PaymentsApi(http);

    await api.createCreditTopUp({ amountUsd: 25, gateway: "stripe" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/payments/credits/top-up");
    expect(call.body).toEqual({ amountUsd: 25, gateway: "stripe" });
  });

  it("handles the credit top-up cancel callback", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new PaymentsApi(http);

    await api.getCreditTopUpCancel();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/credits/top-up/cancel");
  });

  it("handles the credit top-up success callback with a session_id query", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new PaymentsApi(http);

    await api.getCreditTopUpSuccess({ session_id: "cs_123" });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/credits/top-up/success");
    expect(call.query.session_id).toEqual(["cs_123"]);
  });

  it("lists credit transactions with pagination query", async () => {
    const { http, last } = mockClient({ data: { transactions: [], total: 0 } });
    const api = new PaymentsApi(http);

    await api.listCreditTransactions({ limit: 10, offset: 20 });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/credits/transactions");
    expect(call.query.limit).toEqual(["10"]);
    expect(call.query.offset).toEqual(["20"]);
  });

  it("gets the Stripe checkout return page", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new PaymentsApi(http);

    await api.getStripeCheckoutReturn({ session_id: "cs_1", status: "cancel" });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/stripe/checkout/return");
    expect(call.query.session_id).toEqual(["cs_1"]);
    expect(call.query.status).toEqual(["cancel"]);
  });

  it("gets the current plan", async () => {
    const { http, last } = mockClient({ data: { plan: "PRO" } });
    const api = new PaymentsApi(http);

    await api.getCurrentPlan();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/stripe/currentPlan");
  });

  it("gets Stripe plans", async () => {
    const { http, last } = mockClient({ data: { plans: [], totalPlans: 0 } });
    const api = new PaymentsApi(http);

    await api.getStripePlans();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/stripe/plans");
  });

  it("creates a Stripe portal session", async () => {
    const { http, last } = mockClient({ data: { portalUrl: "https://portal" } });
    const api = new PaymentsApi(http);

    await api.createStripePortalSession();

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/payments/stripe/portal");
  });

  it("gets the Stripe portal return page", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new PaymentsApi(http);

    await api.getStripePortalReturn();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/payments/stripe/portal/return");
  });

  it("purchases a Stripe plan with a JSON body", async () => {
    const { http, last } = mockClient({ data: { checkoutUrl: "https://checkout" } });
    const api = new PaymentsApi(http);

    const result = await api.purchaseStripePlan({ plan: "PRO_YEARLY", couponCode: "SAVE10" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/payments/stripe/purchasePlan");
    expect(call.body).toEqual({ plan: "PRO_YEARLY", couponCode: "SAVE10" });
    expect(result).toEqual({ checkoutUrl: "https://checkout" });
  });
});
