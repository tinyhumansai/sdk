import type { HttpClient, RequestOptions } from "../http.js";

export interface RedeemInviteBody {
  code: string;
}

export interface InviteStatusQuery {
  code: string;
}

/**
 * The current user's invite codes, redemption, and code-status checks.
 */
export class InviteApi {
  constructor(private readonly http: HttpClient) {}

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
