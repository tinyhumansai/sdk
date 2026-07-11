import { describe, expect, it } from "vitest";

import { CouponsApi } from "../src/api/coupons.js";
import { mockClient } from "./helpers.js";

describe("CouponsApi", () => {
  it("creates a coupon with a JSON body", async () => {
    const { http, last } = mockClient({ status: 201, data: { id: "cpn_1" } });
    const api = new CouponsApi(http);

    const result = await api.createCoupon({ amountUsd: 5, activationType: "IMMEDIATE" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/coupons/admin");
    expect(call.body).toEqual({ amountUsd: 5, activationType: "IMMEDIATE" });
    expect(result).toEqual({ id: "cpn_1" });
  });

  it("lists coupons with query parameters", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new CouponsApi(http);

    await api.listCoupons({ isActive: true, page: 2, limit: 10 });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/coupons/admin");
    expect(call.query.isActive).toEqual(["true"]);
    expect(call.query.page).toEqual(["2"]);
    expect(call.query.limit).toEqual(["10"]);
  });

  it("encodes the couponId when deleting a coupon", async () => {
    const { http, last } = mockClient({ data: { deactivated: true } });
    const api = new CouponsApi(http);

    await api.deleteCoupon("cpn/1");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/coupons/admin/cpn%2F1");
  });

  it("lists the current user's coupons", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new CouponsApi(http);

    await api.listMyCoupons();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/coupons/me");
  });

  it("redeems a coupon with a JSON body", async () => {
    const { http, last } = mockClient({ data: { credited: 5 } });
    const api = new CouponsApi(http);

    const result = await api.redeemCoupon({ code: "ABCD-EFGH" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/coupons/redeem");
    expect(call.body).toEqual({ code: "ABCD-EFGH" });
    expect(result).toEqual({ credited: 5 });
  });
});
