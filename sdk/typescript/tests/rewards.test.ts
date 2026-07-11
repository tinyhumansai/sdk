import { describe, expect, it } from "vitest";

import { RewardsApi } from "../src/api/rewards.js";
import { mockClient } from "./helpers.js";

describe("RewardsApi", () => {
  it("unlinks the Discord account", async () => {
    const { http, last } = mockClient({ data: { unlinked: true } });
    const api = new RewardsApi(http);

    await api.unlinkDiscord();

    const call = last();
    expect(call.method).toBe("DELETE");
    expect(call.path).toBe("/rewards/discord");
  });

  it("gets the rewards snapshot", async () => {
    const { http, last } = mockClient({ data: { points: 10 } });
    const api = new RewardsApi(http);

    await api.getMyRewards();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/rewards/me");
  });
});
