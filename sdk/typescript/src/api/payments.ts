import type { HttpClient, RequestOptions } from "../http.js";

export type CoinbasePlan = "BASIC" | "PRO";
export type CoinbaseInterval = "annual";
export type CreditGateway = "stripe" | "coinbase";
export type StripePlan = "BASIC_MONTHLY" | "BASIC_YEARLY" | "PRO_MONTHLY" | "PRO_YEARLY";

export interface CreateCoinbaseChargeBody {
  /** Subscription plan (BASIC: $10/month, PRO: $25/month). */
  plan: CoinbasePlan;
  /** Billing interval (crypto payments are annual-only). */
  interval?: CoinbaseInterval;
  metadata?: Record<string, unknown>;
}

export interface GetCoinbaseChargeQuery {
  /** Sync the charge status with Coinbase Commerce before returning. */
  sync?: boolean;
}

export interface UpdateAutoRechargeBody {
  enabled?: boolean;
  thresholdUsd?: number;
  rechargeAmountUsd?: number;
}

export interface UpdateAutoRechargeCardBody {
  /** Mark this card as the default payment method. */
  isDefault?: boolean;
  billingDetails?: Record<string, unknown>;
  [key: string]: unknown;
}

export interface CreateCreditTopUpBody {
  /** Amount in USD to add as credits (capped by MAX_TOPUP_USD). */
  amountUsd: number;
  /** Payment gateway to use. */
  gateway?: CreditGateway;
}

export interface CreditTopUpSuccessQuery {
  session_id: string;
}

export interface ListCreditTransactionsQuery {
  limit?: number;
  offset?: number;
}

export interface StripeCheckoutReturnQuery {
  session_id?: string;
  status?: "cancel";
}

export interface PurchaseStripePlanBody {
  /** Subscription plan identifier. */
  plan: StripePlan;
  /** Optional promotion code (case-insensitive). */
  couponCode?: string;
}

/**
 * Payments: Stripe subscriptions, Coinbase Commerce charges, credit balance and
 * top-ups, transaction history, the customer portal, and auto-recharge settings.
 */
export class PaymentsApi {
  constructor(private readonly http: HttpClient) {}

  /** Create a Coinbase Commerce charge. */
  createCoinbaseCharge<T = unknown>(
    body: CreateCoinbaseChargeBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/payments/coinbase/charge", body, options);
  }

  /** Get charge status. */
  getCoinbaseCharge<T = unknown>(
    gatewayTransactionId: string,
    query?: GetCoinbaseChargeQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/payments/coinbase/charge/${encodeURIComponent(gatewayTransactionId)}`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** Get Stripe auto-recharge settings for top-up credits. */
  getAutoRecharge<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/credits/auto-recharge", options);
  }

  /** Update Stripe auto-recharge settings for top-up credits. */
  updateAutoRecharge<T = unknown>(
    body: UpdateAutoRechargeBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>("/payments/credits/auto-recharge", body, options);
  }

  /** List saved Stripe cards for auto recharge. */
  listAutoRechargeCards<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/credits/auto-recharge/cards", options);
  }

  /** Create a Stripe SetupIntent for adding a saved card. */
  createAutoRechargeCardSetupIntent<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      "/payments/credits/auto-recharge/cards/setup-intent",
      undefined,
      options,
    );
  }

  /** Update a saved Stripe card for auto recharge. */
  updateAutoRechargeCard<T = unknown>(
    paymentMethodId: string,
    body?: UpdateAutoRechargeCardBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/payments/credits/auto-recharge/cards/${encodeURIComponent(paymentMethodId)}`,
      body,
      options,
    );
  }

  /** Delete a saved Stripe card for auto recharge. */
  deleteAutoRechargeCard<T = unknown>(
    paymentMethodId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/payments/credits/auto-recharge/cards/${encodeURIComponent(paymentMethodId)}`,
      options,
    );
  }

  /** Get the current user's credit balance. */
  getCreditBalance<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/credits/balance", options);
  }

  /** Create a credit top-up payment. */
  createCreditTopUp<T = unknown>(
    body: CreateCreditTopUpBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/payments/credits/top-up", body, options);
  }

  /** Handle canceled credit top-up (callback). */
  getCreditTopUpCancel<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/credits/top-up/cancel", options);
  }

  /** Handle successful credit top-up (callback). */
  getCreditTopUpSuccess<T = unknown>(
    query: CreditTopUpSuccessQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/payments/credits/top-up/success", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Get paginated credit transaction history. */
  listCreditTransactions<T = unknown>(
    query?: ListCreditTransactionsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/payments/credits/transactions", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Public Stripe Checkout redirect target (browser hand-off). */
  getStripeCheckoutReturn<T = unknown>(
    query?: StripeCheckoutReturnQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/payments/stripe/checkout/return", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Get current subscription plan for authenticated user. */
  getCurrentPlan<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/stripe/currentPlan", options);
  }

  /** Get all available subscription plans. */
  getStripePlans<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/stripe/plans", options);
  }

  /** Create a Stripe Customer Portal session. */
  createStripePortalSession<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/payments/stripe/portal", undefined, options);
  }

  /** Stripe Customer Portal return page. */
  getStripePortalReturn<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/payments/stripe/portal/return", options);
  }

  /** Create a Stripe Checkout Session for subscription purchase. */
  purchaseStripePlan<T = unknown>(
    body: PurchaseStripePlanBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/payments/stripe/purchasePlan", body, options);
  }
}
