import { describe, expect, it } from "vitest";

import { ReferralApi } from "../src/api/referral.js";
import { mockClient } from "./helpers.js";

describe("ReferralApi", () => {
  it("claims a referral with a JSON body", async () => {
    const { http, last } = mockClient({ data: { claimed: true } });
    const api = new ReferralApi(http);

    await api.claimReferral({ code: "REF123" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/referral/claim");
    expect(call.body).toEqual({ code: "REF123" });
  });

  it("fetches referral stats", async () => {
    const { http, last } = mockClient({ data: { earnings: 0 } });
    const api = new ReferralApi(http);

    await api.getReferralStats();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/referral/stats");
  });
});
