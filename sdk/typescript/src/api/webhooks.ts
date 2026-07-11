import type { HttpClient, RequestOptions } from "../http.js";

export interface CreateCoreWebhookBody {
  name: string;
  description?: string;
}

export interface UpdateCoreWebhookBody {
  name?: string;
  description?: string;
  isActive?: boolean;
}

export interface WebhookTunnelBandwidth {
  remainingBudgetUsd: number;
}

/**
 * Webhooks: inbound provider webhook receivers (Composio, Discord, GitHub,
 * Stripe, Coinbase, Sentry, Telegram) plus user-managed webhook tunnels and
 * their public ingress endpoints.
 */
export class WebhooksApi {
  constructor(private readonly http: HttpClient) {}

  /** Receive a Composio trigger webhook (HMAC-verified by the backend). */
  receiveComposioWebhook<T = unknown>(body: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/webhooks/composio", body, options);
  }

  /** Create a new webhook tunnel. */
  createCoreWebhook<T = unknown>(
    body: CreateCoreWebhookBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/webhooks/core", body, options);
  }

  /** List the current user's webhook tunnels. */
  listCoreWebhooks<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/webhooks/core", options);
  }

  /** Get the remaining USD budget available for webhook tunnel bandwidth. */
  getCoreWebhookBandwidth<T = WebhookTunnelBandwidth>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/webhooks/core/bandwidth", options);
  }

  /** Get a specific webhook tunnel by id. */
  getCoreWebhook<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/webhooks/core/${encodeURIComponent(id)}`, options);
  }

  /** Update a webhook tunnel. */
  updateCoreWebhook<T = unknown>(
    id: string,
    body: UpdateCoreWebhookBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(`/webhooks/core/${encodeURIComponent(id)}`, body, options);
  }

  /** Delete a webhook tunnel. */
  deleteCoreWebhook<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(`/webhooks/core/${encodeURIComponent(id)}`, options);
  }

  /** Receive a Discord interaction webhook (Ed25519-verified by the backend). */
  receiveDiscordWebhook<T = unknown>(body: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/webhooks/discord", body, options);
  }

  /** Receive a GitHub webhook event (HMAC-SHA256-verified by the backend). */
  receiveGithubWebhook<T = unknown>(body: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/webhooks/github", body, options);
  }

  /** Forward a payload to a tunnel via its public ingress endpoint. */
  forwardWebhookIngress<T = unknown>(
    uuid: string,
    body?: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(`/webhooks/ingress/${encodeURIComponent(uuid)}`, body, options);
  }

  /** Forward a payload to a tunnel with a downstream path suffix. */
  forwardWebhookIngressWithPath<T = unknown>(
    uuid: string,
    path: string,
    body?: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/webhooks/ingress/${encodeURIComponent(uuid)}/${encodeURIComponent(path)}`,
      body,
      options,
    );
  }

  /** Receive a Coinbase Commerce payment webhook. */
  receiveCoinbasePaymentWebhook<T = unknown>(
    body: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/webhooks/payments/coinbase", body, options);
  }

  /** Receive a Stripe payment webhook. */
  receiveStripePaymentWebhook<T = unknown>(
    body: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/webhooks/payments/stripe", body, options);
  }

  /** Receive a Sentry internal-integration webhook (HMAC-SHA256-verified). */
  receiveSentryWebhook<T = unknown>(body: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/webhooks/sentry", body, options);
  }

  /** Receive a Telegram webhook update for the master bot. */
  receiveTelegramWebhook<T = unknown>(body: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/webhooks/telegram", body, options);
  }

  /** Receive a Telegram webhook update for a managed (per-user) bot. */
  receiveManagedTelegramWebhook<T = unknown>(
    botId: string | number,
    body: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/webhooks/telegram/managed/${encodeURIComponent(String(botId))}`,
      body,
      options,
    );
  }
}
