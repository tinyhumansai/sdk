import type { HttpClient, RequestOptions } from "../http.js";

export type AnalyticsGranularity = "hour" | "day" | "week" | "month";

export type AdminAnnouncementSeverity = "INFO" | "WARNING" | "CRITICAL";

export type ProviderCategory =
  | "llm"
  | "embeddings"
  | "voice"
  | "integration"
  | "observability"
  | "search"
  | "infrastructure"
  | "other";

export interface DateRangeQuery {
  startDate?: string;
  endDate?: string;
}

export interface DashboardQuery {
  startDate?: string;
  endDate?: string;
  engagementThreshold?: number;
}

export interface ChannelAnalyticsQuery {
  startDate?: string;
  endDate?: string;
  granularity?: AnalyticsGranularity;
  topLimit?: number;
}

export interface HomeAnalyticsQuery {
  startDate?: string;
  endDate?: string;
  granularity?: AnalyticsGranularity;
}

export interface InferenceUsageQuery {
  startDate?: string;
  endDate?: string;
  granularity?: AnalyticsGranularity;
  topLimit?: number;
}

export interface LeaderboardQuery {
  startDate?: string;
  endDate?: string;
  sortBy?: "activeDays" | "prompts" | "totalTokens" | "costUsd";
  limit?: number;
  excludeTeamEmails?: boolean;
}

export interface ProviderCreditsQuery {
  category?: ProviderCategory;
}

export interface UserChannelAnalyticsQuery {
  startDate?: string;
  endDate?: string;
  granularity?: AnalyticsGranularity;
}

export interface UserUsageAnalyticsQuery {
  startDate?: string;
  endDate?: string;
  granularity?: AnalyticsGranularity;
}

export interface ListAnnouncementsQuery {
  isActive?: "true" | "false";
  page?: number;
  limit?: number;
}

export interface AdminAnnouncementCta {
  label?: string;
  url?: string;
}

export interface CreateAnnouncementBody {
  title: string;
  body: string;
  severity?: AdminAnnouncementSeverity;
  isActive?: boolean;
  startsAt?: string;
  expiresAt?: string;
  cta?: AdminAnnouncementCta;
}

export interface UpdateAnnouncementBody {
  title?: string;
  body?: string;
  severity?: AdminAnnouncementSeverity;
  isActive?: boolean;
  startsAt?: string;
  expiresAt?: string;
  cta?: AdminAnnouncementCta;
}

export interface ListAuditLogsQuery {
  page?: number;
  limit?: number;
  action?: string;
  adminId?: string;
  targetId?: string;
  from?: string;
  to?: string;
}

export interface AdminCreateCouponBody {
  [key: string]: unknown;
}

export interface BulkCreateCouponsBody {
  count?: number;
  amountUsd?: number;
  [key: string]: unknown;
}

export interface UpdateCouponBody {
  expiresAt?: string;
  isActive?: boolean;
}

export interface CreateInvestorBody {
  [key: string]: unknown;
}

export interface UpdateInvestorBody {
  [key: string]: unknown;
}

export interface ListInvestorEventsQuery {
  page?: number;
  limit?: number;
}

export interface MascotViewModelInput {
  name?: string;
  type?: "number" | "boolean" | "color" | "string" | "enum";
  description?: string;
}

export interface CreateMascotBody {
  /** lowercase alphanumeric/hyphen, max 64 chars */
  id: string;
  name: string;
  /** Drives client cache invalidation */
  version: string;
  description?: string;
  defaultState?: string;
  stateToPose?: Record<string, unknown>;
  viewModelInputs?: MascotViewModelInput[];
  /** Base64-encoded .riv binary */
  rivBase64: string;
}

export interface UpdateMascotBody {
  [key: string]: unknown;
}

export interface BulkGrantCreditsBody {
  userIds: string[];
  credits: number;
  reason?: string;
  idempotencyKey?: string;
}

export interface AdminListCreditTransactionsQuery {
  page?: number;
  limit?: number;
}

export interface GrantUserCreditsBody {
  action: "ADD" | "DEDUCT";
  amountUsd: number;
  reason: string;
  idempotencyKey?: string;
}

export interface GrantSubscriptionBody {
  plan: string;
  reason: string;
}

export interface CancelSubscriptionBody {
  reason: string;
}

/**
 * Admin-only routes: analytics dashboards, announcements, investors, coupons,
 * mascots, user credit/subscription management, and audit logs. These routes
 * authenticate via a bearer token or an x-admin-service-token header.
 */
export class AdminApi {
  constructor(private readonly http: HttpClient) {}

  // Analytics

  /** Get admin activity analytics from getActivity. */
  getAnalyticsActivity<T = unknown>(
    query?: DateRangeQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/activity", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Messaging channel analytics — per-channel volume, day-by-day series, and top users (admin only). */
  getAnalyticsChannels<T = unknown>(
    query?: ChannelAnalyticsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/channels", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Dashboard summary — DAU, MAU, events, API performance, feature usage, and engaged users (admin only). */
  getAnalyticsDashboard<T = unknown>(
    query?: DashboardQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/dashboard", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Get backend event analytics from getBackendEvents. */
  getAnalyticsBackendEvents<T = unknown>(
    query?: DateRangeQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/events/backend", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Recent paying users (subscriptions) and topups (admin only). */
  getAnalyticsFinancialsDetails<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/admin/analytics/financials/details", options);
  }

  /** Home dashboard metrics — signups, active users, prompts, tokens (admin only). */
  getAnalyticsHome<T = unknown>(
    query?: HomeAnalyticsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/home", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Inference usage analytics — time-bucketed prompts, tokens, cost, revenue, top users (admin only). */
  getAnalyticsInferenceUsage<T = unknown>(
    query?: InferenceUsageQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/inference/usage", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Active-user leaderboard ranked by active days, prompts, tokens, or cost (admin only). */
  getAnalyticsLeaderboard<T = unknown>(
    query?: LeaderboardQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/leaderboard", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Provider credit/limit health snapshot (admin only). */
  getAnalyticsProviderCredits<T = unknown>(
    query?: ProviderCreditsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/analytics/providers/credits", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  // Announcements

  /** List announcements (admin only). */
  listAnnouncements<T = unknown>(
    query?: ListAnnouncementsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/announcements", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Create an announcement (admin service token). */
  createAnnouncement<T = unknown>(
    body: CreateAnnouncementBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/admin/announcements", body, options);
  }

  /** Get an announcement by id (admin only). */
  getAnnouncement<T = unknown>(
    announcementId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/admin/announcements/${encodeURIComponent(announcementId)}`,
      options,
    );
  }

  /** Update an announcement (admin service token). */
  updateAnnouncement<T = unknown>(
    announcementId: string,
    body: UpdateAnnouncementBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/admin/announcements/${encodeURIComponent(announcementId)}`,
      body,
      options,
    );
  }

  /** Delete an announcement (admin service token). */
  deleteAnnouncement<T = unknown>(
    announcementId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/admin/announcements/${encodeURIComponent(announcementId)}`,
      options,
    );
  }

  // Management

  /** List admin audit-log entries (admin only). */
  listAuditLogs<T = unknown>(
    query?: ListAuditLogsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/admin/audit-logs", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  // Coupons

  /** Create a coupon (admin service token). */
  createCoupon<T = unknown>(
    body: AdminCreateCouponBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/admin/coupons", body, options);
  }

  /** Bulk-create coupons (admin service token). */
  bulkCreateCoupons<T = unknown>(
    body: BulkCreateCouponsBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/admin/coupons/bulk", body, options);
  }

  /** Update a coupon's expiry/active flag (admin service token). */
  updateCoupon<T = unknown>(
    couponId: string,
    body: UpdateCouponBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/admin/coupons/${encodeURIComponent(couponId)}`,
      body,
      options,
    );
  }

  /** Deactivate a coupon (admin service token). */
  deleteCoupon<T = unknown>(couponId: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(
      `/admin/coupons/${encodeURIComponent(couponId)}`,
      options,
    );
  }

  // Investors

  /** Create an investor (admin service token). */
  createInvestor<T = unknown>(
    body: CreateInvestorBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/admin/investors", body, options);
  }

  /** Get an investor by id (admin only). */
  getInvestor<T = unknown>(investorId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(
      `/admin/investors/${encodeURIComponent(investorId)}`,
      options,
    );
  }

  /** Update an investor (admin service token). */
  updateInvestor<T = unknown>(
    investorId: string,
    body: UpdateInvestorBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.put<T>(
      `/admin/investors/${encodeURIComponent(investorId)}`,
      body,
      options,
    );
  }

  /** Delete an investor (admin service token). */
  deleteInvestor<T = unknown>(investorId: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(
      `/admin/investors/${encodeURIComponent(investorId)}`,
      options,
    );
  }

  /** Get event analytics for an investor (admin only). */
  getInvestorAnalytics<T = unknown>(
    investorId: string,
    query?: DateRangeQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/admin/investors/${encodeURIComponent(investorId)}/analytics`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** List events for an investor, paginated (admin only). */
  listInvestorEvents<T = unknown>(
    investorId: string,
    query?: ListInvestorEventsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/admin/investors/${encodeURIComponent(investorId)}/events`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  // Mascots

  /** List all mascots with provenance (admin service token). */
  listMascots<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/admin/mascots", options);
  }

  /** Upload a custom Rive mascot (admin service token). */
  createMascot<T = unknown>(body: CreateMascotBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/admin/mascots", body, options);
  }

  /** Update a custom mascot's metadata or binary (admin service token). */
  updateMascot<T = unknown>(
    id: string,
    body: UpdateMascotBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.put<T>(`/admin/mascots/${encodeURIComponent(id)}`, body, options);
  }

  /** Delete a custom mascot (admin service token). */
  deleteMascot<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(`/admin/mascots/${encodeURIComponent(id)}`, options);
  }

  // Users

  /** Add credits to multiple users (admin service token). */
  bulkGrantUserCredits<T = unknown>(
    body: BulkGrantCreditsBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/admin/users/credits/bulk", body, options);
  }

  /** Get a user's profile and credit balance (admin only). */
  getAdminUser<T = unknown>(userId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/admin/users/${encodeURIComponent(userId)}`, options);
  }

  /** Per-user messaging channel breakdown across telegram/discord/web (admin only). */
  getUserChannelAnalytics<T = unknown>(
    userId: string,
    query?: UserChannelAnalyticsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/admin/users/${encodeURIComponent(userId)}/analytics/channels`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** Per-user usage analytics — prompts, tokens, cost, tools (admin only). */
  getUserUsageAnalytics<T = unknown>(
    userId: string,
    query?: UserUsageAnalyticsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/admin/users/${encodeURIComponent(userId)}/analytics/usage`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** Add or deduct a user's credits (admin service token). */
  grantUserCredits<T = unknown>(
    userId: string,
    body: GrantUserCreditsBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/admin/users/${encodeURIComponent(userId)}/credits`,
      body,
      options,
    );
  }

  /** List a user's credit transactions (admin only). */
  listUserCreditTransactions<T = unknown>(
    userId: string,
    query?: AdminListCreditTransactionsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/admin/users/${encodeURIComponent(userId)}/credits/transactions`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** Grant a paid subscription plan (admin service token). */
  grantUserSubscription<T = unknown>(
    userId: string,
    body: GrantSubscriptionBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/admin/users/${encodeURIComponent(userId)}/subscription`,
      body,
      options,
    );
  }

  /** Cancel a user's subscription and demote to free (admin service token). */
  cancelUserSubscription<T = unknown>(
    userId: string,
    body: CancelSubscriptionBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/admin/users/${encodeURIComponent(userId)}/subscription`,
      { ...options, body },
    );
  }
}
