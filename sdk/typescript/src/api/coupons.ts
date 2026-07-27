import type { HttpClient, RequestOptions } from "../http.js";

export interface RedeemCouponBody {
  /** Coupon code in ABCD-EFGH format. */
  code: string;
}

/**
 * The current user's redeemed coupons and coupon redemption.
 */
export class CouponsApi {
  constructor(private readonly http: HttpClient) {}

  /** List the current user's redeemed coupons. */
  listMyCoupons<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/coupons/me", options);
  }

  /** Redeem a coupon code. */
  redeemCoupon<T = unknown>(body: RedeemCouponBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/coupons/redeem", body, options);
  }
}
