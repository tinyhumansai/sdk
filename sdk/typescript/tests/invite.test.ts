import { describe, expect, it } from "vitest";

import { InviteApi } from "../src/api/invite.js";
import { mockClient } from "./helpers.js";

describe("InviteApi", () => {
  it("lists the current user's codes", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new InviteApi(http);

    await api.listMyCodes();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/invite/my-codes");
  });

  it("redeems an invite with a JSON body", async () => {
    const { http, last } = mockClient({ data: { redeemed: true } });
    const api = new InviteApi(http);

    await api.redeemInvite({ code: "ABCD" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/invite/redeem");
    expect(call.body).toEqual({ code: "ABCD" });
  });

  it("checks invite status with the code as a query parameter", async () => {
    const { http, last } = mockClient({ data: { valid: true } });
    const api = new InviteApi(http);

    await api.getInviteStatus({ code: "ABCD" });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/invite/status");
    expect(call.query.code).toEqual(["ABCD"]);
  });
});
