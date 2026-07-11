import { describe, expect, it } from "vitest";

import { InvestorsApi } from "../src/api/investors.js";
import { mockClient } from "./helpers.js";

describe("InvestorsApi", () => {
  it("gets an investor deck by slug, encoding the path", async () => {
    const { http, last } = mockClient({ data: { slug: "acme fund" } });
    const api = new InvestorsApi(http);

    await api.getInvestorPage("acme fund");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/investors/acme%20fund");
  });

  it("tracks an investor page event with a JSON body", async () => {
    const { http, last } = mockClient({ status: 201, data: { tracked: true } });
    const api = new InvestorsApi(http);

    await api.trackInvestorEvent("acme", { eventType: "DECK_VIEW", page: "hero" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/investors/acme/events");
    expect(call.body).toEqual({ eventType: "DECK_VIEW", page: "hero" });
  });
});
