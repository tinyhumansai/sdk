import type { HttpClient, RequestOptions } from "../http.js";

export interface CreateCampaignInviteBody {
  maxUses: number;
  code?: string;
}

export interface RedeemInviteBody {
  code: string;
}

export interface InviteStatusQuery {
  code: string;
}

/**
 * Invite codes: campaign code management (admin), the current user's personal
 * codes, redemption, and code-status checks.
 */
export class InviteApi {
  constructor(private readonly http: HttpClient) {}

  /** Create a campaign invite code (admin only). */
  createCampaignInvite<T = unknown>(
    body: CreateCampaignInviteBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/invite/campaign", body, options);
  }

  /** List all campaign invite codes (admin only). */
  listCampaignInvites<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/invite/campaign", options);
  }

  /** Deactivate a campaign invite code (admin only). */
  deleteCampaignInvite<T = unknown>(codeId: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(`/invite/campaign/${encodeURIComponent(codeId)}`, options);
  }

  /** List the current user's invite codes with usage info. */
  listMyCodes<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/invite/my-codes", options);
  }

  /** Redeem an invite code. */
  redeemInvite<T = unknown>(body: RedeemInviteBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/invite/redeem", body, options);
  }

  /** Check if an invite code is valid and available. */
  getInviteStatus<T = unknown>(query: InviteStatusQuery, options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/invite/status", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }
}
