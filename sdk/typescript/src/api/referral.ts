import type { HttpClient, RequestOptions } from "../http.js";

export interface ClaimReferralBody {
  /** Referral code extracted from the referral link. */
  code: string;
  /** Optional device fingerprint for abuse prevention. */
  deviceFingerprint?: string;
}

/**
 * Referral link claiming and earnings/referral-list statistics.
 */
export class ReferralApi {
  constructor(private readonly http: HttpClient) {}

  /** Claim a referral link for the authenticated user. */
  claimReferral<T = unknown>(
    body: ClaimReferralBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/referral/claim", body, options);
  }

  /** Fetch referral link, earnings summary, and referral list. */
  getReferralStats<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/referral/stats", options);
  }
}
