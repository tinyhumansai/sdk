import type { HttpClient, RequestOptions } from "../http.js";

export interface ClaimRewardBody {
  rewardType: string;
}

/**
 * Backend-backed rewards snapshot and connected-account management.
 */
export class RewardsApi {
  constructor(private readonly http: HttpClient) {}

  /** Disconnect (unlink) the authenticated user's Discord account. */
  unlinkDiscord<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.delete<T>("/rewards/discord", options);
  }

  /** Get the authenticated user's backend-backed rewards snapshot. */
  getMyRewards<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/rewards/me", options);
  }

  /** Claim an unlocked reward. Repeated claims are idempotent. */
  claimReward<T = unknown>(
    body: ClaimRewardBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/rewards/claim", body, options);
  }
}
