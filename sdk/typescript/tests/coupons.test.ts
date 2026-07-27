import { describe, expect, it } from "vitest";

import { CouponsApi } from "../src/api/coupons.js";
import { mockClient } from "./helpers.js";

describe("CouponsApi", () => {
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
