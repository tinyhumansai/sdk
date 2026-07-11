import type { HttpClient, RequestOptions } from "../http.js";

export type CouponActivationType = "IMMEDIATE" | "CONDITIONAL";
export type CouponActivationCondition = "SUBSCRIBE_PAID_PLAN";

export interface CreateCouponBody {
  amountUsd: number;
  activationType?: CouponActivationType;
  activationCondition?: CouponActivationCondition;
  expiresAt?: string;
  expiryDays?: number;
  code?: string;
  metadata?: Record<string, unknown>;
}

export interface ListCouponsQuery {
  isActive?: boolean;
  redeemed?: boolean;
  activationType?: CouponActivationType;
  page?: number;
  limit?: number;
}

export interface RedeemCouponBody {
  /** Coupon code in ABCD-EFGH format. */
  code: string;
}

/**
 * Coupons: admin coupon creation and management, the current user's redeemed
 * coupons, and coupon redemption.
 */
export class CouponsApi {
  constructor(private readonly http: HttpClient) {}

  /** Create a coupon (admin only). */
  createCoupon<T = unknown>(body: CreateCouponBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/coupons/admin", body, options);
  }

  /** List all coupons (admin only). */
  listCoupons<T = unknown>(query?: ListCouponsQuery, options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/coupons/admin", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Deactivate a coupon (admin only). */
  deleteCoupon<T = unknown>(couponId: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(`/coupons/admin/${encodeURIComponent(couponId)}`, options);
  }

  /** List the current user's redeemed coupons. */
  listMyCoupons<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/coupons/me", options);
  }

  /** Redeem a coupon code. */
  redeemCoupon<T = unknown>(body: RedeemCouponBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/coupons/redeem", body, options);
  }
}
