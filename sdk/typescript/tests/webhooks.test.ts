import { describe, expect, it } from "vitest";

import { WebhooksApi } from "../src/api/webhooks.js";
import { mockClient } from "./helpers.js";

describe("WebhooksApi", () => {
  it("receives a Composio webhook with a JSON body", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.receiveComposioWebhook({ type: "gmail.new" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/webhooks/composio");
    expect(call.body).toEqual({ type: "gmail.new" });
  });

  it("creates a webhook tunnel with a JSON body", async () => {
    const { http, last } = mockClient({ data: { id: "wt_1" } });
    const api = new WebhooksApi(http);

    const result = await api.createCoreWebhook({ name: "my tunnel", description: "d" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/webhooks/core");
    expect(call.body).toEqual({ name: "my tunnel", description: "d" });
    expect(result).toEqual({ id: "wt_1" });
  });

  it("lists webhook tunnels", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new WebhooksApi(http);

    await api.listCoreWebhooks();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/webhooks/core");
  });

  it("gets webhook tunnel bandwidth and unwraps the envelope", async () => {
    const { http, last } = mockClient({ data: { remainingBudgetUsd: 4.2 } });
    const api = new WebhooksApi(http);

    const result = await api.getCoreWebhookBandwidth();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/webhooks/core/bandwidth");
    expect(result).toEqual({ remainingBudgetUsd: 4.2 });
  });

  it("gets a webhook tunnel by id, encoding the path segment", async () => {
    const { http, last } = mockClient({ data: { id: "a/b" } });
    const api = new WebhooksApi(http);

    await api.getCoreWebhook("a/b");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/webhooks/core/a%2Fb");
  });

  it("updates a webhook tunnel with a JSON body", async () => {
    const { http, last } = mockClient({ data: { id: "wt_1" } });
    const api = new WebhooksApi(http);

    await api.updateCoreWebhook("wt_1", { isActive: false });

    const call = last();
    expect(call.method).toBe("PATCH");
    expect(call.path).toBe("/webhooks/core/wt_1");
    expect(call.body).toEqual({ isActive: false });
  });

  it("deletes a webhook tunnel", async () => {
    const { http, last } = mockClient({ data: { deleted: true } });
    const api = new WebhooksApi(http);

    await api.deleteCoreWebhook("wt_1");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/webhooks/core/wt_1");
  });

  it("receives a Discord webhook", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new WebhooksApi(http);

    await api.receiveDiscordWebhook({ type: 1 });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/webhooks/discord");
    expect(last().body).toEqual({ type: 1 });
  });

  it("receives a GitHub webhook", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.receiveGithubWebhook({ action: "published" });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/webhooks/github");
  });

  it("forwards a payload to a tunnel ingress endpoint", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.forwardWebhookIngress("uuid-1", { hello: "world" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/webhooks/ingress/uuid-1");
    expect(call.body).toEqual({ hello: "world" });
  });

  it("forwards a payload to a tunnel ingress endpoint with a path suffix", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.forwardWebhookIngressWithPath("uuid-1", "sub/path", { a: 1 });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/webhooks/ingress/uuid-1/sub%2Fpath");
    expect(call.body).toEqual({ a: 1 });
  });

  it("receives a Coinbase payment webhook", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.receiveCoinbasePaymentWebhook({ event: {} });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/webhooks/payments/coinbase");
  });

  it("receives a Stripe payment webhook", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.receiveStripePaymentWebhook({ type: "checkout.session.completed" });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/webhooks/payments/stripe");
  });

  it("receives a Sentry webhook", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.receiveSentryWebhook({ action: "created" });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/webhooks/sentry");
  });

  it("receives a Telegram webhook update", async () => {
    const { http, last } = mockClient({ data: { received: true } });
    const api = new WebhooksApi(http);

    const result = await api.receiveTelegramWebhook({ update_id: 1 });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/webhooks/telegram");
    expect(result).toEqual({ received: true });
  });

  it("receives a managed Telegram webhook update, encoding the bot id", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new WebhooksApi(http);

    await api.receiveManagedTelegramWebhook(12345, { update_id: 2 });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/webhooks/telegram/managed/12345");
    expect(call.body).toEqual({ update_id: 2 });
  });
});
